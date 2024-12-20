use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

pub trait Key: Sized + Copy + PartialEq + Default {
    fn new(v: usize) -> Self;

    fn index(&self) -> usize;

    fn none_val() -> Self;

    fn is_none(self) -> bool {
        self == Self::none_val()
    }
}

pub struct PrimaryMap<K: Key, V> {
    values: Vec<V>,
    freelist: Vec<K>,
}

impl<K: Key, V> PrimaryMap<K, V> {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            freelist: Vec::new(),
        }
    }

    pub fn insert(&mut self, val: V) -> K {
        if let Some(key) = self.freelist.pop() {
            self.values[key.index()] = val;
            key
        } else {
            self.values.push(val);
            K::new(self.values.len() - 1)
        }
    }
}

impl<K: Key, V> Index<K> for PrimaryMap<K, V> {
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        &self.values[index.index()]
    }
}

impl<K: Key, V> IndexMut<K> for PrimaryMap<K, V> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        &mut self.values[index.index()]
    }
}

#[macro_export]
macro_rules! impl_key {
    (pub struct $key:ident ($inner_type:ty); ) => {
        use crate::support::slotmap::Key;
        #[derive(Clone, Copy, PartialEq)]
        pub struct $key($inner_type);

        impl Key for $key {
            fn new(v: usize) -> Self {
                Self(v as $inner_type)
            }

            fn index(&self) -> usize {
                self.0 as usize
            }

            fn none_val() -> Self {
                Self(<$inner_type>::max_value())
            }
        }

        impl Default for $key {
            fn default() -> Self {
                Self::none_val()
            }
        }
    };
}

pub struct SecondaryMap<K: Key, V> {
    values: Vec<V>,
    phantom: PhantomData<K>,
}

impl<K: Key, V: Default> SecondaryMap<K, V> {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn insert(&mut self, key: K, val: V) -> K {
        if self.values.len() <= key.index() {
            self.values.resize_with(key.index() + 1, Default::default);
        }
        self.values[key.index()] = val;
        key
    }
}

impl<K: Key, V> IndexMut<K> for SecondaryMap<K, V> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        assert!(index.index() < self.values.len());
        &mut self.values[index.index()]
    }
}

impl<K: Key, V> Index<K> for SecondaryMap<K, V> {
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        assert!(index.index() < self.values.len());
        &self.values[index.index()]
    }
}
