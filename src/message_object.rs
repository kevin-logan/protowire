use crate::group::Group;
use crate::i32::I32;
use crate::i64::I64;
use crate::len::Len;
use crate::varint::Varint;
use crate::wire_data::WireData;

pub enum MessageObject {
    Varint(Varint),
    I64(I64),
    Len(Len),
    Group(Group),
    EGroup,
    I32(I32),
}

impl MessageObject {
    pub fn serialize(self) -> WireData {
        let mut dest = bytes::BytesMut::with_capacity(self.byte_len());
        self.serialize_into(&mut dest);

        WireData::Mut(dest)
    }

    pub(crate) fn serialize_into(self, dest: &mut bytes::BytesMut) {
        match self {
            MessageObject::Varint(value) => dest.extend_from_slice(value.0.as_ref()),
            MessageObject::I64(value) => dest.extend_from_slice(value.0.as_ref()),
            MessageObject::Len(value) => {
                // need to concatenate the length Varint and the value
                dest.extend_from_slice(value.length.0.as_ref());
                dest.extend_from_slice(value.inner.as_ref());
            }
            MessageObject::Group(Group {
                end_field_id,
                fields,
            }) => {
                for field in fields {
                    field.serialize_into(dest);
                }
                dest.extend_from_slice(end_field_id.0.as_ref());
            }
            MessageObject::EGroup => {}
            MessageObject::I32(value) => dest.extend_from_slice(value.0.as_ref()),
        }
    }

    pub fn byte_len(&self) -> usize {
        match self {
            MessageObject::Varint(value) => value.0.len(),
            MessageObject::I64(value) => value.0.len(),
            MessageObject::Len(value) => {
                // need to concatenate the length Varint and the value
                value.length.0.len() + value.inner.len()
            }
            MessageObject::Group(Group {
                end_field_id,
                fields,
            }) => {
                let mut len = 0;
                for field in fields {
                    len += field.tag.0.len();
                    len += field.data.byte_len();
                }
                len + end_field_id.0.len()
            }
            MessageObject::EGroup => 0,
            MessageObject::I32(value) => value.0.len(),
        }
    }

    pub fn wire_type(&self) -> u64 {
        match self {
            MessageObject::Varint(_) => 0,
            MessageObject::I64(_) => 1,
            MessageObject::Len(_) => 2,
            MessageObject::Group(_) => 3,
            MessageObject::EGroup => 4,
            MessageObject::I32(_) => 5,
        }
    }
}
