use crate::hyperminhash::{RegVector, NUM_REGISTERS};
use super::dma::CByteArray;
use std::mem::size_of;

/// RegVector impl which stores registers as 16-bit integer array.
/// Each integer is stored in BigEndian.
pub struct DenseVector {
    data: CByteArray,
}

impl DenseVector {
    pub const SINGLE_REGISTER_BYTES: usize = size_of::<u16>();
    pub const DENSE_BYTES: usize = NUM_REGISTERS * DenseVector::SINGLE_REGISTER_BYTES;

    pub fn wrap(data: CByteArray) -> Self {
        Self { data, }
    }
}

impl RegVector for DenseVector {
    fn get(&self, idx: usize) -> u32 {
        let offset = idx * DenseVector::SINGLE_REGISTER_BYTES;

        let mut result = 0u16;
        result |= (self.data[offset + 1] as u16) << 0;
        result |= (self.data[offset + 0] as u16) << 8;

        result as u32
    }

    fn set(&mut self, idx: usize, value: u32) {
        let offset = idx * DenseVector::SINGLE_REGISTER_BYTES;

        self.data[offset + 1] = ((value >> 0) & 0xff) as u8;
        self.data[offset + 0] = ((value >> 8) & 0xff) as u8;
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}
