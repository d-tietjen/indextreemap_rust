use std::{error::Error, fmt};

use crate::{
    stc::{Item, Node, Pointer},
    IndexTreeMap, KEY_ARRAY,
};

const BULK_MAX_KEYS: usize = KEY_ARRAY - 1;
const BULK_MAX_CHILDREN: usize = BULK_MAX_KEYS + 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SortedBuildError;

impl fmt::Display for SortedBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("entries must be sorted by strictly increasing unique keys")
    }
}

impl Error for SortedBuildError {}

impl<K, V> IndexTreeMap<K, V> {
    pub(crate) fn from_sorted_unique_items(items: Vec<Item<K, V>>) -> Self {
        let size = items.len();

        if size == 0 {
            return IndexTreeMap::new();
        }

        let (mut nodes, mut separators) = build_leaf_level(items);

        while nodes.len() > 1 {
            (nodes, separators) = build_parent_level(nodes, separators);
        }

        IndexTreeMap {
            root: nodes
                .pop()
                .expect("non-empty bulk build must produce a root"),
            size,
        }
    }
}

impl<K: Ord, V> IndexTreeMap<K, V> {
    /// Builds a map from entries that are already sorted by unique key.
    ///
    /// This validates the input ordering, then constructs the tree in bulk
    /// instead of repeatedly inserting each entry.
    pub fn try_from_sorted_unique_iter<I>(iter: I) -> Result<Self, SortedBuildError>
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let items = iter
            .into_iter()
            .map(|(key, value)| Item::new(key, value))
            .collect::<Vec<_>>();

        if !items.windows(2).all(|window| window[0].key < window[1].key) {
            return Err(SortedBuildError);
        }

        Ok(Self::from_sorted_unique_items(items))
    }
}

fn build_leaf_level<K, V>(items: Vec<Item<K, V>>) -> (Vec<Box<Node<K, V>>>, Vec<Item<K, V>>) {
    let mut iter = items.into_iter().peekable();
    let mut nodes = Vec::new();
    let mut separators = Vec::new();

    while iter.peek().is_some() {
        let mut node = Node::new();

        for index in 0..BULK_MAX_KEYS {
            let Some(item) = iter.next() else {
                break;
            };

            node.keys[index] = Some(item);
            node.n += 1;
        }

        if iter.peek().is_some() {
            let separator_index = node.n - 1;
            separators.push(
                node.keys[separator_index]
                    .take()
                    .expect("non-final bulk leaf must have a separator key"),
            );
            node.n -= 1;
        }

        nodes.push(node);
    }

    debug_assert_eq!(separators.len() + 1, nodes.len());
    (nodes, separators)
}

fn build_parent_level<K, V>(
    children: Vec<Box<Node<K, V>>>,
    separators: Vec<Item<K, V>>,
) -> (Vec<Box<Node<K, V>>>, Vec<Item<K, V>>) {
    debug_assert_eq!(separators.len() + 1, children.len());

    let total_children = children.len();
    let mut child_iter = children.into_iter();
    let mut separator_iter = separators.into_iter();
    let mut parents = Vec::new();
    let mut parent_separators = Vec::new();
    let mut consumed_children = 0;

    while consumed_children < total_children {
        let remaining = total_children - consumed_children;
        let group_children = parent_group_size(remaining);
        let mut parent = Node::new();
        parent.leaf = false;

        for child_index in 0..group_children {
            let child = child_iter
                .next()
                .expect("bulk parent construction must consume every child");
            let counter = child.size();
            parent.pointers[child_index] = Some(Pointer { child, counter });

            if child_index < group_children - 1 {
                parent.keys[child_index] = Some(
                    separator_iter
                        .next()
                        .expect("bulk parent construction must consume internal separators"),
                );
                parent.n += 1;
            }
        }

        consumed_children += group_children;

        if consumed_children < total_children {
            parent_separators.push(
                separator_iter
                    .next()
                    .expect("bulk parent construction must promote group separators"),
            );
        }

        parents.push(parent);
    }

    debug_assert_eq!(parent_separators.len() + 1, parents.len());
    (parents, parent_separators)
}

fn parent_group_size(remaining: usize) -> usize {
    if remaining <= BULK_MAX_CHILDREN {
        remaining
    } else if remaining - BULK_MAX_CHILDREN == 1 {
        BULK_MAX_CHILDREN - 1
    } else {
        BULK_MAX_CHILDREN
    }
}
