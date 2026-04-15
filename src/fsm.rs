/// This is highly inspired by Postgres' FSM implementation:
/// https://github.com/postgres/postgres/blob/master/src/backend/storage/freespace/fsmpage.c

pub struct BinaryTree {
    arr: Vec<u8>,
    non_leaf_nodes: usize,
}

fn left_child(x: usize) -> usize {
    2 * x + 1
}
fn right_child(x: usize) -> usize {
    2 * x + 2
}
fn parent(x: usize) -> usize {
    (x - 1) / 2
}

impl BinaryTree {
    pub fn new(number_of_leaves: usize) -> Self {
        let nodes = 2 * number_of_leaves - 1;
        let non_leaf_nodes = nodes - number_of_leaves;
        let arr = vec![0; nodes];
        Self { 
            arr, 
            non_leaf_nodes,
        }
    }

    pub fn set_available_space(&mut self, slot: usize, available_space: u8) {
        let mut node_number = self.non_leaf_nodes + slot;
        if node_number >= self.arr.len() {
            return;
        }

        // If the value is the same as before, we can skip the update and traversal
        // But if the root is not greater than the new available space, we need to correct it
        if self.arr[node_number] == available_space && available_space <= self.arr[0] {
            return;
        }

        self.arr[node_number] = available_space;

        loop {
            // update parent node
            node_number = parent(node_number);
            let left_child_index = left_child(node_number);
            let right_child_index = left_child_index + 1;

            let mut new_value = self.arr[left_child_index];

            if right_child_index < self.arr.len() {
                // if right value is hight take that one
                 new_value = new_value.max(self.arr[right_child_index]);
            }

            let old_value = self.arr[node_number];

            if old_value == new_value {
                break;
            }

            self.arr[node_number] = new_value;

            if node_number == 0 {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fsm::{BinaryTree, left_child, parent, right_child};

    #[test]
    fn should_update_parents() {
        let mut tree = BinaryTree::new(8);
        tree.set_available_space(0, 200);

        assert_eq!(tree.arr[7], 200); // the leaf
        assert_eq!(tree.arr[3], 200); // level 2
        assert_eq!(tree.arr[1], 200); // level 1
        assert_eq!(tree.arr[0], 200); // root

        for (i, val) in tree.arr.iter().enumerate() {
            if i == 0 || i == 1 || i == 3 || i == 7 {
                continue;
            }

            assert_eq!(*val, 0);
        }
    }


    #[test]
    fn should_traverse_tree() {
        // nodes are named: 0, 1, 2, ...
        // right node from root is 2:
        assert_eq!(right_child(0), 2);
        assert_eq!(left_child(0), 1);
        // right node from 1 is 4:
        assert_eq!(right_child(1), 4);
        assert_eq!(left_child(1), 3);
        // right node from 4 is 10:
        assert_eq!(right_child(4), 10);
        assert_eq!(left_child(4), 9);

        // parent of 9 is 4:
        assert_eq!(parent(9), 4);
        // parrent of 4 is 1:
        assert_eq!(parent(4), 1);
        // parent of 1 is 0:
        assert_eq!(parent(1), 0);
    }

}