#[derive(Debug)]
pub struct Stack {
    data: Vec<u128>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { data: Vec::new() }
    }

    pub fn from(from: Vec<u128>) -> Self {
        Stack { data: from }
    }

    pub fn push(&mut self, item: u128) {
        self.data.push(item);
    }

    pub fn pop(&mut self) -> Option<u128> {
        self.data.pop()
    }

    pub fn swap(&mut self, index_a: usize, index_b: usize) {
        self.data.swap(index_a, index_b);
    }

    pub fn get(&self, index: usize) -> Option<&u128> {
        self.data.get(index)
    }

    pub fn dump(&self) -> &Vec<u128> {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl PartialEq for Stack {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.data == other.data
    }
}
