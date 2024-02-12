#![feature(extract_if)]

mod callback;
mod command;
mod dispatcher;
mod docker;
mod instance;
mod network;
mod proxy;
mod reaper;
mod registry;
mod tap;
mod vmm;

use crate::network::Network;
use callback::CallbackServer;
use dispatcher::Dispatcher;
use proxy::ProxyServer;
use registry::Registry;

use tracing::info;
use tracing_subscriber;

const CALLBACK_PORT: u16 = 3000;
const PROXY_PORT: u16 = 8123;
const MAX_WAITERS: usize = 1000;

#[tokio::main]
async fn main() {
    let args = command::args();

    tracing_subscriber::fmt()
        .with_max_level(args.log_lvl)
        .init();

    info!("starting up");

    let shutdown = tokio_util::sync::CancellationToken::new();

    let registry = Registry::new(&args.registry);
    let network = Network::new("10.100.0.0/16");

    let callback_server = CallbackServer::listen(CALLBACK_PORT);
    let proxy_server = ProxyServer::listen(PROXY_PORT, MAX_WAITERS, shutdown.clone());

    let dispatcher = Dispatcher::new(callback_server, proxy_server, network, registry);
    let handle = dispatcher.spawn();

    tokio::signal::ctrl_c().await.unwrap();

    info!("(ctrl-c) Shutting down...");
    shutdown.cancel();

    handle.await.unwrap();
    info!("Dispatcher has terminated");

    info!("Exiting");
}
