use core::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
pub struct StaticVec<T: Default + Copy, const N: usize> {
    ranges: [T; N],
    len: usize,
}

impl<T: Default + Copy, const N: usize> StaticVec<T, N> {
    pub fn push(&mut self, elem: T) {
        self.ranges[self.len] = elem;
        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T: Default + Copy, const N: usize> Index<usize> for StaticVec<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.ranges[index]
    }
}

impl<T: Default + Copy, const N: usize> IndexMut<usize> for StaticVec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.ranges[index]
    }
}

impl<T: Default + Copy, const N: usize> Default for StaticVec<T, N> {
    fn default() -> Self {
        let ranges = [T::default(); N];
        Self { ranges, len: 0 }
    }
}

impl<T: Default + Copy, const N: usize> IntoIterator for StaticVec<T, N> {
    type Item = T;
    type IntoIter = <[T; N] as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.ranges.into_iter()
    }
}
