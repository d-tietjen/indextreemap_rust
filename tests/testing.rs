#[cfg(test)]
pub mod tests {
    use indextreemap::IndexTreeMap;

    const SCOPE: usize = 10_000;

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

    //* I32 TESTS *//
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
        assert_eq!(
            tree.replace(&(removed as i32), removed + 1),
            Some(removed)
        );
        assert_eq!(tree.get(&(removed as i32)), Some(&(removed + 1)));
    }

    //* USIZE TESTS *//
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
        assert_eq!(
            tree.replace(&removed, removed + 1),
            Some(removed)
        );
        assert_eq!(tree.get(&removed), Some(&(removed + 1)));
    }

    //* BYTE ARRAY TESTS *//
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
}
