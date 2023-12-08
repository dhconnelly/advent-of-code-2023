use core::{
    cmp::{Ord, Ordering},
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

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn empty() -> Self {
        Self { data: [T::default(); N], len: 0 }
    }

    pub fn of(t: T) -> Self {
        Self { data: [t; N], len: N }
    }
}

impl<T: Default + Copy, const N: usize> Index<usize> for StaticVec<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &(&self.data[..self.len])[index]
    }
}

impl<T: Default + Copy, const N: usize> IndexMut<usize> for StaticVec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut (&mut self.data[..self.len])[index]
    }
}

impl<T: Default + Copy, const N: usize> IntoIterator for StaticVec<T, N> {
    type Item = T;
    type IntoIter = Take<<[T; N] as IntoIterator>::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter().take(self.len)
    }
}

impl<T: Default + Copy, const N: usize> StaticVec<T, N> {
    pub fn sort(&mut self, cmp: impl FnMut(&T, &T) -> Ordering) {
        (&mut self.data[..self.len]).sort_by(cmp);
    }

    pub fn search<K: Ord>(&self, t: &K, f: impl Fn(&T) -> K) -> Option<&T> {
        (&self.data[..self.len]).binary_search_by_key(t, f).map(|i| &self[i]).ok()
    }
}
