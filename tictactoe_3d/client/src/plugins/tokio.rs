use bevy::prelude::*;
use log::info;
use tictactoe_3d_api_lib::RpcClient;
pub struct TokioRuntime;

use crate::cloud;

impl TokioRuntime {
    pub fn test_fn() -> bool {
        true
    }
}

impl Plugin for TokioRuntime {
    fn build(&self, app: &mut App) {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                tokio_main(app).await;
            });
    }
}

async fn tokio_main(_app: &mut App) {
    info!("Hello from tokio runtime!");

    let client = cloud::Client::connect().await.unwrap();

    let jones = client.connection.get_name_available("Jones").await.unwrap();

    info!("Jones : {}", jones);
}
