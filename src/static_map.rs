use crate::static_vec::StaticVec;
use core::hash::{Hash, Hasher, SipHasher};

pub struct StaticMap<K, V, const NUM_BUCKETS: usize, const BUCKET_SIZE: usize>
where
    K: Clone + Copy + Default + Ord + PartialEq + Hash,
    V: Clone + Copy + Default,
{
    buckets: StaticVec<StaticVec<(K, V), BUCKET_SIZE>, NUM_BUCKETS>,
    len: usize,
}

impl<K, V, const NUM_BUCKETS: usize, const BUCKET_SIZE: usize>
    StaticMap<K, V, NUM_BUCKETS, BUCKET_SIZE>
where
    K: Clone + Copy + Default + Ord + PartialEq + Hash,
    V: Clone + Copy + Default,
{
    pub fn new() -> Self {
        Self { buckets: StaticVec::of(StaticVec::empty()), len: 0 }
    }

    // ugh: https://www.reddit.com/r/rust/comments/kv34ey/is_there_an_easy_way_to_provide_const_default/
    // TODO: modify StaticVec to use MaybeUninit instead of Default
    pub const fn empty_of(ignored: (K, V)) -> Self {
        Self { buckets: StaticVec::of(StaticVec::empty_of(ignored)), len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        for bucket in self.buckets.iter_mut() {
            bucket.clear();
        }
        self.len = 0;
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let bucket = &self.buckets[Self::bucket_id(key)];
        let bucket_idx = bucket.iter().position(|(k, _)| k == key);
        if let Some(i) = bucket_idx {
            Some(&bucket[i].1)
        } else {
            None
        }
    }

    fn bucket_id(key: &K) -> usize {
        let mut hasher = SipHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % NUM_BUCKETS
    }

    pub fn insert(&mut self, key: K, value: V) {
        let bucket = &mut self.buckets[Self::bucket_id(&key)];
        let bucket_idx = bucket.iter().position(|(k, _)| k == &key);
        if let Some(i) = bucket_idx {
            bucket[i].1 = value;
        } else {
            bucket.push((key, value));
            self.len += 1;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.buckets.iter().flat_map(|bucket| bucket.iter())
    }
}

impl<K, V, const NUM_BUCKETS: usize, const BUCKET_SIZE: usize> Default
    for StaticMap<K, V, NUM_BUCKETS, BUCKET_SIZE>
where
    K: Clone + Copy + Default + Ord + PartialEq + Hash,
    V: Clone + Copy + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct StaticSet<K, const NUM_BUCKETS: usize, const BUCKET_SIZE: usize>
where
    K: Clone + Copy + Default + Ord + PartialEq + Hash,
{
    data: StaticMap<K, bool, NUM_BUCKETS, BUCKET_SIZE>,
}

impl<K, const NUM_BUCKETS: usize, const BUCKET_SIZE: usize> StaticSet<K, NUM_BUCKETS, BUCKET_SIZE>
where
    K: Clone + Copy + Default + Ord + PartialEq + Hash,
{
    pub fn new() -> Self {
        Self { data: StaticMap::new() }
    }

    pub fn insert(&mut self, value: K) {
        self.data.insert(value, true);
    }

    pub fn contains(&self, value: &K) -> bool {
        *self.data.get(value).unwrap_or(&false)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &K> {
        self.data.iter().filter(|(_, v)| *v).map(|(k, _)| k)
    }
}

impl<K, const NUM_BUCKETS: usize, const BUCKET_SIZE: usize> Default
    for StaticSet<K, NUM_BUCKETS, BUCKET_SIZE>
where
    K: Clone + Copy + Default + Ord + PartialEq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}
