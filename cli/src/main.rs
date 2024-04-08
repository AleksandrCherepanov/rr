use crate::cli::*;
use clap::Parser;

use cli::dot::cli_dot;
use cli::line::cli_line;
use cli::model::cli_model;
use cli::skeleton::cli_skeleton;
use cli::triangle::cli_triangle;

pub mod cli;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Dots(args) => {
            let result = cli_dot(args);
            match result {
                Ok(_) => (),
                Err(err) => println!("{}", err),
            }
        }
        Commands::Lines(args) => {
            let result = cli_line(args);
            match result {
                Ok(_) => (),
                Err(err) => println!("{}", err),
            }
        }
        Commands::Triangles(args) => {
            let result = cli_triangle(args);
            match result {
                Ok(_) => (),
                Err(err) => println!("{}", err),
            }
        }
        Commands::Skeleton(args) => {
            let result = cli_skeleton(args);
            match result {
                Ok(_) => (),
                Err(err) => println!("{}", err),
            }
        }
        Commands::Model(args) => {
            let result = cli_model(args);
            match result {
                Ok(_) => (),
                Err(err) => println!("{}", err),
            }
        }
    }
}
