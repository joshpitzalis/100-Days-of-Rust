// ⛳️ Step 1 - Install dependancies and get everything running
// cargo add edit owo-colors slug dialoguer directories clap --features clap/derive --features clap/env

use clap::{CommandFactory, Parser, Subcommand, error::ErrorKind};
use directories::UserDirs;
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
    /// write something in your garden
    ///
    /// This command will open your $EDITOR, wait for you
    /// to write something, and then save the file to your
    /// garden
    Write {
        /// Optionally set a title for what you are going to write about
        #[clap(short, long)]
        title: Option<String>,
    },
}

/// Get the user's garden directory, which by default
/// is placed in their home directory
fn get_default_garden_dir() -> Option<PathBuf> {
    UserDirs::new().map(|dirs| {
        dirs.home_dir()
            .join("Desktop/100-Days-of-Rust/day-24-garden-errors/garden")
    })
}

fn main() -> Result<(), std::io::Error> {
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
        Commands::Write { title } => day_24_garden_errors::write(garden_path, title),
    }
}

// // ⛳️ Step 2 - Turning standard errors into diagnostics
// // cargo add miette -F miette/fancy

// // `miette` provides beautiful error reporting with context.

// use clap::{CommandFactory, Parser, Subcommand, error::ErrorKind};
// use directories::UserDirs;
// use miette::IntoDiagnostic;
// use std::path::PathBuf;

// #[derive(Parser, Debug, Clone)]
// #[clap(version)]
// struct Args {
//     #[clap(short = 'p', long, env)]
//     garden_path: Option<PathBuf>,

//     #[command(subcommand)]
//     cmd: Commands,
// }
// #[derive(Subcommand, Debug, Clone)]
// enum Commands {
//     /// write something in your garden
//     ///
//     /// This command will open your $EDITOR, wait for you
//     /// to write something, and then save the file to your
//     /// garden
//     Write {
//         /// Optionally set a title for what you are going to write about
//         #[clap(short, long)]
//         title: Option<String>,
//     },
// }

// /// Get the user's garden directory, which by default
// /// is placed in their home directory
// fn get_default_garden_dir() -> Option<PathBuf> {
//     UserDirs::new().map(|dirs| {
//         dirs.home_dir()
//             .join("Desktop/100-Days-of-Rust/day-24-garden-errors/garden")
//     })
// }

// // fn main() -> Result<(), std::io::Error> {
// fn main() -> miette::Result<()> {
//     let args = Args::parse();
//     dbg!(args.clone());

//     let Some(garden_path) = args.garden_path.or_else(get_default_garden_dir) else {
//         let mut cmd = Args::command();
//         cmd.error(
//                ErrorKind::ValueValidation,
//                format!(
//                    "garden directory not provided and home directory unavailable for default garden directory"
//                ),
//            )
//            .exit()
//     };
//     if !garden_path.exists() {
//         let mut cmd = Args::command();
//         cmd.error(
//             ErrorKind::ValueValidation,
//             format!(
//                 "garden directory `{}` doesn't exist, or is inaccessible",
//                 garden_path.display()
//             ),
//         )
//         .exit()
//     };

//     dbg!(garden_path.clone());
//     match args.cmd {
//         Commands::Write { title } => {
//             // day_24_garden_errors::write(garden_path, title)
//             day_24_garden_errors::write(garden_path, title).into_diagnostic()
//         }
//     }
// }

// // cargo run -- write -t "My New Post"
// // then delete the temp file manually to see the new error message

// // ⛳️ Step 3 - Building our own errors
// // cargo add thiserror tempfile

// use clap::{CommandFactory, Parser, Subcommand, error::ErrorKind};
// use directories::UserDirs;
// use miette::IntoDiagnostic;
// use std::path::PathBuf;

// #[derive(Parser, Debug, Clone)]
// #[clap(version)]
// struct Args {
//     #[clap(short = 'p', long, env)]
//     garden_path: Option<PathBuf>,

