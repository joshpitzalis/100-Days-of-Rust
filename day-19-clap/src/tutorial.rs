fn main() {
    println!("Hello, world!");
}

// // ⛳️ Step 1 - Install Clap and parse CLI arguments
// // Clap with the `derive` feature lets us define CLI arguments using Rust structs.
// // Run: cargo add clap -F derive
// //
// // `#[derive(Parser)]` generates code to parse command-line arguments into our struct.
// // `#[derive(Debug)]` lets us use `dbg!` to inspect values.
// // `#[clap(...)]` attributes configure each argument's behavior.
// use clap::Parser;
//
// /// Scaffold a new post for your blog
// #[derive(Parser, Debug)]
// #[clap(version)]
// struct Args {
//     /// The layout the post should use
//     #[clap(short, long, default_value = "post")]
//     layout: String,
//
//     /// Tags to include
//     #[clap(short, long = "tag")]
//     tags: Vec<String>,
//
//     /// The title of the post.
//     ///
//     /// If not provided, the filename will be generated
//     #[clap(short = 'T', long, default_value = "A Post")]
//     title: String,
//
//     /// Should this post be published?
//     #[clap(short, long, default_value = "draft")]
//     status: String,
//
//     /// Where to put the file
//     #[clap(short, long, default_value = "content")]
//     output_dir: String,
// }
//
// fn main() {
//     // `Args::parse()` reads command-line arguments and returns our struct
//     let args = Args::parse();
//     // `dbg!` prints the value with file/line info - great for debugging
//     // `&args` passes a reference so we can still use `args` afterward
//     dbg!(&args);
// }

// // ⛳️ Step 2 - Create a file with format! and fs::write
// // `format!` concatenates strings using a template syntax.
// // `fs::write` creates a file and writes content to it.
// // It returns a `Result` - we must handle potential errors.
// use clap::Parser;
// use std::fs;
//
// /// Scaffold a new post for your blog
// #[derive(Parser, Debug)]
// #[clap(version)]
// struct Args {
//     /// The layout the post should use
//     #[clap(short, long, default_value = "post")]
//     layout: String,
//
//     /// Tags to include
//     #[clap(short, long = "tag")]
//     tags: Vec<String>,
//
//     /// The title of the post.
//     ///
//     /// If not provided, the filename will be generated
//     #[clap(short = 'T', long, default_value = "A Post")]
//     title: String,
//
//     /// Should this post be published?
//     #[clap(short, long, default_value = "draft")]
//     status: String,
//
//     /// Where to put the file
//     #[clap(short, long, default_value = "content")]
//     output_dir: String,
// }
//
// fn main() {
//     let args = Args::parse();
//     dbg!(&args);
//
//     // `format!` works like `println!` but returns a String instead of printing
//     let filename = format!("{}/{}.md", args.output_dir, args.title);
//
//     // `fs::write` returns Result<()> - it can fail (e.g., directory doesn't exist)
//     // `.expect()` crashes with our message if there's an error - better than `.unwrap()`
//     fs::write(&filename, &args.title)
//         .expect("Failed to write file");
// }

// // ⛳️ Step 3 - Use PathBuf for safer path handling
// // `PathBuf` is Rust's type for file paths - safer than string concatenation.
// // `.join()` combines path segments correctly for any OS.
// // `.set_extension()` adds a file extension.
// // `.exists()` checks if a path exists on disk.
// use clap::Parser;
// use std::{fs, path::PathBuf};
//
// /// Scaffold a new post for your blog
// #[derive(Parser, Debug)]
// #[clap(version)]
// struct Args {
//     /// The layout the post should use
//     #[clap(short, long, default_value = "post")]
//     layout: String,
//
//     /// Tags to include
//     #[clap(short, long = "tag")]
//     tags: Vec<String>,
//
//     /// The title of the post.
//     ///
//     /// If not provided, the filename will be generated
//     #[clap(short = 'T', long, default_value = "A Post")]
//     title: String,
//
//     /// Should this post be published?
//     #[clap(short, long, default_value = "draft")]
//     status: String,
//
//     /// Where to put the file
//     #[clap(short, long, default_value = "content")]
//     output_dir: PathBuf,
// }
//
// fn main() {
//     let args = Args::parse();
//     dbg!(&args);
//
//     // `.join()` safely combines path segments
//     // `mut` is needed because `.set_extension()` modifies the PathBuf
//     let mut filename = args.output_dir.join(&args.title);
//     filename.set_extension("md");
//
//     // `.display()` converts PathBuf to a printable format
//     // PathBuf doesn't implement Display directly (for Unicode safety reasons)
//     fs::write(&filename, &args.title)
//         .expect(&format!("Failed to write file at {}", filename.display()));
// }

