#[cfg(test)]
pub mod tests {

    use indextreemap::IndexTreeMap;

    const SCOPE: usize = 2_000;

    #[test]
    fn clear_test() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i.to_string(), i)
        }

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get(&i.to_string()));
            assert!(tree.get(&i.to_string()).is_some());
        }

        tree.clear();

        for i in 0..SCOPE {
            tree.insert(i.to_string(), i)
        }

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get(&i.to_string()));
            assert!(tree.get(&i.to_string()).is_some());
        }
    }

    //* STRING TESTS *//
    // * * Expansive Testing has a time complexity of O(SCOPE * SCOPE)
    #[test]
    fn string_expansive_test() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            for j in 0..i {
                tree.insert(j.to_string(), j)
            }

            // println!("{tree:#?}");

            for k in 0..i {
                // println!("{i}: {:?}", tree.get(&i.to_string()));
                assert!(tree.get(&k.to_string()).is_some());
            }
        }
    }

    #[test]
    fn string_get() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i.to_string(), i)
        }

        // println!("{tree:#?}");

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get(&i.to_string()));
            assert!(tree.get(&i.to_string()).is_some());
        }
    }

    #[test]
    fn string_get_mut() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i.to_string(), i)
        }

        // println!("{tree:#?}");

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get_mut(&i.to_string()));
            assert!(tree.get_mut(&i.to_string()).is_some());
        }
    }

    #[test]
    fn string_get_index() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i.to_string(), i)
        }

        // println!("{tree:#?}");

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get_from_index(i));
            assert!(tree.get_from_index(i).is_some());
        }
    }

    #[test]
    fn string_index_from_key() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i.to_string(), i)
        }

        // println!("{tree:#?}");

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get_index_from_key(&i.to_string()));
            assert!(tree.get_index_from_key(&i.to_string()).is_some());
        }
    }

    #[test]
    fn string_get_mut_index() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i.to_string(), i)
        }

        // println!("{tree:#?}");

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get_mut_from_index(i));
            assert!(tree.get_mut_from_index(i).is_some());
        }
    }

    #[test]
    fn string_remove() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i.to_string(), i)
        }

        let removed = SCOPE / 3 + 1;
        tree.remove(&removed.to_string());

        for (key, value) in tree.iter() {
            assert_ne!(key, &removed.to_string());
            assert_ne!(value, &removed)
        }
    }

    // #[test]
    // fn string_remove_index() {
    //     let mut tree = IndexTreeMap::new();
    //     for i in 0..SCOPE {
    //         tree.insert(i.to_string(), i)
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
    fn string_replace() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i.to_string(), i)
        }

        let removed = SCOPE / 3 + 1;
        assert_eq!(
            tree.replace(&removed.to_string(), removed + 1),
            Some(removed)
        );
        assert_eq!(tree.get(&removed.to_string()), Some(&(removed + 1)));
    }

    #[test]
    fn string_split_off() {
        for a in 10..SCOPE {
            let mut tree = IndexTreeMap::new();
            for i in 0..a {
                tree.insert(i.to_string(), i)
            }

            // println!("{tree:#?}");
            let split_key = &(a / 3 + 1).to_string();
            let split_tree = tree.split_off(split_key);
            // println!("{a}, SK {split_key}");
            // println!("OLD TREE \n {tree:#?}");
            // println!("NEW TREE: \n {split_tree:#?}");

            assert_eq!(tree.size + split_tree.size, a);
            for (key, _) in tree.iter() {
                assert!(key < split_key)
            }
            for (key, _) in split_tree.iter() {
                assert!(key >= split_key)
            }
        }
    }

    //* I32 TESTS *//
    // * * Expansive Testing has a time complexity of O(SCOPE * SCOPE)
    #[test]
    fn i32_expansive_test() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            for j in 0..i {
                tree.insert(j as i32, j)
            }

            // println!("{tree:#?}");

            for k in 0..i {
                // println!("{i}: {:?}", tree.get(&i.to_string()));
                assert!(tree.get(&(k as i32)).is_some());
            }
        }
    }

    #[test]
    fn i32_get() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i as i32, i)
        }

        // println!("{tree:#?}");

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get(&(i as i32)));
            assert!(tree.get(&(i as i32)).is_some());
        }
    }

    #[test]
    fn i32_get_index() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i as i32, i)
        }

        // println!("{tree:#?}");

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get_from_index(i));
            assert!(tree.get_from_index(i).is_some());
        }
    }

    #[test]
    fn i32_index_from_key() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i as i32, i)
        }

        // println!("{tree:#?}");

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get_index_from_key(&i.to_string()));
            assert!(tree.get_index_from_key(&(i as i32)).is_some());
        }
    }

    #[test]
    fn i32_get_mut() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i as i32, i)
        }

        // println!("{tree:#?}");

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get_mut(&(i as i32)));
            assert!(tree.get_mut(&(i as i32)).is_some());
        }
    }

    #[test]
    fn i32_get_mut_index() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i as i32, i)
        }

        // println!("{tree:#?}");

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get_mut_from_index(i));
            assert!(tree.get_mut_from_index(i).is_some());
        }
    }

    #[test]
    fn i32_remove() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i as i32, i)
        }

        let removed = SCOPE / 3 + 1;
        tree.remove(&(removed as i32));

        for (key, value) in tree.iter() {
            assert_ne!(key, &(removed as i32));
            assert_ne!(value, &removed)
        }
    }

    #[test]
    fn i32_replace() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i as i32, i)
        }

        let removed = SCOPE / 3 + 1;
        assert_eq!(tree.replace(&(removed as i32), removed + 1), Some(removed));
        assert_eq!(tree.get(&(removed as i32)), Some(&(removed + 1)));
    }

    #[test]
    fn i32_split_off() {
        for a in 10..SCOPE {
            let mut tree = IndexTreeMap::new();
            for i in 0..a {
                tree.insert(i as i32, i)
            }

            // println!("{tree:#?}");
            let split_key = &((a / 3 + 1) as i32);
            let split_tree = tree.split_off(split_key);
            // println!("{a}, SK {split_key}");
            // println!("OLD TREE \n {tree:#?}");
            // println!("NEW TREE: \n {split_tree:#?}");

            assert_eq!(tree.size + split_tree.size, a);
            for (key, _) in tree.iter() {
                assert!(key < split_key)
            }
            for (key, _) in split_tree.iter() {
                assert!(key >= split_key)
            }
        }
    }

    //* USIZE TESTS *//
    // * * Expansive Testing has a time complexity of O(SCOPE * SCOPE)
    #[test]
    fn usize_expansive_test() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            for j in 0..i {
                tree.insert(j, j)
            }

            // println!("{tree:#?}");

            for k in 0..i {
                // println!("{i}: {:?}", tree.get(&i.to_string()));
                assert!(tree.get(&k).is_some());
            }
        }
    }

    #[test]
    fn usize_get() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i, i)
        }

        // println!("{tree:#?}");

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get(&i));
            assert!(tree.get(&i).is_some());
        }
    }

    #[test]
    fn usize_get_index() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i, i)
        }

        // println!("{tree:#?}");

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get_from_index(i));
            assert!(tree.get_from_index(i).is_some());
        }
    }

    #[test]
    fn usize_index_from_key() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i, i)
        }

        // println!("{tree:#?}");

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get_index_from_key(&i.to_string()));
            assert!(tree.get_index_from_key(&i).is_some());
        }
    }

    #[test]
    fn usize_get_mut() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i, i)
        }

        // println!("{tree:#?}");

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get_mut(&i));
            assert!(tree.get_mut(&i).is_some());
        }
    }

    #[test]
    fn usize_get_mut_index() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i, i)
        }

        // println!("{tree:#?}");

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get_mut_from_index(i));
            assert!(tree.get_mut_from_index(i).is_some());
        }
    }

    #[test]
    fn usize_remove() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i, i)
        }

        let removed = SCOPE / 3 + 1;
        tree.remove(&removed);

        for (key, value) in tree.iter() {
            assert_ne!(key, &removed);
            assert_ne!(value, &removed)
        }
    }

    #[test]
    fn usize_replace() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i, i)
        }

        let removed = SCOPE / 3 + 1;
        assert_eq!(tree.replace(&removed, removed + 1), Some(removed));
        assert_eq!(tree.get(&removed), Some(&(removed + 1)));
    }

    #[test]
    fn usize_split_off() {
        for a in 10..SCOPE {
            let mut tree = IndexTreeMap::new();
            for i in 0..a {
                tree.insert(i, i)
            }

            // println!("{tree:#?}");
            let split_key = &(a / 3 + 1);
            let split_tree = tree.split_off(split_key);
            // println!("{a}, SK {split_key}");
            // println!("OLD TREE \n {tree:#?}");
            // println!("NEW TREE: \n {split_tree:#?}");

            assert_eq!(tree.size + split_tree.size, a);
            for (key, _) in tree.iter() {
                assert!(key < split_key)
            }
            for (key, _) in split_tree.iter() {
                assert!(key >= split_key)
            }
        }
    }

    //* BYTE ARRAY TESTS *//
    // * * Expansive Testing has a time complexity of O(SCOPE * SCOPE)
    #[test]
    fn byte_array_expansive_test() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            for j in 0..i {
                tree.insert(j.to_le_bytes(), j)
            }

            // println!("{tree:#?}");

            for k in 0..i {
                // println!("{i}: {:?}", tree.get(&i.to_string()));
                assert!(tree.get(&k.to_le_bytes()).is_some());
            }
        }
    }

    #[test]
    fn byte_array_get() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i.to_le_bytes(), i)
        }

        // println!("{tree:#?}");

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get(&i.to_le_bytes()));
            assert!(tree.get(&i.to_le_bytes()).is_some());
        }
    }

    #[test]
    fn byte_array_get_index() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i.to_le_bytes(), i)
        }

        // println!("{tree:#?}");

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get_from_index(i));
            assert!(tree.get_from_index(i).is_some());
        }
    }

    #[test]
    fn byte_array_index_from_key() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i.to_le_bytes(), i)
        }

        // println!("{tree:#?}");

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get_index_from_key(&i.to_string()));
            assert!(tree.get_index_from_key(&i.to_le_bytes()).is_some());
        }
    }

    #[test]
    fn byte_array_get_mut() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i.to_le_bytes(), i)
        }

        // println!("{tree:#?}");

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get_mut(&i.to_le_bytes()));
            assert!(tree.get_mut(&i.to_le_bytes()).is_some());
        }
    }

    #[test]
    fn byte_array_get_mut_index() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i.to_le_bytes(), i)
        }

        // println!("{tree:#?}");

        for i in 0..SCOPE {
            // println!("{i}: {:?}", tree.get_mut_from_index(i));
            assert!(tree.get_mut_from_index(i).is_some());
        }
    }

    #[test]
    fn byte_array_remove() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i.to_le_bytes(), i)
        }

        let removed = SCOPE / 3 + 1;
        tree.remove(&removed.to_le_bytes());

        for (key, value) in tree.iter() {
            assert_ne!(key, &removed.to_le_bytes());
            assert_ne!(value, &removed)
        }
    }

    #[test]
    fn byte_array_replace() {
        let mut tree = IndexTreeMap::new();
        for i in 0..SCOPE {
            tree.insert(i.to_le_bytes(), i)
        }

        let removed = SCOPE / 3 + 1;
        assert_eq!(
            tree.replace(&removed.to_le_bytes(), removed + 1),
            Some(removed)
        );
        assert_eq!(tree.get(&removed.to_le_bytes()), Some(&(removed + 1)));
    }

    #[test]
    fn byte_array_split_off() {
        for a in 10..SCOPE {
            let mut tree = IndexTreeMap::new();
            for i in 0..a {
                tree.insert(i.to_le_bytes(), i)
            }

            // println!("{tree:#?}");
            let split_key = &(a / 3 + 1).to_le_bytes();
            let split_tree = tree.split_off(split_key);
            // println!("{a}, SK {split_key}");
            // println!("OLD TREE \n {tree:#?}");
            // println!("NEW TREE: \n {split_tree:#?}");

            assert_eq!(tree.size + split_tree.size, a);
            for (key, _) in tree.iter() {
                assert!(key < split_key)
            }
            for (key, _) in split_tree.iter() {
                assert!(key >= split_key)
            }
        }
    }
}
