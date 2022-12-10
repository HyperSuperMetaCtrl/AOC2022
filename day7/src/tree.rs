use anyhow::{anyhow, Result};
use std::cell::{Ref, RefCell, RefMut};

#[derive(Debug)]
pub struct Tree<T> {
    arena: Vec<Option<RefCell<Node<T>>>>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct NodeId {
    pub index: usize,
}

#[derive(Debug)]
pub struct Node<T> {
    id: NodeId,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
    pub data: T,
}

impl<T> Node<T> {
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn children(&self) -> &Vec<NodeId> {
        &self.children
    }
    pub fn parent(&self) -> &Option<NodeId> {
        &self.parent
    }
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Self { arena: vec![] }
    }
    pub fn add_node(&mut self, data: T) -> NodeId {
        let index = self.arena.len();
        let node = Node {
            id: NodeId { index },
            children: Vec::new(),
            parent: None,
            data,
        };
        self.arena.push(Some(RefCell::from(node)));
        NodeId { index }
    }
    pub fn add_child(&mut self, parent_id: NodeId, child_id: NodeId) -> Result<()> {
        let Some(mut parent) = self.get_mut(parent_id) else {
            return Err(anyhow!("Parent not Found"));
        };
        let Some(mut child) = self.get_mut(child_id) else {
            return Err(anyhow!("Child not Found"));
        };
        if child.parent.is_some() {
            return Err(anyhow!("Child already has a parent, detach first"));
        }
        child.parent = Some(parent_id);
        if !parent.children.contains(&child_id) {
            parent.children.push(child_id);
        }
        Ok(())
    }
    pub fn children(&self, id: NodeId) -> Vec<NodeId> {
        let Some(parent) = self.get(id) else {
            return vec![];
        };
        parent.children.clone()
    }
    pub fn parent(&self, id: NodeId) -> Option<NodeId> {
        self.get(id)?.parent
    }
    pub fn get(&self, id: NodeId) -> Option<Ref<Node<T>>> {
        let node = self.arena.get(id.index)?;
        if let Some(node) = node {
            Some(node.borrow())
        } else {
            None
        }
    }
    pub fn get_mut(&self, id: NodeId) -> Option<RefMut<Node<T>>> {
        let node = self.arena.get(id.index)?;
        if let Some(node) = node {
            Some(node.borrow_mut())
        } else {
            None
        }
    }
    pub fn add_children(&mut self, parent: NodeId, children: &[NodeId]) -> Result<()> {
        for child in children {
            self.add_child(parent, *child)?;
        }
        Ok(())
    }
}