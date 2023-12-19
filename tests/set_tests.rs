#[cfg(test)]
pub mod tests {

    use indextreemap::IndexTreeSet;
    use sha2::{Digest, Sha256};

    const SCOPE: usize = 2_000;

    fn hash(n: &[u8]) -> String {
        let mut sha256 = Sha256::new();
        sha256.update(n);
        hex::encode(sha256.finalize())
    }

    #[test]
    fn insert_test() {
        let mut tree = IndexTreeSet::new();
        for i in 0..SCOPE {
            tree.insert(i.to_string());
        }

        println!("{tree:#?}")
    }

    #[test]
    fn clear_test() {
        let mut tree = IndexTreeSet::new();
        for i in 0..SCOPE {
            tree.insert(hash(i.to_le_bytes().as_slice()))
        }

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get(&hash(i.to_le_bytes().as_slice())));
            assert!(tree.contains_key(&hash(i.to_le_bytes().as_slice())));
        }

        tree.clear();

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get(&hash(i.to_le_bytes().as_slice())));
            assert!(!tree.contains_key(&hash(i.to_le_bytes().as_slice())));
        }

        for i in 0..SCOPE {
            tree.insert(hash(i.to_le_bytes().as_slice()))
        }

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get(&hash(i.to_le_bytes().as_slice())));
            assert!(tree.contains_key(&hash(i.to_le_bytes().as_slice())));
        }
    }

    //* STRING TESTS *//
    // * * Expansive Testing has a time complexity of O(SCOPE * SCOPE)
    #[test]
    fn string_expansive_test() {
        let mut tree = IndexTreeSet::new();
        for i in 0..SCOPE {
            for j in 0..i {
                tree.insert(hash(j.to_le_bytes().as_slice()))
            }

            // println!("{tree:#?}");

            for k in 0..i {
                // println!("{i}: {:?}", tree.get(&hash(i.to_le_bytes().as_slice())));
                assert!(tree.contains_key(&hash(k.to_le_bytes().as_slice())));
            }
        }
    }

    #[test]
    fn string_index_from_key() {
        let mut tree = IndexTreeSet::new();
        for i in 0..SCOPE {
            tree.insert(hash(i.to_le_bytes().as_slice()))
        }

        // println!("{tree:#?}");

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get_index_from_key(&hash(i.to_le_bytes().as_slice())));
            assert!(tree
                .get_index_from_key(&hash(i.to_le_bytes().as_slice()))
                .is_some());
        }
    }

    #[test]
    fn string_remove() {
        let mut tree = IndexTreeSet::new();
        for i in 0..SCOPE {
            tree.insert(hash(i.to_le_bytes().as_slice()))
        }

        let removed = SCOPE / 3 + 1;
        tree.remove(&hash(removed.to_le_bytes().as_slice()));

        for key in tree.iter() {
            assert_ne!(key, &removed.to_string());
        }
    }

    // #[test]
    // fn string_remove_index() {
    //     let mut tree = IndexTreeSet::new();
    //     for i in 0..SCOPE {
    //         tree.insert(hash(i.to_le_bytes().as_slice()), i)
    //     }

    //     // println!("{tree:#?}");

    //     for i in (0..SCOPE).rev() {
    //         let tree_copy = tree.clone();
    //         let key_value: (&String, &usize) = tree_copy.get_key_value_from_index(i).unwrap();

    //         tree.remove_from_index(i);

    //         println!("{tree:#?}");

    //         for (key, value) in tree.iter() {
    //             assert_ne!(key, key_value.0);
    //             assert_ne!(value, key_value.1)
    //         }
    //     }
    // }

    #[test]
    fn string_split_off() {
        for a in 10..SCOPE {
            let mut tree = IndexTreeSet::new();
            for i in 0..a {
                tree.insert(hash(i.to_le_bytes().as_slice()))
            }

            // println!("{tree:#?}");
            let split_key = &(a / 3 + 1).to_string();
            let split_tree = tree.split_off(split_key);
            // println!("{a}, SK {split_key}");
            // println!("OLD TREE \n {tree:#?}");
            // println!("NEW TREE: \n {split_tree:#?}");

            assert_eq!(tree.len() + split_tree.len(), a);
            for key in tree.iter() {
                assert!(key < split_key)
            }
            for key in split_tree.iter() {
                assert!(key >= split_key)
            }
        }
    }

    #[test]
    fn string_split_off_at_index() {
        for a in 1..SCOPE {
            let mut tree = IndexTreeSet::new();
            for i in 0..a {
                tree.insert(hash(i.to_le_bytes().as_slice()))
            }

            // println!("{tree:#?}");
            let split_index = a / 3;

            let tree_copy = tree.clone();
            let split_key = tree_copy.get_key_from_index(split_index).unwrap();
            let split_tree = tree.split_off_from_index(split_index);
            // println!("{a}, SI: {split_index}, SK: {split_key}");
            // println!("OLD TREE \n {tree:#?}");
            // println!("NEW TREE: \n {split_tree:#?}");

            // for (i, (k,v)) in tree.iter().enumerate() {
            //     println!("{i}: {k} {v}");
            // }

            assert_eq!(tree.len() + split_tree.len(), a);
            for key in tree.iter() {
                assert!(key < split_key)
            }
            for key in split_tree.iter() {
                assert!(key >= split_key)
            }
        }
    }

    //* I32 TESTS *//
    // * * Expansive Testing has a time complexity of O(SCOPE * SCOPE)
    #[test]
    fn i32_expansive_test() {
        let mut tree = IndexTreeSet::new();
        for i in 0..SCOPE {
            for j in 0..i {
                tree.insert(j as i32)
            }

            // println!("{tree:#?}");

            for k in 0..i {
                // println!("{i}: {:?}", tree.get(&hash(i.to_le_bytes().as_slice())));
                assert!(tree.contains_key(&(k as i32)));
            }
        }
    }

    #[test]
    fn i32_remove() {
        let mut tree = IndexTreeSet::new();
        for i in 0..SCOPE {
            tree.insert(i as i32)
        }

        let removed = SCOPE / 3 + 1;
        tree.remove(&(removed as i32));

        for key in tree.iter() {
            assert_ne!(key, &(removed as i32));
        }
    }

    #[test]
    fn i32_split_off() {
        for a in 10..SCOPE {
            let mut tree = IndexTreeSet::new();
            for i in 0..a {
                tree.insert(i as i32)
            }

            // println!("{tree:#?}");
            let split_key = &((a / 3 + 1) as i32);
            let split_tree = tree.split_off(split_key);
            // println!("{a}, SK {split_key}");
            // println!("OLD TREE \n {tree:#?}");
            // println!("NEW TREE: \n {split_tree:#?}");

            assert_eq!(tree.len() + split_tree.len(), a);
            for key in tree.iter() {
                assert!(key < split_key)
            }
            for key in split_tree.iter() {
                assert!(key >= split_key)
            }
        }
    }

    //* USIZE TESTS *//
    // * * Expansive Testing has a time complexity of O(SCOPE * SCOPE)
    #[test]
    fn usize_expansive_test() {
        let mut tree = IndexTreeSet::new();
        for i in 0..SCOPE {
            for j in 0..i {
                tree.insert(j)
            }

            // println!("{tree:#?}");

            for k in 0..i {
                // println!("{i}: {:?}", tree.get(&hash(i.to_le_bytes().as_slice())));
                assert!(tree.contains_key(&k));
            }
        }
    }

    #[test]
    fn usize_index_from_key() {
        let mut tree = IndexTreeSet::new();
        for i in 0..SCOPE {
            tree.insert(i)
        }

        // println!("{tree:#?}");

        for i in 0..SCOPE {
            assert_eq!(tree.get_index_from_key(&i).unwrap(), i);
        }
    }

    #[test]
    fn usize_remove() {
        let mut tree = IndexTreeSet::new();
        for i in 0..SCOPE {
            tree.insert(i)
        }

        let removed = SCOPE / 3 + 1;
        tree.remove(&removed);

        for key in tree.iter() {
            assert_ne!(key, &removed);
        }
    }

    #[test]
    fn usize_split_off() {
        for a in 10..SCOPE {
            let mut tree = IndexTreeSet::new();
            for i in 0..a {
                tree.insert(i)
            }

            // println!("{tree:#?}");
            let split_key = &(a / 3 + 1);
            let split_tree = tree.split_off(split_key);
            // println!("{a}, SK {split_key}");
            // println!("OLD TREE \n {tree:#?}");
            // println!("NEW TREE: \n {split_tree:#?}");

            assert_eq!(tree.len() + split_tree.len(), a);
            for key in tree.iter() {
                assert!(key < split_key)
            }
            for key in split_tree.iter() {
                assert!(key >= split_key)
            }
        }
    }

    //* BYTE ARRAY TESTS *//
    // * * Expansive Testing has a time complexity of O(SCOPE * SCOPE)
    #[test]
    fn byte_array_expansive_test() {
        let mut tree = IndexTreeSet::new();
        for i in 0..SCOPE {
            for j in 0..i {
                tree.insert(j.to_le_bytes())
            }

            // println!("{tree:#?}");

            for k in 0..i {
                // println!("{i}: {:?}", tree.get(&hash(i.to_le_bytes().as_slice())));
                assert!(tree.contains_key(&k.to_le_bytes()));
            }
        }
    }

    #[test]
    fn byte_array_index_from_key() {
        let mut tree = IndexTreeSet::new();
        for i in 0..SCOPE {
            tree.insert(i.to_le_bytes())
        }

        // println!("{tree:#?}");

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get_index_from_key(&hash(i.to_le_bytes().as_slice())));
            assert!(tree.get_index_from_key(&i.to_le_bytes()).is_some());
        }
    }

    #[test]
    fn byte_array_remove() {
        let mut tree = IndexTreeSet::new();
        for i in 0..SCOPE {
            tree.insert(i.to_le_bytes())
        }

        let removed = SCOPE / 3 + 1;
        tree.remove(&removed.to_le_bytes());

        for key in tree.iter() {
            assert_ne!(key, &removed.to_le_bytes());
        }
    }

    #[test]
    fn byte_array_split_off() {
        for a in 10..SCOPE {
            let mut tree: IndexTreeSet<[u8; 8]> = IndexTreeSet::new();
            for i in 0..a {
                tree.insert(i.to_le_bytes())
            }

            // println!("{tree:#?}");
            let split_key = &(a / 3 + 1).to_le_bytes();
            let split_tree = tree.split_off(split_key);
            // println!("{a}, SK {split_key}");
            // println!("OLD TREE \n {tree:#?}");
            // println!("NEW TREE: \n {split_tree:#?}");

            assert_eq!(tree.len() + split_tree.len(), a);
            for key in tree.iter() {
                assert!(key < split_key)
            }
            for key in split_tree.iter() {
                assert!(key >= split_key)
            }
        }
    }
}
