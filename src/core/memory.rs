use alloy_primitives::U256;

#[derive(Debug)]
pub struct Memory {
    inner: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Memory { inner: vec![] }
    }

    pub fn mstore(&mut self, offset: usize, data: U256) {
        let bytes: [u8; 32] = data.to_be_bytes();
        if self.inner.len() < offset + 32 {
            self.inner.resize(offset + 32, 0);
        }
        self.inner[offset..offset + 32].copy_from_slice(&bytes);
    }

    pub fn mload(&mut self, offset: usize) -> U256 {
        if self.inner.len() < offset + 32 {
            self.inner.resize(offset + 32, 0);
        }
        U256::from_be_slice(&self.inner[offset..offset + 32])
    }
}
