use anyhow::Result;
use clap::{Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Select};
use std::collections::HashMap;

mod kubectl;
use kubectl::nodes::Nodes;

/// A tool to pregenerate some commands for k8s
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generates node ssh command
    Node { ssh: Option<String> },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

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
    let node_keys = nodes_with_regions.keys().cloned().collect::<Vec<String>>();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select node to ssh to")
        .default(0)
        .items(&node_keys[..])
        .interact()?
        .to_string();

    let someding = node_keys.get(selection.parse::<usize>()?).unwrap();
    let region = nodes_with_regions.get(&someding.to_string()).unwrap();
    let last_char = region.chars().last().unwrap();
    let region_trimmed = if last_char.is_alphabetic() {
        region.trim_end_matches(last_char)
    } else {
        region
    };
    match &cli.command {
        Commands::Node { .. } => {
            println!("aws ssm start-session --region {region_trimmed} --target {someding} ")
        }
    }
    Ok(())
}
