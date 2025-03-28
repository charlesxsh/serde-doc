
use core::str;

use clap::{Parser, Subcommand};
use anyhow::Result;
use serde_doc::generators::get_generator;

#[derive(Parser)]
#[command(
    version = "0.1.0",
    about = "A cargo extension CLI for generating documentation for serde structs",
    override_usage = "cargo serde-doc [OPTIONS] <COMMAND> [ARGS]",
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to the Cargo.toml file or directory containing it
    #[arg(short, long, default_value = ".")]
    manifest_path: String,
}

#[derive(Subcommand)]
enum Commands {
    /// List available serde structs
    List,
    /// Generate files using a generator
    Gen {
        /// Name of the generator to use
        generator: String,
        /// Destination path for the generated files
        #[arg(short, long)]
        output: Option<String>,

        /// List of structs to generate
        /// If not provided, all structs will be generated
        #[arg(short, long)]
        structs: Option<Vec<String>>,

        /// List of files to be included
        /// If not provided, all files will be included
        #[arg(short, long)]
        files: Option<Vec<String>>,
    },

}


fn main() -> Result<()> {
    let mut args = std::env::args().collect::<Vec<_>>();

    // This handles running as `cargo serde-doc ...`
    if args.get(1).map(String::as_str) == Some("serde-doc") {
        args.remove(1); // Remove the subcommand part
    }

    let cli = Cli::parse_from(args);


    match &cli.command {
        Commands::List => {
            handle_list(cli)
        }
        Commands::Gen { generator: _, output: _, files:_, structs:_ } => {
            handle_gen(cli)
        }
    }

    
}

fn handle_list(args: Cli) -> Result<()> {
    let mut ctx = serde_doc::Context::new();
    serde_doc::extract::process_path(&mut ctx, args.manifest_path)?;
    let structs = ctx.files.iter()
        .flat_map(|file| file.structs.iter())
        .collect::<Vec<_>>();
    for unit in structs {
        println!("{}", unit.name);
    }
    Ok(())
}

fn handle_gen(args: Cli) -> Result<()> {
    let mut ctx = serde_doc::Context::new();
    serde_doc::extract::process_path(&mut ctx, args.manifest_path)?;
    match args.command {
        Commands::Gen { generator, output, files, structs } => {
            let generator = get_generator(&generator)?;

            let config = serde_doc::generators::GeneratorConfig {
                output,
                structs,
                files
            };

            generator.generate(&ctx, &config)?;
        }
        _ => {
            return Err(anyhow::anyhow!("Invalid command"));
        }
    };
    
    Ok(())
}