//     #[command(subcommand)]
//     cmd: Commands,
// }
// #[derive(Subcommand, Debug, Clone)]
// enum Commands {
//     /// write something in your garden
//     ///
//     /// This command will open your $EDITOR, wait for you
//     /// to write something, and then save the file to your
//     /// garden
//     Write {
//         /// Optionally set a title for what you are going to write about
//         #[clap(short, long)]
//         title: Option<String>,
//     },
// }

// /// Get the user's garden directory, which by default
// /// is placed in their home directory
// fn get_default_garden_dir() -> Option<PathBuf> {
//     UserDirs::new().map(|dirs| {
//         dirs.home_dir()
//             .join("Desktop/100-Days-of-Rust/day-24-garden-errors/garden")
//     })
// }

// // fn main() -> Result<(), std::io::Error> {
// fn main() -> miette::Result<()> {
//     let args = Args::parse();
//     dbg!(args.clone());

//     let Some(garden_path) = args.garden_path.or_else(get_default_garden_dir) else {
//         let mut cmd = Args::command();
//         cmd.error(
//                ErrorKind::ValueValidation,
//                format!(
//                    "garden directory not provided and home directory unavailable for default garden directory"
//                ),
//            )
//            .exit()
//     };
//     if !garden_path.exists() {
//         let mut cmd = Args::command();
//         cmd.error(
//             ErrorKind::ValueValidation,
//             format!(
//                 "garden directory `{}` doesn't exist, or is inaccessible",
//                 garden_path.display()
//             ),
//         )
//         .exit()
//     };

//     dbg!(garden_path.clone());
//     match args.cmd {
//         Commands::Write { title } => {
//             // day_24_garden_errors::write(garden_path, title)
//             day_24_garden_errors::write(garden_path, title).into_diagnostic()
//         }
//     }
// }

// // cargo run -- write -t "My New Post"
// // then delete the temp file manually to see the new error message

// // ⛳️ Step 4 - Adding custom context to errors

// use clap::{CommandFactory, Parser, Subcommand, error::ErrorKind};
// use directories::UserDirs;
// use miette::Context;
// use std::path::PathBuf;

// #[derive(Parser, Debug, Clone)]
// #[clap(version)]
// struct Args {
//     #[clap(short = 'p', long, env)]
//     garden_path: Option<PathBuf>,

//     #[command(subcommand)]
//     cmd: Commands,
// }
// #[derive(Subcommand, Debug, Clone)]
// enum Commands {
//     /// write something in your garden
//     ///
//     /// This command will open your $EDITOR, wait for you
//     /// to write something, and then save the file to your
//     /// garden
//     Write {
//         /// Optionally set a title for what you are going to write about
//         #[clap(short, long)]
//         title: Option<String>,
//     },
// }

// /// Get the user's garden directory, which by default
// /// is placed in their home directory
// fn get_default_garden_dir() -> Option<PathBuf> {
//     UserDirs::new().map(|dirs| {
//         dirs.home_dir()
//             .join("Desktop/100-Days-of-Rust/day-24-garden-errors/garden")
//     })
// }

// // fn main() -> Result<(), std::io::Error> {
// fn main() -> miette::Result<()> {
//     let args = Args::parse();
//     dbg!(args.clone());

//     let Some(garden_path) = args.garden_path.or_else(get_default_garden_dir) else {
//         let mut cmd = Args::command();
//         cmd.error(
//                ErrorKind::ValueValidation,
//                format!(
//                    "garden directory not provided and home directory unavailable for default garden directory"
//                ),
//            )
//            .exit()
//     };
//     if !garden_path.exists() {
//         let mut cmd = Args::command();
//         cmd.error(
//             ErrorKind::ValueValidation,
//             format!(
//                 "garden directory `{}` doesn't exist, or is inaccessible",
//                 garden_path.display()
//             ),
//         )
//         .exit()
//     };

//     dbg!(garden_path.clone());
//     match args.cmd {
//         Commands::Write { title } => {
//             // day_24_garden_errors::write(garden_path, title).into_diagnostic()
//             day_24_garden_errors::write(garden_path, title).wrap_err("garden::write")
//         }
//     }
// }

// // cargo run -- write
// // then delete the temp file manually to see the new error message
