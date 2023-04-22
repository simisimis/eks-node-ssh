use anyhow::Result;
use clap::{Parser, Subcommand};

mod kubectl;
use kubectl::nodes;

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
    /// Composes node ssh command
    Node { ssh: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    //    match &cli.command {
    //        Commands::Node { .. } => {
    //            println!("aws ssm start-session --region {region_trimmed} --target {someding} ")
    //        }
    //    }
    match &cli.command {
        Commands::Node { .. } => nodes::generate_node_ssh_cmd()?,
    };
    Ok(())
}
