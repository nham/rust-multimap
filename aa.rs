// First shot at an AA tree. Mostly copied from libcollections/treemap.rs
type Link<T> = Option<Box<T>>;

pub struct AATree<K, V> {
    root: Link<AANode<K, V>>,
    length: uint
}

struct AANode<K, V> {
    key: K,
    value: V,
    left: Link<AANode<K, V>>,
    right: Link<AANode<K, V>>,
    level: uint
}

impl<K: Ord, V> AANode<K, V> {
    pub fn new(key: K, value: V) -> AANode<K, V> {
        AANode { key: key, value: value, left: None, right: None, level: 1 }
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
fn skew<K: Ord, V>(node: &mut Box<AANode<K, V>>) {
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
fn split<K: Ord, V>(node: &mut Box<AANode<K, V>>) {
    if node.right.as_ref().map_or(false,
      |x| x.right.is_some() && x.right.get_ref().level == node.level) {
        let mut save = node.right.take_unwrap();
        swap(&mut node.right, &mut save.left); // save.left now None
        save.level += 1;
        swap(node, &mut save);
        node.left = Some(save);
    }
}

impl<K: Ord, V> AATree<K, V> {
    // standard binary search tree lookup
    fn find<'a>(&'a self, key: &K) -> Option<&'a V> {
        let mut current: &Link<AANode<K, V>> = &self.root;
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

    fn insert<K: Ord, V>(&mut self, key: K, value: V) -> Option<V> {
        match self.root {
            None => {
                self.root = Some(box AANode::new(key, value));
                None
            },
            Some(ref mut node) => {
                match key.cmp(&node.key) {
                    Less => {
                        let inserted = ins
                    },
                    Greater => {
                    },
                    Equal => {
                    },
                }
            },
        }
    }
}

fn main() {
}
