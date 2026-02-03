// use std::path::PathBuf;

// ⛳️ Step 4 - Splitting code to a library
// pub fn write(garden_path: PathBuf, title: Option<String>) {
//     dbg!("in write", garden_path, title);
// }

// // ⛳️ Step 5 - Creating a file
// use edit::Builder;
// use std::path::PathBuf;

// pub fn write(garden_path: PathBuf, _title: Option<String>) -> Result<(), std::io::Error> {
//     let (_file, filepath) = Builder::new()
//         .suffix(".md")
//         .rand_bytes(5)
//         .tempfile_in(garden_path)?
//         .keep()?;
//     dbg!(filepath);
//     Ok(())
// }

// // ⛳️ Step 6 - Setting the file title
// use edit::{Builder, edit_file};
// use std::{io::Write, path::PathBuf};

// pub fn write(garden_path: PathBuf, title: Option<String>) -> Result<(), std::io::Error> {
//     let (mut file, filepath) = Builder::new()
//         .suffix(".md")
//         .rand_bytes(5)
//         .tempfile_in(garden_path)?
//         .keep()?;
//     dbg!(filepath.clone());
//     let template = format!("# {}", title.unwrap_or("".to_string()));
//     file.write_all(template.as_bytes())?;
//     edit_file(filepath.clone())?;
//     Ok(())
// }

// // cargo run -- write -t "My New Post"
// // export EDITOR="zed -w"

// // ⛳️ Step 7 - renaming the file
// // cargo add slug

// use edit::{Builder, edit_file};
// use std::{fs, io::Write, path::PathBuf};

// pub fn write(garden_path: PathBuf, title: Option<String>) -> Result<(), std::io::Error> {
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
//         Some(raw_title) => slug::slugify(raw_title),
//         None => {
//             todo!("ask for filename");
//         }
//     };

//     let mut dest = garden_path.join(filename);
//     dest.set_extension("md");
//     fs::rename(filepath, dest.clone())?;
//     dbg!(dest);

//     Ok(())
// }

// // cargo run -- write -t "My New Post"

// // ⛳️ Step 8 - asking for a filename
// // cargo add dialoguer

// use dialoguer::Input;
// use edit::{Builder, edit_file};
// use std::{fs, io::Write, path::PathBuf};

// pub fn write(garden_path: PathBuf, title: Option<String>) -> Result<(), std::io::Error> {
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
//         Some(raw_title) => slug::slugify(raw_title),
//         None => {
//             // todo!("ask for filename");
//             let file_name: String = Input::new()
//                 .with_prompt("Enter filename:")
//                 .interact_text()
//                 .unwrap();
//             slug::slugify(file_name)
//         }
//     };

//     let mut dest = garden_path.join(filename);
//     dest.set_extension("md");
//     fs::rename(filepath, dest.clone())?;
//     dbg!(dest);

//     Ok(())
// }

// // cargo run -- write

// // ⛳️ Step 9 - confirm filename

// use dialoguer::{Confirm, Input};
// use edit::{Builder, edit_file};
// use std::{fs, io::Write, path::PathBuf};

// pub fn write(garden_path: PathBuf, title: Option<String>) -> Result<(), std::io::Error> {
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
//                     "Current title is \"{raw_title}\", do you want to use it?"
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
//                 .with_prompt("Enter filename:")
//                 .interact_text()
//                 .unwrap();
//             slug::slugify(file_name)
//         }
//     };

//     let mut dest = garden_path.join(filename);
//     dest.set_extension("md");
//     fs::rename(filepath, dest.clone())?;
//     dbg!(dest);

//     Ok(())
// }

// // cargo run -- write

// // ⛳️ Step 9 - de-duplicate filenames

// use dialoguer::{Confirm, Input};
// use edit::{Builder, edit_file};
// use std::{fs, io::Write, path::PathBuf};

// pub fn write(garden_path: PathBuf, title: Option<String>) -> Result<(), std::io::Error> {
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
//                     "Current title is \"{raw_title}\", do you want to use it?"
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
//                 .with_prompt("Enter filename:")
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

// // cargo run -- write -t "My New Post"

// // ⛳️ Step 10 - add some colour
// // cargo add owo-colors

// use dialoguer::{Confirm, Input};
// use edit::{Builder, edit_file};
// use owo_colors::OwoColorize;
// use std::{fs, io::Write, path::PathBuf};

// pub fn write(garden_path: PathBuf, title: Option<String>) -> Result<(), std::io::Error> {
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

// // cargo run -- write -t "My New Post"
