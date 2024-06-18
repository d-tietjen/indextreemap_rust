use std::fmt::Debug;

use crate::{KEY_ARRAY, POINTER_ARRAY};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Output<K, V> {
    #[default]
    Null,
    KeyIsNew,
    KeyExists,
    NewKeyPointer(Option<Box<Item<K, V>>>, Option<Pointer<K, V>>),
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pointer<K, V> {
    pub child: Box<Node<K, V>>,
    pub counter: usize,
}

impl<K: Default, V: Default> Pointer<K, V> {
    pub fn new() -> Pointer<K, V> {
        Pointer {
            child: Node::new(),
            counter: 0,
        }
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Node<K, V> {
    pub keys: [Option<Box<Item<K, V>>>; KEY_ARRAY],
    pub n: usize, // the number of keys stored in the node
    pub leaf: bool,
    pub pointers: [Option<Pointer<K, V>>; POINTER_ARRAY],
}

impl<K: Default, V: Default> Node<K, V> {
    pub fn new() -> Box<Node<K, V>> {
        Box::default()
    }
}

impl<K, V> Node<K, V> {
    pub fn is_full(&self) -> bool {
        self.n == KEY_ARRAY
    }

    pub fn is_empty(&self) -> bool {
        for item in &self.keys {
            match item {
                Some(_) => return false,
                None => continue,
            }
        }
        true
    }

    pub fn size(&self) -> usize {
        let mut count = 0;
        count += self.keys.iter().filter(|key| key.is_some()).count();
        for p in &self.pointers {
            match p {
                Some(p) => count += p.counter,
                None => continue,
            }
        }
        count
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Item<K, V> {
    pub key: Box<K>,
    pub value: Box<V>,
}

impl<K, V> Item<K, V> {
    pub fn new(key: K, value: V) -> Item<K, V> {
        Item {
            key: Box::new(key),
            value: Box::new(value),
        }
    }
}
