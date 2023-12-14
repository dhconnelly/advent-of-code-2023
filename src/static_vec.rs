use core::{
    cmp::{Ord, Ordering},
    fmt::Debug,
    hash::Hash,
    iter::Take,
    ops::{Index, IndexMut, Range, RangeFull},
};

#[derive(Clone, Copy)]
pub struct StaticVec<T: Default + Copy, const N: usize> {
    data: [T; N],
    len: usize,
}

impl<T: Default + Copy, const N: usize> StaticVec<T, N> {
    pub fn from<const M: usize>(data: [T; M]) -> Self {
        data.into_iter().collect()
    }

    pub fn push(&mut self, elem: T) {
        self.data[self.len] = elem;
        self.len += 1;
    }

    pub fn pop(&mut self) {
        self.len -= 1;
    }

    pub fn clear(&mut self) {
        self.len = 0
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

    pub fn binary_search_by_key<K: Ord>(&self, t: &K, f: impl Fn(&T) -> K) -> Option<usize> {
        self.data[..self.len].binary_search_by_key(t, f).ok()
    }
}

impl<T: Default + Copy + PartialEq, const N: usize> StaticVec<T, N> {
    pub fn contains(&self, t: &T) -> bool {
        self.data[..self.len].contains(t)
    }
}

impl<T: Default + Copy + PartialEq, const N: usize> PartialEq for StaticVec<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && (&self[0..self.len]) == (&other[0..other.len])
    }
}

impl<T: Default + Copy + Hash, const N: usize> Hash for StaticVec<T, N> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.data[..self.len].hash(state)
    }
}

impl<T: Default + Copy + Eq, const N: usize> Eq for StaticVec<T, N> {}

impl<T: Default + Copy, const N: usize> Index<usize> for StaticVec<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &(self.data[..self.len])[index]
    }
}

impl<T: Default + Copy, const N: usize> Index<RangeFull> for StaticVec<T, N> {
    type Output = [T];
    fn index(&self, _: RangeFull) -> &Self::Output {
        &self.data[..self.len]
    }
}

impl<T: Default + Copy, const N: usize> Index<Range<usize>> for StaticVec<T, N> {
    type Output = <[T; N] as Index<Range<usize>>>::Output;
    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.data[range]
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

impl<T: Default + Copy, const N: usize> Default for StaticVec<T, N> {
    fn default() -> Self {
        Self::empty()
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
