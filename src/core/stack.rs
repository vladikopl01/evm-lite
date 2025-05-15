use alloy_primitives::U256;

#[derive(Debug)]
pub struct Stack {
    inner: Vec<U256>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { inner: Vec::new() }
    }

    pub fn push(&mut self, item: U256) {
        self.inner.push(item);
    }

    pub fn pop(&mut self) -> U256 {
        self.inner.pop().expect("Stack underflow")
    }

    pub fn swap(&mut self, n: usize) {
        let len = self.inner.len();
        if n < len {
            self.inner.swap(len - 1, len - n - 1);
        } else {
            panic!("Stack underflow");
        }
    }

    pub fn peek(&self, n: usize) -> U256 {
        *self
            .inner
            .get(self.inner.len() - n)
            .expect("Stack underflow")
    }

    pub fn dump(&self) -> &[U256] {
        &self.inner
    }
}

impl PartialEq for Stack {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}
