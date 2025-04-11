use hyperprocess_macro::hyperprocess;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use hyperware_process_lib::Address;
use caller_utils::task_scheduler::*;

// Task status and Task struct are now imported from caller-utils
// As they're generated from the WIT files

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TaskSchedulerState {
    tasks: HashMap<String, Task>,
    nodes: Vec<String>,
    our_node: Option<String>,
}

#[hyperprocess(
    name = "Task Scheduler",
    ui = Some(HttpBindingConfig::default()),
    endpoints = vec![
        Binding::Http {
            path: "/api",
            config: HttpBindingConfig::new(false, false, false, None),
        }, 
        Binding::Ws {
            path: "/ws",
            config: WsBindingConfig::new(false, false, false),
        }
    ],
    save_config = SaveOptions::EveryMessage,
    wit_world = "hyperprocess-app-template-dot-os-v0"
)]

impl TaskSchedulerState {
    #[init]
    async fn initialize(&mut self) {
        println!("Initializing Task Scheduler");
        self.tasks = HashMap::new();
        self.nodes = Vec::new();
        
        // In a real implementation, we would get our node ID from the system
        self.our_node = Some("current.node.id".to_string());
    }

    // Create a new task
    #[local]
    #[http]
    async fn create_task(&mut self, description: String) -> String {
        let task_id = Uuid::new_v4().to_string();
        let our_node = self.our_node.clone().unwrap_or_else(|| "unknown.node.id".to_string());
        
        let task = Task {
            id: task_id.clone(),
            description,
            assigned_node: None,
            status: TaskStatus::Pending,
            creator_node: our_node,
        };
        
        self.tasks.insert(task_id.clone(), task);
        task_id
    }
    
    // Get a specific task by ID
    #[local]
    #[remote]
    #[http]
    async fn get_task(&self, task_id: String) -> Option<Task> {
        self.tasks.get(&task_id).cloned()
    }
    
    // Get all tasks
    #[local]
    #[remote]
    #[http]
    async fn get_all_tasks(&self) -> Vec<Task> {
        self.tasks.values().cloned().collect()
    }
    
    // Delegate a task to another node
    #[local]
    #[http]
    async fn delegate_task(&mut self, task_id: String, target_node: String) -> bool {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.assigned_node = Some(target_node.clone());
            task.status = TaskStatus::Assigned;
            
            // Create an Address from the target node string
            let target_address = Address::from(target_node);
            
            // Get a clone of the task to send
            let task_clone = task.clone();
            
            // Use hyper-bindgen generated RPC stubs
            match assign_task_remote_rpc(&target_address, task_clone).await {
                caller_utils::SendResult::Success(_) => true,
                _ => {
                    // If the RPC fails, revert the task status
                    task.assigned_node = None;
                    task.status = TaskStatus::Pending;
                    false
                }
            }
        } else {
            false
        }
    }
    
    // Receive a task assignment from another node
    #[remote]
    async fn assign_task(&mut self, task: Task) -> bool {
        println!("Received task assignment: {}", task.id);
        self.tasks.insert(task.id.clone(), task);
        true
    }
    
    // Mark a task as completed
    #[local]
    #[http]
    async fn complete_task(&mut self, task_id: String) -> bool {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.status = TaskStatus::Completed;
            
            // Get the creator node address
            let creator_address = Address::from(task.creator_node.clone());
            
            // Send completion notification to creator node
            match notify_completion_remote_rpc(&creator_address, task_id.clone()).await {
                caller_utils::SendResult::Success(_) => true,
                _ => {
                    // If notification fails, we'll still mark it complete locally
                    // but log the failure
                    println!("Failed to notify creator node about task completion");
                    true
                }
            }
        } else {
            false
        }
    }
    
    // Receive a completion notification from another node
    #[remote]
    async fn notify_completion(&mut self, task_id: String) -> bool {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            println!("Received completion notification for task: {}", task_id);
            task.status = TaskStatus::Completed;
            true
        } else {
            println!("Received completion notification for unknown task: {}", task_id);
            false
        }
    }
}
