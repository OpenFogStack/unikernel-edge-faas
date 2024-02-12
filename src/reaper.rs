use std::sync::Arc;

use crate::instance::Instance;
use tokio::sync::{mpsc, Semaphore};

pub struct Reaper {
    sender: mpsc::UnboundedSender<Instance>,
    handle: tokio::task::JoinHandle<()>,
}

impl Reaper {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        let handle = tokio::spawn(Self::reap_loop(receiver));

        Self { sender, handle }
    }

    pub async fn await_shutdown(self) {
        drop(self.sender);
        self.handle.await.unwrap();
    }

    async fn reap_loop(mut receiver: mpsc::UnboundedReceiver<Instance>) {
        // We try to do this in parallel because this might take a while
        let n: u32 = 50;
        let semaphore = Arc::new(Semaphore::new(n as usize));
        while let Some(instance) = receiver.recv().await {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            tokio::spawn(async move {
                instance.kill().await;
                drop(permit);
            });
        }

        let _ = semaphore.acquire_many(n).await.unwrap();
    }

    pub fn reap_one(&self, instance: Instance) {
        self.sender.send(instance).unwrap();
    }
}
