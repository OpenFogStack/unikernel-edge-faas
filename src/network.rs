use crate::tap::Tap;
use ipnet::Ipv4Net;
use std::{
    collections::VecDeque,
    net::Ipv4Addr,
    sync::{Arc, Mutex},
};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

pub struct NetLease {
    subnets_ref: Arc<Mutex<VecDeque<Ipv4Net>>>,
    pub netif: Tap,
    subnet: Ipv4Net,
}

impl NetLease {
    pub fn ifname(&self) -> &str {
        self.netif.name()
    }

    pub fn host_addr(&self) -> Ipv4Addr {
        let mut h = self.subnet.hosts().collect::<Vec<_>>();
        h.sort();
        h.get(0).unwrap().to_owned()
    }

    pub fn guest_addr(&self) -> Ipv4Addr {
        let mut h = self.subnet.hosts().collect::<Vec<_>>();
        h.sort();
        h.get(1).unwrap().to_owned()
    }

    pub fn netmask(&self) -> Ipv4Addr {
        self.subnet.netmask()
    }

    pub fn release(self) {
        debug!("Releasing interface {}", self.ifname());
        if let Err(e) = self.netif.remove() {
            error!("Failed to release interface: {}", e);
        }
        let mut subnets = self.subnets_ref.lock().unwrap();
        subnets.push_back(self.subnet);
    }
}

#[allow(dead_code)]
pub struct Network {
    subnets: Arc<Mutex<VecDeque<Ipv4Net>>>,
    receiver: async_channel::Receiver<NetLease>,
    shutdown: CancellationToken,
}

#[allow(dead_code)]
impl Network {
    async fn producer_task(
        channel: async_channel::Sender<NetLease>,
        subnets: Arc<Mutex<VecDeque<Ipv4Net>>>,
        shutdown: CancellationToken,
    ) {
        loop {
            let net: Ipv4Net = if let Ok(mut subnets) = subnets.lock() {
                // TODO: Error handling
                subnets.pop_front().unwrap()
            } else {
                continue;
            };

            let mut addrs = net.hosts().collect::<Vec<_>>();
            addrs.sort();
            let tap = Tap::create("faas%d", &addrs.get(0).unwrap(), &net.netmask()).unwrap();
            let lease = NetLease {
                subnets_ref: subnets.clone(),
                netif: tap,
                subnet: net,
            };

            info!(
                "Creating lease (tap={}, ip={})",
                lease.ifname(),
                lease.host_addr()
            );

            channel.send(lease).await.unwrap();

            if shutdown.is_cancelled() {
                break;
            }
        }

        debug!("Stopping netif allocation");
    }

    pub fn new(net: &str) -> Self {
        let net: Ipv4Net = net.parse().unwrap();
        let subnets = net.subnets(30).unwrap().collect::<VecDeque<Ipv4Net>>();

        info!("Allocating network with {} subnets", subnets.len());

        let subnets = Arc::new(Mutex::new(subnets));
        let (sender, receiver) = async_channel::bounded::<NetLease>(100);

        let shutdown = CancellationToken::new();

        tokio::spawn(Self::producer_task(
            sender,
            subnets.clone(),
            shutdown.clone(),
        ));

        Network {
            subnets,
            receiver,
            shutdown,
        }
    }

    pub async fn allocate(&self) -> Option<NetLease> {
        match self.receiver.recv().await {
            Ok(lease) => Some(lease),
            Err(_) => None,
        }
    }

    pub async fn destroy(&mut self) {
        // Draining remaining network interfaces
        info!("Destroying network. Removing network interfaces");
        self.shutdown.cancel();
        while let Some(netif) = self.allocate().await {
            netif.release();
        }
        info!("All network interfaces removed");
    }
}