// // ⛳️ Step 4 - Add proper error handling with Clap's error display
// // `if let Err(e) = ...` pattern matches only the error case.
// // `CommandFactory` gives us access to Clap's error formatting.
// // `ErrorKind` categorizes what type of error occurred.
// use clap::{error::ErrorKind, CommandFactory, Parser};
// use std::{fs, path::PathBuf};
//
// /// Scaffold a new post for your blog
// #[derive(Parser, Debug)]
// #[clap(version)]
// struct Args {
//     /// The layout the post should use
//     #[clap(short, long, default_value = "post")]
//     layout: String,
//
//     /// Tags to include
//     #[clap(short, long = "tag")]
//     tags: Vec<String>,
//
//     /// The title of the post.
//     ///
//     /// If not provided, the filename will be generated
//     #[clap(short = 'T', long, default_value = "A Post")]
//     title: String,
//
//     /// Should this post be published?
//     #[clap(short, long, default_value = "draft")]
//     status: String,
//
//     /// Where to put the file
//     #[clap(short, long, default_value = "content")]
//     output_dir: PathBuf,
// }
//
// fn main() {
//     let args = Args::parse();
//     dbg!(&args);
//
//     // Check if output directory exists before trying to write
//     if !args.output_dir.exists() {
//         // `Args::command()` gives us access to Clap's Command for error display
//         let mut cmd = Args::command();
//         cmd.error(
//             ErrorKind::ValueValidation,
//             format!("output directory `{}` doesn't exist", args.output_dir.display()),
//         )
//         .exit();
//     }
//
//     let mut filename = args.output_dir.join(&args.title);
//     filename.set_extension("md");
//
//     // `if let Err(error)` only runs if fs::write returns an Err
//     // This is cleaner than `.expect()` for user-facing errors
//     if let Err(error) = fs::write(&filename, &args.title) {
//         let mut cmd = Args::command();
//         cmd.error(
//             ErrorKind::Io,
//             format!("failed to write file at `{}`\n\t{}", filename.display(), error),
//         )
//         .exit();
//     }
// }

