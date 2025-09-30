use std::cell::RefCell;
use std::rc::{Rc, Weak};

type NodeRef = Rc<RefCell<Node>>;
type WeakNodeRef = Weak<RefCell<Node>>;

struct FileSystem {
    root: NodeRef,
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            root: Node::new_directory("/"),
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_children() {
        let fs = FileSystem::new();
        let home = fs.create_directory(&fs.root, "/home");
        let bashrc = fs.create_file(&home, ".bashrc", 10);

        assert!(bashrc.borrow().parent.is_some())
    }
}
