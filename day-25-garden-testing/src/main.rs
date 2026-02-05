// ⛳️ Step 1 - Make sure everything works
// cargo add edit owo-colors slug dialoguer directories thiserror tempfile clap miette -F miette/fancy -F clap/derive -F clap/env

use clap::{CommandFactory, Parser, Subcommand, error::ErrorKind};
use directories::UserDirs;
use miette::Context;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[clap(version)]
struct Args {
    #[clap(short = 'p', long, env)]
    garden_path: Option<PathBuf>,

    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Write {
        #[clap(short, long)]
        title: Option<String>,
    },
}

fn get_default_garden_dir() -> Option<PathBuf> {
    UserDirs::new().map(|dirs| {
        dirs.home_dir()
            .join("Desktop/100-Days-of-Rust/day-25-garden-testing/garden")
    })
}

fn main() -> miette::Result<()> {
    let args = Args::parse();
    dbg!(args.clone());

    let Some(garden_path) = args.garden_path.or_else(get_default_garden_dir) else {
        let mut cmd = Args::command();
        cmd.error(
               ErrorKind::ValueValidation,
               format!(
                   "garden directory not provided and home directory unavailable for default garden directory"
               ),
           )
           .exit()
    };
    if !garden_path.exists() {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::ValueValidation,
            format!(
                "garden directory `{}` doesn't exist, or is inaccessible",
                garden_path.display()
            ),
        )
        .exit()
    };

    dbg!(garden_path.clone());
    match args.cmd {
        Commands::Write { title } => {
            // day_24_garden_errors::write(garden_path, title).into_diagnostic()
            day_25_garden_testing::write(garden_path, title).wrap_err("garden::write")
        }
    }
}

// cargo run -- write -t "My New Post"
