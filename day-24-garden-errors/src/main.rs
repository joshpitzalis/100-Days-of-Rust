fn main() {
    println!("Hello, world!");
}

// // ⛳️ Step 14 - Custom errors with thiserror and miette
// // Run: cargo add miette -F fancy
// // Run: cargo add thiserror tempfile
// //
// // `thiserror` generates Error trait implementations from enums.
// // `miette` provides beautiful error reporting with context.
// //
// // `#[error("...")]` sets the error message.
// // `#[from]` auto-generates From impl for error conversion.
// // `#[diagnostic(code(...))]` adds error codes for miette.
// //
// // === Create src/lib.rs error type ===
// // use miette::Diagnostic;
// // use thiserror::Error;
// //
// // #[derive(Error, Diagnostic, Debug)]
// // pub enum GardenVarietyError {
// //     #[error(transparent)]
// //     #[diagnostic(code(garden::io_error))]
// //     IoError(#[from] std::io::Error),
// //
// //     #[error("failed to keep tempfile: {0}")]
// //     #[diagnostic(code(garden::tempfile_keep_error))]
// //     TempfileKeepError(#[from] tempfile::PersistError),
// // }
// //
// // // Change write() return type:
// // pub fn write(...) -> Result<(), GardenVarietyError> {
// // === end error type ===
// //
// // === Update main.rs ===
// use clap::{error::ErrorKind, CommandFactory, Parser, Subcommand};
// use directories::UserDirs;
// use miette::{Context, IntoDiagnostic};
// use std::path::PathBuf;
//
// #[derive(Parser, Debug)]
// #[clap(version)]
// struct Args {
//     #[clap(short = 'p', long, env)]
//     garden_path: Option<PathBuf>,
//
//     #[command(subcommand)]
//     cmd: Commands,
// }
//
// #[derive(Subcommand, Debug)]
// enum Commands {
//     Write {
//         #[clap(short, long)]
//         title: Option<String>,
//     },
// }
//
// fn get_default_garden_dir() -> Option<PathBuf> {
//     UserDirs::new().map(|dirs| dirs.home_dir().join("garden"))
// }
//
// // miette::Result enables fancy error display
// fn main() -> miette::Result<()> {
//     let args = Args::parse();
//
//     let Some(garden_path) = args.garden_path.or_else(get_default_garden_dir) else {
//         Args::command()
//             .error(ErrorKind::ValueValidation, "garden directory not provided")
//             .exit()
//     };
//
//     if !garden_path.exists() {
//         Args::command()
//             .error(
//                 ErrorKind::ValueValidation,
//                 format!("garden directory `{}` doesn't exist", garden_path.display()),
//             )
//             .exit()
//     };
//
//     match args.cmd {
//         Commands::Write { title } => {
//             // wrap_err adds context to errors
//             garden::write(garden_path, title).wrap_err("garden::write")?;
//         }
//     }
//     Ok(())
// }
