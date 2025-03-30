use crate::wire_data::WireData;

use anyhow::{anyhow, Result};

pub struct Varint(pub(crate) WireData);

impl std::default::Default for Varint {
    fn default() -> Self {
        Self::new(u64::default())
    }
}

impl Varint {
    pub fn new(value: u64) -> Self {
        let bytes = bytes::BytesMut::with_capacity(10);
        let mut result = Self(WireData::Mut(bytes));

        result.set(value);

        result
    }

    pub fn new_proto_int32(value: i32) -> Self {
        let bytes = bytes::BytesMut::with_capacity(10);
        let mut result = Self(WireData::Mut(bytes));

        result.set_proto_int32(value);

        result
    }

    pub fn new_proto_int64(value: i64) -> Self {
        let bytes = bytes::BytesMut::with_capacity(10);
        let mut result = Self(WireData::Mut(bytes));

        result.set_proto_int64(value);

        result
    }

    pub fn new_proto_sint32(value: i32) -> Self {
        let bytes = bytes::BytesMut::with_capacity(10);
        let mut result = Self(WireData::Mut(bytes));

        result.set_proto_sint32(value);

        result
    }

    pub fn new_proto_sint64(value: i64) -> Self {
        let bytes = bytes::BytesMut::with_capacity(10);
        let mut result = Self(WireData::Mut(bytes));

        result.set_proto_sint64(value);

        result
    }

    pub fn from(mut data: WireData) -> Result<(Self, WireData)> {
        let mut len = 0;
        let mut valid = false;
        for byte in data.iter() {
            len += 1;
            if byte & 0b1000_0000 == 0 {
                valid = true;
                break;
            }
        }
        if !valid {
            Err(anyhow!("Varint message has no terminating byte"))
        } else if len > 10 {
            Err(anyhow!("Varint message is too long"))
        } else {
            let remainder = data.split_off(len);

            Ok((Self(data), remainder))
        }
    }

    pub fn get(&self) -> u64 {
        let buf = self.0.as_ref();
        let mut result = 0u64;

        for (index, byte) in buf.iter().enumerate() {
            let byte = byte & 0b0111_1111;
            result |= (byte as u64) << (index * 7);
        }
        result
    }

    pub fn set(&mut self, value: u64) {
        let varint_bytes = Self::encode(value);
        self.set_raw(varint_bytes);
    }

    pub fn as_proto_int32(&self) -> i32 {
        let value = self.get() as u32;
        value as i32
    }

    pub fn set_proto_int32(&mut self, value: i32) {
        let varint_bytes = Self::encode_int32(value);
        self.set_raw(varint_bytes);
    }

    pub fn as_proto_int64(&self) -> i64 {
        self.get() as i64
    }

    pub fn set_proto_int64(&mut self, value: i64) {
        let varint_bytes = Self::encode(value as u64);
        self.set_raw(varint_bytes);
    }

    pub fn as_proto_sint32(&self) -> i32 {
        let value = self.get() as i32;
        // conversion from 2s complement to zigzag for N: (N << 1) ^ (N >> 31)
        // so inverse of that conversion for N: (N >> 1) ^ (-(N & 1))
        (value >> 1) ^ (-(value & 1))
    }

    pub fn set_proto_sint32(&mut self, value: i32) {
        let varint_bytes = Self::encode_sint32(value);
        self.set_raw(varint_bytes);
    }

    pub fn as_proto_sint64(&self) -> i64 {
        let value = self.get() as i64;
        // conversion from 2s complement to zigzag for N: (N << 1) ^ (N >> 64)
        // so inverse of that conversion for N: (N >> 1) ^ (-(N & 1))
        (value >> 1) ^ (-(value & 1))
    }

    pub fn set_proto_sint64(&mut self, value: i64) {
        let varint_bytes = Self::encode_sint64(value);
        self.set_raw(varint_bytes);
    }

    pub fn set_raw(&mut self, mut varint_bytes: [u8; 10]) {
        let mut last_index = 9;
        while last_index > 0 {
            if varint_bytes[last_index] != 0 {
                break;
            } else {
                last_index -= 1;
            }
        }

        let len = last_index + 1;
        for val in varint_bytes.iter_mut().take(last_index) {
            // set continuation bit on all bytes _below_ last_index
            *val |= 0b1000_0000;
        }

        // we don't care about our existing data (since we're clearing)
        // so use get_mut_or_default to avoid unnecessary the data copy
        // if our data is currently const
        let bytes = self.0.get_mut_or_default();

        bytes.clear();
        bytes.extend_from_slice(&varint_bytes[0..len]);
    }

