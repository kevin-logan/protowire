use crate::wire_data::WireData;

use anyhow::{anyhow, Result};

pub struct I32(pub(crate) WireData);

impl std::default::Default for I32 {
    fn default() -> Self {
        Self::new(i32::default())
    }
}

impl I32 {
    pub fn new(value: i32) -> Self {
        let bytes = bytes::BytesMut::with_capacity(4);
        let mut result = Self(WireData::Mut(bytes));

        result.set(value);

        result
    }

    pub fn new_float(value: f32) -> Self {
        let bytes = bytes::BytesMut::with_capacity(4);
        let mut result = Self(WireData::Mut(bytes));

        result.set_float(value);

        result
    }

    pub fn from(mut data: WireData) -> Result<(Self, WireData)> {
        if data.len() < 4 {
            Err(anyhow!("I32 must be 4 bytes"))
        } else {
            let remainder = data.split_off(4);
            Ok((Self(data), remainder))
        }
    }

    pub fn get(&self) -> i32 {
        // safety: an I32 must only be constructed from a 4 byte buffer
        i32::from_le_bytes(self.0.as_ref().try_into().unwrap())
    }

    pub fn get_float(&self) -> f32 {
        // safety: an I32 must only be constructed from a 4 byte buffer
        f32::from_le_bytes(self.0.as_ref().try_into().unwrap())
    }

    pub fn set(&mut self, value: i32) {
        let our_bytes = self.0.get_mut_or_default();

        our_bytes.resize(4, 0);
        let value_bytes = value.to_le_bytes();
        for i in 0..4 {
            our_bytes[i] = value_bytes[i];
        }
    }

    pub fn set_float(&mut self, value: f32) {
        let our_bytes = self.0.get_mut_or_default();

        our_bytes.resize(4, 0);
        let value_bytes = value.to_le_bytes();
        for i in 0..4 {
            our_bytes[i] = value_bytes[i];
        }
    }
}
