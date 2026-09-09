use std::sync::Arc;
use crate::ForgeServer;

pub struct Runtime {
    server: Arc<ForgeServer>,
}

impl Runtime {
    pub fn new() -> Self {
        Self { server: Arc::new(ForgeServer::new()) }
    }

    pub async fn run(&self, addr: &str) -> anyhow::Result<()> {
        self.server.start(addr).await
    }

    pub fn server(&self) -> Arc<ForgeServer> {
        self.server.clone()
    }
}

impl Default for Runtime {
    fn default() -> Self { Self::new() }
}
