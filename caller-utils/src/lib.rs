wit_bindgen::generate!({
    path: "target/wit",
    world: "types-hyperprocess-app-template-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

/// Generated caller utilities for RPC function stubs

pub use hyperware_app_common::SendResult;
pub use hyperware_app_common::send;
use hyperware_process_lib::Address;
use serde_json::json;

// Import types from each interface
pub use crate::hyperware::process::hyperprocess::*;

/// Generated RPC stubs for the hyperprocess interface
pub mod hyperprocess {
    use crate::*;

    /// Generated stub for `add-to-state` local RPC call
    pub async fn add_to_state_local_rpc(target: &Address, key: String, value: String) -> SendResult<bool> {
        let request = json!({"AddToState": (key, value)});
        send::<bool>(&request, target, 30).await
    }
    
    /// Generated stub for `get-state` remote RPC call
    pub async fn get_state_remote_rpc(target: &Address) -> SendResult<Vec<(String, String)>> {
        let request = json!({"GetState" : {}});
        send::<Vec<(String, String)>>(&request, target, 30).await
    }
    
    /// Generated stub for `get-state` local RPC call
    pub async fn get_state_local_rpc(target: &Address) -> SendResult<Vec<(String, String)>> {
        let request = json!({"GetState" : {}});
        send::<Vec<(String, String)>>(&request, target, 30).await
    }
    
    // Generated stub for `get-state-http` http RPC call
    // HTTP endpoint - uncomment to implement
    // pub async fn get_state_http_http_rpc(_target: &str) -> SendResult<Vec<(String, String)>> {
    //     // TODO: Implement HTTP endpoint
    //     SendResult::Success(Vec::new())
    // }
    
    
}

