use crate::cli::*;
use clap::Parser;

use cli::dot::cli_dot;
use cli::line::cli_line;
use cli::skeleton::cli_skeleton;
use cli::triangle::cli_triangle;

pub mod cli;

fn main() {
    let cli = Cli::parse();
    //{
    //command: Commands::Triangles(TrianglesArgs {
    //width: Some(100),
    //height: Some(100),
    //target: format!("../examples/triangles/target.tga"),
    //source: format!("../examples/triangles/source.txt"),
    //filled: true
    //})
    //};

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
            println!("{:?}", args.source);
            println!("{:?}", args.target);
            if let Some(texture) = args.texture.as_deref() {
                println!("{:?}", texture);
            }
        }
    }
}
