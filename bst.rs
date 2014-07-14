type Link<T> = Option<Box<T>>;

struct BSTNode<K, V> {
    key: K,
    value: V,
    left: Link<BSTNode<K, V>>,
    right: Link<BSTNode<K, V>>,
    size: uint,
}

impl<K, V> BSTNode<K, V> {
    fn leaf(key: K, value: V) -> BSTNode<K, V> {
        BSTNode { key: key, value: value, left: None, right: None, size: 1 }
    }

    fn left_size(&self) -> uint {
        match self.left {
            None => 0u,
            Some(ref b) => (*b).size
        }
    }

    fn right_size(&self) -> uint {
        match self.right {
            None => 0u,
            Some(ref b) => (*b).size
        }
    }
}

fn getlink<K: Ord, V: Copy>(link: &Link<BSTNode<K,V>>, key: &K) -> Option<V> {
    match *link {
        None => None,
        Some(ref b) => (*b).get(key)
    }
}

fn insertlink<K: Ord, V: Copy>(link: &mut Link<BSTNode<K,V>>, key: K, value: V) {
    match *link {
        None => *link = Some(box BSTNode::leaf(key, value)),
        Some(ref mut b) => (*b).insert(key, value)
    }
}

impl<K: Ord, V: Copy> BSTNode<K, V> {
    fn get(&self, key: &K) -> Option<V> {
        match self.key.cmp(key) {
            Less => getlink(&self.left, key),
            Greater => getlink(&self.right, key),
            Equal => Some(self.value)
        }
    }

    fn insert(&mut self, key: K, value: V) {
        match key.cmp(&self.key) {
            Equal => self.value = value,
            Less =>  {
                insertlink(&mut self.left, key, value);
                self.size += 1;
            },
            Greater => {
                insertlink(&mut self.right, key, value);
                self.size += 1;
            }
        }
    }
}

struct BST<K, V> {
    root: Option<BSTNode<K, V>>,
}

impl<K, V> BST<K, V> {
    fn new() -> BST<K, V> { BST { root: None } }

    fn leaf(key: K, value: V) -> BST<K, V> {
        BST { root: Some(BSTNode::leaf(key, value)) }
    }

    fn get_node<'a>(&'a self) -> &'a BSTNode<K, V> {
        match self.root {
            None => fail!("BST is empty"),
            Some(ref b) => b,
        }
    }
}

impl<K: Ord, V: Copy> BST<K, V> {
    fn get(&self, key: &K) -> Option<V> {
        match self.root {
            None => None,
            Some(ref n) => n.get(key)
        }
    }

    fn insert(&mut self, key: K, value: V) {
        match self.root {
            None => {
                *self = BST::leaf(key, value);
            },
            Some(ref mut n) => {
                n.insert(key, value);
            }
        }
    }
}

fn main() {
    println!("c < f = {}, f < k = {}", 'c' < 'f', 'f' < 'k');
    let mut x = BST::leaf('a', 44u);

    println!("{}", x.get(&'a'));
    println!("{}", x.get(&'b'));

    let mut y = BST::new();

    println!("before insert, y[f] = {}", y.get(&'f'));

    y.insert('f', 44u);
    println!("after insert, y[f] = {}", y.get(&'f'));
    println!("after insert, y[k] = {}", y.get(&'k'));

    {
        let n = y.get_node();
        println!("### Now y root node size = {}", n.size);
    }

    y.insert('k', 21u);
    println!("after insert 2, y[f] = {}", y.get(&'f'));
    println!("after insert 2, y[k] = {}", y.get(&'k'));

    {
        let n = y.get_node();
        println!("### Now y root node size = {}", n.size);
    }

    y.insert('f', 13u);
    println!("after insert 3, y[f] = {}", y.get(&'f'));
    println!("after insert 3, y[k] = {}", y.get(&'k'));

    {
        let n = y.get_node();
        println!("### Now y root node size = {}", n.size);
    }

    y.insert('c', 4u);
    println!("after insert 4, y[f] = {}", y.get(&'f'));
    println!("after insert 4, y[k] = {}", y.get(&'k'));
    println!("after insert 4, y[c] = {}", y.get(&'c'));

    {
        let n = y.get_node();
        println!("### Now y root node size = {}", n.size);
        println!("### Now y root node left_size = {}", n.left_size());
        println!("### Now y root node right_size = {}", n.right_size());
    }

    y.insert('b', 5u);
    y.insert('d', 6u);
    println!("after insert 5, y[f] = {}", y.get(&'f'));
    println!("after insert 5, y[k] = {}", y.get(&'k'));
    println!("after insert 5, y[c] = {}", y.get(&'c'));
    println!("after insert 5, y[b] = {}", y.get(&'b'));
    println!("after insert 5, y[d] = {}", y.get(&'d'));

    {
        let n = y.get_node();
        println!("### Now y root node size = {}", n.size);
        println!("### Now y root node left_size = {}", n.left_size());
        println!("### Now y root node right_size = {}", n.right_size());
    }
}
