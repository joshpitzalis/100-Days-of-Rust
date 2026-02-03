// === src/lib.rs ===
// Starting point: Your completed lib.rs from Part 1
// We'll add tests to this file

fn main() {
    println!("Run `cargo test` to execute tests!");
}

// // ⛳️ Step 1 - Extract testable function from write()
// // Unit tests work best on small, focused functions.
// // Extract `title_from_content` to parse markdown headings.
// //
// // `input.lines()` returns an iterator over lines.
// // `find()` returns Option<&str> - first line matching predicate.
// // `map()` transforms the found line, stripping the "# " prefix.
// //
// // === Add to src/lib.rs ===
// fn title_from_content(input: &str) -> Option<String> {
//     input
//         .lines()
//         .find(|v| v.starts_with("# "))
//         .map(|line| line.trim_start_matches("# ").to_string())
// }
//
// // Update write() to use it:
// // let document_title = title.or_else(|| title_from_content(&contents));
// // === end addition ===

// // ⛳️ Step 2 - Add unit tests in the same file
// // Unit tests live in a `mod tests` block with `#[cfg(test)]`.
// //
// // `#[cfg(test)]` - only compile this module when running tests.
// // `use super::*` - bring parent module's items into scope.
// // `#[test]` - marks a function as a test case.
// // `assert_eq!(left, right)` - panics if values don't match.
// //
// // Test names should describe what they test.
// //
// // === Add to bottom of src/lib.rs ===
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn title_from_empty_string() {
//         assert_eq!(title_from_content(""), None);
//     }
//
//     #[test]
//     fn title_from_content_string() {
//         assert_eq!(
//             title_from_content("# some title"),
//             Some("some title".to_string())
//         );
//     }
// }
// // === end addition ===
// //
// // Run with: cargo test

// // ⛳️ Step 3 - Write a failing test to find a bug
// // Good practice: write a test for edge cases you suspect might fail.
// //
// // A line with just "# " (hash, space, nothing else) should return None,
// // but our current implementation returns Some("").
// //
// // === Add to mod tests ===
//     #[test]
//     fn title_from_content_no_title() {
//         // This test will FAIL - reveals a bug!
//         assert_eq!(title_from_content("# "), None);
//     }
// // === end addition ===
// //
// // Run `cargo test` to see it fail:
// // left: `Some("")`
// // right: `None`

// // ⛳️ Step 4 - Fix the bug with find_map and strip_prefix
// // `find_map` combines find() and map() - finds and transforms in one step.
// // `strip_prefix()` returns Some(rest) if prefix exists, None otherwise.
// // `and_then()` chains Option operations - runs closure only if Some.
// // `then_some()` on bool returns Some(value) if true, None if false.
// // `.not()` from std::ops::Not is method form of `!` operator.
// //
// // === Replace title_from_content in src/lib.rs ===
// use std::ops::Not;
//
// fn title_from_content(input: &str) -> Option<String> {
//     input.lines().find_map(|line| {
//         line.strip_prefix("# ").and_then(|title| {
//             title
//                 .is_empty()
//                 .not()
//                 .then_some(title.to_string())
//         })
//     })
// }
// // === end replacement ===
// //
// // Now all tests pass!

// // ⛳️ Step 5 - Create integration test file
// // Integration tests live in `tests/` directory, separate from src/.
// // Each file in tests/ is compiled as its own crate.
// // Integration tests only access your library's public interface.
// //
// // Run: mkdir tests && touch tests/integration.rs
// //
// // === tests/integration.rs ===
// // Run: cargo add --dev assert_cmd
// //
// // `assert_cmd` helps test CLI binaries.
// // `cargo_bin("garden")` finds the compiled binary.
// // `.assert()` runs command and enables assertions.
// // `.success()` verifies exit code 0.
// // `.stderr("")` verifies no error output.
//
// /// Make sure help runs - indicates binary works
// #[test]
// fn test_help() {
//     assert_cmd::Command::cargo_bin("garden")
//         .unwrap()
//         .arg("--help")
//         .assert()
//         .success()
//         .stderr("");
// }
//
// /// Make sure write subcommand exists
// #[test]
// fn test_write_help() {
//     assert_cmd::Command::cargo_bin("garden")
//         .unwrap()
//         .arg("write")
//         .arg("--help")
//         .assert()
//         .success()
//         .stderr("");
// }
// // === end tests/integration.rs ===

// // ⛳️ Step 6 - Add dev dependencies for interactive testing
// // Testing interactive CLIs requires special tools.
// //
// // Run: cargo add --dev rexpect predicates assert_fs
// //
// // `rexpect` - expect-style testing for interactive programs (Unix only)
// // `assert_fs` - temporary directories and file assertions
// // `predicates` - composable test assertions
// //
// // rexpect doesn't work on Windows, so we conditionally include it:
// //
// // === Add to Cargo.toml ===
// // [dev-dependencies]
// // assert_cmd = "2"
// // assert_fs = "1"
// // predicates = "3"
// //
// // [target.'cfg(not(windows))'.dev-dependencies]
// // rexpect = "0.5"
// // === end Cargo.toml ===

// // ⛳️ Step 7 - Create a fake editor script
// // Our CLI opens $EDITOR - we need to simulate this in tests.
// // A simple bash script that appends text to the file works.
// //
// // Run: touch tests/fake-editor.sh && chmod +x tests/fake-editor.sh
// //
// // === tests/fake-editor.sh ===
// // echo "testing" >> $1;
// // === end fake-editor.sh ===
// //
// // `$1` is the first argument (the temp file path).
// // `>>` appends to the file (preserves our template).

// // ⛳️ Step 8 - Write setup helper function
// // Tests often share setup code - extract to a helper function.
// //
// // `TempDir::new()` creates an auto-cleaning temporary directory.
// // `cargo_bin()` finds the compiled test binary.
// // `current_dir()` gets the project root for finding fake-editor.sh.
// // `.env()` sets environment variables for the subprocess.
// //
// // Returns tuple of (Command, TempDir) so test owns the temp dir.
// //
// // === Add to tests/integration.rs ===
// use assert_fs::prelude::*;
// use predicates::prelude::*;
// use std::error::Error;
// use std::process::Command;
//
// #[cfg(not(target_os = "windows"))]
// use assert_fs::TempDir;
// #[cfg(not(target_os = "windows"))]
// use rexpect::session::spawn_command;
//
// #[cfg(not(target_os = "windows"))]
// fn setup_command() -> Result<(Command, TempDir), Box<dyn Error>> {
//     let temp_dir = assert_fs::TempDir::new()?;
//
//     let bin_path = assert_cmd::cargo::cargo_bin("garden");
//     let fake_editor_path = std::env::current_dir()?
//         .join("tests")
//         .join("fake-editor.sh");
//
//     if !fake_editor_path.exists() {
//         panic!("fake editor script not found at {:?}", fake_editor_path);
//     }
//
//     let mut cmd = Command::new(bin_path);
//     cmd.env("EDITOR", fake_editor_path.into_os_string())
//         .env("GARDEN_PATH", temp_dir.path())
//         .env("NO_COLOR", "true"); // Disable colors for easier assertions
//
//     Ok((cmd, temp_dir))
// }
// // === end addition ===

// // ⛳️ Step 9 - Write interactive test with rexpect
// // `spawn_command()` starts the process and returns a PtySession.
// // `exp_string()` waits for and consumes expected output.
// // `exp_regex()` matches output against a regex pattern.
// // `send_line()` sends input to the process (simulates typing + Enter).
// // `exp_eof()` waits for process to exit.
// //
// // `Box<dyn Error>` - any error type works (convenient for tests).
// // `temp_dir.child("file.md")` gets path to file in temp dir.
// // `predicate::path::exists()` asserts file exists.
// //
// // === Add to tests/integration.rs ===
// #[cfg(not(target_os = "windows"))]
// #[test]
// fn test_write_with_title() -> Result<(), Box<dyn Error>> {
//     let (mut cmd, temp_dir) = setup_command()?;
//
//     cmd.arg("write").arg("-t").arg("atitle");
//
//     let mut process = spawn_command(cmd, Some(30000))?; // 30s timeout
//
//     // Expect the confirmation prompt
//     process.exp_string("current title: ")?;
//     process.exp_string("atitle")?;
//     process.exp_regex("\\s*")?; // whitespace/newlines
//     process.exp_string("Do you want a different title? (y/N): ")?;
//
//     // Send "N" to accept the title
//     process.send_line("N")?;
//
//     // Wait for process to exit
//     process.exp_eof()?;
//
//     // Assert the file was created
//     temp_dir
//         .child("atitle.md")
//         .assert(predicate::path::exists());
//
//     Ok(())
// }
// // === end addition ===

// // ⛳️ Step 10 - Create extension trait for reusable assertions
// // Extension traits add methods to existing types.
// // Define the trait, then impl it for the type you want to extend.
// //
// // `PtySession` is rexpect's process handle.
// // Our `exp_title()` encapsulates the 4-step title assertion.
// //
// // This lets us write `process.exp_title("atitle")?` instead of
// // repeating 4 lines of expectations in every test.
// //
// // === Add to tests/integration.rs ===
// #[cfg(not(target_os = "windows"))]
// use rexpect::session::PtySession;
//
// #[cfg(not(target_os = "windows"))]
// trait GardenExpectations {
//     fn exp_title(&mut self, title: &str) -> Result<(), rexpect::error::Error>;
// }
//
// #[cfg(not(target_os = "windows"))]
// impl GardenExpectations for PtySession {
//     fn exp_title(&mut self, title: &str) -> Result<(), rexpect::error::Error> {
//         self.exp_string("current title: ")?;
//         self.exp_string(title)?;
//         self.exp_regex("\\s*")?;
//         self.exp_string("Do you want a different title? (y/N): ")?;
//         Ok(())
//     }
// }
// // === end addition ===

// // ⛳️ Step 11 - Refactor test to use extension trait
// // Now our test is much cleaner and more readable.
// // The extension trait hides the implementation details.
// //
// // === Replace test_write_with_title ===
// #[cfg(not(target_os = "windows"))]
// #[test]
// fn test_write_with_title() -> Result<(), Box<dyn Error>> {
//     let (mut cmd, temp_dir) = setup_command()?;
//
//     cmd.arg("write").arg("-t").arg("atitle");
//
//     let mut process = spawn_command(cmd, Some(30000))?;
//
//     process.exp_title("atitle")?;  // Clean!
//     process.send_line("N")?;
//     process.exp_eof()?;
//
//     temp_dir
//         .child("atitle.md")
//         .assert(predicate::path::exists());
//
//     Ok(())
// }
// // === end replacement ===

// // ⛳️ Step 12 - Add test for title from file content
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
// // Let's verify with a test!
// //
// // === Add to tests/integration.rs ===
// #[cfg(not(target_os = "windows"))]
// #[test]
// fn test_write_with_written_title() -> Result<(), Box<dyn Error>> {
//     let (mut cmd, temp_dir) = setup_command()?;
//
//     // No -t flag - title comes from file content
//     cmd.arg("write");
//
//     let mut process = spawn_command(cmd, Some(30000))?;
//
//     // fake-editor.sh appends "testing" → file has "# testing"
//     process.exp_title("testing")?;
//     process.send_line("N")?;
//     process.exp_eof()?;
//
//     temp_dir
//         .child("testing.md")
//         .assert(predicate::path::exists());
//
//     Ok(())
// }
// // === end addition ===

// // ⛳️ Step 13 - Add test for choosing different title
// // Test the "y" path where user wants to change the title.
// //
// // After sending "y", the CLI prompts for a new filename.
// // We send "custom-name" and verify that file is created.
// //
// // === Add to tests/integration.rs ===
// #[cfg(not(target_os = "windows"))]
// #[test]
// fn test_write_change_title() -> Result<(), Box<dyn Error>> {
//     let (mut cmd, temp_dir) = setup_command()?;
//
//     cmd.arg("write").arg("-t").arg("original");
//
//     let mut process = spawn_command(cmd, Some(30000))?;
//
//     process.exp_title("original")?;
//     process.send_line("y")?;  // Want different title
//
//     // Expect the "enter filename" prompt
//     process.exp_string("Enter filename")?;
//     process.exp_string("> ")?;
//
//     process.send_line("custom-name")?;
//     process.exp_eof()?;
//
//     // File should use the custom name, not "original"
//     temp_dir
//         .child("custom-name.md")
//         .assert(predicate::path::exists());
//
//     // Original should NOT exist
//     temp_dir
//         .child("original.md")
//         .assert(predicate::path::missing());
//
//     Ok(())
// }
// // === end addition ===

// // ⛳️ Step 14 - Final integration.rs with all tests
// // Here's the complete test file with all pieces together.
// //
// // === Complete tests/integration.rs ===
// use assert_fs::prelude::*;
// use predicates::prelude::*;
// use std::error::Error;
// use std::process::Command;
//
// #[cfg(not(target_os = "windows"))]
// use assert_fs::TempDir;
// #[cfg(not(target_os = "windows"))]
// use rexpect::session::{spawn_command, PtySession};
//
// // === Non-interactive tests (work on all platforms) ===
//
// #[test]
// fn test_help() {
//     assert_cmd::Command::cargo_bin("garden")
//         .unwrap()
//         .arg("--help")
//         .assert()
//         .success()
//         .stderr("");
// }
//
// #[test]
// fn test_write_help() {
//     assert_cmd::Command::cargo_bin("garden")
//         .unwrap()
//         .arg("write")
//         .arg("--help")
//         .assert()
//         .success()
//         .stderr("");
// }
//
// // === Interactive tests (Unix only) ===
//
// #[cfg(not(target_os = "windows"))]
// fn setup_command() -> Result<(Command, TempDir), Box<dyn Error>> {
//     let temp_dir = assert_fs::TempDir::new()?;
//     let bin_path = assert_cmd::cargo::cargo_bin("garden");
//     let fake_editor_path = std::env::current_dir()?
//         .join("tests")
//         .join("fake-editor.sh");
//
//     if !fake_editor_path.exists() {
//         panic!("fake editor script not found");
//     }
//
//     let mut cmd = Command::new(bin_path);
//     cmd.env("EDITOR", fake_editor_path.into_os_string())
//         .env("GARDEN_PATH", temp_dir.path())
//         .env("NO_COLOR", "true");
//
//     Ok((cmd, temp_dir))
// }
//
// #[cfg(not(target_os = "windows"))]
// trait GardenExpectations {
//     fn exp_title(&mut self, title: &str) -> Result<(), rexpect::error::Error>;
// }
//
// #[cfg(not(target_os = "windows"))]
// impl GardenExpectations for PtySession {
//     fn exp_title(&mut self, title: &str) -> Result<(), rexpect::error::Error> {
//         self.exp_string("current title: ")?;
//         self.exp_string(title)?;
//         self.exp_regex("\\s*")?;
//         self.exp_string("Do you want a different title? (y/N): ")?;
//         Ok(())
//     }
// }
//
// #[cfg(not(target_os = "windows"))]
// #[test]
// fn test_write_with_title() -> Result<(), Box<dyn Error>> {
//     let (mut cmd, temp_dir) = setup_command()?;
//     cmd.arg("write").arg("-t").arg("atitle");
//
//     let mut process = spawn_command(cmd, Some(30000))?;
//     process.exp_title("atitle")?;
//     process.send_line("N")?;
//     process.exp_eof()?;
//
//     temp_dir.child("atitle.md").assert(predicate::path::exists());
//     Ok(())
// }
//
// #[cfg(not(target_os = "windows"))]
// #[test]
// fn test_write_with_written_title() -> Result<(), Box<dyn Error>> {
//     let (mut cmd, temp_dir) = setup_command()?;
//     cmd.arg("write");
//
//     let mut process = spawn_command(cmd, Some(30000))?;
//     process.exp_title("testing")?;
//     process.send_line("N")?;
//     process.exp_eof()?;
//
//     temp_dir.child("testing.md").assert(predicate::path::exists());
//     Ok(())
// }
//
// #[cfg(not(target_os = "windows"))]
// #[test]
// fn test_write_change_title() -> Result<(), Box<dyn Error>> {
//     let (mut cmd, temp_dir) = setup_command()?;
//     cmd.arg("write").arg("-t").arg("original");
//
//     let mut process = spawn_command(cmd, Some(30000))?;
//     process.exp_title("original")?;
//     process.send_line("y")?;
//     process.exp_string("Enter filename")?;
//     process.exp_string("> ")?;
//     process.send_line("custom-name")?;
//     process.exp_eof()?;
//
//     temp_dir.child("custom-name.md").assert(predicate::path::exists());
//     temp_dir.child("original.md").assert(predicate::path::missing());
//     Ok(())
// }
// // === end complete tests/integration.rs ===
