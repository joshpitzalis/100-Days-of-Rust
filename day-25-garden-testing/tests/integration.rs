// //  ⛳️ Step 3 - Create an integration test
// // cargo add --dev assert_cmd
// // mkdir -p tests && touch tests/integration.rs

// // Each file in tests/ is compiled as its own crate.
// // Integration tests only access your library's public interface.

// // `assert_cmd` helps test CLI binaries.
// // `cargo_bin("garden")` finds the compiled binary.
// // `.assert()` runs command and enables assertions.
// // `.success()` verifies exit code 0.
// // `.stderr("")` verifies no error output.

// // Make sure help runs - indicates binary works
// #[test]
// fn test_help() {
//     let mut cmd = assert_cmd::Command::new(assert_cmd::cargo::cargo_bin!("day-25-garden-testing"));
//     cmd.arg("--help").assert().success().stderr("");
// }

// /// Make sure write subcommand exists
// #[test]
// fn test_write_help() {
//     let mut cmd = assert_cmd::Command::new(assert_cmd::cargo::cargo_bin!("day-25-garden-testing"));
//     cmd.arg("write").arg("--help").assert().success().stderr("");
// }

// // Run cargo test

// //  ⛳️ Step 4 - Create an interactive test
// // cargo add --dev rexpect predicates assert_fs
// // touch tests/fake-editor.sh && chmod +x tests/fake-editor.sh

// // === tests/fake-editor.sh ===
// // echo "testing" >> $1;
// // === end fake-editor.sh ===

// // `rexpect` - expect-style testing for interactive programs (Unix only)
// // `assert_fs` - temporary directories and file assertions
// // `predicates` - composable test assertions

// use assert_fs::TempDir;
// use assert_fs::prelude::*;
// use predicates::prelude::*;
// use rexpect::session::spawn_command;
// use std::error::Error;
// use std::process::Command;

// // rexpect doesn't work on Windows, so we conditionally include it:

// // `TempDir::new()` creates an auto-cleaning temporary directory.
// //  `current_dir()` gets the project root for finding fake-editor.sh.
// // `spawn_command()` starts the process and returns a PtySession.
// // `exp_string()` waits for and consumes expected output.
// // `exp_regex()` matches output against a regex pattern.
// // `send_line()` sends input to the process (simulates typing + Enter).
// // `exp_eof()` waits for process to exit.
// //
// // `Box<dyn Error>` - any error type works (convenient for tests).
// // `temp_dir.child("file.md")` gets path to file in temp dir.
// // `predicate::path::exists()` asserts file exists.

// #[cfg(not(target_os = "windows"))]
// #[test]

// fn test_write_with_title() -> Result<(), Box<dyn Error>> {
//     let temp_dir = assert_fs::TempDir::new()?;

//     let bin_path = assert_cmd::cargo::cargo_bin!("day-25-garden-testing");
//     let fake_editor_path = std::env::current_dir()?
//         .join("tests")
//         .join("fake-editor.sh");
//     if !fake_editor_path.exists() {
//         panic!("fake editor shell script could not be found")
//     }

//     let mut cmd = Command::new(bin_path);
//     cmd.env("VISUAL", fake_editor_path.as_os_str())
//         .env("EDITOR", fake_editor_path.into_os_string())
//         .env("GARDEN_PATH", temp_dir.path())
//         .arg("write")
//         .arg("-t")
//         .arg("atitle");

//     let mut process = spawn_command(cmd, Some(30000))?;

//     // dialoguer Confirm outputs: Current title is "atitle", do you want to use it? [y/n]
//     process.exp_string("Current title is \"")?;
//     process.exp_string("atitle")?;
//     process.exp_string("\", do you want to use it?")?;
//     process.send_line("y")?;
//     process.exp_eof()?;

//     temp_dir
//         .child("atitle.md")
//         .assert(predicate::path::exists());
//     Ok(())
// }

// //  ⛳️ Step 5 - Create extension trait for reusable assertions

// use assert_fs::{TempDir, prelude::*};
// use predicates::prelude::*;
// use rexpect::session::spawn_command;
// use std::error::Error;
// use std::process::Command;

// #[cfg(not(target_os = "windows"))]
// fn setup_command() -> Result<(Command, TempDir), Box<dyn Error>> {
//     let temp_dir = assert_fs::TempDir::new()?;

