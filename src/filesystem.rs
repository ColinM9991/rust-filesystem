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
        self.add_child(&directory);

        directory
    }

    pub fn create_file(&self, name: &str, size: u64) -> NodeRef {
        let file = Node::new_file(name, size);
        self.add_child(&file);

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

    pub fn get_size(&self) -> u64 {
        self.root.borrow().get_size()
    }

    pub fn get_root(&self) -> &NodeRef {
        &self.root
    }

    fn add_child(&self, child: &NodeRef) {
        match &mut self.current_dir.borrow_mut().node_type {
            NodeType::Directory { children } => {
                children.push(Rc::clone(child));

                child.borrow_mut().set_parent(&self.current_dir);
            }
            NodeType::File { .. } => panic!("files cannot have children"),
        }
    }

    fn resolve_path(&self, path: &str) -> Result<NodeRef, String> {
        let current = match path {
            "/" => Rc::clone(&self.root),
            ".." => self
                .current_dir
                .borrow()
                .get_parent()
                .map_or(Rc::clone(&self.root), |e| e),
            dir => self.find_child(dir).ok_or_else(|| "No directory found")?,
        };

        Ok(current)
    }

    fn find_child(&self, name: &str) -> Option<NodeRef> {
        self.current_dir
            .borrow()
            .get_children()?
            .iter()
            .find(|e| e.borrow().name == name)
            .map(|e| Rc::clone(e))
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

    #[test]
    fn change_dir_parent_from_root_stays_at_root() {
        let mut fs = FileSystem::new();

        let res = fs.change_dir("..");
        assert!(res.is_ok());
        assert!(Rc::ptr_eq(&fs.root, &fs.current_dir));
    }
}
