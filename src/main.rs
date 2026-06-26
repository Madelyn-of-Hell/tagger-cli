mod operation;

use clap::{Arg, ArgAction, Command};
use crate::operation::{AddTag, Operation, RemoveTag, SearchTags};

fn main() -> Result<(), TagError> {
    let app = App::parse()?;
    Ok(())
}
struct App {
    operation: Operation,
}
impl App {
    fn new(operation: Operation) -> Self { Self { operation } }

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
                    Err(TagError::new(String::from("'remove' operation requires a tag name to remove.")))
                }
            }
            other => Err(TagError::new(format!("Unknown command: {:?}", other))),
        }
    }
}
#[derive(Debug)]
struct TagError {
    error: String,
}
impl TagError {
    fn new(error: String) -> Self {
        Self { error }
    }
}