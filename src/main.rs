use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    // URL containing the desired URIs to download
    uri: String, 

    /// Optional dir
    #[arg(short, long, value_name = "FILE")]
    directory: Option<PathBuf>,
    
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Lists available files    
    List {
        #[arg(short, long)]
        uri: bool, // TODO: Replace w list of uri types
    },
}

fn main() {
    println!("Hello, world!");
    /*
    * Send request to URL for the html
    * Parse html, extract .stl/.3mf/.step uris
    * 
    *
    *
    * */
}

