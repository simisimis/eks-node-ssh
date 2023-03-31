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

fn main() {
    let get_nodes_cmd = Command::new("sh")
        .arg("-c")
        .arg("kubectl get nodes -ojson")
        .output()
        .expect("failed executing ec2command");
    let nodes: Nodes =
        serde_json::from_slice(&get_nodes_cmd.stdout).expect("failed to Deserialize");
    let rez: Vec<String> = nodes
        .items
        .iter()
        .map(|v| v.spec.provider_id.clone())
        .collect();
    println!("{:#?}", rez);
}
