// use std::{collections::VecDeque};

// use crate::page_based_bplustree::{btree_store::StoreMetaData, node::Node};

// // Preemptive B+ Tree
// #[derive(Debug)]
// pub struct BTree {
//     root: Node,
//     max_degree: usize, // number of children (max keys are: max_degree - 1, min keys are: )
// }

// impl BTree {
//     pub fn new(max_degree: usize) -> Self {
//         BTree { 
//             root: Node::new(max_degree), 
//             max_degree,
//         }
//     }


//     pub fn print_tree(&self) {
//         let height = self.root.depth(0);
//         let mut queue = VecDeque::new();
//         queue.push_back((&self.root, 1));
//         let mut current_level = 0;

//         while !queue.is_empty() {
//             let nodes_in_queue = queue.len();

//             for _ in 0..nodes_in_queue {
//                 let (node, level) = queue.pop_front().unwrap();

//                 if level != current_level {
//                     println!();
//                     current_level = level;

//                     let indent = (4 * height - 4 * current_level) as usize;
//                     print!("{:indent$}", "", indent = indent);
//                 }

//                 print!("[");
//                 let keys = node.keys.iter().map(|k| k.to_string()).collect::<Vec<String>>().join(",");
//                 print!("{}", keys);
//                 print!("]");

//                 // spacing between nodes
//                 let gap = 2usize.pow((height - current_level) as u32) + 2;
//                 print!("{:gap$}", "", gap = gap);

//                 for child in &node.children {
//                     queue.push_back((child, level + 1));
//                 }
//             }

//         }
//     }

//     #[cfg(test)]
//     pub fn validate(&self) {
//         self.root.validate(None, None);
//     }

//     pub fn find(&self, key: u32) -> Option<&V> {
//         self.root.find(key)
//     }

//     pub fn insert(&mut self, key: u32, value: V) {
//         if self.root.is_full() {
//             let (lnode, rnode, root_key) = self.root.split();
//             let new_root = Node {
//                 values: Vec::new(),
//                 keys: vec![root_key],
//                 children: vec![lnode, rnode],
//                 max_degree: self.max_degree,
//                 root: true,
//             };

//             self.root = new_root;
//         }
//         self.root.insert(key, value);
//         // check invariants
//     }

//     pub fn delete(&mut self, key: u32) -> Option<V> {
//         let res = self.root.delete(key);

//         // if root became internal node with no keys, collapse height
//         if self.root.keys.is_empty() && !self.root.is_leaf() {
//             // take first child as new root
//             if !self.root.children.is_empty() {
//                 let mut new_root = self.root.children.remove(0);
//                 new_root.root = true;
//                 self.root = new_root;
//             }
//         }

//         res
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::BTree;


//     #[test]
//     fn init_and_add_values() {
//         let mut btree =BTree::<i32>::new(4);
//         btree.insert(10, 10);
//         btree.insert(5, 5);
//         btree.insert(80, 80);
//         btree.insert(90, 90);
//         btree.insert(1, 1);
//         btree.validate();
//     }

//     #[test]
//     fn split_root() {
//         let mut btree =BTree::<i32>::new(4);
//         btree.insert(1, 1);
//         btree.insert(50, 50);
//         btree.insert(100, 100);
//         btree.insert(75, 75);
//         btree.insert(2, 2);
//         btree.insert(3,3);
//         btree.insert(80, 80);
//         btree.insert(200, 200);
//         btree.insert(55, 55);
//         btree.insert(60, 60);
//         btree.insert(65, 65);
//         btree.validate();
//     }

//     #[test]
//     fn find_and_delete() {
//         let mut btree =BTree::<i32>::new(4);
//         btree.insert(1, 1);
//         btree.insert(50, 50);
//         btree.insert(100, 100);
//         btree.insert(75, 75);
//         btree.insert(2, 2);
//         btree.insert(3,3);
//         btree.insert(80, 80);
//         btree.insert(200, 200);
//         btree.insert(55, 55);
//         btree.insert(60, 60);
//         btree.insert(65, 65);

//         let val = btree.find(55);
//         assert!(val.is_some());
//         assert_eq!(*val.unwrap(), 55);

//         btree.delete(55);

//         let val = btree.find(55);
//         assert!(val.is_none());

//         let val = btree.find(200);
//         assert!(val.is_some());
//         assert_eq!(*val.unwrap(), 200);

//         let val = btree.find(4);
//         assert!(val.is_none());

//         let val = btree.find(1);
//         assert!(val.is_some());
//         assert_eq!(*val.unwrap(), 1);
//     }
// }

