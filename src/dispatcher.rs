use crate::reaper::Reaper;
use crate::{callback::CallbackServer, proxy::ProxyServer};
use crate::{instance::Instance, network::Network, registry::Registry};
use axum::http;
use hyper::{body::Body, client::HttpConnector, Client, Uri};
use std::net::Ipv4Addr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
use tracing::{debug, info, warn};

type ResponseChannel = tokio::sync::oneshot::Sender<Result<http::Response<Body>, String>>;

#[derive(Debug)]
pub struct Invocation {
    pub function: String,
    pub path: String,
    pub request: http::Request<Body>,
    pub response: ResponseChannel,
}

#[allow(dead_code)]
impl Invocation {
    pub fn respond(self, res: Result<http::Response<Body>, String>) {
        let _ = self.response.send(res);
    }

    pub fn respond_err(self, err: String) {
        self.respond(Err(err));
    }

    pub fn respond_ok(self, res: http::Response<Body>) {
        self.respond(Ok(res));
    }
}

enum InstanceState {
    BOOTING,
    READY,
    ERROR,
}

struct FunctionInstance {
    function_name: String,
    instance: Instance,
    state: InstanceState,
    semaphore: Arc<Semaphore>,
    max_users: usize,
    keepalive: Duration,
    last_used: Instant,
}

impl FunctionInstance {
    fn try_acquire(&mut self) -> Result<InstanceHandle, ()> {
        InstanceHandle::try_from(self)
    }
}

struct InstanceHandle {
    permit: OwnedSemaphorePermit,
    instance_id: String,
    ip: Ipv4Addr,
    client: Client<HttpConnector>,
}

impl TryFrom<&mut FunctionInstance> for InstanceHandle {
    type Error = ();

    fn try_from(fi: &mut FunctionInstance) -> Result<Self, Self::Error> {
        let permit = fi.semaphore.clone().try_acquire_owned().map_err(|_| ())?;
        fi.last_used = Instant::now();

        Ok(Self {
            permit,
            instance_id: fi.instance.id(),
            ip: fi.instance.ip(),
            client: fi.instance.client(),
        })
    }
}

pub struct Dispatcher {
    callback_server: CallbackServer,
    proxy_server: ProxyServer,
    network: Network,
    registry: Registry,
    instances: Arc<Mutex<Vec<FunctionInstance>>>,
    reaper: Reaper,
}

impl Dispatcher {
    pub fn new(
        callback_server: CallbackServer,
        proxy_server: ProxyServer,
        network: Network,
        registry: Registry,
    ) -> Self {
        let instances = Arc::new(Mutex::new(Vec::new()));
        let reaper = Reaper::new();

        Self {
            callback_server,
            proxy_server,
            network,
            registry,
            instances,
            reaper,
        }
    }

    pub fn spawn(self) -> tokio::task::JoinHandle<()> {
        return self.dispatch_loop();
    }

    fn get_available_instance(&mut self, function_name: &str) -> Option<InstanceHandle> {
        let mut instances = self.instances.lock().unwrap();

        for instance in instances.iter_mut() {
            if instance.function_name != function_name {
                continue;
            }

            let InstanceState::READY = instance.state else {
                continue;
            };

            if let Ok(handle) = instance.try_acquire() {
                return Some(handle);
            }
        }

        return None;
    }

