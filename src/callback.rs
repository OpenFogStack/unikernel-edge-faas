use axum::{
    extract::{Path, State},
    http::Request,
    routing, Router,
};
use hyper::body::Body;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tracing::{error, info};

type Waitlist = Arc<Mutex<HashMap<String, Arc<tokio::sync::Notify>>>>;

#[allow(dead_code)]
#[derive(Debug)]
pub struct CallbackServer {
    server_task: tokio::task::JoinHandle<()>,
    waiters: Waitlist,
}

impl CallbackServer {
    pub fn listen(port: u16) -> Self {
        let waiters = Arc::new(Mutex::new(HashMap::new()));
        let wc = waiters.clone();
        let server_task = tokio::spawn(async move {
            Self::start_callback_listener(wc, port).await;
        });

        Self {
            server_task,
            waiters,
        }
    }

    pub fn register(&self, instance_id: &str) -> Arc<tokio::sync::Notify> {
        let notify = Arc::new(tokio::sync::Notify::new());
        let mut waiters = self.waiters.lock().unwrap();

        let res = waiters.insert(instance_id.to_owned(), notify.clone());
        assert!(res.is_none());

        notify
    }

    #[allow(dead_code)]
    pub fn unregister(&self, instance_id: &str) -> Result<Arc<tokio::sync::Notify>, ()> {
        let mut waiters = self.waiters.lock().unwrap();

        match waiters.remove(instance_id) {
            Some(n) => Ok(n),
            None => Err(()),
        }
    }

    async fn start_callback_listener(waiters: Waitlist, port: u16) {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let app = Router::new()
            .route("/ready/:instance_id", routing::get(Self::ready_handler))
            .with_state(waiters);
        info!("Callback server listening on port {}", port);

        // Make sure we don't try to keep idle connections alive, because
        // by killing the vms connections don't get shutdown correctly
        // and we end up with a large number of open tcp connections and
        // eventually exceed the limit of file descriptors we can have open.
        axum::Server::bind(&addr)
            .http1_keepalive(false)
            .http2_keep_alive_interval(None)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }

    async fn ready_handler(
        Path(instance_id): Path<String>,
        State(waiters): State<Waitlist>,
        _req: Request<Body>,
    ) -> &'static str {
        let mut waiters = waiters.lock().unwrap();
        match waiters.remove(&instance_id) {
            Some(n) => {
                n.notify_one();
            }
            None => {
                error!("Spourious ready notification for instance {}", instance_id);
            }
        }

        "ok"
    }
}
