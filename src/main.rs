use dialoguer::{theme::ColorfulTheme, Select};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    let nodes: Vec<String> = nodes
        .items
        .iter()
        .map(|v| v.spec.provider_id.clone())
        .collect();
    let mut nodes_with_regions = HashMap::new();

    for s in nodes {
        let (node_id, node_region) = s
            .split('/')
            .nth(4)
            .zip(s.split('/').nth(3))
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .unwrap();
        nodes_with_regions.insert(node_id, node_region);
    }
    println!("{:#?}", nodes_with_regions);
    let node_keys = nodes_with_regions.keys().cloned().collect::<Vec<String>>();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select node to ssh to")
        .default(0)
        .items(&node_keys[..])
        .interact()
        .unwrap()
        .to_string();

    let someding = node_keys.get(selection.parse::<usize>().unwrap());
    println!(
        "You're about to ssh somewhere in {}!",
        nodes_with_regions
            .get(&someding.unwrap().to_string())
            .unwrap()
    );
}
