use anyhow::{anyhow, Result};
use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell, RefMut};
use std::slice::Iter;

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

impl<'a, T> Node<T> {
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
    pub fn arena_iter<'a>(&'a self) -> ArenaIter<'a, T> {
        ArenaIter {
            vec_iter: self.arena.iter(),
        }
    }
    pub fn dfs_iter<'a>(&'a self, root: NodeId) -> DfsIterMut<'a, T> {
        DfsIterMut {
            tree: self,
            stack: vec![root],
            stack_out: vec![],
        }
    }
}
pub struct ArenaIter<'a, T> {
    vec_iter: Iter<'a, Option<RefCell<Node<T>>>>,
}

impl<'a, T> Iterator for ArenaIter<'a, T> {
    type Item = &'a Option<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.vec_iter.next()
    }
}

pub struct DfsIterMut<'a, T> {
    tree: &'a Tree<T>,
    stack: Vec<NodeId>,
    stack_out: Vec<NodeId>,
}
impl<'a, T> Iterator for DfsIterMut<'a, T> {
    type Item = Option<RefMut<'a, Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() {
            let node = self.stack.pop()?;
            self.stack_out.push(node);
            self.tree
                .children(node)
                .iter()
                .for_each(|child| self.stack.push(*child));
        }
        let id = self.stack_out.pop()?;
        Some(self.tree.get_mut(id))
    }
}
