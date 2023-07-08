use std::cmp::Ordering::{Equal, Greater, Less};

use crate::{stc::Node, KEY_ARRAY, POINTER_ARRAY};

impl<K: Ord + Clone, V: Clone> Node<K, V> {
    pub fn remove(&mut self, key: &K) -> Option<(K, V)> {
        if self.leaf {
            'search: for index in 0..KEY_ARRAY {
                if let Some(item) = self.keys[index].as_mut() {
                    match key.cmp(&item.key) {
                        Less => return None,
                        Equal => return self.leaf_remove_key(index),
                        Greater => continue 'search,
                    }
                }
            }
            None
        } else {
            'search: for index in 0..KEY_ARRAY {
                match self.keys[index].as_mut() {
                    Some(item) => match key.cmp(&item.key) {
                        Less => {
                            if let Some(pointer) = self.pointers[index].as_mut() {
                                let output = pointer.child.remove(key);
                                if output.is_some() {
                                    pointer.counter -= 1;
                                }
                                return output;
                            } else {
                                return None;
                            }
                        }
                        Equal => return self.node_remove_key(index),
                        Greater => {
                            if index >= KEY_ARRAY - 1 {
                                if let Some(pointer) = self.pointers[index + 1].as_mut() {
                                    let output = pointer.child.remove(key);
                                    if output.is_some() {
                                        pointer.counter -= 1;
                                    }
                                    return output;
                                } else {
                                    return None;
                                }
                            } else {
                                continue 'search;
                            }
                        }
                    },
                    None => {
                        if let Some(pointer) = self.pointers[index].as_mut() {
                            let output = pointer.child.remove(key);
                            if output.is_some() {
                                pointer.counter -= 1;
                            }
                            return output;
                        } else {
                            return None;
                        }
                    }
                }
            }
            None
        }
    }
}

impl<K: Ord, V> Node<K, V> {
    pub fn node_remove_key(&mut self, index: usize) -> Option<(K, V)> {
        // if both pointer children pointers[index] + pointers[index+1] are less than KEY_ARRAY, then we can merge the right pointer to the left pointer
        let output = self.keys[index].take().map(|item| (*item.key, *item.value));
        if output.is_some() {
            self.n -= 1;
        }
        if let [Some(left_pointer), Some(right_pointer)] = self.pointers[index..index + 2].as_mut()
        {
            if left_pointer.child.n + right_pointer.child.n < KEY_ARRAY {
                self.merge_pointers_from_removed_node(index);
            } else {
                self.replace_removed_key(index);
            }
        } else {
            self.replace_removed_key(index);
        }
        output
    }

    pub fn merge_pointers_from_removed_node(&mut self, index: usize) {
        if let [Some(left_pointer), Some(right_pointer)] = self.pointers[index..index + 2].as_mut()
        {
            left_pointer.counter += right_pointer.counter;
            left_pointer.child.n += right_pointer.child.n;
            let mut count = 0;
            for loc in 0..KEY_ARRAY {
                match &left_pointer.child.keys[loc] {
                    Some(_) => continue,
                    None => {
                        left_pointer.child.keys[loc] = right_pointer.child.keys[count].take();
                        count += 1;
                        continue;
                    }
                }
            }
            count = 0;
            for loc in 0..POINTER_ARRAY {
                match &left_pointer.child.pointers[loc] {
                    Some(_) => continue,
                    None => {
                        left_pointer.child.pointers[loc] =
                            right_pointer.child.pointers[count].take();
                        count += 1;
                        continue;
                    }
                }
            }

            self.pointers[index + 1] = None;
            for i in 0..(KEY_ARRAY - 1) {
                if i >= index {
                    self.keys[i] = self.keys[i + 1].take();
                    self.pointers[i + 1] = self.pointers[i + 2].take();
                }
            }
        }

        if self.is_empty() {
            if let Some(pointer) = self.pointers[index].as_mut() {
                self.keys = pointer.child.take_keys();
                self.n = pointer.child.n;
                self.leaf = pointer.child.leaf;
                self.pointers = pointer.child.take_pointers();
            }
        }
    }

    pub fn replace_removed_key(&mut self, index: usize) {
        match self.pointers[index].as_mut() {
            Some(p) => {
                self.keys[index] = {
                    let output = p.child.take_last_key();
                    if output.is_some() {
                        p.counter -= 1;
                    }
                    output
                }
            }
            None => {
                for i in 0..(KEY_ARRAY - 1) {
                    if i >= index {
                        self.keys[i] = self.keys[i + 1].take();
                        self.pointers[i] = self.pointers[i + 1].take()
                    }
                }
                self.pointers[KEY_ARRAY - 1] = self.pointers[KEY_ARRAY].take()
            }
        };
    }

    pub fn leaf_remove_key(&mut self, index: usize) -> Option<(K, V)> {
        let output = self.keys[index].take().map(|item| (*item.key, *item.value));
        if output.is_some() {
            self.n -= 1;
        }
        for i in 0..(KEY_ARRAY - 1) {
            if i >= index {
                self.keys[i] = self.keys[i + 1].take();
            }
        }
        output
    }
}
