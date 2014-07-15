type Link<T> = Option<Box<T>>;

struct Node2<K, V> {
    key: K,
    value: V,
    left: Link<Node23<K, V>>,
    right: Link<Node23<K, V>>,
    size: uint,
}

struct Node3<K, V> {
    key1: K,
    value1: V,
    key2: K,
    value2: V,
    left: Link<Node23<K, V>>,
    middle: Link<Node23<K, V>>,
    right: Link<Node23<K, V>>,
    size: uint,
}

enum Node23<K, V> {
    Node2(Node2<K, V>),
    Node3(Node3<K, V>),
}

impl<K, V> Node23<K, V> {
    fn leaf2(key: K, value: V) -> Node23<K, V> {
        Node2(Node2 { key: key, value: value, 
                      left: None, right: None, size: 1 })
    }

    fn leaf3(key1: K, value1: V, key2: K, value2: V) -> Node23<K, V> {
        Node3(Node3 { key1: key1, value1: value1, key2: key2, value2: value2, 
                      left: None, middle: None, right: None, size: 1 })
    }

    fn is2(&self) -> bool {
        match *self {
            Node2(_) => true,
            _        => false,
        }
    }

    fn is3(&self) -> bool {
        match *self {
            Node3(_) => true,
            _        => false,
        }
    }
}

fn getlink<K: Ord, V: Copy>(link: &Link<Node23<K,V>>, key: &K) -> Option<V> {
    match *link {
        None => None,
        Some(ref b) => (*b).get(key)
    }
}

fn insertlink<K: Ord, V: Copy>(link: &mut Link<Node23<K,V>>, key: K, value: V) {
    match *link {
        None => *link = Some(box Node23::leaf2(key, value)),
        Some(ref mut b) => (*b).insert(key, value)
    }
}

impl<K: Ord, V: Copy> Node23<K, V> {
    fn get(&self, key: &K) -> Option<V> {
        match *self {
            Node2(ref n) =>
                match key.cmp(&n.key) {
                    Equal => Some(n.value),
                    Less => getlink(&n.left, key),
                    Greater => getlink(&n.right, key),
                },
            Node3(ref n) =>
                match key.cmp(&n.key1) {
                    Equal => Some(n.value1),
                    Less => getlink(&n.left, key),
                    Greater =>
                        match key.cmp(&n.key2) {
                            Equal => Some(n.value2),
                            Less => getlink(&n.middle, key),
                            Greater => getlink(&n.right, key),
                        },
                },
        }
    }

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
}


pub struct Tree23<K, V> {
    root: Option<Node23<K, V>>,
}

impl<K, V> Tree23<K, V> {
    pub fn new() -> Tree23<K, V> { Tree23 { root: None } }

    pub fn leaf2(key: K, value: V) -> Tree23<K, V> {
        Tree23 { root: Some(Node23::leaf2(key, value)) }
    }

    pub fn leaf3(key1: K, value1: V, key2: K, value2: V) -> Tree23<K, V> {
        Tree23 { root: Some(Node23::leaf3(key1, value1, key2, value2)) }
    }
}

impl<K: Ord, V: Copy> Tree23<K, V> {
    pub fn get(&self, key: &K) -> Option<V> {
        match self.root {
            None => None,
            Some(ref n) => n.get(key)
        }
    }

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
}


fn main() {
}
