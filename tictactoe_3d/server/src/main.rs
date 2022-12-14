use jsonrpsee::core::{async_trait, Error};
use jsonrpsee::server::ServerBuilder;
use log::info;
use std::collections::HashSet;
use std::default::Default;
use std::sync::{Arc, RwLock};

use tictactoe_3d_api_lib::{get_server_url, RpcServer};

pub struct RpcServerImpl {
    player_names: Arc<RwLock<HashSet<String>>>,
}

impl Default for RpcServerImpl {
    fn default() -> Self {
        info!("Creating RpcServer");
        let mut default_names: HashSet<String> = HashSet::new();
        default_names.insert("Jones".to_string());
        RpcServerImpl {
            player_names: Arc::new(RwLock::new(default_names)),
        }
    }
}

#[async_trait]
impl RpcServer for RpcServerImpl {
    async fn get_name_available(&self, name: &str) -> Result<bool, Error> {
        let lock = self.player_names.read().unwrap();
        let result = !lock.contains(name);
        info!("get_name_available : {} {}", name, result);
        Ok(result)
    }

    async fn claim_name(&self, name: &str) -> Result<bool, Error> {
        let mut lock = self.player_names.write().unwrap();
        let result = lock.insert(name.to_string());
        info!("claim_name : {} {}", name, result);
        Ok(result)
    }

    // Note that the server's subscription method must return `SubscriptionResult`.
    // fn subscribe_storage(
    //     &self,
    //     mut sink: SubscriptionSink,
    //     _keys: Option<Vec<ExampleStorageKey>>,
    // ) -> SubscriptionResult {
    //     let _ = sink.send(&vec![[0; 32]]);
    //     Ok(())
    // }
}

#[cfg(test)]
mod rpcserver_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_name_used() {
        let server = RpcServerImpl::default();
        assert_eq!(
            server
                .get_name_available("Jones".to_string())
                .await
                .unwrap(),
            false
        );
    }

    #[tokio::test]
    async fn test_get_name() {
        let server = RpcServerImpl::default();
        assert_eq!(
            server
                .get_name_available("NotJones".to_string())
                .await
                .unwrap(),
            true
        );
    }
    #[tokio::test]

    async fn test_set_name() {
        let server = RpcServerImpl::default();
        assert_eq!(
            server
                .get_name_available("NotJones".to_string())
                .await
                .unwrap(),
            true
        );
        assert_eq!(
            server.claim_name("NotJones".to_string()).await.unwrap(),
            true
        );
        assert_eq!(
            server
                .get_name_available("NotJones".to_string())
                .await
                .unwrap(),
            false
        );
        assert_eq!(
            server.claim_name("NotJones".to_string()).await.unwrap(),
            false
        );
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    meerkat_common::logging::init_logging()?;

    info!("3D tic tac toe server!");

    let server = ServerBuilder::default().build(get_server_url()).await?;

    let addr = server.local_addr()?;
    let handle = server.start(RpcServerImpl::default().into_rpc()).unwrap();

    info!("Using address : {}", addr);

    handle.stopped().await;

    Ok(())
}
