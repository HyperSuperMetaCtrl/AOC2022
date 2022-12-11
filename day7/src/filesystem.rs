use anyhow::{anyhow, Result};
use day7::tree::*;
use thiserror::Error;

#[derive(Debug)]
pub struct File {
    pub is_dir: bool,
    pub name: String,
    pub size: Option<usize>,
}

#[derive(Debug)]
pub struct FileSystem {
    file_tree: Tree<File>,
    pwd: NodeId,
    root: NodeId,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("File not found")]
    FileNotFoundError,
    #[error("File already exists")]
    FileAlreadyExistsError,
    #[error("File name not allowed")]
    FileNameNotAllowError,
    #[error("Operation not permitted")]
    OperationNotPermittedError,
}

impl FileSystem {
    pub fn new() -> Self {
        let file = File {
            is_dir: true,
            name: "/".to_string(),
            size: None,
        };
        let mut file_tree = Tree::new();
        let pwd = file_tree.add_node(file);
        Self {
            file_tree,
            pwd,
            root: pwd,
        }
    }

    pub fn cd(&mut self, file_name: &str) -> Result<(), Error> {
        match file_name {
            "/" => {
                self.pwd = self.root;
                return Ok(());
            }
            ".." => {
                self.pwd = self
                    .file_tree
                    .parent(self.pwd)
                    .ok_or(Error::FileNotFoundError)?;
                return Ok(());
            }
            _ => (),
        };
        let pwd = self.file_tree.get(self.pwd).unwrap();
        let children = pwd.children();

        for child in children {
            if let Some(c) = self.file_tree.get(*child) {
                if c.data.name == file_name && c.data.is_dir {
                    self.pwd = *child;
                    return Ok(());
                }
            }
        }
        Err(Error::FileNotFoundError)
    }
    pub fn mkfile(&mut self, file: &str, size: usize) -> Result<(), Error> {
        self.make_element(file, false, Some(size))
    }
    pub fn mkdir(&mut self, file: &str) -> Result<(), Error> {
        self.make_element(file, true, None)
    }
    fn make_element(&mut self, file: &str, is_dir: bool, size: Option<usize>) -> Result<(), Error> {
        if file == "/" {
            return Err(Error::FileNameNotAllowError);
        };
        let parent = self.file_tree.get(self.pwd).unwrap().id();
        let children = self.file_tree.children(parent);
        for child in children {
            if let Some(c) = self.file_tree.get(child) {
                if c.data.name == file {
                    return Err(Error::FileAlreadyExistsError);
                }
            }
        }
        let child = self.file_tree.add_node(File {
            is_dir,
            name: file.to_string(),
            size,
        });
        self.file_tree
            .add_child(parent, child)
            .or(Err(Error::OperationNotPermittedError))?;
        Ok(())
    }
    fn update_sizes(fs: &mut Self, id: NodeId) {
        unimplemented!();
    }
}
