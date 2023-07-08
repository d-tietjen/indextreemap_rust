use std::cmp::Ordering::{Equal, Greater, Less};

use crate::{stc::Node, KEY_ARRAY};

impl<K: Ord, V> Node<K, V> {
    pub fn get(&self, key: &K) -> Option<(&K, &V)> {
        'search: for index in 0..KEY_ARRAY {
            match &self.keys[index] {
                Some(item) => match key.cmp(&item.key) {
                    Less => {
                        if let Some(pointer) = &self.pointers[index] {
                            return pointer.child.get(key);
                        } else {
                            return None;
                        }
                    }
                    Equal => return Some((item.key.as_ref(), item.value.as_ref())),
                    Greater => {
                        if index >= KEY_ARRAY - 1 {
                            if let Some(pointer) = &self.pointers[index + 1] {
                                return pointer.child.get(key);
                            } else {
                                return None;
                            }
                        } else {
                            continue 'search;
                        }
                    }
                },
                None => {
                    if let Some(pointer) = &self.pointers[index] {
                        return pointer.child.get(key);
                    } else {
                        return None;
                    }
                }
            }
        }
        None
    }

    pub fn get_index_from_key(&self, key: &K, mut usize: usize) -> Option<usize> {
        'search: for index in 0..KEY_ARRAY {
            match &self.keys[index] {
                Some(item) => match key.cmp(&item.key) {
                    Less => {
                        if let Some(pointer) = &self.pointers[index] {
                            return pointer.child.get_index_from_key(key, usize);
                        } else {
                            return None;
                        }
                    }
                    Equal => return Some(usize),
                    Greater => {
                        if index >= KEY_ARRAY - 1 {
                            if let Some(pointer) = &self.pointers[index + 1] {
                                return pointer.child.get_index_from_key(key, usize);
                            } else {
                                return None;
                            }
                        } else {
                            usize += if let Some(pointer) = &self.pointers[index] {
                                pointer.counter
                            } else {
                                0
                            };
                            usize += 1;
                            continue 'search;
                        }
                    }
                },
                None => {
                    if let Some(pointer) = &self.pointers[index] {
                        return pointer.child.get_index_from_key(key, usize);
                    } else {
                        return None;
                    }
                }
            }
        }
        None
    }

    pub fn get_mut(&mut self, key: &K) -> Option<(&mut K, &mut V)> {
        'search: for (index, item) in self.keys.iter_mut().enumerate() {
            match item {
                Some(item) => match key.cmp(&item.key) {
                    Less => {
                        if let Some(pointer) = self.pointers[index].as_mut() {
                            return pointer.child.get_mut(key);
                        } else {
                            return None;
                        }
                    }
                    Equal => return Some((item.key.as_mut(), item.value.as_mut())),
                    Greater => {
                        if index >= KEY_ARRAY - 1 {
                            if let Some(pointer) = self.pointers[index + 1].as_mut() {
                                return pointer.child.get_mut(key);
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
                        return pointer.child.get_mut(key);
                    } else {
                        return None;
                    }
                }
            }
        }
        None
    }
}

impl<K, V> Node<K, V> {
    pub fn get_from_index(&self, mut index: usize) -> Option<(&K, &V)> {
        if self.leaf {
            self.keys[index]
                .as_ref()
                .map(|item| (item.key.as_ref(), item.value.as_ref()))
        } else {
            for loc in 0..KEY_ARRAY {
                if let Some(pointer) = &self.pointers[loc] {
                    if index < pointer.counter {
                        return pointer.child.get_from_index(index);
                    } else {
                        index -= pointer.counter
                    };
                    if index == 0 {
                        return self.keys[loc]
                            .as_ref()
                            .map(|item| (item.key.as_ref(), item.value.as_ref()));
                    } else {
                        index -= 1;
                        continue;
                    }
                } else {
                    continue;
                }
            }
            None
        }
    }

    pub fn get_mut_from_index(&mut self, mut index: usize) -> Option<(&mut K, &mut V)> {
        if self.leaf {
            self.keys[index]
                .as_mut()
                .map(|item| (item.key.as_mut(), item.value.as_mut()))
        } else {
            for (loc, pointer) in self.pointers.iter_mut().enumerate() {
                match pointer {
                    Some(pointer) => {
                        if index < pointer.counter {
                            return pointer.child.get_mut_from_index(index);
                        } else {
                            index -= pointer.counter
                        };
                        if index == 0 {
                            return self.keys[loc]
                                .as_mut()
                                .map(|item| (item.key.as_mut(), item.value.as_mut()));
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
