use core::cmp::Ordering;

use crate::static_vec::StaticVec;

#[derive(Debug, Clone, Copy, Default)]
struct Node<K, V>
where
    K: Clone + Copy + Default + Eq + Ord,
    V: Clone + Copy + Default,
{
    key: K,
    value: V,
    left: Option<u16>,
    right: Option<u16>,
}

pub struct StaticTreeMap<K, V, const N: usize>
where
    K: Clone + Copy + Default + Eq + Ord,
    V: Clone + Copy + Default,
{
    arena: StaticVec<Node<K, V>, N>,
    root: Option<u16>,
}

impl<K, V, const N: usize> StaticTreeMap<K, V, N>
where
    K: Clone + Copy + Default + Eq + Ord,
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

    pub fn len(&self) -> usize {
        self.arena.len()
    }

    pub fn is_empty(&self) -> bool {
        self.root == None
    }

    fn alloc(&mut self, key: K, value: V) -> Option<u16> {
        self.arena.push(Node { key, value, left: None, right: None });
        Some(self.arena.len() as u16 - 1)
    }

    fn insert_from(&mut self, ptr: u16, key: K, value: V) {
        let ptr = ptr as usize;
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

    fn get_from(&self, ptr: u16, key: &K) -> Option<&V> {
        let ptr = ptr as usize;
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

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        TreeMapIterator::new(self)
    }
}

struct TreeMapIterator<'a, K, V, const N: usize>
where
    K: Clone + Copy + Default + Eq + Ord,
    V: Clone + Copy + Default,
{
    map: &'a StaticTreeMap<K, V, N>,
    stack: StaticVec<u16, 1024>,
}

impl<'a, K, V, const N: usize> TreeMapIterator<'a, K, V, N>
where
    K: Clone + Copy + Default + Eq + Ord,
    V: Clone + Copy + Default,
{
    fn new(map: &'a StaticTreeMap<K, V, N>) -> Self {
        match map.root {
            Some(i) => Self { map, stack: StaticVec::from([i]) },
            None => Self { map, stack: StaticVec::empty() },
        }
    }
}

impl<'a, K, V, const N: usize> Iterator for TreeMapIterator<'a, K, V, N>
where
    K: Clone + Copy + Default + Eq + Ord,
    V: Clone + Copy + Default,
{
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            None
        } else {
            let i = self.stack.pop() as usize;
            let node = &self.map.arena[i];
            if let Some(i) = node.left {
                self.stack.push(i);
            }
            if let Some(i) = node.right {
                self.stack.push(i);
            }
            Some((&node.key, &node.value))
        }
    }
}

pub struct StaticTreeSet<K: Clone + Copy + Default + Ord + Eq, const N: usize>(
    StaticTreeMap<K, bool, N>,
);

impl<K: Clone + Copy + Default + Ord + Eq, const N: usize> StaticTreeSet<K, N> {
    pub fn new() -> Self {
        Self(StaticTreeMap::new())
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, value: K) {
        self.0.insert(value, true);
    }

    pub fn contains(&self, value: &K) -> bool {
        *self.0.get(value).unwrap_or(&false)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &K> {
        self.0.iter().map(|(k, _)| k)
    }
}
