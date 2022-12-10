use anyhow::{anyhow, Result};
use day7::tree::*;

#[derive(Debug)]
pub struct File {
    is_dir: bool,
    name: String,
    size: Option<usize>,
}

#[derive(Debug)]
pub struct FileSystem {
    file_tree: Tree<File>,
    pwd: NodeId,
    root: NodeId,
}

impl FileSystem {
    pub fn new() -> Self {
        let mut file = File {
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

    pub fn cd(&mut self, file_name: &str) -> Result<()> {
        match file_name {
            "/" => {
                self.pwd = self.root;
                return Ok(());
            }
            ".." => {
                self.pwd = self
                    .file_tree
                    .parent(self.pwd)
                    .ok_or(anyhow!("no parent"))?;
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
        Err(anyhow!("directory not found"))
    }
    pub fn mkfile(&mut self, file: &str, size: usize) -> Result<()> {
        let parent = self.file_tree.get(self.pwd).unwrap().id();
        let children = self.file_tree.children(parent);
        for child in children {
            if let Some(c) = self.file_tree.get(child) {
                if c.data.name == file {
                    return Err(anyhow!("file already exitst"));
                }
            }
        }
        let child = self.file_tree.add_node(File {
            is_dir: false,
            name: file.to_string(),
            size: Some(size),
        });
        self.file_tree.add_child(parent, child)?;
        Ok(())
    }
    pub fn mkdir(&mut self, file: &str) -> Result<()> {
        let parent = self.file_tree.get(self.pwd).unwrap().id();
        let children = self.file_tree.children(parent);
        for child in children {
            if let Some(c) = self.file_tree.get(child) {
                if c.data.name == file {
                    return Err(anyhow!("file already exitst"));
                }
            }
        }
        let child = self.file_tree.add_node(File {
            is_dir: true,
            name: file.to_string(),
            size: None,
        });
        self.file_tree.add_child(parent, child)?;
        Ok(())
    }
}
