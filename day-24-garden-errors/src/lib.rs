// ⛳️ Step 1 - Everything works

use dialoguer::{Confirm, Input};
use edit::{Builder, edit_file};
use owo_colors::OwoColorize;
use std::{fs, io::Write, path::PathBuf};

pub fn write(garden_path: PathBuf, title: Option<String>) -> Result<(), std::io::Error> {
    let (mut file, filepath) = Builder::new()
        .suffix(".md")
        .rand_bytes(5)
        .tempfile_in(garden_path.clone())?
        .keep()?;
    dbg!(filepath.clone());
    // let template = format!("# {}", title.clone().unwrap_or("".to_string()));
    let template = format!("# {}", title.as_deref().unwrap_or(""));
    file.write_all(template.as_bytes())?;
    edit_file(filepath.clone())?;
    let contents = fs::read_to_string(filepath.clone())?;

    let document_title = title.or_else(|| {
        contents
            .lines()
            .find(|v| v.starts_with("# "))
            .map(|line| line.trim_start_matches("# ").to_string())
    });

    let filename = match document_title {
        Some(raw_title) => {
            let confirmation = Confirm::new()
                .with_prompt(format!(
                    "Current title is \"{}\", do you want to use it?",
                    raw_title.green().bold()
                ))
                .interact()
                .unwrap();

            if confirmation {
                Some(slug::slugify(raw_title))
            } else {
                None
            }
        }
        None => None,
    };

    let filename = match filename {
        Some(name) => name,
        None => {
            let file_name: String = Input::new()
                .with_prompt(format!("{}", "Enter filename".blue().bold()))
                .interact_text()
                .unwrap();
            slug::slugify(file_name)
        }
    };

    for attempt in 0.. {
        let mut dest = garden_path.join(if attempt == 0 {
            filename.clone()
        } else {
            format!("{filename}{:03}", -attempt)
        });
        dest.set_extension("md");
        if dest.exists() {
            continue;
        }
        fs::rename(filepath, &dest)?;
        break;
    }

    Ok(())
}

// cargo run -- write -t "My New Post"

// // ⛳️ Step 2 - Creating customs errors

// // `thiserror` generates Error trait implementations from enums.
// //
// // `#[error("...")]` sets the error message.
// // `#[diagnostic(code(...))]` adds error codes for miette.
// // `#[from]` auto-generates From impl for error conversion.

// use dialoguer::{Confirm, Input};
// use edit::{Builder, edit_file};
// use miette::Diagnostic;
// use owo_colors::OwoColorize;
// use std::{fs, io::Write, path::PathBuf};
// use thiserror::Error;

// #[derive(Error, Diagnostic, Debug)]
// pub enum GardenVarietyError {
//     // Now when we use ? on an io::Error it automatically gets turned into a GardenVarietyError::IoError().
//     #[error(transparent)]
//     #[diagnostic(code(garden::io_error))]
//     IoError(#[from] std::io::Error),

//     // This mostly works, but while the PersistError we’ve previously covered has the relevant implementation to convert to an io::Error, it doesn’t have one for our custom error.
//     #[error("failed to keep tempfile: {0}")]
//     #[diagnostic(code(garden::tempfile_keep_error))]
//     TempfileKeepError(#[from] tempfile::PersistError),
// }

// pub fn write(
//     garden_path: PathBuf,
//     title: Option<String>,
// ) -> miette::Result<(), GardenVarietyError> {
//     let (mut file, filepath) = Builder::new()
//         .suffix(".md")
//         .rand_bytes(5)
//         .tempfile_in(garden_path.clone())?
//         .keep()?;
//     dbg!(filepath.clone());
//     // let template = format!("# {}", title.clone().unwrap_or("".to_string()));
//     let template = format!("# {}", title.as_deref().unwrap_or(""));
//     file.write_all(template.as_bytes())?;
//     edit_file(filepath.clone())?;
//     let contents = fs::read_to_string(filepath.clone())?;

//     let document_title = title.or_else(|| {
//         contents
//             .lines()
//             .find(|v| v.starts_with("# "))
//             .map(|line| line.trim_start_matches("# ").to_string())
//     });

//     let filename = match document_title {
//         Some(raw_title) => {
//             let confirmation = Confirm::new()
//                 .with_prompt(format!(
//                     "Current title is \"{}\", do you want to use it?",
//                     raw_title.green().bold()
//                 ))
//                 .interact()
//                 .unwrap();

