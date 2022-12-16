use anyhow::Result;
use bevy::prelude::*;
use jsonrpsee::ws_client::{WsClient, WsClientBuilder};
use log::info;
use std::sync::{Arc, RwLock};
use tictactoe_3d_api_lib::{get_local_server_url, RpcClient};
use tokio::{self, runtime::Runtime};

use crate::cloud::Client;

#[derive(Default)]
pub struct ClientData {
    pub get_name_available: bool,
}
#[derive(Resource, Clone)]
pub struct CloudClient {
    _runtime: Arc<Runtime>,
    connection: Arc<WsClient>,
    handle: tokio::runtime::Handle,
    pub data: Arc<RwLock<ClientData>>,
}

impl CloudClient {
    pub async fn connect() -> Result<WsClient> {
        let url = get_local_server_url();
        info!("Connecting to : {}", url);

        Ok(WsClientBuilder::default().build(url).await?)
    }

    pub fn get_name_available(&self, name: String) {
        if name.is_empty() {
            let mut data = self.data.write().unwrap();
            data.get_name_available = false;
            info!("Setting get_name_available : {}", data.get_name_available);
            return;
        }

        let mut clone = self.clone();
        let connection = self.connection.clone();

        self.handle.spawn(async move {
            let new_value = connection.get_name_available(&name).await.unwrap();

            let mut data = clone.data.write().unwrap();
            data.get_name_available = new_value;
            info!("Setting get_name_available : {}", data.get_name_available);
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
            data: Arc::new(RwLock::new(ClientData::default())),
        }
    }
}
