use hyperprocess_macro::hyperprocess;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct HyperprocessState {
    state: HashMap<String, String>,
}

#[hyperprocess(
    name = "Client",
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

impl HyperprocessState {
    #[init]
    async fn initialize(&mut self) {
        println!("init");
        self.state = HashMap::new();
    }

    // Local Hyperware request
    #[local]
    async fn add_to_state(&mut self, key: String, value: String) -> bool {
        self.state.insert(key, value);
        true
    }

    // Double annotation for endpoint accepting both local and remote Hyperware requests
    #[local]
    #[remote]   
    async fn get_state(&self) -> Vec<(String, String)> {
        self.state.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    // HTTP endpoint, will need to be a POST request on the frontend
    // to the /api endpoint
    #[http]
    async fn get_state_http(&self) -> Vec<(String, String)> {
        self.state.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

}
