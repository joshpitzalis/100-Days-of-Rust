// use clap::Parser;
use camino::Utf8PathBuf;
use clap::{CommandFactory, Parser, error::ErrorKind};
use serde::Serialize;
use std::fs;

// use std::{fs, path::PathBuf};

// cargo add serde serde_yaml -F serde/derive

// #[derive(Parser, Debug)]
// struct Args {
//     #[clap(short, long, default_value = "post")]
//     layout: String,

//     #[clap(short, long = "tag")]
//     tags: Vec<String>,

//     #[clap(short = 'T', long, default_value = "A Post")]
//     title: String,

//     #[clap(short, long, default_value = "draft")]
//     status: String,

//     #[clap(short, long, default_value = "content")]
//     output_dir: String,
// }

/// Scaffold a new post for your blog
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// The layout the post should use
    #[clap(short, long, default_value = "post")]
    layout: Layout,

    /// Tags to include
    #[clap(short, long = "tag")]
    tags: Vec<String>,

    /// The title of the post.
    ///
    /// If not provided, the filename will be generated
    #[clap(short = 'T', long, default_value = "A Post")]
    title: String,

    /// Should this post be published?
    #[clap(short, long, default_value = "draft")]
    status: PostStatus,

    /// Where to put the file
    #[clap(short, long, default_value = "content")]
    // output_dir: String,
    // output_dir: PathBuf,
    output_dir: Utf8PathBuf,
}

// fn main() {
//     let args = Args::parse();
//     dbg!(args);
// }

fn main() {
    let args = Args::parse();
    // dbg!(&args);

    if !args.output_dir.exists() {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::ValueValidation,
            format!("output directory `{}` doesn't exist", args.output_dir),
        )
        .exit();
    }

    let post_slug = slug::slugify(args.title.clone());
    // dbg!(&post_slug);
    // let filename = format!("{}/{}.md", args.output_dir, args.title);
    let mut filename = args.output_dir.join(args.title.clone());
    filename.set_extension("md");

    let frontmatter = Frontmatter {
        layout: args.layout,
        tags: args.tags,
        status: args.status,
        title: args.title.clone(),
        // slug: slug::slugify(&args.title),
        slug: post_slug.clone(),
    };
    // println!("Creating file: {}...", filename.display());
    // println!("Creating file: {}...", post_slug.clone());

    // fs::write(filename, args.title).expect("Failed to create file");

    let yaml = serde_yaml::to_string(&frontmatter).expect("Failed to serialize frontmatter");
    // dbg!(yaml);

    let file_contents = format!("{yaml}\n---\n\n # {}", args.title);

    if let Err(error) = fs::write(filename.clone(), file_contents) {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::Io,
            format!(
                "failed to write file at `{filename}`\n\t{}",
                // filename.display(),
                error
            ),
        )
        .exit();
    }
}

// #[derive(Debug, Serialize)]
// struct Frontmatter {
//     layout: String,
//     tags: Vec<String>,
//     status: String,
//     title: String,
//     slug: String,
// }

#[derive(Debug, Serialize)]
struct Frontmatter {
    layout: Layout,
    tags: Vec<String>,
    status: PostStatus,
    title: String,
    slug: String,
}

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Layout {
    /// blog post
    #[default]
    Post,
    /// image gallery
    Gallery,
    /// code example
    Code,
}

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
enum PostStatus {
    /// Draft, don't publish
    #[default]
    Draft,
    /// Needs Review
    NeedsReview,
    /// Publishable
    Publish,
}
