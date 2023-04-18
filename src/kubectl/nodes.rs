use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nodes {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub spec: Spec,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    #[serde(rename = "providerID")]
    pub provider_id: String,
}

impl Nodes {
    pub fn get_nodes() -> Result<Nodes> {
        let kubectl_nodes = Command::new("sh")
            .arg("-c")
            .arg("kubectl get nodes -ojson")
            .output()
            .expect("failed executing `kubectl get nodes`");
        let nodes = serde_json::from_slice(&kubectl_nodes.stdout)?;
        Ok(nodes)
    }
}
