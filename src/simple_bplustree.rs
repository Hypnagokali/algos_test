use std::{collections::VecDeque, mem};

enum FindKeyResponse {
    GreaterThanTheLast(usize),
    Equal(usize),
    LessThan(usize)
}

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
    fn depth(&self, level: u16) -> u16 {
        let first = self.children.first();

        if let Some(first) = first {
            first.depth(level + 1)
        } else {
            level + 1
        }
    }

    pub fn min_keys(&self) -> usize {
        (self.max_keys() as f32 / 2.0).ceil() as usize
    }

    pub fn max_keys(&self) -> usize {
        self.max_degree - 1
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    #[cfg(test)]
    fn validate(&self, min_key: Option<u32>, max_key: Option<u32>) {
        self.check_node_invariants();
        if let Some(min_key) = min_key {
            assert!(self.keys.iter().all(|k| *k >= min_key), "All Keys must be greater or equal than min_key. min_key: {}, keys:{:?}", min_key, self.keys);
        }

        if let Some(max_key) = max_key {
            assert!(self.keys.iter().all(|k| *k < max_key), "All Keys must be less than max_key. max_key: {}, keys:{:?}", max_key, self.keys);
        }

        for i in 0..self.children.len() {
            let child_min = match i {
                0 => min_key,
                _ => Some(self.keys[i - 1]),
            };

            let child_max = match i {
                i if i < self.keys.len() => Some(self.keys[i]),
                _ => max_key,
            };

            self.children[i].validate(child_min, child_max);
        }
    }

    #[cfg(test)]
    fn check_node_invariants(&self) {
        assert!(!self.keys.is_empty(), "Keys must never be empty: {:?}", self);
        if self.is_leaf() {
            assert_eq!(self.children.len(), 0, "Children in leaf must be always empty");
            assert_eq!(self.values.len(), self.keys.len(), "Every key must have a value in a leaf");
        } else {
            assert_eq!(
                self.children.len(),
                self.keys.len() + 1, 
                "Internal node must have one more children than keys. keys: {:?}, children: {:?}", self.keys, self.children);
            assert_eq!(self.values.len(), 0, "Internal node must not have values");
            assert!(!self.children.is_empty(), "Children must not be empty if not leaf: {:?}", self);
        }

        assert!(self.max_degree > self.keys.len(), "Max degree must be greater than key len. Keys: {:?}", self.keys);

        assert!(self.keys.windows(2).all(|pair| pair[0] < pair[1]), "Keys must be sorted. Keys in this node: {:?}", self.keys);
    }

    // returns new left node, new right node and the key (K) for the parent
    fn split(&mut self) -> (Node<V>, Node<V>, u32) {
        // check invariants before split
        let middle_value_index = self.keys.len() / 2;

        let mut right_keys = self.keys.split_off(middle_value_index);
        let mut right_children = Vec::new();
        let mut right_values = Vec::new();

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
        let left_keys = mem::take(&mut self.keys);

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

    fn find_key_index(&self, key: u32) -> FindKeyResponse {
        // TODO: replace with binary search
        for (i, &k) in self.keys.iter().enumerate() {
            if key < k {
                return FindKeyResponse::LessThan(i);
            } else if key == k {
                return FindKeyResponse::Equal(i);
            }
        }
        
        FindKeyResponse::GreaterThanTheLast(self.keys.len().saturating_sub(1))
    }

    fn insert_key_value(&mut self, key: u32, value: V) {
        match self.find_key_index(key) {
            FindKeyResponse::LessThan(i) => {
                self.keys.insert(i, key);
                self.values.insert(i, value);
            },
            FindKeyResponse::GreaterThanTheLast(_) => {
                self.keys.push(key);
                self.values.push(value);
            },
            FindKeyResponse::Equal(_) => {},
        }      
 
        #[cfg(test)]
        self.check_node_invariants();
    }
    
    pub fn insert(&mut self, key: u32, value: V) {
        // if is leaf, then insert key and value
        if self.is_leaf() {
            self.insert_key_value(key, value); 
        } else {
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
        self.keys.len() >= self.max_keys()
    }

    pub fn can_lend_keys(&self) -> bool {
        self.keys.len() > self.min_keys()
    }

    pub fn is_less_than_minimal(&self) -> bool {
        self.keys.len() < self.min_keys()
    }

    pub fn find(&self, key: u32) -> Option<&V> {
        match self.find_key_index(key) {
            // is leaf
            FindKeyResponse::GreaterThanTheLast(_) if self.is_leaf() => None,
            FindKeyResponse::LessThan(_) if self.is_leaf() => None,
            FindKeyResponse::Equal(i) if self.is_leaf() => Some(&self.values[i]),
            // internal node
            FindKeyResponse::GreaterThanTheLast(i) 
                | FindKeyResponse::Equal(i) => self.children[i + 1].find(key),
            FindKeyResponse::LessThan(i) => self.children[i].find(key)
        }
    }

    // Delete a key from this subtree. Returns the removed value if present.
    pub fn delete(&mut self, key: u32) -> Option<V> {
        if self.is_leaf() {
            // try to find key in this leaf
            if let Some(pos) = self.keys.iter().position(|k| *k == key) {
                let _k = self.keys.remove(pos);
                let v = self.values.remove(pos);
                return Some(v);
            }
            return None;
        }

        let mut node_index = self.keys.iter().enumerate()
            .find(|(_, k)| key < **k)
            .map(|(i, _)| i)
            .unwrap_or(self.children.len() - 1);

        // Refactoring: 
        // self.merge(node_index)
        if self.children[node_index].is_less_than_minimal() {
            if node_index > 0 && self.children[node_index - 1].can_lend_keys() {
                // split the children slice to get two non-overlapping mutable refs
                let (left_slice, right_slice) = self.children.split_at_mut(node_index);
                let left = &mut left_slice[node_index - 1];
                let child = &mut right_slice[0];

                if child.is_leaf() {
                    let k = left.keys.pop().unwrap();
                    let v = left.values.pop().unwrap();
                    child.keys.insert(0, k);
                    child.values.insert(0, v);
                    self.keys[node_index - 1] = child.keys[0];
                } else {
                    let left_key = left.keys.pop().unwrap();
                    let left_child = left.children.pop().unwrap();
                    let parent_key = self.keys[node_index - 1];
                    child.keys.insert(0, parent_key);
                    child.children.insert(0, left_child);
                    self.keys[node_index - 1] = left_key;
                }
            } else if node_index + 1 < self.children.len() && self.children[node_index + 1].can_lend_keys() {
                // borrow from right sibling using split_at_mut with position node_index+1
                let (left_slice, right_slice) = self.children.split_at_mut(node_index + 1);
                let child = &mut left_slice[node_index];
                let right = &mut right_slice[0];

                if child.is_leaf() {
                    let k = right.keys.remove(0);
                    let v = right.values.remove(0);
                    child.keys.push(k);
                    child.values.push(v);
                    self.keys[node_index] = right.keys[0];
                } else {
                    let right_key = right.keys.remove(0);
                    let right_child = right.children.remove(0);
                    let parent_key = self.keys[node_index];
                    child.keys.push(parent_key);
                    child.children.push(right_child);
                    self.keys[node_index] = right_key;
                }
            } else {
                // must merge with a sibling
                if node_index > 0 {
                    let left_index = node_index - 1;
                    let mut right_node = self.children.remove(node_index);
                    let left_node = &mut self.children[left_index];

                    if left_node.is_leaf() {
                        left_node.keys.extend(std::mem::take(&mut right_node.keys));
                        left_node.values.extend(std::mem::take(&mut right_node.values));
                        self.keys.remove(left_index);
                    } else {
                        let sep = self.keys.remove(left_index);
                        left_node.keys.push(sep);
                        // TODO: use std::mem:take here? Or everywhere drain?
                        left_node.keys.extend(right_node.keys.drain(..));
                        left_node.children.extend(right_node.children.drain(..));
                    }

                    node_index = left_index;
                } else {
                    // merge child and right sibling
                    let mut right_node = self.children.remove(node_index + 1);
                    let new_separator = self.keys.remove(node_index);
                    let child_node: &mut Node<V> = &mut self.children[node_index];
                    if child_node.is_leaf() {
                        child_node.keys.extend(right_node.keys.drain(..));
                        child_node.values.extend(right_node.values.drain(..));
                    } else {
                        child_node.keys.push(new_separator);
                        child_node.keys.extend(right_node.keys.drain(..));
                        child_node.children.extend(right_node.children.drain(..));
                    }
                }
            }
        }

        self.children[node_index].delete(key)
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


    pub fn print_tree(&self) {
        let height = self.root.depth(0);
        let mut queue = VecDeque::new();
        queue.push_back((&self.root, 1));
        let mut current_level = 0;

        while !queue.is_empty() {
            let nodes_in_queue = queue.len();

            for _ in 0..nodes_in_queue {
                let (node, level) = queue.pop_front().unwrap();

                if level != current_level {
                    println!();
                    current_level = level;

                    let indent = (4 * height - 4 * current_level) as usize;
                    print!("{:indent$}", "", indent = indent);
                }

                print!("[");
                let keys = node.keys.iter().map(|k| k.to_string()).collect::<Vec<String>>().join(",");
                print!("{}", keys);
                print!("]");

                // spacing between nodes
                let gap = 2usize.pow((height - current_level) as u32) + 2;
                print!("{:gap$}", "", gap = gap);

                for child in &node.children {
                    queue.push_back((child, level + 1));
                }
            }

        }
    }

    #[cfg(test)]
    pub fn validate(&self) {
        self.root.validate(None, None);
    }

    pub fn find(&self, key: u32) -> Option<&V> {
        self.root.find(key)
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

    pub fn delete(&mut self, key: u32) -> Option<V> {
        let res = self.root.delete(key);

        // if root became internal node with no keys, collapse height
        if self.root.keys.is_empty() && !self.root.is_leaf() {
            // take first child as new root
            if !self.root.children.is_empty() {
                let mut new_root = self.root.children.remove(0);
                new_root.root = true;
                self.root = new_root;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::BTree;


    #[test]
    fn init_and_add_values() {
        let mut btree =BTree::<i32>::new(4);
        btree.insert(10, 10);
        btree.insert(5, 5);
        btree.insert(80, 80);
        btree.insert(90, 90);
        btree.insert(1, 1);
        btree.validate();
    }

    #[test]
    fn split_root() {
        let mut btree =BTree::<i32>::new(4);
        btree.insert(1, 1);
        btree.insert(50, 50);
        btree.insert(100, 100);
        btree.insert(75, 75);
        btree.insert(2, 2);
        btree.insert(3,3);
        btree.insert(80, 80);
        btree.insert(200, 200);
        btree.insert(55, 55);
        btree.insert(60, 60);
        btree.insert(65, 65);
        btree.validate();
    }

    #[test]
    fn find_and_delete() {
        let mut btree =BTree::<i32>::new(4);
        btree.insert(1, 1);
        btree.insert(50, 50);
        btree.insert(100, 100);
        btree.insert(75, 75);
        btree.insert(2, 2);
        btree.insert(3,3);
        btree.insert(80, 80);
        btree.insert(200, 200);
        btree.insert(55, 55);
        btree.insert(60, 60);
        btree.insert(65, 65);

        let val = btree.find(55);
        assert!(val.is_some());
        assert_eq!(*val.unwrap(), 55);

        btree.delete(55);

        let val = btree.find(55);
        assert!(val.is_none());

        let val = btree.find(200);
        assert!(val.is_some());
        assert_eq!(*val.unwrap(), 200);

        let val = btree.find(4);
        assert!(val.is_none());

        let val = btree.find(1);
        assert!(val.is_some());
        assert_eq!(*val.unwrap(), 1);
    }
}

