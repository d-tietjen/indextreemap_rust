use crate::{stc::Node, IndexTreeMap, IndexTreeSet};

#[derive(Clone, Copy)]
pub struct NodeCursor<'a, K, V> {
    pub node: &'a Node<K, V>,
    pub index: usize,
}

fn cursor_stack<'a, K, V>(root: &'a Node<K, V>, len: usize) -> Vec<NodeCursor<'a, K, V>> {
    let mut stack = Vec::with_capacity(8);
    if len > 0 {
        push_left_edge(&mut stack, root);
    }
    stack
}

#[inline]
fn push_left_edge<'a, K, V>(stack: &mut Vec<NodeCursor<'a, K, V>>, mut node: &'a Node<K, V>) {
    loop {
        stack.push(NodeCursor { node, index: 0 });

        match node.pointers[0].as_ref() {
            Some(pointer) => node = pointer.child.as_ref(),
            None => break,
        }
    }
}

#[inline]
fn next_entry<'a, K, V>(stack: &mut Vec<NodeCursor<'a, K, V>>) -> Option<(&'a K, &'a V)> {
    loop {
        let frame = stack.last_mut()?;

        if frame.index >= frame.node.n {
            stack.pop();
            continue;
        }

        let index = frame.index;
        frame.index += 1;

        let Some(item) = frame.node.keys[index].as_ref() else {
            continue;
        };

        let right_child = frame.node.pointers[index + 1]
            .as_ref()
            .map(|pointer| pointer.child.as_ref());
        let output = (&item.key, &item.value);

        if let Some(child) = right_child {
            push_left_edge(stack, child);
        }

        return Some(output);
    }
}

//Iterator
pub struct IndexTreeIterator<'a, K, V> {
    pub tree: &'a IndexTreeMap<K, V>,
    pub index: usize,
    pub stack: Vec<NodeCursor<'a, K, V>>,
}

impl<'a, K, V> IndexTreeIterator<'a, K, V> {
    pub(crate) fn new(tree: &'a IndexTreeMap<K, V>) -> Self {
        IndexTreeIterator {
            tree,
            index: 0,
            stack: cursor_stack(tree.root.as_ref(), tree.size),
        }
    }
}

impl<'a, K, V> Iterator for IndexTreeIterator<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let output = next_entry(&mut self.stack)?;
        self.index += 1;
        Some(output)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.tree.size.saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

//Iterator
pub struct IndexTreeSetIterator<'a, K> {
    pub tree: &'a IndexTreeSet<K>,
    pub index: usize,
    pub stack: Vec<NodeCursor<'a, K, ()>>,
}

impl<'a, K> IndexTreeSetIterator<'a, K> {
    pub(crate) fn new(tree: &'a IndexTreeSet<K>) -> Self {
        IndexTreeSetIterator {
            tree,
            index: 0,
            stack: cursor_stack(tree.map.root.as_ref(), tree.map.size),
        }
    }
}

impl<'a, K> Iterator for IndexTreeSetIterator<'a, K> {
    type Item = &'a K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (key, _) = next_entry(&mut self.stack)?;
        self.index += 1;
        Some(key)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.tree.map.size.saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

// // IntoIterator
// pub struct IndexTreeIntoIterator<K, V> {
//     pub tree: IndexTreeMap<K, V>,
// }

// impl<K: Ord + Clone, V: Clone> IntoIterator for IndexTreeMap<K, V> {
//     type Item = (K, V);
//     type IntoIter = IndexTreeIntoIterator<K, V>;

//     /// Creates a consuming iterator visiting all the keys, in sorted PartialOrder. The map cannot be used after calling this.
//     ///
//     /// # Example
//     ///
//     /// Basic usage:
//     /// ```rust
//     /// use std::collections::BTreeMap;
//     ///
//     /// let mut map = BTreeMap::new();
//     /// map.insert(2, "b");
//     /// map.insert(1, "a");
//     ///
//     /// let items: Vec<(i32, &str)> = map.into_iter().collect();
//     /// assert_eq!(items, [(1, "a"), (2, "b")]);
//     /// ```
//     fn into_iter(self) -> Self::IntoIter {
//         IndexTreeIntoIterator { tree: self }
//     }
// }

// impl<K: Ord + Clone, V: Clone> Iterator for IndexTreeIntoIterator<K, V> {
//     type Item = (K, V);

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.tree.size == 0 {
//             return None;
//         }
//         self.tree.remove_from_index(0)
//     }
// }

// FromIter
impl<K: Ord, V> IndexTreeMap<K, V> {
    fn add(&mut self, item: (K, V)) {
        self.insert(item.0, item.1);
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for IndexTreeMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut c = IndexTreeMap::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}

//Keys
pub struct IndexTreeKeys<'a, K, V> {
    pub tree: &'a IndexTreeMap<K, V>,
    pub index: usize,
    pub stack: Vec<NodeCursor<'a, K, V>>,
}

impl<'a, K, V> IndexTreeKeys<'a, K, V> {
    pub(crate) fn new(tree: &'a IndexTreeMap<K, V>) -> Self {
        IndexTreeKeys {
            tree,
            index: 0,
            stack: cursor_stack(tree.root.as_ref(), tree.size),
        }
    }
}

impl<'a, K, V> Iterator for IndexTreeKeys<'a, K, V> {
    type Item = &'a K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (key, _) = next_entry(&mut self.stack)?;
        self.index += 1;
        Some(key)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.tree.size.saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

//Values
pub struct IndexTreeValues<'a, K, V> {
    pub tree: &'a IndexTreeMap<K, V>,
    pub index: usize,
    pub stack: Vec<NodeCursor<'a, K, V>>,
}

impl<'a, K, V> IndexTreeValues<'a, K, V> {
    pub(crate) fn new(tree: &'a IndexTreeMap<K, V>) -> Self {
        IndexTreeValues {
            tree,
            index: 0,
            stack: cursor_stack(tree.root.as_ref(), tree.size),
        }
    }
}

impl<'a, K, V> Iterator for IndexTreeValues<'a, K, V> {
    type Item = &'a V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (_, value) = next_entry(&mut self.stack)?;
        self.index += 1;
        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.tree.size.saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}
