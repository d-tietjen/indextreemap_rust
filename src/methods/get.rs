use std::cmp::Ordering::{Equal, Greater, Less};

use crate::{stc::Node, KEY_ARRAY};

impl<K: Ord, V> Node<K, V> {
    #[inline]
    pub fn get(&self, key: &K) -> Option<(&K, &V)> {
        match self.search_key(key) {
            Ok(index) => self.keys[index]
                .as_ref()
                .map(|item| (&item.key, &item.value)),
            Err(index) => self.pointers[index]
                .as_ref()
                .and_then(|pointer| pointer.child.get(key)),
        }
    }

    #[inline]
    pub fn get_index_from_key(&self, key: &K, mut usize: usize) -> Option<usize> {
        match self.search_key(key) {
            Ok(index) => {
                for loc in 0..index {
                    usize += self.pointers[loc]
                        .as_ref()
                        .map(|pointer| pointer.counter)
                        .unwrap_or(0);
                    usize += 1;
                }
                usize += self.pointers[index]
                    .as_ref()
                    .map(|pointer| pointer.counter)
                    .unwrap_or(0);
                Some(usize)
            }
            Err(index) => {
                for loc in 0..index {
                    usize += self.pointers[loc]
                        .as_ref()
                        .map(|pointer| pointer.counter)
                        .unwrap_or(0);
                    usize += 1;
                }
                self.pointers[index]
                    .as_ref()
                    .and_then(|pointer| pointer.child.get_index_from_key(key, usize))
            }
        }
    }

    #[inline]
    pub fn get_mut(&mut self, key: &K) -> Option<(&mut K, &mut V)> {
        match self.search_key(key) {
            Ok(index) => self.keys[index]
                .as_mut()
                .map(|item| (&mut item.key, &mut item.value)),
            Err(index) => self.pointers[index]
                .as_mut()
                .and_then(|pointer| pointer.child.get_mut(key)),
        }
    }
}

impl<K: Ord, V> Node<K, V> {
    #[inline]
    fn search_key(&self, key: &K) -> Result<usize, usize> {
        let mut low = 0;
        let mut high = self.n;

        while low < high {
            let mid = (low + high) / 2;
            let Some(item) = self.keys[mid].as_ref() else {
                high = mid;
                continue;
            };

            match key.cmp(&item.key) {
                Less => high = mid,
                Equal => return Ok(mid),
                Greater => low = mid + 1,
            }
        }

        Err(low)
    }
}

impl<K, V> Node<K, V> {
    #[inline]
    pub fn get_from_index(&self, mut index: usize) -> Option<(&K, &V)> {
        if self.leaf {
            self.keys[index]
                .as_ref()
                .map(|item| (&item.key, &item.value))
        } else {
            for loc in 0..KEY_ARRAY {
                if let Some(pointer) = &self.pointers[loc] {
                    if index < pointer.counter {
                        return pointer.child.get_from_index(index);
                    } else {
                        index -= pointer.counter
                    };
                    if index == 0 {
                        return self.keys[loc].as_ref().map(|item| (&item.key, &item.value));
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

    #[inline]
    pub fn get_mut_from_index(&mut self, mut index: usize) -> Option<(&mut K, &mut V)> {
        if self.leaf {
            self.keys[index]
                .as_mut()
                .map(|item| (&mut item.key, &mut item.value))
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
                                .map(|item| (&mut item.key, &mut item.value));
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
