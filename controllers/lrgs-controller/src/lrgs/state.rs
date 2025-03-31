use kube::Client;
use serde::Serialize;
use serde_json::json;

// Context for our reconciler
#[derive(Clone)]
pub struct AppData {
    /// kubernetes client
    pub client: Client
}

impl AppData {
    pub fn metrics(&self) -> String {
        "test".into()
    }

    pub async fn diagnostics(&self) -> impl Serialize {
        json!("test diag")
    }
}