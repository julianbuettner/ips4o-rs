pub struct StackVec<const CAP: usize, T> {
    items: [T; CAP],
    len: usize,
}

impl<const CAP: usize, T> StackVec<CAP, T> {
    pub fn new() -> Self {
        Self {
            items: unsafe { std::mem::zeroed() },
            len: 0,
        }
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.len
    }
    #[allow(dead_code)]
    pub fn push(&mut self, item: T) {
        self.items[self.len] = item;
        self.len += 1;
    }
    #[allow(dead_code)]
    pub fn pop(&mut self) -> T {
        let mut element: T = unsafe {std::mem::zeroed()};
        self.len -= 1;
        std::mem::swap(&mut self.items[self.len], &mut element);
        element
    }
    #[allow(dead_code)]
    pub fn sort(&mut self) where T: Ord {
        self.items[..self.len].sort();
    }
    #[allow(dead_code)]
    pub fn as_slice(&self) -> &[T] {
        &self.items[..self.len]
    }
    #[allow(dead_code)]
    pub fn full(&self) -> bool {
        self.len == CAP
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.len = 0;
    }
}

impl<const CAP: usize, T> Default for StackVec<CAP, T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stack_vec() {
        let mut v: StackVec<8, i32> = StackVec::new();
        v.push(4);
        v.push(99);
        assert_eq!(v.len(), 2);
        assert_eq!(v.pop(), 99);
        assert_eq!(v.pop(), 4);
    }
}
