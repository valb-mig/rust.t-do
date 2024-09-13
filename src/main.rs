// Use Clap 'default' lib

use clap::{Parser, Subcommand};

use task::{StatusTask, Task};
pub mod task;

#[derive(Parser, Debug)]
#[command(version, about = "Task manager", long_about = None)]
struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug)]
enum Commands {
    Add(Task),
}

fn main() {

    let args = Args::parse();

    match args.command {
        Commands::Add(add) => {
            println!("add: {:?}", add);
        }
    }
}
