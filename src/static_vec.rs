use core::{
    cmp::{Ord, Ordering},
    fmt::Debug,
    iter::Take,
    ops::{Index, IndexMut},
};

#[derive(Clone, Copy)]
pub struct StaticVec<T: Default + Copy, const N: usize> {
    data: [T; N],
    len: usize,
}

impl<T: Default + Copy, const N: usize> StaticVec<T, N> {
    pub fn push(&mut self, elem: T) {
        self.data[self.len] = elem;
        self.len += 1;
    }

    pub fn pop(&mut self) -> T {
        let val = core::mem::take(&mut self.data[self.len - 1]);
        self.len -= 1;
        val
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn empty() -> Self {
        Self { data: [T::default(); N], len: 0 }
    }

    pub fn of(t: T) -> Self {
        Self { data: [t; N], len: N }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data[..self.len].iter()
    }

    pub fn sort_by(&mut self, cmp: impl FnMut(&T, &T) -> Ordering) {
        self.data[..self.len].sort_by(cmp);
    }

    pub fn binary_search_by_key<K: Ord>(
        &self,
        t: &K,
        f: impl Fn(&T) -> K,
    ) -> Option<usize> {
        self.data[..self.len].binary_search_by_key(t, f).ok()
    }
}

impl<T: Default + Copy + Ord, const N: usize> StaticVec<T, N> {
    pub fn binary_search(&self, key: &T) -> Option<usize> {
        self.data[..self.len].binary_search(key).ok()
    }
}

impl<T: Default + Copy + PartialEq, const N: usize> StaticVec<T, N> {
    pub fn contains(&self, t: &T) -> bool {
        self.data[..self.len].contains(t)
    }
}

impl<T: Default + Copy, const N: usize> Index<usize> for StaticVec<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &(self.data[..self.len])[index]
    }
}

impl<T: Default + Copy, const N: usize> IndexMut<usize> for StaticVec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[..self.len][index]
    }
}

impl<T: Default + Copy, const N: usize> IntoIterator for StaticVec<T, N> {
    type Item = T;
    type IntoIter = Take<<[T; N] as IntoIterator>::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter().take(self.len)
    }
}

impl<T: Default + Copy + Debug, const N: usize> Debug for StaticVec<T, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (&self.data[..self.len]).fmt(f)
    }
}

impl<T: Default + Copy, const N: usize> FromIterator<T> for StaticVec<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut vec = StaticVec::empty();
        for item in iter {
            vec.push(item);
        }
        vec
    }
}
