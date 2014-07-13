type Link<T> = Option<Box<T>>;

struct Node2<K, V> {
    key: K,
    val: V,
    link1: Link<TwoThreeNode<T>>,
    link2: Link<TwoThreeNode<T>>,
}

struct Node3<K, V> {
    key1: K,
    val1: V,
    key2: K,
    val2: V,
    link1: Link<TwoThreeNode<T>>,
    link2: Link<TwoThreeNode<T>>,
    link3: Link<TwoThreeNode<T>>,
}

enum Node23<T> {
    Node2(TwoNode<T>),
    Node3(ThreeNode<T>),
}

enum Tree23<T> {
    Nil,
    Node(Link<Node23<T>>),
}

impl<K: Ord, V> Tree23<K, V> {
    fn get(&self, key: K) -> Option<V> {

    }

}
