// cargo add clap -F env -F derive
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short = 'p', long, env)]
    garden_path: Option<PathBuf>,

    #[command(subcommand)]
    cmd: Commands,
}
#[derive(Subcommand, Debug)]
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

fn main() {
    let args = Args::parse();
    dbg!(args);
}

// cargo run -- -p some-garden write -t "My New Post"

// // ⛳️ Step 2 - Making the commands do stuff

// // cargo add directories

// use clap::{CommandFactory, Parser, Subcommand, error::ErrorKind};
// use directories::UserDirs;
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
//     UserDirs::new().map(|dirs| dirs.home_dir().join("garden"))
// }

// fn main() {
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

//     dbg!(garden_path);
//     match args.cmd {
//         Commands::Write { title } => {
//             dbg!(title);
//         }
//     }
// }

// // cargo run -- write -t "My New Post"
// // mkdir ~/Desktop/100-Days-of-Rust/day-23-garden/garden
// // cargo run -- write -t "My New Post"

// // ⛳️ Step 3 - Splitting code to a library

// use clap::{CommandFactory, Parser, Subcommand, error::ErrorKind};
// use directories::UserDirs;
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
//     UserDirs::new().map(|dirs| dirs.home_dir().join("garden"))
// }

// fn main() {
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
//         Commands::Write { title } => day_23_garden::write(garden_path, title),
//     }
// }

// // cargo run -- write -t "My New Post"

// // ⛳️ Step 4 - Splitting code to a library
// // cargo add edit

// use clap::{CommandFactory, Parser, Subcommand, error::ErrorKind};
// use directories::UserDirs;
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
//             .join("Desktop/100-Days-of-Rust/day-23-garden/garden")
//     })
// }

// // fn main() {
// fn main() -> Result<(), std::io::Error> {
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
//         Commands::Write { title } => day_23_garden::write(garden_path, title),
//     }
// }

// // cargo run -- write -t "My New Post"
