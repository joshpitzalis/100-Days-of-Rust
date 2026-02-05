// ⛳️ Step 1 - Make sure everything works
use dialoguer::{Confirm, Input};
use edit::{Builder, edit_file};
use miette::Diagnostic;
use owo_colors::OwoColorize;
use std::{fs, io::Write, path::PathBuf};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum GardenVarietyError {
    #[error(transparent)]
    #[diagnostic(code(garden::io_error))]
    IoError(#[from] std::io::Error),

    #[error("failed to keep tempfile: {0}")]
    #[diagnostic(code(garden::tempfile_keep_error))]
    TempfileKeepError(#[from] tempfile::PersistError),

    #[error("failed to create tempfile: {0}")]
    #[diagnostic(
        code(garden::tempfile_create_error),
        help("Please ensure that your garden path directory makes sense.")
    )]
    TempfileCreationError(std::io::Error),
}

pub fn write(
    garden_path: PathBuf,
    title: Option<String>,
) -> miette::Result<(), GardenVarietyError> {
    let (mut file, filepath) = Builder::new()
        .suffix(".md")
        .rand_bytes(5)
        .tempfile_in(garden_path.clone())
        .map_err(|e| GardenVarietyError::TempfileCreationError(e))?
        .keep()?;
    dbg!(filepath.clone());
    // let template = format!("# {}", title.clone().unwrap_or("".to_string()));
    let template = format!("# {}", title.as_deref().unwrap_or(""));
    file.write_all(template.as_bytes())?;
    drop(file); // Close the file handle before opening in editor
    edit_file(filepath.clone())?;
    let contents = fs::read_to_string(filepath.clone())?;

    let document_title = title.or_else(|| {
        contents
            .lines()
            .find(|v| v.starts_with("# "))
            .map(|line| line.trim_start_matches("# ").to_string())
    });

    // let document_title = title.or_else(|| title_from_content(contents.clone()));

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

// // ⛳️ Step 2 - Add unit tests in the same file
// // Unit tests work best on small, focused functions.
// // Extract `title_from_content` to parse markdown headings.
// //
// // `input.lines()` returns an iterator over lines.
// // `find()` returns Option<&str> - first line matching predicate.
// // `map()` transforms the found line, stripping the "# " prefix.

// fn title_from_content(input: String) -> Option<String> {
//     input
//         .lines()
//         .find(|v| v.starts_with("# "))
//         .map(|line| line.trim_start_matches("# ").to_string())
// }

// // Update write() to use it:
// // let document_title = title.or_else(|| title_from_content(&contents));

// // Unit tests live in a `mod tests` block with `#[cfg(test)]`.
// //
// // `#[cfg(test)]` - only compile this module when running tests.
// // `use super::*` - bring parent module's items into scope.
// // `#[test]` - marks a function as a test case.
// // `assert_eq!(left, right)` - panics if values don't match.
// //
// // Test names should describe what they test.

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn title_from_empty_string() {
//         assert_eq!(title_from_content("".to_string()), None);
//     }

//     #[test]
//     fn title_from_content_string() {
//         assert_eq!(
//             title_from_content("# some title".to_string()),
//             Some("some title".to_string())
//         );
//     }
// }

// // Run with: cargo test

// // ⛳️ Step 2 - Write a failing test to find a bug
// // cargo install cargo-watch

// // A line with just "# " (hash, space, nothing else) should return None,
// // but our current implementation returns Some("").
// //

// // fn title_from_content(input: String) -> Option<String> {
// //     input
// //         .lines()
// //         .find(|v| v.starts_with("# "))
// //         .map(|line| line.trim_start_matches("# ").to_string())
// // }

// // use std::ops::Not;

// fn title_from_content(input: String) -> Option<String> {
//     input.lines().find_map(|line| {
//         // // `find_map` combines find() and map() - finds and transforms in one step.
//         // // `strip_prefix()` returns Some(rest) if prefix exists, None otherwise.
//         // // `and_then()` chains Option operations - runs closure only if Some.
//         // // `then_some()` on bool returns Some(value) if true, None if false.
//         // // `.not()` from std::ops::Not is method form of `!` operator.

//         line.strip_prefix("# ").and_then(|title| {
//             if title.is_empty() {
//                 None
//             } else {
//                 Some(title.to_string())
//             }
//         })

//         // line.strip_prefix("# ").and_then(|title| {
//         //     if title.is_empty() {
//         //         None
//         //     } else {
//         //         Some(title.to_string())
//         //     }
//         // })

//         // line.strip_prefix("# ")
//         //     .and_then(|title| (!title.is_empty()).then_some(title.to_string()))

//         // line.strip_prefix("# ")
//         //     .and_then(|title| title.is_empty().not().then_some(title.to_string()))
//     })
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn title_from_content_no_title() {
//         // This test will FAIL - reveals a bug!
//         assert_eq!(title_from_content("# ".to_string()), None);
//     }
// }

// // Run cargo watch -x "test title_from_content_no_title"

// //  ⛳️ Step 3 - Create an integration test
// // cargo add --dev assert_cmd
// // mkdir -p tests && touch tests/integration.rs

// // Integration tests live in `tests/` directory, go there...

// use std::ops::Not;

// fn title_from_content(input: String) -> Option<String> {
//     input.lines().find_map(|line| {
//         line.strip_prefix("# ")
//             .and_then(|title| title.is_empty().not().then_some(title.to_string()))
//     })
// }

// //  ⛳️ Step 4 - Create an interactive test
// // cargo add --dev rexpect predicates assert_fs

// // `rexpect` - expect-style testing for interactive programs (Unix only)
// // `assert_fs` - temporary directories and file assertions
// // `predicates` - composable test assertions

// // rexpect doesn't work on Windows, so we conditionally include it:

// // Integration tests live in `tests/` directory, go there...

// use std::ops::Not;

// fn title_from_content(input: String) -> Option<String> {
//     input.lines().find_map(|line| {
//         line.strip_prefix("# ")
//             .and_then(|title| title.is_empty().not().then_some(title.to_string()))
//     })
// }
