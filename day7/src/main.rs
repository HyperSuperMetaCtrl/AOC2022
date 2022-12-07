use anyhow::Result;
use std::{cell::RefCell, env, fs::read_to_string, rc::Rc, rc::Weak};

const FILENAME: &str = "input.txt";

#[derive(Debug)]
enum Command<'a> {
    LS(Vec<&'a str>),
    CD(&'a str),
}
fn parse_input(input: &str) -> Vec<Command> {
    let input: Vec<Vec<&str>> = input
        .split("$ ")
        .skip(1)
        .map(|x| x.lines().collect()) // split output at newlines
        .collect();

    input
        .into_iter()
        .map(|v| match v[0] {
            "ls" => Command::LS(v.into_iter().skip(1).collect::<Vec<&str>>()), //get files and dirs
            "cd .." => Command::CD(".."),
            _ => Command::CD(v[0].split(" ").nth(1).unwrap()), //get destination
        })
        .collect()
}
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() >= 2 { &args[1] } else { FILENAME };
    let input = read_to_string(path)?;
    let parsed = parse_input(&input);
    dbg!(parsed);

    Ok(())
}

struct Element<'a> {
    name: &'a str,
    size: Option<usize>,
}

enum File<'a> {
    File(Element<'a>),
    Directory(Element<'a>),
}

struct Tree<T> {
    nodes: Vec<Node<T>>,
}

struct NodeId {
    index: usize,
}

struct Node<T> {
    parent: Option<Weak<RefCell<Node<T>>>>,
    children: Vec<Rc<RefCell<Node<T>>>>,
    data: T,
}

impl<T> Tree<T> {
    fn new() -> Self {
        Tree { nodes: Vec::new() }
    }
    fn get(&self, id: NodeId) -> Option<&Node<T>> {
        self.nodes.get(id.index)
    }
}
