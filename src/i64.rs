use crate::wire_data::WireData;

use anyhow::{anyhow, Result};

pub struct I64(pub(crate) WireData);

impl std::default::Default for I64 {
    fn default() -> Self {
        Self::new(i64::default())
    }
}

impl I64 {
    pub fn new(value: i64) -> Self {
        let bytes = bytes::BytesMut::with_capacity(8);
        let mut result = Self(WireData::Mut(bytes));

        result.set(value);

        result
    }

    pub fn new_double(value: f64) -> Self {
        let bytes = bytes::BytesMut::with_capacity(8);
        let mut result = Self(WireData::Mut(bytes));

        result.set_double(value);

        result
    }

    pub fn from(mut data: WireData) -> Result<(Self, WireData)> {
        if data.len() < 8 {
            Err(anyhow!("I64 must be 8 bytes"))
        } else {
            let remainder = data.split_off(8);
            Ok((Self(data), remainder))
        }
    }

    pub fn get(&self) -> i64 {
        // safety: an I64 must only be constructed from an 8 byte buffer
        i64::from_le_bytes(self.0.as_ref().try_into().unwrap())
    }

    pub fn set(&mut self, value: i64) {
        let our_bytes = self.0.get_mut_or_default();

        our_bytes.resize(8, 0);
        let value_bytes = value.to_le_bytes();
        for i in 0..8 {
            our_bytes[i] = value_bytes[i];
        }
    }

    pub fn get_double(&self) -> f64 {
        // safety: an I64 must only be constructed from an 8 byte buffer
        f64::from_le_bytes(self.0.as_ref().try_into().unwrap())
    }

    pub fn set_double(&mut self, value: f64) {
        let our_bytes = self.0.get_mut_or_default();

        our_bytes.resize(8, 0);
        let value_bytes = value.to_le_bytes();
        for i in 0..8 {
            our_bytes[i] = value_bytes[i];
        }
    }
}
