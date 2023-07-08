use crate::{
    stc::{Item, Node, Pointer},
    KEY_ARRAY, POINTER_ARRAY,
};

impl<K, V> Node<K, V> {
    pub fn take_keys(&mut self) -> [Option<Box<Item<K, V>>>; KEY_ARRAY] {
        let mut array: [Option<Box<Item<K, V>>>; KEY_ARRAY] = Default::default();

        #[allow(clippy::needless_range_loop)]
        for index in 0..KEY_ARRAY {
            array[index] = self.keys[index].take();
        }
        array
    }
    pub fn take_pointers(&mut self) -> [Option<Pointer<K, V>>; POINTER_ARRAY] {
        let mut array: [Option<Pointer<K, V>>; POINTER_ARRAY] = Default::default();

        #[allow(clippy::needless_range_loop)]
        for index in 0..POINTER_ARRAY {
            array[index] = self.pointers[index].take();
        }
        array
    }
}

impl<K: Ord, V> Node<K, V> {
    pub fn take_last_key(&mut self) -> Option<Box<Item<K, V>>> {
        for index in (0..KEY_ARRAY).rev() {
            match &self.keys[index] {
                Some(_) => {
                    let output = self.keys[index].take();
                    if output.is_some() {
                        self.n -= 1;
                    }
                    if self.leaf {
                        self.replace_removed_key(index)
                    }
                    return output;
                }
                None => continue,
            }
        }
        None
    }
}
