use std::cell::RefCell;
use std::rc::{Rc, Weak};

type NodeRef = Rc<RefCell<Node>>;
type WeakNodeRef = Weak<RefCell<Node>>;

struct FileSystem {
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

    pub fn create_directory(&self, parent: &NodeRef, name: &str) -> NodeRef {
        let directory = Node::new_directory(name);
        self.add_child(parent, &directory);

        directory
    }

    pub fn create_file(&self, parent: &NodeRef, name: &str, size: u64) -> NodeRef {
        let file = Node::new_file(name, size);
        self.add_child(parent, &file);

        file
    }

    fn add_child(&self, parent: &NodeRef, child: &NodeRef) {
        if let NodeType::Directory { children } = &mut parent.borrow_mut().node_type {
            children.push(Rc::clone(child));

            child.borrow_mut().parent = Some(Rc::downgrade(parent));
        } else {
            panic!("files cannot have children")
        }
    }

    fn change_dir(&mut self, name: &str) -> Result<(), String> {
        let target = self.resolve_path(name)?;

        if !target.borrow().is_directory() {
            return Err("Target is not a directory".to_string());
        }

        self.current_dir = target;

        Ok(())
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
            dir => {
                let child = self.find_child(dir);

                child.ok_or_else(|| "No directory found")?
            }
        };

        Ok(current)
    }

    fn find_child(&self, name: &str) -> Option<NodeRef> {
        if let NodeType::Directory { children } = &self.current_dir.borrow().node_type {
            let child = children
                .iter()
                .find(|&e| e.borrow().name == name)
                .map(|e| e.clone());

            child
        } else {
            None
        }
    }
}

pub enum NodeType {
    Directory { children: Vec<NodeRef> },
    File { size: u64 },
}

pub struct Node {
    name: String,
    node_type: NodeType,

    parent: Option<WeakNodeRef>,
}

impl Node {
    pub fn new_file(name: &str, size: u64) -> NodeRef {
        Rc::new(RefCell::new(Node {
            name: name.to_string(),
            node_type: NodeType::File { size },
            parent: None,
        }))
    }

    pub fn new_directory(name: &str) -> NodeRef {
        Rc::new(RefCell::new(Node {
            name: name.to_string(),
            node_type: NodeType::Directory {
                children: Vec::new(),
            },
            parent: None,
        }))
    }

    pub fn is_directory(&self) -> bool {
        matches!(self.node_type, NodeType::Directory { .. })
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
}
