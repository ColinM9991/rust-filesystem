use crate::node::{Node, NodeRef, NodeType};
use std::rc::Rc;

pub struct FileSystem {
    root: NodeRef,
    current_dir: NodeRef,
}

impl FileSystem {
    pub fn new() -> Self {
        let root = Node::new_directory("/");

        FileSystem {
            root: Rc::clone(&root),
            current_dir: root,
        }
    }

    pub fn create_directory(&self, name: &str) -> NodeRef {
        let directory = Node::new_directory(name);
        self.add_child(&self.current_dir, &directory);

        directory
    }

    pub fn create_file(&self, name: &str, size: u64) -> NodeRef {
        let file = Node::new_file(name, size);
        self.add_child(&self.current_dir, &file);

        file
    }

    pub fn change_dir(&mut self, name: &str) -> Result<(), String> {
        let target = self.resolve_path(name)?;

        if !target.borrow().is_directory() {
            return Err("Target is not a directory".to_string());
        }

        self.current_dir = target;

        Ok(())
    }

    fn add_child(&self, parent: &NodeRef, child: &NodeRef) {
        if let NodeType::Directory { children } = &mut parent.borrow_mut().node_type {
            children.push(Rc::clone(child));

            child.borrow_mut().parent = Some(Rc::downgrade(parent));
        } else {
            panic!("files cannot have children")
        }
    }

    fn resolve_path(&self, path: &str) -> Result<NodeRef, String> {
        let current = match path {
            "/" => Rc::clone(&self.root),
            ".." => self
                .current_dir
                .borrow()
                .parent
                .as_ref()
                .and_then(|t| t.upgrade())
                .ok_or_else(|| "Already at root directory".to_string())?,
            dir => self.find_child(dir).ok_or_else(|| "No directory found")?,
        };

        Ok(current)
    }

    fn find_child(&self, name: &str) -> Option<NodeRef> {
        if let NodeType::Directory { children } = &self.current_dir.borrow().node_type {
            children
                .iter()
                .find(|&e| e.borrow().name == name)
                .map(|e| Rc::clone(e))
        } else {
            None
        }
    }

    pub fn get_size(&self) -> u64 {
        self.root.borrow().get_size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filesystem_current_dir_equal_root() {
        let fs = FileSystem::new();

        assert!(Rc::ptr_eq(&fs.root, &fs.current_dir));
    }

    #[test]
    fn change_dir_valid_directory() {
        let mut fs = FileSystem::new();
        let home = fs.create_directory("home");

        let res = fs.change_dir(&home.borrow().name);
        assert!(res.is_ok());
        assert!(Rc::ptr_eq(&home, &fs.current_dir));
    }

    #[test]
    fn change_dir_invalid_file() {
        let mut fs = FileSystem::new();

        let home = fs.create_directory("home");
        let res = fs.change_dir(&home.borrow().name);
        assert!(res.is_ok());
        assert!(Rc::ptr_eq(&home, &fs.current_dir));

        let bashrc = fs.create_file(".bashrc", 10);
        let res = fs.change_dir(&bashrc.borrow().name);
        assert!(res.is_err());
        assert!(Rc::ptr_eq(&home, &fs.current_dir));
    }
}
