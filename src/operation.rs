use std::path::PathBuf;
use std::fs;
use std::fs::{DirEntry};
use crate::{tag_error, App, TagError};
#[cfg(windows)]
use std::os::windows::fs::symlink_file as symlink;
#[cfg(unix)]
use std::os::unix::fs::symlink as symlink;


#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Search(SearchTags),
    Add(AddTag),
    Remove(RemoveTag),
    Tag(FileTag)

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

    //TODO: make this not ugly as fuck
    pub fn execute(&self) -> Result<Vec<PathBuf>, TagError> {
        let dir = App::dir();
        for tag in &self.tags {
            if !dir.join(tag).exists() {
                return Err(tag_error!(format!("Tag {:?} does not exist.", tag)))}}
        let mut directories = self.tags.iter().filter_map(|tag| {
                fs::read_dir(dir.join(tag)).ok()
            })
            .map(|dir| dir.filter_map(|item| item.ok()).collect::<Vec<DirEntry>>())
            .collect::<Vec<Vec<DirEntry>>>();
        directories.sort_by(|a,b| a.len().cmp(&b.len()));
        let smallest = directories.last();
        if let Some(smallest) = smallest {
            let candidates = smallest.iter().filter_map(|entry| {
                let is_sym = entry.path().is_symlink();
                let ext_match = {
                    if let Some(search_extension) = &self.extension {
                        if let Some(given_extension) = entry.path().extension() {
                            let given_extension = given_extension.to_string_lossy().to_string();
                            search_extension == &given_extension
                        } else {false}
                    } else {true}
                };
                let snippet_match = {
                    if let Some(search_pattern) = &self.name_snippet {
                        if let Some(file_name) = entry.path().file_name() {
                            let file_name = file_name.to_string_lossy().to_string();
                            file_name.contains(search_pattern)
                        } else {false}
                    } else {true}
                };
                if is_sym && ext_match && snippet_match {
                    if directories.iter().all(|tag_members|
                        tag_members.iter().any(|file| file.path() == entry.path())
                    ) {
                        Some(entry)
                    } else {
                        None
                    }
                } else {
                    None
                }

            }).collect::<Vec<&DirEntry>>();
            return Ok(candidates.iter().map(|dir|
                dir.path().strip_prefix(App::dir()).expect("Something has gone wrong with the filesystem. pls dm me or make an issue").to_owned()).collect())
        }
        unreachable!("To make it here you have to have listed entirely real tags but also no tags")
    }
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
    pub fn execute(&self) -> Result<(), TagError> {
        let mut new_tag = App::dir();
        if let Some(parent) = &self.parent {
            new_tag.push(parent);
        }
        new_tag.push(&self.tag);

        if new_tag.is_dir() {
            return Err(tag_error!("Tag Already Exists!"))
        } else {
            if let Err(message) = fs::create_dir(new_tag) {
                return Err(tag_error!(format!("IO Error: {}", message.to_string())))
            }
        }
        Ok(())
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct RemoveTag{
    pub tag: String,
}

impl RemoveTag {
    pub fn new(tag: String) -> Self {
        Self { tag }
    }
    pub fn execute(&self) -> Result<(), TagError> {
        let mut tag = App::dir();
        tag.push(&self.tag);
        if tag.is_dir() {
            if let Err(message) = fs::remove_dir(tag) {
                return Err(tag_error!(format!("IO Error: {}", message.to_string())))
            }
        } else {
            return Err(tag_error!("Tag not found..."));
        }
        Ok(())
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct FileTag {
    path: PathBuf,
    tags: Vec<PathBuf>,
    correct: bool
}
impl FileTag {
    pub fn new(path: PathBuf, tags: Vec<PathBuf>, correct:bool) -> Self {
        Self { path: path.canonicalize().unwrap(), tags, correct}
    }
    pub fn execute(&self) -> Result<(), TagError> {
        let dir = App::dir();
        if !self.path.exists() {
            return Err(tag_error!(format!("File {:?} doesn't exist!", self.path)));
        }
        for tag in &self.tags {
            let absolute_tag_path = dir.join(&tag);
            if !&absolute_tag_path.exists() {
                if self.correct {
                    if let Err(error) = fs::create_dir_all(&absolute_tag_path) {
                        return Err(tag_error!(format!("IO Error: {} on tag {:?}", error.to_string(), &absolute_tag_path)))
                    }
                } else {
                    return Err(tag_error!(format!("Tag {:?} doesn't exist", tag)));
                }
            }
        }

        let file_name = self.path.file_name().expect("Path didn't end in a valid file.");
        for tag in &self.tags {
            let absolute_tag_path = dir.join(&tag);
            if let Err(error) = symlink(&self.path, absolute_tag_path.join(file_name)) {
                return Err(tag_error!(format!("IO Error: {}", error.to_string())))
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::App;
    use super::*;

    #[test]
    fn test_add_execute() {
        App::reset();
        let data_dir = App::dir();
        const TEST_ADD_TAG_NAME: &str = "add_test";
        // Test creating tag
        let add_task = AddTag::new(String::from(TEST_ADD_TAG_NAME), None);
        let result = add_task.execute();
        assert!(result.is_ok(), "Test add operation failed: {:?}", result.err().unwrap());
        assert!(data_dir.join(TEST_ADD_TAG_NAME).is_dir(), "Test tag didn't exist!");

        // Test executing the same function twice
        let repeat_result = add_task.execute();
        assert_eq!(tag_error!("Tag Already Exists!"), repeat_result.err().unwrap(), "Repeated action succeeded erroneously!");

        // Test creating child tag
        let child_tag = AddTag::new(String::from("child"), Some(String::from(TEST_ADD_TAG_NAME)));
        let child_result = child_tag.execute();
        assert!(child_result.is_ok(), "Child add operation failed!");
        assert!(data_dir.join(TEST_ADD_TAG_NAME).join("child").is_dir(), "Child tag didn't exist!");
        App::reset();
    }

    #[test]
    fn test_remove_execute() {
        App::reset();
        let data_dir = App::dir();
        const TEST_REMOVE_TAG_NAME: &str = "remove_tag";
        let new_tag = data_dir.join(TEST_REMOVE_TAG_NAME);

        // Setup
        fs::create_dir(data_dir.join(TEST_REMOVE_TAG_NAME)).unwrap();
        assert!(new_tag.is_dir(), "Test setup (creating the tag) somehow failed ;_;");

        let remove_action = RemoveTag::new(String::from(TEST_REMOVE_TAG_NAME));
        let removal_result = remove_action.execute();
        assert!(removal_result.is_ok(), "Valid remove operation failed!");
        assert!(!new_tag.exists(), "Tag wasn't removed!");

        let repeat_remove_action = remove_action.execute();
        assert_eq!(tag_error!("Tag not found..."), repeat_remove_action.err().unwrap(), "Repeated removal didn't throw the expected error!");
        App::reset();
    }

    #[test]
    fn test_search_execute() {
        App::reset();
        let data_dir = App::dir();
        const TEST_SEARCH_TAG_NAME: &str = "search_tag";
        let tag_dir = data_dir.join(TEST_SEARCH_TAG_NAME);
        fs::create_dir(&tag_dir).unwrap();
        const FILE1_NAME:&str = "file1.txt";
        fs::write(data_dir.join(FILE1_NAME), "").unwrap();
        symlink(data_dir.join(FILE1_NAME), tag_dir.join(FILE1_NAME)).unwrap();

        const FILE2_NAME:&str = "file2.md";
        fs::write(&data_dir.join(FILE2_NAME), "").unwrap();
        symlink(data_dir.join(FILE2_NAME), tag_dir.join(FILE2_NAME)).unwrap();


        let search_action_1 = SearchTags::new(vec![PathBuf::from(TEST_SEARCH_TAG_NAME)],None, None);
        let search_action_2 = SearchTags::new(vec![PathBuf::from(TEST_SEARCH_TAG_NAME)],Some(String::from("md")), None);
        let search_action_3 = SearchTags::new(vec![PathBuf::from(TEST_SEARCH_TAG_NAME)],None, Some(String::from("1")));

        assert_eq!(vec![PathBuf::from("search_tag/file1.txt"),PathBuf::from("search_tag/file2.md")], search_action_1.execute().unwrap(), "Failed to do a basic tag search");
        assert_eq!(vec![PathBuf::from("search_tag/file2.md")], search_action_2.execute().unwrap(), "Failed to filter a search by extension");
        assert_eq!(vec![PathBuf::from("search_tag/file1.txt")], search_action_3.execute().unwrap(), "Failed to filter a search by name snippet");
        App::reset();
    }

    #[test]
    fn test_tag_execute() {
        App::reset();

        let data_dir = App::dir();
        const TEST_SEARCH_TAG_NAME: &str = "file_tag";
        let file_tag_dir = data_dir.join(TEST_SEARCH_TAG_NAME);
        fs::create_dir(&file_tag_dir).unwrap();
        const FILENAME:&str = "file1.txt";
        let real_file_path = data_dir.join(FILENAME);
        fs::write(&real_file_path, "").unwrap();
        let tags = vec![PathBuf::from("tag_1"),PathBuf::from("tag_2"),PathBuf::from("tag_3")];
        let tag_file_action_1 = FileTag::new(real_file_path.clone(), tags.clone(), false);
        assert!(tag_file_action_1.execute().is_err(), "Tag operation succeeded where it should have failed due to invalid tags");
        let tag_file_action_2 = FileTag::new(real_file_path.clone(), tags.clone(), true);
        let action_2_result = tag_file_action_2.execute();
        assert!(action_2_result.is_ok(), "Tag operation failed where it should have succeeded: {}", action_2_result.err().unwrap());
        for tag in &tags {
            assert!(data_dir.join(tag).join(FILENAME).is_symlink())
        }

        App::reset();
        fs::write(&real_file_path, "").unwrap();
        fs::create_dir(data_dir.join("tag_1")).unwrap();
        fs::create_dir(data_dir.join("tag_2")).unwrap();
        fs::create_dir(data_dir.join("tag_3")).unwrap();

        let tag_file_action_3 = FileTag::new(data_dir.join(FILENAME), tags.clone(), false);
        assert!(tag_file_action_3.execute().is_ok(), "Tag operation failed where it should have succeeded");
        for tag in &tags {
            assert!(data_dir.join(tag).join(FILENAME).is_symlink())
        }
        App::reset();
    }
}