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
pub use crate::hyperware::process::task_scheduler::*;

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

/// Generated RPC stubs for the task_scheduler interface
pub mod task_scheduler {
    use crate::*;

    /// Generated stub for `create-task` local RPC call
    pub async fn create_task_local_rpc(target: &Address, description: String) -> SendResult<String> {
        let request = json!({"CreateTask": description});
        send::<String>(&request, target, 30).await
    }
    
    // Generated stub for `create-task` http RPC call
    // HTTP endpoint - uncomment to implement
    // pub async fn create_task_http_rpc(_target: &str, _description:  String) -> SendResult<String> {
    //     // TODO: Implement HTTP endpoint
    //     SendResult::Success(String::new())
    // }
    
    /// Generated stub for `get-task` remote RPC call
    pub async fn get_task_remote_rpc(target: &Address, task_id: String) -> SendResult<Option<Task>> {
        let request = json!({"GetTask": task_id});
        send::<Option<Task>>(&request, target, 30).await
    }
    
    /// Generated stub for `get-task` local RPC call
    pub async fn get_task_local_rpc(target: &Address, task_id: String) -> SendResult<Option<Task>> {
        let request = json!({"GetTask": task_id});
        send::<Option<Task>>(&request, target, 30).await
    }
    
    // Generated stub for `get-task` http RPC call
    // HTTP endpoint - uncomment to implement
    // pub async fn get_task_http_rpc(_target: &str, _task_id:  String) -> SendResult<Option<Task>> {
    //     // TODO: Implement HTTP endpoint
    //     SendResult::Success(None)
    // }
    
    /// Generated stub for `get-all-tasks` remote RPC call
    pub async fn get_all_tasks_remote_rpc(target: &Address) -> SendResult<Vec<Task>> {
        let request = json!({"GetAllTasks" : {}});
        send::<Vec<Task>>(&request, target, 30).await
    }
    
    /// Generated stub for `get-all-tasks` local RPC call
    pub async fn get_all_tasks_local_rpc(target: &Address) -> SendResult<Vec<Task>> {
        let request = json!({"GetAllTasks" : {}});
        send::<Vec<Task>>(&request, target, 30).await
    }
    
    // Generated stub for `get-all-tasks` http RPC call
    // HTTP endpoint - uncomment to implement
    // pub async fn get_all_tasks_http_rpc(_target: &str) -> SendResult<Vec<Task>> {
    //     // TODO: Implement HTTP endpoint
    //     SendResult::Success(Vec::new())
    // }
    
    /// Generated stub for `delegate-task` local RPC call
    pub async fn delegate_task_local_rpc(target: &Address, task_id: String, target_node: String) -> SendResult<bool> {
        let request = json!({"DelegateTask": (task_id, target_node)});
        send::<bool>(&request, target, 30).await
    }
    
    // Generated stub for `delegate-task` http RPC call
    // HTTP endpoint - uncomment to implement
    // pub async fn delegate_task_http_rpc(_target: &str, _task_id:  String, _target_node:  String) -> SendResult<bool> {
    //     // TODO: Implement HTTP endpoint
    //     SendResult::Success(false)
    // }
    
    /// Generated stub for `assign-task` remote RPC call
    pub async fn assign_task_remote_rpc(target: &Address, task: Task) -> SendResult<bool> {
        let request = json!({"AssignTask": task});
        send::<bool>(&request, target, 30).await
    }
    
    /// Generated stub for `complete-task` local RPC call
    pub async fn complete_task_local_rpc(target: &Address, task_id: String) -> SendResult<bool> {
        let request = json!({"CompleteTask": task_id});
        send::<bool>(&request, target, 30).await
    }
    
    // Generated stub for `complete-task` http RPC call
    // HTTP endpoint - uncomment to implement
    // pub async fn complete_task_http_rpc(_target: &str, _task_id:  String) -> SendResult<bool> {
    //     // TODO: Implement HTTP endpoint
    //     SendResult::Success(false)
    // }
    
    /// Generated stub for `notify-completion` remote RPC call
    pub async fn notify_completion_remote_rpc(target: &Address, task_id: String) -> SendResult<bool> {
        let request = json!({"NotifyCompletion": task_id});
        send::<bool>(&request, target, 30).await
    }
    
    
}

