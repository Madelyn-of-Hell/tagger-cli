mod operation;

use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use clap::{Arg, ArgAction, Command};
use directories::ProjectDirs;
use crate::operation::{AddTag, Operation, RemoveTag, SearchTags, FileTag};

fn main() -> Result<(), TagError> {
    let app = App::parse()?;
    app.execute()?;
    Ok(())
}
struct App {
    operation: Operation,
}

impl App {
    pub fn reset() {
        fs::remove_dir_all(App::dir());
        fs::create_dir(App::dir());
    }

    pub fn dir() -> PathBuf {
        ProjectDirs::from("com", "madelyn_belmen","tagger")
            .expect("Couldn't create ProjectDirs object")
            .data_dir()
            .to_path_buf()
    }

    fn new(operation: Operation) -> Self {
        Self { operation }
    }

    fn parse() -> Result<Self, TagError> {
        let args = Command::new("Tagger")
            .about("Command line interface for a tag-based filesystem designed by Madelyn Belmen")
            .subcommands([
                Command::new("search")
                    .about("Search files by multiple fields")
                    .args([
                        Arg::new("Tag").short('t').long("tag").required(false).action(ArgAction::Append).help("Filter for the given tag"),
                        Arg::new("Extension").short('e').long("extension").required(false).action(ArgAction::Set).help("Filter by file extension"),
                        Arg::new("Name").short('n').long("name").required(false).action(ArgAction::Set).help("A name (or name fragment) to search for"),
                    ]),
                Command::new("add")
                    .about("Add a new tag, either as a head or a child tag.")
                    .args([
                        Arg::new("Name").short('n').long("name").required(true).action(ArgAction::Set).help("The name of the new tag"),
                        Arg::new("Parent").short('p').long("parent").required(false).action(ArgAction::Set).help("The new tag's parent"),
                    ]),
                Command::new("remove")
                    .about("Remove a tag")
                    .args([
                        Arg::new("Name").short('n').long("name").required(true).action(ArgAction::Set).help("The name of the tag to be removed"),
                    ]),
                Command::new("tag")
                    .about("Apply a tag to a file.")
                    .args([
                        Arg::new("File").short('f').long("file").required(true).action(ArgAction::Set).help("The file to tag"),
                        Arg::new("Tag").short('t').long("tag").required(true).action(ArgAction::Append).help("The tag(s) to apply"),
                        Arg::new("Correct").short('c').long("correct").required(false).action(ArgAction::SetTrue)
                    ])

            ]);
        match args.get_matches().subcommand() {
            Some(("search", search_args)) => {
                let mut search_op = SearchTags::default();
                if let Some(tags) = search_args.get_many::<String>("Tag") {
                    tags.into_iter().for_each(|tag| search_op.add_tag(tag));
                }
                if let Some(file_extension) = search_args.get_one::<String>("Extension") {
                    search_op.name_snippet = Some(file_extension.clone());
                }
                if let Some(name) = search_args.get_one::<String>("Name") {
                    search_op.name_snippet = Some(name.clone());
                }
                let op_wrapper = Operation::Search(search_op);
                let app = App::new(op_wrapper);
                Ok(app)
            }
            Some(("add", add_args)) => {
                if let Some(name) = add_args.get_one::<String>("Name") {
                    let mut add_op = AddTag::new(name.clone(), None);
                    if let Some(parent) = add_args.get_one::<String>("Parent") {
                        add_op.parent = Some(parent.clone());
                    }
                    let op_wrapper = Operation::Add(add_op);
                    let app = App::new(op_wrapper);

                    Ok(app)
                } else {Err(TagError::new(String::from("'add' operation requires a name.")))}
            }
            Some(("remove", remove_args)) => {
                if let Some(name) = remove_args.get_one::<String>("Name") {
                    let remove_op = RemoveTag::new(name.clone());
                    let op_wrapper = Operation::Remove(remove_op);
                    let app = App::new(op_wrapper);

                    Ok(app)
                } else {
                    Err(tag_error!("'remove' operation requires a tag name to remove."))
                }
            }
            Some(("tag", tag_args)) => {
                if let Some(file) = tag_args.get_one::<String>("File") {
                    if let Some(tags) = tag_args.get_many::<String>("Tag") {
                        let tag_op = FileTag::new(
                            PathBuf::from(file),
                            tags.map(|tag| PathBuf::from(tag)).collect::<Vec<PathBuf>>(),
                            tag_args.get_flag("Correct")
                        );
                        let op_wrapper = Operation::Tag(tag_op);
                        let app = App::new(op_wrapper);
                        Ok(app)
                    } else {
                        Err(tag_error!("You must use at least one tag!"))
                    }
                } else {
                    Err(tag_error!("You need to specify a path for the file to tag!"))
                }
            },
            _ => {unreachable!()},
        }
    }

    fn execute(&self) -> Result<(), TagError> {
        match &self.operation {
            Operation::Add(op) => {op.execute()?}
            Operation::Remove(op) => {op.execute()?}
            Operation::Search(op) => {
                let search_results = op.execute()?;
                for search_result in search_results {
                    if let Some(result_name) = search_result.file_name() {
                        println!("Found entry: {}", result_name.to_string_lossy().to_string())
                    }
                }
                ()
            },
            Operation::Tag(op) => {op.execute()?}
        }
        Ok(())
    }
}
#[derive(Debug)]
#[derive(PartialEq)]
struct TagError {
    error: String,
}
impl TagError {
    fn new(error: String) -> Self {
        Self { error }
    }
}
impl Display for TagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

#[macro_export]
macro_rules! tag_error {
    ($error:expr) => {
        TagError::new(String::from($error))
    }
}