//             if confirmation {
//                 Some(slug::slugify(raw_title))
//             } else {
//                 None
//             }
//         }
//         None => None,
//     };

//     let filename = match filename {
//         Some(name) => name,
//         None => {
//             let file_name: String = Input::new()
//                 .with_prompt(format!("{}", "Enter filename".blue().bold()))
//                 .interact_text()
//                 .unwrap();
//             slug::slugify(file_name)
//         }
//     };

//     for attempt in 0.. {
//         let mut dest = garden_path.join(if attempt == 0 {
//             filename.clone()
//         } else {
//             format!("{filename}{:03}", -attempt)
//         });
//         dest.set_extension("md");
//         if dest.exists() {
//             continue;
//         }
//         fs::rename(filepath, &dest)?;
//         break;
//     }

//     Ok(())
// }

// // cargo run -- write

// // ⛳️ Step 5 - Adding even more context to errors

// use dialoguer::{Confirm, Input};
// use edit::{Builder, edit_file};
// use miette::Diagnostic;
// use owo_colors::OwoColorize;
// use std::{fs, io::Write, path::PathBuf};
// use thiserror::Error;

// #[derive(Error, Diagnostic, Debug)]
// pub enum GardenVarietyError {
//     #[error(transparent)]
//     #[diagnostic(code(garden::io_error))]
//     IoError(#[from] std::io::Error),

//     #[error("failed to keep tempfile: {0}")]
//     #[diagnostic(code(garden::tempfile_keep_error))]
//     TempfileKeepError(#[from] tempfile::PersistError),

//     #[error("failed to create tempfile: {0}")]
//     #[diagnostic(
//         code(garden::tempfile_create_error),
//         help("Please ensure that your garden path directory makes sense.")
//     )]
//     TempfileCreationError(std::io::Error),
// }

// pub fn write(
//     garden_path: PathBuf,
//     title: Option<String>,
// ) -> miette::Result<(), GardenVarietyError> {
//     let (mut file, filepath) = Builder::new()
//         .suffix(".md")
//         .rand_bytes(5)
//         .tempfile_in("garden_path.clone()")
//         .map_err(|e| GardenVarietyError::TempfileCreationError(e))?
//         .keep()?;
//     dbg!(filepath.clone());
//     // let template = format!("# {}", title.clone().unwrap_or("".to_string()));
//     let template = format!("# {}", title.as_deref().unwrap_or(""));
//     file.write_all(template.as_bytes())?;
//     edit_file(filepath.clone())?;
//     let contents = fs::read_to_string(filepath.clone())?;

//     let document_title = title.or_else(|| {
//         contents
//             .lines()
//             .find(|v| v.starts_with("# "))
//             .map(|line| line.trim_start_matches("# ").to_string())
//     });

//     let filename = match document_title {
//         Some(raw_title) => {
//             let confirmation = Confirm::new()
//                 .with_prompt(format!(
//                     "Current title is \"{}\", do you want to use it?",
//                     raw_title.green().bold()
//                 ))
//                 .interact()
//                 .unwrap();

//             if confirmation {
//                 Some(slug::slugify(raw_title))
//             } else {
//                 None
//             }
//         }
//         None => None,
//     };

//     let filename = match filename {
//         Some(name) => name,
//         None => {
//             let file_name: String = Input::new()
//                 .with_prompt(format!("{}", "Enter filename".blue().bold()))
//                 .interact_text()
//                 .unwrap();
//             slug::slugify(file_name)
//         }
//     };

//     for attempt in 0.. {
//         let mut dest = garden_path.join(if attempt == 0 {
//             filename.clone()
//         } else {
//             format!("{filename}{:03}", -attempt)
//         });
//         dest.set_extension("md");
//         if dest.exists() {
//             continue;
//         }
//         fs::rename(filepath, &dest)?;
//         break;
//     }

//     Ok(())
// }

// cargo run -- write
// Can you fix the error?

// ## Errors
// Errors have varying levels of importance. Sometimes you know they aren’t going to happen and you can .unwrap(), other times we can use a tool like clap to report validation errors.

// Further, we can return errors from functions using Results with different kinds of errors. In our binary, we want to display the errors to the user so we convert into a miette report, but in our library we want to build out a custom error that represents how our application can actually fail, so we use thiserror.

// Custom errors can be as complex or simple as you want. We started out with what was effectively a wrapper for any io::Error, and moved into adding more context to the errors we felt needed more attention.

// Overall, you get to choose how to handle errors, and Rust has a wide variety of tools that span everything from “ignoring them” to “full being able to match on what happened”.
