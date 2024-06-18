use crate::{
    stc::{
        Item, Node,
        Output::{self, KeyExists, KeyIsNew, NewKeyPointer, Null},
        Pointer,
    },
    KEY_ARRAY,
};
use std::cmp::Ordering::{Equal, Greater, Less};

impl<K: Ord + Clone, V: Clone> Node<K, V> {
    pub fn insert(&mut self, key: K, value: V) -> Output<K, V> {
        // if node is a leaf then the node has no pointers

        match self.leaf {
            true => {
                // match self.binary_search(&key) {
                //     Ok(index) => {
                //         if let Some(k) = self.keys[index].as_mut() {
                //             k.value = Box::new(value);
                //             return KeyExists;
                //         }
                //     }
                //     Err(index) => match &self.keys[index] {
                //         Some(_) => self.insert_to_keys(index, key, value),
                //         None => {
                //             self.keys[index] = Some(Box::new(Item::new(key, value)));
                //             self.n += 1;
                //         }
                //     },
                // }

                'search: for index in 0..self.keys.len() {
                    if let Some(k) = self.keys[index].as_mut() {
                        match &key.cmp(&k.key) {
                            Less => {
                                self.insert_to_keys(index, key, value);
                                break;
                            }
                            Equal => {
                                k.value = Box::new(value);
                                return KeyExists;
                            }
                            Greater => continue 'search,
                        }
                    } else {
                        self.keys[index] = Some(Box::new(Item::new(key, value)));
                        self.n += 1;
                        break;
                    }
                }

                if self.n == KEY_ARRAY {
                    return self.split_leaf();
                } else {
                    return Null;
                }
            }
            false => {
                'search: for index in 0..self.keys.len() {
                    if let Some(k) = self.keys[index].as_mut() {
                        match key.cmp(&k.key) {
                            Less => {
                                let output = if let Some(pointer) = self.pointers[index].as_mut() {
                                    pointer.child.insert(key, value)
                                } else {
                                    let mut pointer = Pointer::new();
                                    pointer.child.insert(key, value);
                                    self.pointers[index] = Some(pointer);
                                    KeyIsNew
                                };
                                match output {
                                    KeyExists => return output,
                                    NewKeyPointer(new_key, new_pointer) => {
                                        self.pointers[index].as_mut().unwrap().counter =
                                            self.pointers[index].as_ref().unwrap().child.size();
                                        return self.insert_key_pointer(
                                            index,
                                            new_key,
                                            new_pointer,
                                        );
                                    }
                                    _ => {
                                        self.pointers[index].as_mut().unwrap().counter += 1;
                                        return Null;
                                    }
                                }
                            }
                            Equal => {
                                k.value = Box::new(value);
                                return KeyExists;
                            }
                            Greater => {
                                continue 'search;
                            }
                        }
                    } else {
                        let output = if let Some(pointer) = self.pointers[index].as_mut() {
                            pointer.child.insert(key, value)
                        } else {
                            let mut pointer = Pointer::new();
                            pointer.child.insert(key, value);
                            self.pointers[index] = Some(pointer);
                            KeyIsNew
                        };
                        match output {
                            KeyExists => return output,
                            NewKeyPointer(new_key, new_pointer) => {
                                self.pointers[index].as_mut().unwrap().counter =
                                    self.pointers[index].as_ref().unwrap().child.size();
                                return self.insert_key_pointer(index, new_key, new_pointer);
                            }
                            _ => {
                                self.pointers[index].as_mut().unwrap().counter += 1;
                                return Null;
                            }
                        }
                        // key does not exist, insert to pointer at [index]
                    }
                }
            }
        }
        Null
    }

    pub fn insert_key_pointer(
        &mut self,
        index: usize,
        key: Option<Box<Item<K, V>>>,
        pointer: Option<Pointer<K, V>>,
    ) -> Output<K, V> {
        let (_, key_right) = self.keys.split_at_mut(index);
        let (_, pointer_right) = self.pointers.split_at_mut(index + 1);

        key_right.rotate_right(1);
        pointer_right.rotate_right(1);

        key_right[0] = key;
        pointer_right[0] = pointer;

        self.n += 1;

        if self.is_full() {
            self.split_parent()
        } else {
            Null
        }
    }

    pub fn insert_to_keys(&mut self, index: usize, key: K, value: V) {
        let (_, right) = self.keys.split_at_mut(index);
        right.rotate_right(1);
        right[0] = Some(Box::new(Item::new(key, value)));
        self.n += 1;
    }

    pub fn update_root(
        &mut self,
        new_key: Option<Box<Item<K, V>>>,
        new_pointer: Option<Pointer<K, V>>,
    ) {
        let mut new_child = Node {
            keys: Default::default(),
            pointers: Default::default(),
            n: 0,
            leaf: self.leaf,
        };

        new_child.keys = self.take_keys();
        new_child.pointers = self.take_pointers();

        new_child.n = new_child.keys.iter().filter(|k| k.is_some()).count();
        let size = new_child.size();

        let pointer = Pointer {
            child: Box::new(new_child),
            counter: size,
        };

        self.pointers[0] = Some(pointer);
        self.keys[0] = new_key;
        self.pointers[1] = new_pointer;
        self.n = 1;
        self.leaf = false;
    }
}
