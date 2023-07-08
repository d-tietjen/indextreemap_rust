use crate::{
    stc::{Node, Output, Pointer},
    KEY_ARRAY,
};

impl<K: Default + Clone, V: Default + Clone> Node<K, V> {
    pub fn split_root(&mut self) {
        let new_root_key = self.keys[KEY_ARRAY / 2].take();

        let new_node = {
            Node {
                keys: Default::default(),
                pointers: Default::default(),
                n: KEY_ARRAY / 2,
                leaf: self.leaf,
            }
        };
        let mut left_node = new_node.clone();
        let mut right_node = new_node;

        let mut left_counter = 0;
        let mut right_counter = 0;
        for index in 0..KEY_ARRAY {
            if index <= KEY_ARRAY / 2 {
                left_node.keys[left_counter] = self.keys[index].take();
                left_node.pointers[left_counter] = self.pointers[index].take();
                left_counter += 1;
            } else if index > KEY_ARRAY / 2 {
                right_node.keys[right_counter] = self.keys[index].take();
                right_node.pointers[right_counter] = self.pointers[index].take();
                right_counter += 1;
            }
        }
        right_node.pointers[right_counter] = self.pointers[KEY_ARRAY].take();

        let left_size = left_node.size();
        let right_size = right_node.size();

        self.keys[0] = new_root_key;
        self.pointers[0] = Some(Pointer {
            child: Box::new(left_node),
            counter: left_size,
        });
        self.pointers[1] = Some(Pointer {
            child: Box::new(right_node),
            counter: right_size,
        });
        self.n = 1;
        self.leaf = false;
    }

    pub fn split_parent(&mut self) -> Output<K, V> {
        // if the parent is full, create a new parent from the median key, then split the new parent, becoming children
        let new_key = self.keys[KEY_ARRAY / 2].take();

        let mut new_node = {
            Node {
                keys: Default::default(),
                pointers: Default::default(),
                n: KEY_ARRAY / 2,
                leaf: self.leaf,
            }
        };
        let mut counter = 0;
        for index in 0..KEY_ARRAY {
            if index > KEY_ARRAY / 2 {
                new_node.keys[counter] = self.keys[index].take();
                new_node.pointers[counter] = self.pointers[index].take();
                counter += 1;
            } else {
                continue;
            }
        }
        new_node.pointers[counter] = self.pointers[KEY_ARRAY].take();

        self.n = self.keys.iter().filter(|k| k.is_some()).count();
        new_node.n = self.keys.iter().filter(|k| k.is_some()).count();

        let size = new_node.size();

        let pointer = Pointer {
            child: Box::new(new_node),
            counter: size,
        };
        Output::NewKeyPointer(new_key, Some(pointer))
    }

    pub fn split_leaf(&mut self) -> Output<K, V> {
        // create new key for parent node
        let new_key = self.keys[KEY_ARRAY / 2].take();

        // create new leaf
        let mut new_leaf = Node {
            keys: Default::default(),
            pointers: Default::default(),
            n: 0,
            leaf: true,
        };

        let mut counter = 0;
        for index in 0..KEY_ARRAY {
            if index > KEY_ARRAY / 2 {
                new_leaf.keys[counter] = self.keys[index].take();
                counter += 1
            } else {
                continue;
            }
        }
        new_leaf.n = counter;

        self.n = self.keys.iter().filter(|k| k.is_some()).count();

        let pointer = Pointer {
            child: Box::new(new_leaf),
            counter,
        };
        Output::NewKeyPointer(new_key, Some(pointer))
    }
}
