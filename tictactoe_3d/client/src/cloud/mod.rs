use anyhow::Result;
use jsonrpsee::ws_client::{WsClient, WsClientBuilder};
use tictactoe_3d_api_lib::{get_local_server_url};
use log::info;

pub struct Client {
    pub connection: WsClient,
}

impl Client {
    pub async fn connect() -> Result<Self> {
        let url = get_local_server_url();
        info!("Connecting to : {}", url);

        let connection = WsClientBuilder::default()
            .build(url)
            .await?;

        let client = Client { connection };
        Ok(client)
    }
}
