use crate::field::Field;
use crate::wire_data::WireData;

pub struct Message(pub(crate) WireData);

impl std::default::Default for Message {
    fn default() -> Self {
        Self::new()
    }
}

impl Message {
    pub fn new() -> Self {
        Message(WireData::Mut(bytes::BytesMut::new()))
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Message(WireData::Mut(bytes::BytesMut::with_capacity(capacity)))
    }

    pub fn push(&mut self, f: Field) {
        f.serialize_into(self.0.get_mut());
    }

    pub fn serialize(self) -> WireData {
        self.0
    }
}

pub struct MessageIter(Message);

impl IntoIterator for Message {
    type IntoIter = MessageIter;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        MessageIter(self)
    }
}

impl Iterator for MessageIter {
    type Item = Field;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 .0.is_empty() {
            return None;
        }

        // replace self.0 with empty data to pass ownership of data to the next field
        let data = std::mem::replace(&mut self.0 .0, WireData::new(vec![]));

        // parse the field or return None
        let (field, remainder) = Field::from(data).ok()?;

        // restore self.0.0 to whatever data remains after the field was parsed
        self.0 .0 = remainder;

        Some(field)
    }
}
