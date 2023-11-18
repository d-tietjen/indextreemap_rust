use std::cmp::Ordering::{Equal, Greater, Less};

use crate::{
    stc::{Node, Output, Pointer},
    KEY_ARRAY, POINTER_ARRAY,
};

impl<K: Default + Ord + Clone, V: Default + Clone> Node<K, V> {
    pub fn split_off(&mut self, key: &K) -> Option<Pointer<K, V>> {
        for (index, item) in self.keys.iter().enumerate() {
            if let Some(item) = item {
                match key.cmp(&item.key) {
                    Less => {
                        let mut output = self.split_at_index(index);

                        if let Some(pointer) = self.pointers[index].as_mut() {
                            output.pointers[0] = pointer.child.split_off(key);
                            pointer.counter = pointer.child.size();
                            if pointer.child.is_empty() {
                                pointer.child.fill_empty_node()
                            }
                        } else {
                            output.fill_first_pointer()
                        }
                        self.fill_pointers();
                        output.fill_pointers();

                        output.n = output.keys.iter().filter(|item| item.is_some()).count();
                        let size = output.size();
                        return Some(Pointer {
                            child: output,
                            counter: size,
                        });
                    }
                    Equal => {
                        let mut output = self.split_at_index(index);
                        output.fill_first_pointer();
                        output.n = output.keys.iter().filter(|item| item.is_some()).count();

                        self.fill_pointers();
                        output.fill_pointers();

                        let size = output.size();
                        return Some(Pointer {
                            child: output,
                            counter: size,
                        });
                    }
                    Greater => continue,
                }
            } else {
                let mut output = self.split_at_index(index);

                if let Some(pointer) = self.pointers[index].as_mut() {
                    output.pointers[0] = pointer.child.split_off(key);
                    pointer.counter = pointer.child.size();
                    if pointer.child.is_empty() {
                        pointer.child.fill_empty_node()
                    }
                } else {
                    output.fill_first_pointer()
                }
                self.fill_pointers();
                output.fill_pointers();

                output.n = output.keys.iter().filter(|item| item.is_some()).count();
                let size = output.size();
                return Some(Pointer {
                    child: output,
                    counter: size,
                });
            }
        }

        None
    }

    pub fn split_off_at_index(&mut self, mut index: usize) -> Option<Pointer<K, V>> {
        if self.leaf {
            let mut output = self.split_at_index(index);
            output.n = output.keys.iter().filter(|item| item.is_some()).count();
            let size = output.size();
            return Some(Pointer {
                child: output,
                counter: size,
            });
        } else {
            for loc in 0..KEY_ARRAY {
                if self.pointers[loc].is_some() {
                    if index < self.pointers[loc].as_ref().unwrap().counter {
                        let mut output = self.split_at_index(loc);
                        let pointer = self.pointers[loc].as_mut().unwrap();
                        output.pointers[0] = pointer.child.split_off_at_index(index);
                        pointer.counter = pointer.child.size();
                        // if pointer.child.is_empty() {
                        //     pointer.child.fill_empty_node();
                        //     output.fill_first_pointer()
                        // }
                        // self.fill_pointers();
                        // output.fill_pointers();

                        output.n = output.keys.iter().filter(|item| item.is_some()).count();
                        let size = output.size();
                        return Some(Pointer {
                            child: output,
                            counter: size,
                        });
                    } else {
                        index -= self.pointers[loc].as_mut().unwrap().counter
                    }

                    if index == 0 {
                        if self.keys[index].is_some() {
                            let mut output = self.split_at_index(loc);
                            output.fill_first_pointer();
                            output.n = output.keys.iter().filter(|item| item.is_some()).count();
                            self.fill_pointers();
                            output.fill_pointers();

                            let size = output.size();
                            return Some(Pointer {
                                child: output,
                                counter: size,
                            });
                        }
                    } else {
                        index -= 1;
                    }
                } else {
                    continue;
                }
            }

            if let Some(pointer) = self.pointers[KEY_ARRAY].as_mut() {
                if index < pointer.counter {
                    let output = pointer.child.split_off_at_index(index);
                    pointer.counter = pointer.child.size();

                    return output;
                } else {
                    return None;
                }
            }
        }

        None
    }

    pub fn fill_empty_root(&mut self) {
        if self.is_empty() {
            if let Some(pointer) = self.pointers[0].as_mut() {
                self.keys = pointer.child.take_keys();
                self.n = pointer.child.n;
                self.leaf = pointer.child.leaf;
                self.pointers = pointer.child.take_pointers();
            }
        }
    }

    pub fn fill_pointers(&mut self) {
        for index in 0..POINTER_ARRAY {
            if let Some(pointer) = self.pointers[index].as_mut() {
                if pointer.counter == 0 {
                    self.pointers[index] = None;
                    continue;
                } else if pointer.child.is_empty() {
                    pointer.child.fill_empty_node()
                }
            }
        }
    }

