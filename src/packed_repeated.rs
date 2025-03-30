use crate::i32::I32;
use crate::i64::I64;
use crate::varint::Varint;
use crate::wire_data::WireData;

pub struct PackedRepeatedVarint(pub(crate) WireData);

impl std::default::Default for PackedRepeatedVarint {
    fn default() -> Self {
        Self::new()
    }
}

impl PackedRepeatedVarint {
    pub fn new() -> Self {
        PackedRepeatedVarint(WireData::Mut(bytes::BytesMut::new()))
    }

    pub fn with_capacity(capacity: usize) -> Self {
        PackedRepeatedVarint(WireData::Mut(bytes::BytesMut::with_capacity(capacity)))
    }

    pub fn push(&mut self, value: Varint) {
        self.0.get_mut().extend_from_slice(value.0.as_ref());
    }
}

pub struct PackedRepeatedVarintIter(PackedRepeatedVarint);

impl IntoIterator for PackedRepeatedVarint {
    type IntoIter = PackedRepeatedVarintIter;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        PackedRepeatedVarintIter(self)
    }
}

impl Iterator for PackedRepeatedVarintIter {
    type Item = Varint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 .0.is_empty() {
            return None;
        }

        // replace self.0 with empty data to pass ownership of data to the next value
        let data = std::mem::replace(&mut self.0 .0, WireData::new(vec![]));

        // parse the value or return None
        let (value, remainder) = Varint::from(data).ok()?;

        // restore self.0 to whatever data remains after the value was parsed
        self.0 .0 = remainder;

        Some(value)
    }
}

pub struct PackedRepeatedI64(pub(crate) WireData);

impl std::default::Default for PackedRepeatedI64 {
    fn default() -> Self {
        Self::new()
    }
}

impl PackedRepeatedI64 {
    pub fn new() -> Self {
        PackedRepeatedI64(WireData::Mut(bytes::BytesMut::new()))
    }

    pub fn with_capacity(capacity: usize) -> Self {
        PackedRepeatedI64(WireData::Mut(bytes::BytesMut::with_capacity(capacity)))
    }

    pub fn push(&mut self, value: I64) {
        self.0.get_mut().extend_from_slice(value.0.as_ref());
    }
}

pub struct PackedRepeatedI64Iter(PackedRepeatedI64);

impl IntoIterator for PackedRepeatedI64 {
    type IntoIter = PackedRepeatedI64Iter;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        PackedRepeatedI64Iter(self)
    }
}

impl Iterator for PackedRepeatedI64Iter {
    type Item = I64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 .0.is_empty() {
            return None;
        }

        // replace self.0 with empty data to pass ownership of data to the next value
        let data = std::mem::replace(&mut self.0 .0, WireData::new(vec![]));

        // parse the value or return None
        let (value, remainder) = I64::from(data).ok()?;

        // restore self.0 to whatever data remains after the value was parsed
        self.0 .0 = remainder;

        Some(value)
    }
}

pub struct PackedRepeatedI32(pub(crate) WireData);

impl std::default::Default for PackedRepeatedI32 {
    fn default() -> Self {
        Self::new()
    }
}

impl PackedRepeatedI32 {
    pub fn new() -> Self {
        PackedRepeatedI32(WireData::Mut(bytes::BytesMut::new()))
    }

    pub fn with_capacity(capacity: usize) -> Self {
        PackedRepeatedI32(WireData::Mut(bytes::BytesMut::with_capacity(capacity)))
    }

    pub fn push(&mut self, value: I32) {
        self.0.get_mut().extend_from_slice(value.0.as_ref());
    }
}

pub struct PackedRepeatedI32Iter(PackedRepeatedI32);

impl IntoIterator for PackedRepeatedI32 {
    type IntoIter = PackedRepeatedI32Iter;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        PackedRepeatedI32Iter(self)
    }
}

impl Iterator for PackedRepeatedI32Iter {
    type Item = I32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 .0.is_empty() {
            return None;
        }

        // replace self.0 with empty data to pass ownership of data to the next value
        let data = std::mem::replace(&mut self.0 .0, WireData::new(vec![]));

        // parse the value or return None
        let (value, remainder) = I32::from(data).ok()?;

        // restore self.0 to whatever data remains after the value was parsed
        self.0 .0 = remainder;

        Some(value)
    }
}