// // ⛳️ Step 5 - Add Frontmatter struct and slug generation
// // Frontmatter is metadata at the start of markdown files (common in static site generators).
// // The `slug` crate converts titles to URL-friendly strings.
// // Run: cargo add slug
// use clap::{error::ErrorKind, CommandFactory, Parser};
// use std::{fs, path::PathBuf};
//
// /// Scaffold a new post for your blog
// #[derive(Parser, Debug)]
// #[clap(version)]
// struct Args {
//     /// The layout the post should use
//     #[clap(short, long, default_value = "post")]
//     layout: String,
//
//     /// Tags to include
//     #[clap(short, long = "tag")]
//     tags: Vec<String>,
//
//     /// The title of the post.
//     ///
//     /// If not provided, the filename will be generated
//     #[clap(short = 'T', long, default_value = "A Post")]
//     title: String,
//
//     /// Should this post be published?
//     #[clap(short, long, default_value = "draft")]
//     status: String,
//
//     /// Where to put the file
//     #[clap(short, long, default_value = "content")]
//     output_dir: PathBuf,
// }
//
// // A struct to hold our blog post metadata
// // We'll serialize this to YAML in the next step
// #[derive(Debug)]
// struct Frontmatter {
//     layout: String,
//     tags: Vec<String>,
//     status: String,
//     title: String,
//     slug: String,
// }
//
// fn main() {
//     let args = Args::parse();
//     dbg!(&args);
//
//     if !args.output_dir.exists() {
//         let mut cmd = Args::command();
//         cmd.error(
//             ErrorKind::ValueValidation,
//             format!("output directory `{}` doesn't exist", args.output_dir.display()),
//         )
//         .exit();
//     }
//
//     // `slug::slugify` converts "My Post Title" to "my-post-title"
//     let post_slug = slug::slugify(&args.title);
//
//     // Use the slug for the filename instead of the raw title
//     let mut filename = args.output_dir.join(&post_slug);
//     filename.set_extension("md");
//
//     // Construct our Frontmatter struct
//     // `.clone()` is needed because we use `args.title` again later
//     let frontmatter = Frontmatter {
//         layout: args.layout,
//         tags: args.tags,
//         status: args.status,
//         title: args.title.clone(),
//         slug: post_slug,
//     };
//     dbg!(&frontmatter);
//
//     if let Err(error) = fs::write(&filename, &args.title) {
//         let mut cmd = Args::command();
//         cmd.error(
//             ErrorKind::Io,
//             format!("failed to write file at `{}`\n\t{}", filename.display(), error),
//         )
//         .exit();
//     }
// }

// // ⛳️ Step 6 - Serialize Frontmatter to YAML with Serde
// // Serde is Rust's serialization framework - it converts structs to/from formats like JSON, YAML.
// // Run: cargo add serde serde_yaml -F serde/derive
// //
// // `#[derive(Serialize)]` generates code to convert our struct to YAML/JSON/etc.
// use clap::{error::ErrorKind, CommandFactory, Parser};
// use serde::Serialize;
// use std::{fs, path::PathBuf};
//
// /// Scaffold a new post for your blog
// #[derive(Parser, Debug)]
// #[clap(version)]
// struct Args {
//     /// The layout the post should use
//     #[clap(short, long, default_value = "post")]
//     layout: String,
//
//     /// Tags to include
//     #[clap(short, long = "tag")]
//     tags: Vec<String>,
//
//     /// The title of the post.
//     ///
//     /// If not provided, the filename will be generated
//     #[clap(short = 'T', long, default_value = "A Post")]
//     title: String,
//
//     /// Should this post be published?
//     #[clap(short, long, default_value = "draft")]
//     status: String,
//
//     /// Where to put the file
//     #[clap(short, long, default_value = "content")]
//     output_dir: PathBuf,
// }
//
// // `Serialize` lets serde_yaml convert this struct to YAML
// #[derive(Debug, Serialize)]
// struct Frontmatter {
//     layout: String,
//     tags: Vec<String>,
//     status: String,
//     title: String,
//     slug: String,
// }
//
// fn main() {
//     let args = Args::parse();
//
//     if !args.output_dir.exists() {
//         let mut cmd = Args::command();
//         cmd.error(
//             ErrorKind::ValueValidation,
//             format!("output directory `{}` doesn't exist", args.output_dir.display()),
//         )
//         .exit();
//     }
//
//     let post_slug = slug::slugify(&args.title);
//
//     let mut filename = args.output_dir.join(&post_slug);
//     filename.set_extension("md");
//
//     let frontmatter = Frontmatter {
//         layout: args.layout,
//         tags: args.tags,
//         status: args.status,
//         title: args.title.clone(),
//         slug: post_slug,
//     };
//
//     // `serde_yaml::to_string` converts our struct to a YAML string
//     // It returns a Result, but serialization rarely fails for simple structs
//     let yaml = serde_yaml::to_string(&frontmatter)
//         .expect("Failed to serialize frontmatter");
//
//     // Build the full file content: YAML frontmatter + separator + markdown heading
//     let file_contents = format!("{yaml}---\n\n# {}", args.title);
//
//     if let Err(error) = fs::write(&filename, file_contents) {
//         let mut cmd = Args::command();
//         cmd.error(
//             ErrorKind::Io,
//             format!("failed to write file at `{}`\n\t{}", filename.display(), error),
//         )
//         .exit();
//     }
// }