//     let bin_path = assert_cmd::cargo::cargo_bin!("day-25-garden-testing");
//     let fake_editor_path = std::env::current_dir()?
//         .join("tests")
//         .join("fake-editor.sh");
//     if !fake_editor_path.exists() {
//         panic!("fake editor shell script could not be found")
//     }

//     let mut cmd = Command::new(bin_path);
//     cmd.env("VISUAL", fake_editor_path.as_os_str())
//         .env("EDITOR", fake_editor_path.into_os_string())
//         .env("GARDEN_PATH", temp_dir.path())
//         .env("NO_COLOR", "true");

//     Ok((cmd, temp_dir))
// }

// #[cfg(not(target_os = "windows"))]
// #[test]

// fn test_write_with_title() -> Result<(), Box<dyn Error>> {
//     let (mut cmd, temp_dir) = setup_command()?;

//     cmd.arg("write").arg("-t").arg("atitle");

//     let mut process = spawn_command(cmd, Some(30000))?;

//     // dialoguer Confirm outputs: Current title is "atitle", do you want to use it? [y/n]
//     process.exp_string("Current title is \"")?;
//     process.exp_string("atitle")?;
//     process.exp_string("\", do you want to use it?")?;

//     process.send_line("y")?;
//     process.exp_eof()?;

//     temp_dir
//         .child("atitle.md")
//         .assert(predicate::path::exists());
//     Ok(())
// }

// // `PtySession` is rexpect's process handle.
// // Our `exp_title()` encapsulates the 4-step title assertion.
// #[cfg(not(target_os = "windows"))]
// use rexpect::session::PtySession;

// // Extension traits add methods to existing types.
// // Define the trait, then impl it for the type you want to extend.
// #[cfg(not(target_os = "windows"))]
// trait GardenExpectations {
//     fn exp_title(&mut self, title: &str) -> Result<(), rexpect::error::Error>;
// }

// #[cfg(not(target_os = "windows"))]
// impl GardenExpectations for PtySession {
//     fn exp_title(&mut self, title: &str) -> Result<(), rexpect::error::Error> {
//         self.exp_string("Current title is \"")?;
//         self.exp_string(title)?;
//         self.exp_regex("\\s*")?;
//         self.exp_string("\", do you want to use it?")?;
//         Ok(())
//     }
// }

// // When no -t flag is passed, title comes from the markdown heading.
// // Our fake-editor.sh writes "testing" to the file.
// // Combined with our "# " template, the file contains "# \ntesting".
// // Wait... that's not right. Let's trace through:
// //
// // 1. Template writes "# " (no title provided)
// // 2. fake-editor appends "testing"
// // 3. File contains "# \ntesting"
// // 4. title_from_content finds "# " but it's empty, returns None
// // 5. Falls back to... our fake editor output "testing" as heading?
// //
// // Actually the fake editor appends, so file is "# testing"

// #[cfg(not(target_os = "windows"))]
// #[test]
// fn test_write_with_written_title() -> Result<(), Box<dyn Error>> {
//     let (mut cmd, temp_dir) = setup_command()?;

//     // No -t flag - title comes from file content
//     cmd.arg("write");

//     let mut process = spawn_command(cmd, Some(30000))?;

//     // fake-editor.sh appends "testing" → file has "# testing"
//     process.exp_title("testing")?;
//     process.send_line("Y")?;
//     process.exp_eof()?;

//     temp_dir
//         .child("testing.md")
//         .assert(predicate::path::exists());

//     Ok(())
// }

// // Test the path where user wants to change the title.
// //
// // After sending "n", the CLI prompts for a new filename.
// // We send "custom-name" and verify that file is created.

// #[cfg(not(target_os = "windows"))]
// #[test]
// fn test_write_change_title() -> Result<(), Box<dyn Error>> {
//     let (mut cmd, temp_dir) = setup_command()?;

//     cmd.arg("write").arg("-t").arg("original");

//     let mut process = spawn_command(cmd, Some(30000))?;

//     process.exp_title("original")?;
//     process.send_line("n")?; // Want different title

//     // Expect the "enter filename" prompt (dialoguer uses ": ")
//     process.exp_string("Enter filename")?;
//     process.exp_string(": ")?;

//     process.send_line("custom-name")?;
//     process.exp_eof()?;

//     // File should use the custom name, not "original"
//     temp_dir
//         .child("custom-name.md")
//         .assert(predicate::path::exists());

//     // Original should NOT exist
//     temp_dir
//         .child("original.md")
//         .assert(predicate::path::missing());

//     Ok(())
// }
