use anyhow::{anyhow, Context, Result};

use crate::message::Message;
use crate::packed_repeated::{PackedRepeatedI32, PackedRepeatedI64, PackedRepeatedVarint};
use crate::varint::Varint;
use crate::wire_data::WireData;

pub struct Len {
    pub(crate) length: Varint,
    pub(crate) inner: WireData,
}

impl std::default::Default for Len {
    fn default() -> Self {
        Self::new()
    }
}

impl Len {
    pub fn new() -> Self {
        Self {
            length: Varint::new(0),
            inner: WireData::Mut(bytes::BytesMut::new()),
        }
    }

    pub fn new_string(s: &str) -> Self {
        Self {
            length: Varint::new(s.len() as u64),
            inner: WireData::Const(bytes::Bytes::copy_from_slice(s.as_bytes())),
        }
    }

    pub fn new_message(m: Message) -> Self {
        Self {
            length: Varint::new(m.0.len() as u64),
            inner: m.0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            length: Varint::new(0),
            inner: WireData::Mut(bytes::BytesMut::with_capacity(capacity)),
        }
    }

    pub fn from(data: WireData) -> Result<(Self, WireData)> {
        let (len, mut len_remainder) =
            Varint::from(data).context("Len must start with a Varint")?;

        // need to split remainder on the given length
        let split_len = len.get() as usize;
        if split_len > len_remainder.len() {
            return Err(anyhow!("Len message overruns buffer"));
        }
        let final_remainder = len_remainder.split_off(split_len);
        Ok((
            Self {
                length: len,
                inner: len_remainder,
            },
            final_remainder,
        ))
    }

    pub fn get_data(&self) -> WireData {
        self.inner.clone()
    }

    pub fn as_str(&self) -> Result<&str> {
        std::str::from_utf8(self.inner.as_ref()).context("Invalid UTF-8")
    }

    pub fn into_message(self) -> Message {
        Message(self.inner)
    }

    pub fn into_packed_repeated_varint(self) -> PackedRepeatedVarint {
        PackedRepeatedVarint(self.inner)
    }

    pub fn into_packed_repeated_i64(self) -> PackedRepeatedI64 {
        PackedRepeatedI64(self.inner)
    }

    pub fn into_packed_repeated_i32(self) -> PackedRepeatedI32 {
        PackedRepeatedI32(self.inner)
    }

    pub fn set_str(&mut self, s: &str) {
        self.set_bytes(s.as_bytes());
    }

    pub fn set_bytes(&mut self, data: &[u8]) {
        let bytes = self.inner.get_mut_or_default();
        let len = data.len();
        bytes.resize(len, 0);
        for i in 0..len {
            bytes[i] = data[i];
        }
        self.length.set(len as u64);
    }

    pub fn set_message(&mut self, m: Message) {
        let wire_data = m.0;
        self.length.set(wire_data.len() as u64);
        self.inner = wire_data;
    }

    pub fn set_packed_repeated_varint(&mut self, r: PackedRepeatedVarint) {
        let wire_data = r.0;
        self.length.set(wire_data.len() as u64);
        self.inner = wire_data;
    }

    pub fn set_packed_repeated_i64(&mut self, r: PackedRepeatedI64) {
        let wire_data = r.0;
        self.length.set(wire_data.len() as u64);
        self.inner = wire_data;
    }

    pub fn set_packed_repeated_i32(&mut self, r: PackedRepeatedI32) {
        let wire_data = r.0;
        self.length.set(wire_data.len() as u64);
        self.inner = wire_data;
    }
}