// // ⛳️ Step 7 - Use enums for type-safe layout and status options
// // Rust enums restrict values to a predefined set - no more typos!
// // `clap::ValueEnum` lets Clap parse enum variants from CLI arguments.
// // `#[serde(rename_all = "kebab-case")]` converts PascalCase to kebab-case in YAML.
// use clap::{error::ErrorKind, CommandFactory, Parser};
// use serde::Serialize;
// use std::{fs, path::PathBuf};
//
// // `ValueEnum` lets Clap parse this enum from command-line strings
// // `Clone` is required by ValueEnum
// // `Default` + `#[default]` sets which variant is used when no value is provided
// // `#[serde(rename_all = "kebab-case")]` outputs "needs-review" instead of "NeedsReview"
// #[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
// #[serde(rename_all = "kebab-case")]
// enum Layout {
//     /// blog post
//     #[default]
//     Post,
//     /// image gallery
//     Gallery,
//     /// code example
//     Code,
// }
//
// #[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
// #[serde(rename_all = "kebab-case")]
// enum PostStatus {
//     /// Draft, don't publish
//     #[default]
//     Draft,
//     /// Needs Review
//     NeedsReview,
//     /// Publishable
//     Publish,
// }
//
// /// Scaffold a new post for your blog
// #[derive(Parser, Debug)]
// #[clap(version)]
// struct Args {
//     /// The layout the post should use
//     // `default_value_t` uses the Default trait (our #[default] variant)
//     // `value_enum` tells Clap this is an enum to parse
//     #[clap(short, long, default_value_t, value_enum)]
//     layout: Layout,
//
//     /// Tags to include
//     #[clap(short, long = "tag")]
//     tags: Vec<String>,
//
//     /// The title of the post.
//     ///
//     /// If not provided, the filename will be generated
//     #[clap(short = 'T', long, default_value = "A Post")]
//     title: String,
//
//     /// Should this post be published?
//     #[clap(short, long, default_value_t, value_enum)]
//     status: PostStatus,
//
//     /// Where to put the file
//     #[clap(short, long, default_value = "content")]
//     output_dir: PathBuf,
// }
//
// #[derive(Debug, Serialize)]
// struct Frontmatter {
//     layout: Layout,
//     tags: Vec<String>,
//     status: PostStatus,
//     title: String,
//     slug: String,
// }
//
// fn main() {
//     let args = Args::parse();
//
//     if !args.output_dir.exists() {
//         let mut cmd = Args::command();
//         cmd.error(
//             ErrorKind::ValueValidation,
//             format!("output directory `{}` doesn't exist", args.output_dir.display()),
//         )
//         .exit();
//     }
//
//     let post_slug = slug::slugify(&args.title);
//
//     let mut filename = args.output_dir.join(&post_slug);
//     filename.set_extension("md");
//
//     let frontmatter = Frontmatter {
//         layout: args.layout,
//         tags: args.tags,
//         status: args.status,
//         title: args.title.clone(),
//         slug: post_slug,
//     };
//
//     let yaml = serde_yaml::to_string(&frontmatter)
//         .expect("Failed to serialize frontmatter");
//
//     let file_contents = format!("{yaml}---\n\n# {}", args.title);
//
//     if let Err(error) = fs::write(&filename, file_contents) {
//         let mut cmd = Args::command();
//         cmd.error(
//             ErrorKind::Io,
//             format!("failed to write file at `{}`\n\t{}", filename.display(), error),
//         )
//         .exit();
//     }
// }
