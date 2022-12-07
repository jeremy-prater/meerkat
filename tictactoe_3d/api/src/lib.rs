use jsonrpsee::core::{Error};
use jsonrpsee::proc_macros::rpc;




const SERVER_PORT: &str = "14000";

pub fn get_server_port() -> String {
    SERVER_PORT.to_string()
}

pub fn get_server_url() -> String {
    "0.0.0.0:".to_string() + SERVER_PORT
}

#[rpc(server, client, namespace = "player")]
pub trait Rpc {
    #[method(name = "get_name_available")]
    async fn get_name_available(&self, name: String) -> Result<bool, Error>;

    #[method(name = "claim_name")]
    async fn claim_name(&self, name: String) -> Result<bool, Error>;

    // Subscription that takes a `StorageKey` as input and produces a `Vec<Hash>`.
    // #[subscription(name = "subscribeStorage" => "override", item = Vec<Hash>)]
    // fn subscribe_storage(&self, keys: Option<Vec<StorageKey>>);
}
