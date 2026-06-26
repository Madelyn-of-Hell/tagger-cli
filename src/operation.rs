use std::ops::Add;
use std::path::PathBuf;
use crate::TagError;

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Search(SearchTags),
    Add(AddTag),
    Remove(RemoveTag),

}
impl Operation {
    pub fn execute(&self) -> Result<(),TagError> {
        match self {
            Operation::Search(search_tags) => search_tags.execute(),
            Operation::Add(add_tag) => add_tag.execute(),
            Operation::Remove(remove_tag) => remove_tag.execute()
        }
        Ok(())
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct SearchTags {
    pub tags: Vec<PathBuf>,
    pub extension: Option<String>,
    pub name_snippet: Option<String>,
}

impl SearchTags {
    pub fn add_tag(&mut self, tag: &String) {
        todo!()
    }
    pub fn execute(&self) {}
}

impl Default for SearchTags {
    fn default() -> Self {
        Self {tags: vec![],extension: None,name_snippet: None}
    }
}
impl SearchTags {
    pub fn new(tags: Vec<PathBuf>, extension: Option<String>, name_snippet: Option<String>) -> Self {
        Self {tags, extension, name_snippet}
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AddTag{
    pub tag: String,
    pub parent: Option<String>,
}
impl AddTag {
    pub fn new(tag: String, parent: Option<String>) -> Self {
        Self { tag, parent }
    }
    pub fn execute(&self) {}
}
#[derive(Debug, PartialEq, Eq)]
pub struct RemoveTag{
    pub tag: String,
}

impl RemoveTag {
    pub fn new(tag: String) -> Self {
        Self { tag }
    }
    pub fn execute(&self) {}
}