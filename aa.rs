use std::mem::{replace, swap, transmute};
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
        Node { key: key, value: value, left: None, right: None, level: 1 }
    }

    fn max(&self) -> &K {
        match self.right {
            None => &self.key,
            Some(ref n) => n.right.get_ref().max(),
        }
    }

    fn min(&self) -> &K {
        match self.left {
            None => &self.key,
            Some(ref n) => n.left.get_ref().min(),
        }
    }

    fn is_bst(&self) -> bool {
        let check_left = match self.left {
            None => true,
            Some(ref n) => (*n).is_bst() && *(*n).max() < self.key,
        };

        if check_left {
            match self.right {
                None => true,
                Some(ref n) => (*n).is_bst() && *(*n).min() > self.key,
            }
        } else { false }
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
    fn new() -> Tree<K, V> {
        Tree { root: None, size: 0 }
    }

    fn is_bst(&self) -> bool {
        match self.root {
            None => true,
            Some(ref r) => (*r).is_bst()
        }
    }

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
        let mut current = &mut self.root as *mut Link<Node<K,V>>;
        let mut path: Vec<*mut Box<Node<K,V>>> = vec!();
        loop { unsafe {
            match *current {
                None => {
                    *current = Some(box Node::new(key, value));
                    loop { // skew/split all the way up the tree
                        match path.pop() {
                            None => break,
                            Some(n) => {
                                let n: &mut Box<Node<K,V>> = transmute(n);
                                skew(n);
                                split(n);
                            }
                        }
                    }
                    self.size += 1;
                    return None;
                },
                Some(ref mut n) => {
                    match key.cmp(&n.key) {
                        Less => {
                            path.push(n as *mut Box<Node<K,V>>);
                            current = &mut n.left as *mut Link<Node<K,V>>;
                        },
                        Greater => {
                            path.push(n as *mut Box<Node<K,V>>);
                            current = &mut n.right as *mut Link<Node<K,V>>;
                        },
                        Equal => {
                            n.key = key;
                            return Some(replace(&mut n.value, value));
                        },
                    }
                },
            }
        }}
    }
}

fn print_node_depth<K: Show, V: Show>(node: &Link<Node<K,V>>, depth: uint) {
    let mut pre = "".to_string();
    if depth > 0 {
        for i in range(0, depth - 1) {
            pre = pre + "   ";
        }

        pre = pre + " - ";
    }

    match *node {
        Some(ref n) => {
            println!("{}{}: {}", pre, n.key, n.value);
            print_node_depth(&n.left, depth + 1);
            print_node_depth(&n.right, depth + 1);
        },
        None => println!("{}", pre),
    }
}

fn print_tree<K: Show, V: Show>(tree: &Tree<K, V>) {
    print_node_depth(&tree.root, 0);
    println!("------------");
}

fn main() {
    let mut t = Tree::new();
    t.insert('e', 5u);

    println!("{} {}", t.find(&'b'), t.find(&'c'));

    println!("printing things:\n------------------");

    print_tree(&t);

    t.insert('b', 88u);
    print_tree(&t);

    t.insert('d', 11u);
    print_tree(&t);

    println!("{}", t.is_bst());
}
