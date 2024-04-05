use crate::cli::*;
use clap::Parser;

pub mod cli;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Dots(args) => {
            println!("{:?}", args.source);
            println!("{:?}", args.target);
        }
        Commands::Lines(args) => {
            println!("{:?}", args.source);
            println!("{:?}", args.target);
        }
        Commands::Triangles(args) => {
            println!("{:?}", args.source);
            println!("{:?}", args.target);
        }
        Commands::Skeleton(args) => {
            println!("{:?}", args.source);
            println!("{:?}", args.target);
        }
        Commands::Model(args) => {
            println!("{:?}", args.source);
            println!("{:?}", args.target);
            if let Some(texture) = args.texture.as_deref() {
                println!("{:?}", texture);
            }
        }
    }
}
