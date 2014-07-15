struct LeafNode<K, V> {
    key: K,
    value: V,
}

struct InteriorNode<'a, K, V> {
    field1: &'a K, // smallest key for a leaf node descending from child 2
    field2: Option<&'a K>, // smallest key for a leaf node descending from child 3
    child1: Box<Node23<'a, K, V>>,
    child2: Box<Node23<'a, K, V>>,
    child3: Option<Box<Node23<'a, K, V>>>,
}

enum Node23<'a, K, V> {
    Leaf(LeafNode<K, V>),
    Interior(InteriorNode<'a, K, V>),
}

pub struct Tree23<'a, K, V> {
    root: Option<Node23<'a, K, V>>,
}

impl<'a, K, V> Tree23<'a, K, V> {
    pub fn new() -> Tree23<'a, K, V> { Tree23 { root: None } }

    fn leaf(key: K, value: V) -> Tree23<'a, K, V> {
        Tree23 { 
            root: Some( Leaf(LeafNode { key: key, value: value }) ) 
        }
    }
}

impl<K: Ord, V: Copy> LeafNode<K, V> {
    pub fn get(&self, key: &K) -> Option<V> {
        if key == &self.key {
            Some(self.value)
        } else {
            None
        }
    }
}

impl<'a, K: Ord, V: Copy> InteriorNode<'a, K, V> {
    pub fn get(&self, key: &K) -> Option<V> {
        match key.cmp(self.field1) {
            Less => self.child1.get(key),
            Equal => self.child2.get(key),
            Greater =>
                if self.child3.is_some() {
                    match key.cmp(self.field2.unwrap()) {
                        Less => self.child2.get(key),
                        _ => self.child3.get_ref().get(key),
                    }
                } else {
                    self.child2.get(key)
                }
        }
    }
}

impl<'a, K: Ord, V: Copy> Node23<'a, K, V> {
    fn get(&self, key: &K) -> Option<V> {
        match *self {
            Leaf(ref leaf) => leaf.get(key),
            Interior(ref node) => node.get(key),
        }
    }

    /*
    fn insert(&mut self, key: K, value: V) {
        match *self {
            Node2(ref mut n) =>
                match key.cmp(&n.key) {
                    Equal => n.value = value,
                    Less =>  {
                        insertlink(&mut n.left, key, value);
                        n.size += 1;
                    },
                    Greater => {
                        insertlink(&mut n.right, key, value);
                        n.size += 1;
                    }
                },
            Node3(ref mut n) => {}
        }
    }
    */
}




impl<'a, K: Ord, V: Copy> Tree23<'a, K, V> {
    pub fn get(&self, key: &K) -> Option<V> {
        match self.root {
            None => None,
            Some(ref n) => n.get(key)
        }
    }

    /*
    pub fn insert(&mut self, key: K, value: V) {
        match self.root {
            None => {
                *self = Tree23::leaf2(key, value);
            },
            Some(ref mut n) => {
                n.insert(key, value);
            }
        }
    }
    */
}


fn main() {
}
