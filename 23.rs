type Link<T> = Option<Box<T>>;

struct Node2<K, V> {
    key: K,
    val: V,
    left: Link<Node23<K, V>>,
    right: Link<Node23<K, V>>,
}

struct Node3<K, V> {
    key1: K,
    val1: V,
    key2: K,
    val2: V,
    left: Link<Node23<K, V>>,
    middle: Link<Node23<K, V>>,
    right: Link<Node23<K, V>>,
}

enum Node23<K, V> {
    Node2(Node2<K, V>),
    Node3(Node3<K, V>),
}

impl<K, V> Node23<K, V> {
    fn leaf2(key: K, val: V) -> Node23<K, V> {
        Node2(Node2 { key: key, val: val, 
                      left: None, right: None, size: 1 })
    }

    fn leaf3(key1: K, val1: V, key2: K, val2: V) -> Node23<K, V> {
        Node3(Node3 { key1: key1, val1: val1, key2: key2, val2: val2, 
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


struct Tree23<K, V> {
    root: Option<Node23<K, V>>,
}
