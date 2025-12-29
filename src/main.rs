use std::mem;

#[derive(Debug)]
struct Node<V> {
    values: Vec<V>,
    keys: Vec<u32>,
    children: Vec<Node<V>>,
    max_degree: usize,
    root: bool,
}

impl<V> Node<V> {
    pub fn new(max_degree: usize) -> Self {
        Self {
            values: Vec::new(),
            keys: Vec::new(),
            children: Vec::new(),
            max_degree,
            root: true,
        }
    }
}

impl<V: std::fmt::Debug> Node<V> {
    fn print(&self) {
        println!("\t{:?}", self.keys);
        for node in self.children.iter() {
            print!("\t");
            node.print();
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    #[cfg(test)]
    fn check_node_invariants(&self) {
        assert!(!self.keys.is_empty(), "Keys must never be empty: {:?}", self);
        if self.is_leaf() {
            assert_eq!(self.children.len(), 0, "Children in leaf must be always empty");
            assert_eq!(self.values.len(), self.keys.len(), "Every key must have a value in a leaf");
        } else {
            assert_eq!(self.children.len() + 1, self.keys.len(), "Internal node must have one more children than keys");
            assert_eq!(self.values.len(), 0, "Internal node must not have values");
            assert!(!self.children.is_empty(), "Children must not be empty if not leaf: {:?}", self);
        }

        assert!(self.max_degree > self.keys.len(), "Max degree must be greater than key len. Keys: {:?}", self.keys);

        if !self.root {
            // If max_degree is 10, a node may have 4 keys at minimum (t - 1 where 2t is our max_degree).
            assert!(((self.max_degree / 2) - 1) <= self.keys.len())
        }

        assert!(self.keys.windows(2).all(|pair| pair[0] < pair[1]), "Keys in this node: {:?}", self.keys);
    }

    // returns new left node, new right node and the key (K) for the parent
    fn split(&mut self) -> (Node<V>, Node<V>, u32) {
        // check invariants before split
        let middle_value_index = self.keys.len() / 2;

        let mut right_keys = self.keys.split_off(middle_value_index);
        let mut right_children = Vec::new();
        let mut right_values = Vec::new();

        let mut left_keys = Vec::new();
        let mut left_children = Vec::new();
        let mut left_values = Vec::new();

        let promoted_key;
        
        if !self.is_leaf() {
            right_children = self.children.split_off(middle_value_index + 1);
            left_children = mem::take(&mut self.children);

            promoted_key = right_keys.remove(0); // Key promotes and gets removed
        } else {
            right_values = self.values.split_off(middle_value_index);
            left_values = mem::take(&mut self.values);

            promoted_key = right_keys[0]; // Key stays in right node and promotes
        }
        left_keys = mem::take(&mut self.keys);

        let left_node = Node {
            values: left_values,
            keys: left_keys,
            children: left_children,
            max_degree: self.max_degree,
            root: false,
        };

        let right_node = Node {
            values: right_values,
            keys: right_keys,
            children: right_children,
            max_degree: self.max_degree,
            root: false,
        };

        (left_node, right_node, promoted_key)
    }

    fn insert_key_value(&mut self, key: u32, value: V) {
        // println!("INSERT KEY = {} WITH VALUE = {:?}", key, value);
        let mut index = 0;
        // TODO: replace with binary search
        for k in self.keys.iter() {
            if key < *k {
                break;
            }
            index += 1;
        }

        self.keys.insert(index, key);
        self.values.insert(index, value);
 

        #[cfg(test)]
        self.check_node_invariants();
    }
    
    pub fn insert(&mut self, key: u32, value: V) {
        // if is leaf, then insert key and value
        if self.is_leaf() {
            // println!("I am a leaf: {:?} -> {:?}", self.keys, self.values);
            self.insert_key_value(key, value); 
        } else {
            // println!("I am an internal node");
            // if not leaf:

            // 1. find correct Node
            let mut node_index= self.keys.iter().enumerate()
                .find(|(_, k)| key < **k)
                .map(|(i, _)| i)
                .unwrap_or(self.children.len() - 1);

            // 2. if Node is full, split
            if self.children[node_index].is_full() {
                    let (lnode, rnode, new_key) = self.children[node_index].split();
                    if self.keys.len() == node_index {
                        // append at the end
                        self.keys.push(new_key);
                        self.children[node_index] = lnode;
                        self.children.push(rnode);

                        if key > new_key {
                            node_index += 1;
                        }
                    } else {
                        self.keys.insert(node_index, new_key);
                        self.children.insert(node_index, rnode);
                        self.children.insert(node_index, lnode);
                        if key < new_key {
                            node_index -= 1;
                        }
                    }
            }
        
            // 3. insert into next node
            self.children[node_index].insert(key, value);
        }
    }

    pub fn is_full(&self) -> bool {
        self.keys.len() >= (self.max_degree - 1)
    }
}

// Preemptive B+ Tree
#[derive(Debug)]
pub struct BTree<V> {
    root: Node<V>,
    max_degree: usize, // number of children (max keys are: max_degree - 1, min keys are: )
}

impl<V: Default + std::fmt::Debug> BTree<V> {
    pub fn new(max_degree: usize) -> Self {
        BTree { 
            root: Node::new(max_degree), 
            max_degree,
        }
    }

    pub fn print(&self) {
        println!("B+Tree:");
        self.root.print();
    } 

    pub fn insert(&mut self, key: u32, value: V) {
        if self.root.is_full() {
            let (lnode, rnode, root_key) = self.root.split();
            let new_root = Node {
                values: Vec::new(),
                keys: vec![root_key],
                children: vec![lnode, rnode],
                max_degree: self.max_degree,
                root: true,
            };

            self.root = new_root;
        }
        self.root.insert(key, value);
        // check invariants
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use crate::BTree;


    #[test]
    fn init_and_add_values() {
        let mut btree =BTree::<i32>::new(6);
        btree.insert(10, 10);
        btree.insert(5, 5);
        btree.insert(80, 80);
        btree.insert(90, 90);
        btree.insert(1, 1);
    }

    #[test]
    fn split_root() {
        let mut btree =BTree::<i32>::new(4);
        btree.insert(1, 1);
        btree.insert(50, 50);
        btree.insert(100, 100);
        btree.insert(75, 75);
        // btree.insert(2, 2);
        // btree.insert(3,3);
        
        btree.print();
        btree.insert(80, 80);
        btree.print();
        // println!("{:?}", btree);
    }
}