    pub fn fill_empty_node(&mut self) {
        if self.is_empty() {
            if let Some(pointer) = self.pointers[0].as_mut() {
                if let Some(key) = pointer.child.take_last_key() {
                    self.keys[0] = Some(key);
                    self.n += 1;
                    pointer.counter -= 1
                }
            }
        }
    }

    pub fn fill_first_pointer(&mut self) {
        if let Some(pointer) = self.pointers[1].as_mut() {
            if pointer.child.n < KEY_ARRAY - 1 {
                if let Some(key) = self.keys[0].take() {
                    // self.n -= 1;
                    pointer.child.keys.rotate_right(1);
                    pointer.child.pointers.rotate_right(1);
                    pointer.child.keys[0] = Some(key);
                    pointer.counter += 1;
                    pointer.child.n += 1;
                    pointer.child.fill_first_pointer();
                }
            }
            for index in 0..(KEY_ARRAY - 1) {
                self.keys[index] = self.keys[index + 1].take();
                self.pointers[index] = self.pointers[index + 1].take()
            }
            self.pointers[KEY_ARRAY - 1] = self.pointers[KEY_ARRAY].take()
        }
    }

    pub fn split_at_index(&mut self, index: usize) -> Box<Node<K, V>> {
        let mut new_node: Box<Node<K, V>> = Node::new();
        new_node.leaf = self.leaf;

        let mut count = 0;

        for loc in 0..KEY_ARRAY {
            if loc >= index {
                new_node.keys[count] = self.keys[loc].take();
                new_node.pointers[count + 1] = self.pointers[loc + 1].take();
                count += 1
            }
        }

        self.n = self.keys.iter().filter(|item| item.is_some()).count();

        new_node
    }
}

impl<K: Default + Clone, V: Default + Clone> Node<K, V> {
    pub fn split_root(&mut self) {
        let new_root_key = self.keys[KEY_ARRAY / 2].take();
        let mut new_node: Box<Node<K, V>> = Node::new();
        new_node.n = KEY_ARRAY / 2;
        new_node.leaf = self.leaf;

        let mut left_node = new_node.clone();
        let mut right_node = new_node;

        let mut left_counter = 0;
        let mut right_counter = 0;
        for index in 0..KEY_ARRAY {
            if index <= KEY_ARRAY / 2 {
                left_node.keys[left_counter] = self.keys[index].take();
                left_node.pointers[left_counter] = self.pointers[index].take();
                left_counter += 1;
            } else if index > KEY_ARRAY / 2 {
                right_node.keys[right_counter] = self.keys[index].take();
                right_node.pointers[right_counter] = self.pointers[index].take();
                right_counter += 1;
            }
        }
        right_node.pointers[right_counter] = self.pointers[KEY_ARRAY].take();

        let left_size = left_node.size();
        let right_size = right_node.size();

        self.keys[0] = new_root_key;
        self.pointers[0] = Some(Pointer {
            child: left_node,
            counter: left_size,
        });
        self.pointers[1] = Some(Pointer {
            child: right_node,
            counter: right_size,
        });
        self.n = 1;
        self.leaf = false;
    }

    pub fn split_parent(&mut self) -> Output<K, V> {
        // if the parent is full, create a new parent from the median key, then split the new parent, becoming children
        let new_key = self.keys[KEY_ARRAY / 2].take();

        let mut new_node = {
            Node {
                keys: Default::default(),
                pointers: Default::default(),
                n: KEY_ARRAY / 2,
                leaf: self.leaf,
            }
        };
        let mut counter = 0;
        for index in 0..KEY_ARRAY {
            if index > KEY_ARRAY / 2 {
                new_node.keys[counter] = self.keys[index].take();
                new_node.pointers[counter] = self.pointers[index].take();
                counter += 1;
            } else {
                continue;
            }
        }
        new_node.pointers[counter] = self.pointers[KEY_ARRAY].take();

        self.n = self.keys.iter().filter(|k| k.is_some()).count();
        new_node.n = self.keys.iter().filter(|k| k.is_some()).count();

        let size = new_node.size();

        let pointer = Pointer {
            child: Box::new(new_node),
            counter: size,
        };
        Output::NewKeyPointer(new_key, Some(pointer))
    }

    pub fn split_leaf(&mut self) -> Output<K, V> {
        // create new key for parent node
        let new_key = self.keys[KEY_ARRAY / 2].take();

        // create new leaf
        let mut new_leaf = Node {
            keys: Default::default(),
            pointers: Default::default(),
            n: 0,
            leaf: true,
        };

        let mut counter = 0;
        for index in 0..KEY_ARRAY {
            if index > KEY_ARRAY / 2 {
                new_leaf.keys[counter] = self.keys[index].take();
                counter += 1
            } else {
                continue;
            }
        }
        new_leaf.n = counter;

        self.n = self.keys.iter().filter(|k| k.is_some()).count();

        let pointer = Pointer {
            child: Box::new(new_leaf),
            counter,
        };
        Output::NewKeyPointer(new_key, Some(pointer))
    }
}
