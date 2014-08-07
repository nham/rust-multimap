use std::mem::replace;

enum Node<K, V> {
    Leaf,
    Internal(InternalNode<K, V>)
}

impl<K: Ord, V> Node<K, V> {
    fn find(&self, key: &K) -> Option<&V> {
        match *self {
            Leaf => None,
            Internal(ref node) =>
                match key.cmp(&node.key) {
                    Less => node.left.find(key),
                    Greater => node.right.find(key),
                    Equal => Some(&node.val),
                },
        }
    }

    // returns `Some(v)` iff `v` was already associated with `key`
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        match *self {
            Leaf => {
                let col = Black; // FIXME: this is way wrong
                let new = InternalNode { color: col, key: key, val: value,
                                         left: box Leaf, right: box Leaf };
                *self = Internal(new);
                None
            },
            Internal(ref mut n) =>
                match key.cmp(&n.key) {
                    Less => n.left.insert(key, value),
                    Greater => n.right.insert(key, value),
                    Equal => Some(replace(&mut n.val, value)),
                },
        }
    }
}

struct InternalNode<K, V> {
    color: Color,
    key: K,
    val: V,
    left: Box<Node<K, V>>,
    right: Box<Node<K, V>>,
}

struct Tree<K, V> {
    root: Option<Node<K, V>>
}

enum Color {
    Red,
    Black
}

impl<K: Ord, V> Tree<K, V> {
    fn find(&self, key: &K) -> Option<&V> {
        match self.root {
            None => None,
            Some(ref n) => n.find(key)
        }
    }
}

fn main() {

}
