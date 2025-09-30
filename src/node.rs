use std::cell::RefCell;
use std::rc::{Rc, Weak};
pub type NodeRef = Rc<RefCell<Node>>;
type WeakNodeRef = Weak<RefCell<Node>>;

pub enum NodeType {
    Directory { children: Vec<NodeRef> },
    File { size: u64 },
}

pub struct Node {
    pub name: String,
    pub node_type: NodeType,

    pub parent: Option<WeakNodeRef>,
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
