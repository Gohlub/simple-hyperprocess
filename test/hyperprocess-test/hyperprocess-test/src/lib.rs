use crate::hyperware::process::tester::{FailResponse, Response as TesterResponse};
use caller_utils::hyperprocess::{add_to_state_local_rpc, get_state_remote_rpc, get_state_local_rpc};
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

    // Test local add call
    test_local_add_call: async {
        let address: Address = ("hyperprocess.os", "hyperprocess", "hyperprocess", "template.os").into();
        let key = "Hello".to_string();
        let value = "World".to_string();
        let result = add_to_state_local_rpc(&address, key, value).await;
        print_to_terminal(0, &format!("add_to_state_local_rpc result: {:?}", result));
        Ok(())
    },

    // Test local get call
    test_local_get_call: async {
        let address: Address = ("hyperprocess.os", "hyperprocess", "hyperprocess", "template.os").into();
        let result = get_state_local_rpc(&address).await;
        print_to_terminal(0, &format!("get_state_local_rpc result: {:?}", result));
        Ok(())
    },

    test_remote_get_call: async {
        let address: Address = ("hyperprocess.os", "hyperprocess", "hyperprocess", "template.os").into();
        // test_remote_call is a helper function that tests a remote call
        // it takes a function, a expected result, and an error message
        // it will fail if the result is not as expected
        let expected_data = vec![("Hello".to_string(), "World".to_string())];
        let result = test_remote_call(
            get_state_remote_rpc(&address),
            expected_data,
            "wrong remote result"
        ).await?;
        print_to_terminal(0, &format!("remote_api_remote_rpc result: {:?}", result));
        Ok(())
    },

);