use core::cmp::Ordering;

use crate::static_vec::StaticVec;

#[derive(Debug, Clone, Copy, Default)]
struct Node<K, V>
where
    K: Clone + Copy + Default + PartialEq + Eq + PartialOrd + Ord,
    V: Clone + Copy + Default,
{
    key: K,
    value: V,
    left: Option<usize>,
    right: Option<usize>,
}

pub struct StaticTreeMap<K, V, const N: usize>
where
    K: Clone + Copy + Default + PartialEq + Eq + PartialOrd + Ord,
    V: Clone + Copy + Default,
{
    arena: StaticVec<Node<K, V>, N>,
    root: Option<usize>,
}

impl<K, V, const N: usize> StaticTreeMap<K, V, N>
where
    K: Clone + Copy + Default + PartialEq + Eq + PartialOrd + Ord,
    V: Clone + Copy + Default,
{
    pub fn new() -> Self {
        Self { arena: StaticVec::empty(), root: None }
    }

    pub const fn empty_of(key: K, value: V) -> Self {
        let ignored = Node { key, value, left: None, right: None };
        let arena = StaticVec::empty_of(ignored);
        Self { arena, root: None }
    }

    pub fn clear(&mut self) {
        self.root = None;
        self.arena.clear();
    }

    fn alloc(&mut self, key: K, value: V) -> Option<usize> {
        self.arena.push(Node { key, value, left: None, right: None });
        Some(self.arena.len() - 1)
    }

    fn insert_from(&mut self, ptr: usize, key: K, value: V) {
        match key.cmp(&self.arena[ptr].key) {
            Ordering::Equal => self.arena[ptr].value = value,
            Ordering::Less => match self.arena[ptr].left {
                None => self.arena[ptr].left = self.alloc(key, value),
                Some(i) => self.insert_from(i, key, value),
            },
            Ordering::Greater => match self.arena[ptr].right {
                None => self.arena[ptr].right = self.alloc(key, value),
                Some(i) => self.insert_from(i, key, value),
            },
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        match self.root {
            None => self.root = self.alloc(key, value),
            Some(i) => self.insert_from(i, key, value),
        }
    }

    fn get_from(&self, ptr: usize, key: &K) -> Option<&V> {
        match key.cmp(&self.arena[ptr].key) {
            Ordering::Equal => Some(&self.arena[ptr].value),
            Ordering::Less => match self.arena[ptr].left {
                None => None,
                Some(i) => self.get_from(i, key),
            },
            Ordering::Greater => match self.arena[ptr].right {
                None => None,
                Some(i) => self.get_from(i, key),
            },
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match self.root {
            None => None,
            Some(i) => self.get_from(i, key),
        }
    }
}
