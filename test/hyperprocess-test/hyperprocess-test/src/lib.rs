use crate::hyperware::process::tester::{FailResponse, Response as TesterResponse};
use caller_utils::task_scheduler::{
    create_task_local_rpc, get_task_local_rpc, get_all_tasks_local_rpc,
    delegate_task_local_rpc, complete_task_local_rpc,
    Task, TaskStatus
};
mod tester_lib;

use hyperware_app_common::SendResult;
use tester_lib::*;
use std::collections::HashMap;

async_test_suite!(
    "test-hyperprocess-app-template-dot-os-v0",

    test_basic_math: async {
        if 2 + 2 != 4 {
            fail!("wrong result");
        }
        Ok(())
    },

    // Test task creation
    test_create_task: async {
        let address: Address = ("hyperprocess.os", "task-scheduler", "task-scheduler", "template.os").into();
        let description = "Test task description".to_string();
        
        // Create a task
        let result = create_task_local_rpc(&address, description.clone()).await;
        match result {
            SendResult::Success(task_id) => {
                print_to_terminal(0, &format!("Created task with ID: {}", task_id));
                
                // Verify the task exists
                let task_result = get_task_local_rpc(&address, task_id.clone()).await;
                match task_result {
                    SendResult::Success(Some(task)) => {
                        if task.description != description {
                            fail!("Task description mismatch");
                        }
                        if task.status != TaskStatus::Pending {
                            fail!("Task status should be Pending");
                        }
                        if task.assigned_node.is_some() {
                            fail!("Task should not be assigned yet");
                        }
                        print_to_terminal(0, &format!("Task verified: {:?}", task));
                    },
                    SendResult::Success(None) => {
                        fail!("Task not found after creation");
                    },
                    _ => {
                        fail!("Failed to get task");
                    }
                }
            },
            _ => {
                fail!("Failed to create task");
            }
        }
        Ok(())
    },

    // Test get all tasks
    test_get_all_tasks: async {
        let address: Address = ("hyperprocess.os", "task-scheduler", "task-scheduler", "template.os").into();
        
        // Create a couple of tasks
        let desc1 = "First test task".to_string();
        let desc2 = "Second test task".to_string();
        
        let task_id1 = match create_task_local_rpc(&address, desc1).await {
            SendResult::Success(id) => id,
            _ => fail!("Failed to create first task")
        };
        
        let task_id2 = match create_task_local_rpc(&address, desc2).await {
            SendResult::Success(id) => id,
            _ => fail!("Failed to create second task")
        };
        
        // Get all tasks
        let result = get_all_tasks_local_rpc(&address).await;
        match result {
            SendResult::Success(tasks) => {
                print_to_terminal(0, &format!("Found {} tasks", tasks.len()));
                
                // Verify both tasks exist
                let task1_exists = tasks.iter().any(|t| t.id == task_id1);
                let task2_exists = tasks.iter().any(|t| t.id == task_id2);
                
                if !task1_exists || !task2_exists {
                    fail!("Not all created tasks were returned");
                }
            },
            _ => {
                fail!("Failed to get all tasks");
            }
        }
        
        Ok(())
    },

    // Testing task delegation would require two nodes, which is more complex for initial testing

);