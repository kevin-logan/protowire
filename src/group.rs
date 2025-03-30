use crate::field::Field;
use crate::message_object::MessageObject;
use crate::varint::Varint;
use crate::wire_data::WireData;

use anyhow::Result;

pub struct Group {
    pub(crate) end_field_id: Varint,
    pub(crate) fields: Vec<Field>,
}

impl Group {
    pub fn new(field_id: u64) -> Self {
        Self {
            end_field_id: Varint::new(field_id << 3 | MessageObject::EGroup.wire_type()),
            fields: Vec::new(),
        }
    }

    pub fn with_capacity(field_id: u64, capacity: usize) -> Self {
        Self {
            end_field_id: Varint::new(field_id << 3 | MessageObject::EGroup.wire_type()),
            fields: Vec::with_capacity(capacity),
        }
    }

    pub fn from(field_id: u64, mut data: WireData) -> Result<(Self, WireData)> {
        let mut fields = Vec::new();
        loop {
            let (field, remainder) = Field::from(data)?;
            match field.get_data() {
                // in well-formed proto it would be impossible to find an EGroup for another
                // field id, as we are using `Field::from` on each field, which would parse
                // the entirety of a nested group (including its EGroup), however because
                // malformed proto could have mismatched EGroups we must verify so the error
                // we raise can be more central to the actual issue in the data rather than
                // raising a red-herring elsewhere when something else randomly fails
                MessageObject::EGroup if field.get_field_id() == field_id => {
                    data = remainder;

                    return Ok((
                        Self {
                            end_field_id: field.tag,
                            fields,
                        },
                        data,
                    ));
                }
                _ => {
                    fields.push(field);
                    data = remainder;
                }
            }
        }
    }

    pub fn get_fields(&self) -> &[Field] {
        &self.fields
    }

    pub fn take_fields(self) -> Vec<Field> {
        self.fields
    }

    pub fn set_fields(&mut self, fields: Vec<Field>) {
        self.fields = fields;
    }

    pub fn push(&mut self, f: Field) {
        self.fields.push(f);
    }
}
