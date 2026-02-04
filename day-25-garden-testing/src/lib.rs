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
