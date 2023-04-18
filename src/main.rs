use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Select};
use std::collections::HashMap;
use std::process::Command;

mod kubectl;
use kubectl::nodes::Nodes;

fn main() -> Result<()> {
    let nodes = Nodes::get_nodes()?;
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

    let someding = node_keys.get(selection.parse::<usize>().unwrap()).unwrap();
    let region = nodes_with_regions.get(&someding.to_string()).unwrap();
    let last_char = region.chars().last().unwrap(); // get the last character
    let region_trimmed = if last_char.is_alphabetic() {
        region.trim_end_matches(last_char)
    } else {
        region
    };
    let ppid_str = std::env::var("SID").unwrap().to_string();
    Command::new("sh")
        .arg("-c")
        .arg(format!("echo aws ssm start-session --region {region_trimmed} --target {someding} > /proc/{ppid_str}/fd/0"))
        .output()
        .expect("failed executing ec2command");
    Ok(())
}
