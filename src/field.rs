use crate::group::Group;
use crate::i32::I32;
use crate::i64::I64;
use crate::len::Len;
use crate::message_object::MessageObject;
use crate::varint::Varint;
use crate::wire_data::WireData;

use anyhow::{anyhow, Context, Result};

pub struct Field {
    pub(crate) tag: Varint,
    pub(crate) data: MessageObject,
}

impl Field {
    pub fn new(id: u64, object: MessageObject) -> Self {
        let tag = (id << 3) | (object.wire_type() & 0b0000_0111);

        Self {
            tag: Varint::new(tag),
            data: object,
        }
    }
    pub fn from(data: WireData) -> Result<(Self, WireData)> {
        let (tag, remainder) = Varint::from(data).context("Field must start with a Varint")?;
        let tag_value = tag.get();
        match Self::wire_type_from_tag(tag_value) {
            0 => Varint::from(remainder)
                .map(|(value, remainder)| {
                    (
                        Self {
                            tag,
                            data: MessageObject::Varint(value),
                        },
                        remainder,
                    )
                })
                .context("Field could not parse Varint"),
            1 => I64::from(remainder)
                .map(|(value, remainder)| {
                    (
                        Self {
                            tag,
                            data: MessageObject::I64(value),
                        },
                        remainder,
                    )
                })
                .context("Field could not parse I64"),
            2 => Len::from(remainder)
                .map(|(value, remainder)| {
                    (
                        Self {
                            tag,
                            data: MessageObject::Len(value),
                        },
                        remainder,
                    )
                })
                .context("Field could not parse Len"),
            3 => Group::from(Self::field_id_from_tag(tag_value), remainder)
                .map(|(value, remainder)| {
                    (
                        Self {
                            tag,
                            data: MessageObject::Group(value),
                        },
                        remainder,
                    )
                })
                .context("Field could not parse Group"),
            4 => Ok((
                Self {
                    tag,
                    data: MessageObject::EGroup,
                },
                remainder,
            )),
            5 => I32::from(remainder)
                .map(|(value, remainder)| {
                    (
                        Self {
                            tag,
                            data: MessageObject::I32(value),
                        },
                        remainder,
                    )
                })
                .context("Field could not parse I32"),
            _ => Err(anyhow!("Invalid wire type")),
        }
    }

    pub fn into_varint(self) -> Option<Varint> {
        match self.data {
            MessageObject::Varint(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_varint(&self) -> Option<&Varint> {
        match self.data {
            MessageObject::Varint(ref value) => Some(value),
            _ => None,
        }
    }

    pub fn into_i64(self) -> Option<I64> {
        match self.data {
            MessageObject::I64(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<&I64> {
        match self.data {
            MessageObject::I64(ref value) => Some(value),
            _ => None,
        }
    }

    pub fn into_len(self) -> Option<Len> {
        match self.data {
            MessageObject::Len(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_len(&self) -> Option<&Len> {
        match self.data {
            MessageObject::Len(ref value) => Some(value),
            _ => None,
        }
    }

    pub fn into_group(self) -> Option<Group> {
        match self.data {
            MessageObject::Group(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_group(&self) -> Option<&Group> {
        match self.data {
            MessageObject::Group(ref value) => Some(value),
            _ => None,
        }
    }

    pub fn into_i32(self) -> Option<I32> {
        match self.data {
            MessageObject::I32(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_i32(&self) -> Option<&I32> {
        match self.data {
            MessageObject::I32(ref value) => Some(value),
            _ => None,
        }
    }

    pub fn get_field_id(&self) -> u64 {
        Self::field_id_from_tag(self.tag.get())
    }

    pub fn get_wire_type(&self) -> u64 {
        Self::wire_type_from_tag(self.tag.get())
    }

    pub fn set_tag(&mut self, field_id: u64, wire_type: u64) {
        self.tag.set(field_id << 3 | wire_type);
    }

    pub fn get_data(&self) -> &MessageObject {
        &self.data
    }

    pub fn get_data_mut(&mut self) -> &mut MessageObject {
        &mut self.data
    }

    pub fn set_data(&mut self, object: MessageObject) {
        self.data = object;
    }

    pub fn field_id_from_tag(tag: u64) -> u64 {
        tag >> 3
    }

    pub fn wire_type_from_tag(tag: u64) -> u64 {
        tag & 0b0000_0111
    }

    pub fn serialize(self) -> WireData {
        let mut dest = bytes::BytesMut::with_capacity(self.tag.0.len() + self.data.byte_len());

        self.serialize_into(&mut dest);

        WireData::Mut(dest)
    }

    pub(crate) fn serialize_into(self, dest: &mut bytes::BytesMut) {
        dest.extend_from_slice(self.tag.0.as_ref());
        self.data.serialize_into(dest);
    }
}