    async fn start_instance_and_forward(&mut self, iv: Invocation) {
        // Load function configuration from the registry
        let function = match self.registry.get_function(&iv.function) {
            Ok(function) => function,
            Err(error) => {
                iv.respond_err(error);
                return;
            }
        };

        // Acquire a nework lease i.e. a preconfigured tap interface for
        // this instance if necessary. This is only required for vms, because
        // docker containers rely on docker networking setup.
        let mut lease = None;
        if function.needs_netif() {
            lease = self.network.allocate().await;
            if lease.is_none() {
                iv.respond_err("Network allocation error".to_owned());
                return;
            }
        }

        let instance_id = uuid::Uuid::new_v4().to_string();

        // Register a notifier for this instance on the callback server.
        // This fires once the instance invokes the callback.
        let ready = self.callback_server.register(&instance_id);

        let instances = self.instances.clone();

        // Starting the instance is slow, because we need to start the
        // hypervisor process or talk to the docker daemon.
        // Move this off the main dispatch loop.
        tokio::spawn(async move {
            let start_time = Instant::now();
            let start_res = Instance::new(&instance_id, &function, lease)
                .await
                .map_err(|e| e.to_string());

            let instance = match start_res {
                Err(msg) => {
                    // TODO: cancel callback notifier
                    iv.respond_err(msg);
                    return;
                }
                Ok(instance) => instance,
            };

            let max_users = function.concurrent_requests;
            let mut fn_instance = FunctionInstance {
                function_name: iv.function.to_owned(),
                instance,
                state: InstanceState::BOOTING,
                semaphore: Arc::new(Semaphore::new(max_users)),
                max_users,
                keepalive: Duration::from_secs(function.keepalive),
                last_used: Instant::now(),
            };

            // This is safe because the semaphore has at least one permit
            let handle = fn_instance.try_acquire().unwrap();

            // If the function is single use i.e. sould only ever handle a
            // single request, we just close the semaphore here so no additional
            // permits can be acquired later on.
            if function.single_use {
                fn_instance.semaphore.close();
            }

            // Add instance to the list of available instances.
            instances.lock().unwrap().push(fn_instance);

            // Wait until instance has invoked ready callback
            if let Err(_) = tokio::time::timeout(Duration::from_secs(25), ready.notified()).await {
                warn!("Ready callback timeout expired for {}", instance_id);

                iv.respond_err("Ready callback timeout".to_owned());
                let mut instances = instances.lock().unwrap();

                // Mark instance as broken. This instance must still be in
                // the instance list as long as we hold the semaphore permit.
                instances
                    .iter_mut()
                    .find(|i| i.instance.id() == instance_id)
                    .unwrap()
                    .state = InstanceState::ERROR;

                return;
            }

            let boot_time = Instant::now() - start_time;
            info!(
                "Instance {} took {}ms to start",
                instance_id,
                boot_time.as_millis()
            );

            // Mark instance as ready
            instances
                .lock()
                .unwrap()
                .iter_mut()
                .find(|i| i.instance.id() == instance_id)
                .unwrap()
                .state = InstanceState::READY;

            forward_request(iv, handle).await;
        });
    }

    async fn dispatch(&mut self, iv: Invocation) {
        // Check if there is a running instance available to handle this
        // request, otherwise we need to spin up a new instance
        if let Some(handle) = self.get_available_instance(&iv.function) {
            info!("Reusing instance {}", handle.instance_id);
            tokio::spawn(forward_request(iv, handle));
        } else {
            self.start_instance_and_forward(iv).await;
        }
    }

    fn reap(&mut self, shutdown: bool) {
        let now = Instant::now();

        self.instances
            .lock()
            .unwrap()
            .extract_if(|i| {
                // If there are still users we cannot kill the instance
                if i.semaphore.available_permits() < i.max_users {
                    return false;
                }

                // Clean up broken instances
                if let InstanceState::ERROR = i.state {
                    debug!("Marking broken instance {} for deletion", i.instance.id());
                    return true;
                }

                // If instance has no users and keepalive expired kill it.
                // If we are shutting down we kill them all.
                if i.last_used + i.keepalive < now || shutdown {
                    debug!("Marking expired instance {} for deletion", i.instance.id());
                    return true;
                }

                return false;

                // TODO: kill more instances if load is high
            })
            .for_each(|i| self.reaper.reap_one(i.instance));
    }

    fn dispatch_loop(mut self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                info!(
                    "Dispatch loop iteration ({} instances running)",
                    self.instances.lock().unwrap().len()
                );

                // Don't wait forever for a new request, such that the reaper gets a
                // chance to run
                let t = Duration::from_secs(1);
                match tokio::time::timeout(t, self.proxy_server.receiver.recv()).await {
                    // Got a new request to handle.
                    Ok(Some(iv)) => self.dispatch(iv).await,

                    // The channel has been closed, which means the proxy has
                    // shutdown and there are no more requests, so we need to
                    // shutdown the dispatcher now.
                    Ok(None) => break,

                    // Timeout...moving on.
                    Err(_) => (),
                }

                // Check if we need to kill some instances
                self.reap(false);
            }

            info!("Dispatcher shutting down");

            // Terminate all running instances
            while !self.instances.lock().unwrap().is_empty() {
                self.reap(true);
                tokio::time::sleep(Duration::from_millis(20)).await;
            }

            // Wait for the reaper task to terminate. Once it has finished
            // all instances have been terminated.
            self.reaper.await_shutdown().await;

            info!("All instances have terminated");

            self.network.destroy().await;

            info!("Network is gone");
        })
    }
}

async fn forward_request(mut iv: Invocation, handle: InstanceHandle) {
    let mut uri = format!("http://{}:8080/{}", handle.ip, iv.path);
    if let Some(query) = iv.request.uri().query() {
        uri.push('?');
        uri.push_str(query);
    }

    *iv.request.uri_mut() = Uri::try_from(uri).unwrap();

    let res = handle
        .client
        .request(iv.request)
        .await
        .map_err(|e| format!("error: {}", e));

    let _ = iv.response.send(res);

    // We are done, so release semaphore held on handle
    drop(handle.permit);
}
