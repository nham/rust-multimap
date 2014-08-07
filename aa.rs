use std::mem::{replace, swap};
use std::fmt::Show;

type Link<T> = Option<Box<T>>;

pub struct Tree<K, V> {
    root: Link<Node<K, V>>,
    size: uint
}

struct Node<K, V> {
    key: K,
    value: V,
    left: Link<Node<K, V>>,
    right: Link<Node<K, V>>,
    level: uint
}

impl<K: Ord, V> Node<K, V> {
    pub fn new(key: K, value: V) -> Node<K, V> {
        Node { key: key, value: value, left: None, right: None, level: 0 }
    }
}
 
// Remove left horizontal link by rotating right
/*
     a      b
    /        \
   b    =>    a
    \        /
     c      c

  provided that a.level == b.level
*/
fn skew<K: Ord, V>(node: &mut Box<Node<K, V>>) {
    if node.left.is_some() && node.left.get_ref().level == node.level {
        let mut save = node.left.take_unwrap();
        swap(&mut node.left, &mut save.right); // save.right now None
        swap(node, &mut save);
        node.right = Some(save);
    }
}

 
// Remove dual horizontal link by rotating left and increasing level of
// the parent
/*
    a            b
     \          / \
      b    =>  a   c
     / \        \
    d   c        d

  provided that a.level == c.level
*/
fn split<K: Ord, V>(node: &mut Box<Node<K, V>>) {
    if node.right.as_ref().map_or(false,
      |x| x.right.is_some() && x.right.get_ref().level == node.level) {
        let mut save = node.right.take_unwrap();
        swap(&mut node.right, &mut save.left); // save.left now None
        save.level += 1;
        swap(node, &mut save);
        node.left = Some(save);
    }
}

impl<K: Ord, V> Tree<K, V> {
    // standard binary search tree lookup, only iterative instead of recursive
    fn find<'a>(&'a self, key: &K) -> Option<&'a V> {
        let mut current: &Link<Node<K, V>> = &self.root;
        loop {
            match *current {
                Some(ref r) => {
                    match key.cmp(&r.key) {
                        Less => current = &r.left,
                        Greater => current = &r.right,
                        Equal => return Some(&r.value)
                    }
                }
                None => return None
            }
        }
    }

    // returns `Some(v)` iff `v` was already associated with `key`
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut current = &mut self.root;
        let mut path: Vec<&mut Box<Node<K,V>>> = vec!();
        loop {
            match *current {
                None => {
                    *current = Some(box Node::new(key, value));
                    loop { // skew/split all the way up the tree
                        match path.pop() {
                            None => break,
                            Some(n) => { skew(n); split(n); }
                        }
                    }
                    return None;
                },
                Some(ref mut n) => {
                    match key.cmp(&n.key) {
                        Less => {
                            path.push(n);
                            current = &mut n.left;
                        },
                        Greater => {
                            path.push(n);
                            current = &mut n.right;
                        },
                        Equal => {
                            n.key = key;
                            return Some(replace(&mut n.value, value));
                        },
                    }
                },
            }
        }
        None
    }
}

fn print_node_level<K: Show, V: Show>(node: &Link<Node<K,V>>, level: uint) {
    let mut pre = "".to_string();
    if level > 0 {
        for i in range(0, level - 1) {
            pre = pre + "   ";
        }

        pre = pre + " - ";
    }

    match *node {
        Some(ref n) => {
            println!("{}{}: {}", pre, n.key, n.value);
            print_node_level(&n.left, level + 1);
            print_node_level(&n.right, level + 1);
        },
        None => println!("{}", pre),
    }
}

fn main() {
}
