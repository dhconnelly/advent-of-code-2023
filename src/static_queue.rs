use crate::static_vec::StaticVec;

pub struct StaticQueue<T: Default + Copy, const N: usize> {
    head: usize,
    tail: usize,
    data: StaticVec<T, N>,
}

impl<T: Default + Copy, const N: usize> StaticQueue<T, N> {
    pub fn new() -> Self {
        Self { data: StaticVec::empty(), head: 0, tail: 0 }
    }

    pub fn front(&self) -> Option<&T> {
        if self.head < self.tail {
            Some(&self.data[self.head])
        } else {
            None
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head < self.tail {
            let head = self.data[self.head];
            self.head += 1;
            Some(head)
        } else {
            None
        }
    }

    pub fn push_back(&mut self, t: T) {
        self.data.push(t);
        self.tail += 1;
    }
}

impl<T: Default + Copy, const N: usize> Default for StaticQueue<T, N> {
    fn default() -> Self {
        Self::new()
    }
}