    pub fn encode(value: u64) -> [u8; 10] {
        // consts for the highest value storable for a certain number of 7-bit bytes
        const ONE: u64 = 0b111_1111;
        const TWO: u64 = 0b11_1111_1111_1111;
        const THREE: u64 = 0b1_1111_1111_1111_1111_1111;
        const FOUR: u64 = 0b1111_1111_1111_1111_1111_1111_1111;
        const FIVE: u64 = 0b111_1111_1111_1111_1111_1111_1111_1111_1111;
        const SIX: u64 = 0b11_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
        const SEVEN: u64 = 0b1_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
        const EIGHT: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
        const NINE: u64 =
            0b111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

        let mut buffer = [0u8; 10];

        // `value >> 0` is used repeatedly below for alignment, but obviously does nothing
        #[allow(clippy::identity_op)]
        if value <= ONE {
            buffer[0] = ((value >> 0) as u8) & 0b0111_1111;
        } else if value <= TWO {
            buffer[0] = (((value >> 0) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[1] = ((value >> 7) as u8) & 0b0111_1111;
        } else if value <= THREE {
            buffer[0] = (((value >> 0) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[1] = (((value >> 7) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[2] = ((value >> 14) as u8) & 0b0111_1111;
        } else if value <= FOUR {
            buffer[0] = (((value >> 0) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[1] = (((value >> 7) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[2] = (((value >> 14) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[3] = ((value >> 21) as u8) & 0b0111_1111;
        } else if value <= FIVE {
            buffer[0] = (((value >> 0) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[1] = (((value >> 7) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[2] = (((value >> 14) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[3] = (((value >> 21) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[4] = ((value >> 28) as u8) & 0b0111_1111;
        } else if value <= SIX {
            buffer[0] = (((value >> 0) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[1] = (((value >> 7) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[2] = (((value >> 14) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[3] = (((value >> 21) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[4] = (((value >> 28) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[5] = ((value >> 35) as u8) & 0b0111_1111;
        } else if value <= SEVEN {
            buffer[0] = (((value >> 0) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[1] = (((value >> 7) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[2] = (((value >> 14) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[3] = (((value >> 21) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[4] = (((value >> 28) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[5] = (((value >> 35) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[6] = ((value >> 42) as u8) & 0b0111_1111;
        } else if value <= EIGHT {
            buffer[0] = (((value >> 0) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[1] = (((value >> 7) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[2] = (((value >> 14) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[3] = (((value >> 21) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[4] = (((value >> 28) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[5] = (((value >> 35) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[6] = (((value >> 42) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[7] = ((value >> 49) as u8) & 0b0111_1111;
        } else if value <= NINE {
            buffer[0] = (((value >> 0) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[1] = (((value >> 7) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[2] = (((value >> 14) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[3] = (((value >> 21) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[4] = (((value >> 28) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[5] = (((value >> 35) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[6] = (((value >> 42) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[7] = (((value >> 49) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[8] = ((value >> 56) as u8) & 0b0111_1111;
        } else {
            // need to use all 10 bytes, first 9 need continuation bit
            buffer[0] = (((value >> 0) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[1] = (((value >> 7) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[2] = (((value >> 14) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[3] = (((value >> 21) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[4] = (((value >> 28) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[5] = (((value >> 35) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[6] = (((value >> 42) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[7] = (((value >> 49) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[8] = (((value >> 56) as u8) & 0b0111_1111) | 0b1000_0000;
            buffer[9] = ((value >> 63) as u8) & 0b0000_0001;
        }

        buffer
    }

    pub fn encode_int32(value: i32) -> [u8; 10] {
        // first cast to same-size u32 to get no-op coercion, then to u64
        Self::encode(value as u32 as u64)
    }

    pub fn encode_int64(value: i64) -> [u8; 10] {
        // no-op coercion
        Self::encode(value as u64)
    }

    pub fn encode_sint32(value: i32) -> [u8; 10] {
        Self::encode_int32((value << 1) ^ (value >> 31))
    }

    pub fn encode_sint64(value: i64) -> [u8; 10] {
        Self::encode_int64((value << 1) ^ (value >> 63))
    }
}
