use crate::stc::Node;
use std::cmp::Ordering::{Equal, Greater, Less};

impl<K: Default + Clone + Ord, V: Default + Clone> Node<K, V> {
    pub fn binary_search(&self, key: &K) -> Result<usize, usize> {
        if self.n == 0 {
            Err(0)
        } else {
            let mut map: [usize; 2] = [0, self.n - 1];
            let mut m;
            while map[0].abs_diff(map[1]) > 1 {
                m = (map[0] + map[1]) / 2;
                if let Some(k) = &self.keys[m] {
                    match key.cmp(&k.key) {
                        Less => map[1] = m - 1,
                        Equal => return Ok(m),
                        Greater => map[0] = m + 1,
                    }
                } else {
                    return Err(m);
                }
            }

            if map[0] == map[1] {
                m = (map[0] + map[1]) / 2;
                if let Some(k) = &self.keys[m] {
                    match key.cmp(&k.key) {
                        Less => Err(m),
                        Equal => Ok(m),
                        Greater => Err(m + 1),
                    }
                } else {
                    Err(m)
                }
            } else {
                m = (map[0] + map[1]) / 2;
                if let Some(k) = &self.keys[m] {
                    match key.cmp(&k.key) {
                        Less => Err(m),
                        Equal => Ok(m),
                        Greater => {
                            m += 1;
                            if let Some(k) = &self.keys[m] {
                                match key.cmp(&k.key) {
                                    Less => Err(m),
                                    Equal => Ok(m),
                                    Greater => Err(m + 1),
                                }
                            } else {
                                Err(m)
                            }
                        }
                    }
                } else {
                    Err(m)
                }
            }
        }
    }
}

// pub fn binary_search(arr: &[usize], key: &usize) -> Result<usize, usize> {
//     if arr.is_empty() {
//         Err(0)
//     } else {
//         let mut map: [usize; 2] = [0, arr.len() - 1];
//         let mut m;
//         while map[0].abs_diff(map[1]) > 1 {
//             m = (map[0] + map[1]) / 2;
//             match key.cmp(&arr[m]) {
//                 Less => map[1] = m - 1,
//                 Equal => return Ok(m),
//                 Greater => map[0] = m + 1,
//             }
//         }

//         if map[0] == map[1] {
//             m = (map[0] + map[1]) / 2;
//             match key.cmp(&arr[m]) {
//                 Less => Err(m),
//                 Equal => Ok(m),
//                 Greater => Err(m + 1),
//             }
//         } else {
//             m = (map[0] + map[1]) / 2;
//             match key.cmp(&arr[m]) {
//                 Less => Err(m),
//                 Equal => Ok(m),
//                 Greater => {
//                     m += 1;
//                     match key.cmp(&arr[m]) {
//                         Less => Err(m),
//                         Equal => Ok(m),
//                         Greater => Err(m + 1),
//                     }
//                 }
//             }
//         }
//     }
// }
