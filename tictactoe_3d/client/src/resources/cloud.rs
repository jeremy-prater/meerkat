use anyhow::Result;
use bevy::prelude::*;
use jsonrpsee::ws_client::{WsClient, WsClientBuilder};
use log::info;
use std::sync::Arc;
use tictactoe_3d_api_lib::{get_local_server_url, RpcClient};
use tokio::{self, runtime::Runtime};

#[derive(Resource, Clone)]
pub struct CloudClient {
    _runtime: Arc<Runtime>,
    connection: Arc<WsClient>,
    handle: tokio::runtime::Handle,

    pub get_name_available: bool,
}

impl CloudClient {
    pub async fn connect() -> Result<WsClient> {
        let url = get_local_server_url();
        info!("Connecting to : {}", url);

        Ok(WsClientBuilder::default().build(url).await?)
    }

    pub fn get_name_available(&self, name: String) {
        let mut clone = self.clone();
        if name.is_empty() {
            clone.get_name_available = false;
            info!("Setting get_name_available : {}", clone.get_name_available);
            return;
        }

        self.handle.spawn(async move {
            let name = name.clone();
            clone.get_name_available = clone.connection.get_name_available(&name).await.unwrap();
            info!("Setting get_name_available : {}", clone.get_name_available);
        });
    }
}

impl Default for CloudClient {
    fn default() -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let connection = runtime.block_on(CloudClient::connect()).unwrap();
        let handle = runtime.handle().clone();

        CloudClient {
            _runtime: Arc::new(runtime),
            connection: Arc::new(connection),
            handle,
            get_name_available: false,
        }
    }
}
