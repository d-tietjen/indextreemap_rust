use std::cmp::Ordering::{Equal, Greater, Less};

use crate::{stc::Node, KEY_ARRAY};

impl<K: Ord, V: Clone> Node<K, V> {
    pub fn replace(&mut self, key: &K, value: V) -> Option<V> {
        'search: for (index, item) in self.keys.iter_mut().enumerate() {
            match item {
                Some(item) => match key.cmp(&item.key) {
                    Less => {
                        if let Some(pointer) = self.pointers[index].as_mut() {
                            return pointer.child.replace(key, value);
                        } else {
                            return None;
                        }
                    }
                    Equal => {
                        let removed_value = Some(*item.value.clone());
                        item.value = Box::new(value);
                        return removed_value;
                    }
                    Greater => {
                        if index >= KEY_ARRAY - 1 {
                            if let Some(pointer) = self.pointers[index + 1].as_mut() {
                                return pointer.child.replace(key, value);
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
                        return pointer.child.replace(key, value);
                    } else {
                        return None;
                    }
                }
            }
        }
        None
    }
}

impl<K, V: Copy> Node<K, V> {
    pub fn replace_from_index(&mut self, mut index: usize, value: V) -> Option<V> {
        if self.leaf {
            if let Some(item) = self.keys[index].as_mut() {
                let removed_value = Some(*item.value);
                item.value = Box::new(value);
                removed_value
            } else {
                None
            }
        } else {
            for (loc, pointer) in self.pointers.iter_mut().enumerate() {
                match pointer {
                    Some(pointer) => {
                        if index < pointer.counter {
                            return pointer.child.replace_from_index(index, value);
                        } else {
                            index -= pointer.counter
                        };
                        if index == 0 {
                            if let Some(item) = self.keys[loc].as_mut() {
                                let removed_value = Some(*item.value);
                                item.value = Box::new(value);
                                return removed_value;
                            } else {
                                return None;
                            }
                        } else {
                            index -= 1;
                            continue;
                        }
                    }

                    None => continue,
                }
            }
            None
        }
    }
}
