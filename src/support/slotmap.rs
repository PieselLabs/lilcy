use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

pub trait Key: Sized + Copy {
    fn new(v: usize) -> Self;

    fn index(&self) -> usize;
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
    (pub struct $entity:ident ($inner_type:ty); ) => {
        #[derive(Clone, Copy)]
        pub struct $entity($inner_type);

        impl crate::support::slotmap::Key for $entity {
            fn new(v: usize) -> Self {
                Self(v as $inner_type)
            }

            fn index(&self) -> usize {
                self.0 as usize
            }
        }
    };
}

pub struct SecondaryMap<K: Key, V> {
    values: Vec<V>,
    phantom: PhantomData<K>,
}

impl<K: Key, V> SecondaryMap<K, V> {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn insert(&mut self, key: K, val: V) -> K {
        if self.values.len() <= key.index() {
            unsafe {
                self.values.reserve(key.index() - self.values.len() + 1);
                self.values.set_len(key.index() + 1);
            }
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
