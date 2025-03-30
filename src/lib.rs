//! Library

mod field;
mod group;
mod i32;
mod i64;
mod len;
mod message;
mod message_object;
mod packed_repeated;
mod varint;
mod wire_data;

pub use field::Field;
pub use group::Group;
pub use i32::I32;
pub use i64::I64;
pub use len::Len;
pub use message::Message;
pub use message_object::MessageObject;
pub use packed_repeated::{PackedRepeatedI32, PackedRepeatedI64, PackedRepeatedVarint};
pub use varint::Varint;
pub use wire_data::WireData;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding() {
        /* Test based off the following protoscope:
            1: -13.37i32
            2: !{
                    1: 13.37
                    2: {"hello, world!"}
            }
            3: {
                    405: 10101
                    32: -5z
                    61: {
                            1: {"hello"}
                            1: {","}
                            1: {" "}
                            1: {"world!"}
                    }
            }
        */
        // outer message
        let mut outer_message = Message::new();
        let field_1 = Field::new(1, MessageObject::I32(I32::new_float(-13.37)));
        let mut field_2 = Group::with_capacity(2, 2);
        let gfield_1 = Field::new(1, MessageObject::I64(I64::new_double(13.37)));
        let gfield_2 = Field::new(2, MessageObject::Len(Len::new_string("hello, world!")));
        field_2.push(gfield_1);
        field_2.push(gfield_2);

        let mut field_3 = Message::new();
        let mfield_405 = Field::new(405, MessageObject::Varint(Varint::new(10101)));
        let mfield_32 = Field::new(32, MessageObject::Varint(Varint::new_proto_sint64(-5)));

        let mut mfield_61 = Message::new();
        let repeated_1 = Field::new(1, MessageObject::Len(Len::new_string("hello")));
        let repeated_2 = Field::new(1, MessageObject::Len(Len::new_string(",")));
        let repeated_3 = Field::new(1, MessageObject::Len(Len::new_string(" ")));
        let repeated_4 = Field::new(1, MessageObject::Len(Len::new_string("world!")));
        mfield_61.push(repeated_1);
        mfield_61.push(repeated_2);
        mfield_61.push(repeated_3);
        mfield_61.push(repeated_4);

        field_3.push(mfield_405);
        field_3.push(mfield_32);
        field_3.push(Field::new(
            61,
            MessageObject::Len(Len::new_message(mfield_61)),
        ));

        outer_message.push(field_1);
        outer_message.push(Field::new(2, MessageObject::Group(field_2)));
        outer_message.push(Field::new(3, MessageObject::Len(Len::new_message(field_3))));

        let serialized = outer_message.serialize();

        // write to file
        let mut file = std::fs::File::create("test_encoding.pb").unwrap();
        std::io::Write::write_all(&mut file, serialized.as_ref()).unwrap();

        impl_complex_test(serialized);
    }

    #[test]
    fn test_varint_encoding() {
        assert_eq!(
            &Varint::encode(150),
            &[
                0b10010110, 0b00000001, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
                0b00000000, 0b00000000, 0b00000000
            ]
        );

        assert_eq!(
            &Varint::encode(-2i64 as u64),
            &[
                0b11111110, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
                0b11111111, 0b11111111, 0b00000001
            ]
        );

        assert_eq!(&Varint::encode(2), &Varint::encode_sint64(1));
        assert_eq!(&Varint::encode(3), &Varint::encode_sint64(-2));

        assert_eq!(
            &Varint::encode(0xfffffffe),
            &Varint::encode_sint64(0x7fffffff)
        );

        assert_eq!(&Varint::encode(2), &Varint::encode_sint32(1));
        assert_eq!(&Varint::encode(3), &Varint::encode_sint32(-2));

        assert_eq!(
            &Varint::encode(0xfffffffe),
            &Varint::encode_sint32(0x7fffffff)
        );
    }

    #[test]
    fn test_varint_view() {
        // basic u64 (example from protobuf protocol documentation)
        let data = WireData::new(vec![0b10010110, 0b0000001]);
        let (view, _) = Varint::from(data).unwrap();
        assert_eq!(view.get(), 150);

        // testing int64 2's complement (example from protobuf protocol documentation)
        let data = WireData::new(vec![
            0b11111110, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
            0b11111111, 0b11111111, 0b00000001,
        ]);
        let (view, _) = Varint::from(data).unwrap();
        assert_eq!(view.as_proto_int64(), -2);

        // testing int32 2's complement (derived)
        let data = WireData::new(vec![
            0b11111011, 0b11111111, 0b11111111, 0b11111111, 0b00001111,
        ]);
        let (view, _) = Varint::from(data).unwrap();
        assert_eq!(view.as_proto_int32(), -5);

        // testing sint32/64 -500 zigzag (via protoscope | xxd -b)
        let data = WireData::new(vec![0b00001000, 0b11100111, 0b00000111]);
        let (field, remainder) = Field::from(data).unwrap();
        assert!(remainder.is_empty());
        let value = field.get_data();
        assert_eq!(field.get_field_id(), 1);
        assert_eq!(field.get_wire_type(), 0);
        let MessageObject::Varint(view) = value else {
            panic!("Value must be Varint")
        };
        assert_eq!(view.as_proto_sint64(), -500);
        assert_eq!(view.as_proto_sint32(), -500);
    }

    #[test]
    fn test_i64_view() {
        // testing I64 1 (via protoscope | xxd -b)
        let data = WireData::new(vec![
            0b00001001, 0b00000001, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ]);
        let (field, remainder) = Field::from(data).unwrap();
        assert!(remainder.is_empty());
        let value = field.get_data();
        assert_eq!(field.get_field_id(), 1);
        assert_eq!(field.get_wire_type(), 1);
        let MessageObject::I64(view) = value else {
            panic!("Value must be I64")
        };
        assert_eq!(view.get(), 1);

        // testing I64 -524 (via protoscope | xxd -b)
        let data = WireData::new(vec![
            0b00001001, 0b11110100, 0b11111101, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
            0b11111111, 0b11111111,
        ]);
        let (field, remainder) = Field::from(data).unwrap();
        assert!(remainder.is_empty());
        let value = field.get_data();
        assert_eq!(field.get_field_id(), 1);
        assert_eq!(field.get_wire_type(), 1);
        let MessageObject::I64(view) = value else {
            panic!("Value must be I64")
        };
        assert_eq!(view.get(), -524);

        // testing I64 13.37 double (via protoscope | xxd -b)
        let data = WireData::new(vec![
            0b00001001, 0b00111101, 0b00001010, 0b11010111, 0b10100011, 0b01110000, 0b10111101,
            0b00101010, 0b01000000,
        ]);
        let (field, remainder) = Field::from(data).unwrap();
        assert!(remainder.is_empty());
        let value = field.get_data();
        assert_eq!(field.get_field_id(), 1);
        assert_eq!(field.get_wire_type(), 1);
        let MessageObject::I64(view) = value else {
            panic!("Value must be I64")
        };
        assert_eq!(view.get_double(), 13.37);
    }

    #[test]
    fn test_i32_view() {
        // testing I32 1 (via protoscope | xxd -b)
        let data = WireData::new(vec![
            0b00001101, 0b00000001, 0b00000000, 0b00000000, 0b00000000,
        ]);
        let (field, remainder) = Field::from(data).unwrap();
        assert!(remainder.is_empty());
        let value = field.get_data();
        assert_eq!(field.get_field_id(), 1);
        assert_eq!(field.get_wire_type(), 5);
        let MessageObject::I32(view) = value else {
            panic!("Value must be I32")
        };
        assert_eq!(view.get(), 1);

        // testing I32 -524 (via protoscope | xxd -b)
        let data = WireData::new(vec![
            0b00001101, 0b11110100, 0b11111101, 0b11111111, 0b11111111,
        ]);
        let (field, remainder) = Field::from(data).unwrap();
        assert!(remainder.is_empty());
        let value = field.get_data();
        assert_eq!(field.get_field_id(), 1);
        assert_eq!(field.get_wire_type(), 5);
        let MessageObject::I32(view) = value else {
            panic!("Value must be I32")
        };
        assert_eq!(view.get(), -524);

        // testing I32 -13.37 float (via protoscope | xxd -b)
        let data = WireData::new(vec![
            0b00001101, 0b10000101, 0b11101011, 0b01010101, 0b11000001,
        ]);
        let (field, remainder) = Field::from(data).unwrap();
        assert!(remainder.is_empty());
        let value = field.get_data();
        assert_eq!(field.get_field_id(), 1);
        assert_eq!(field.get_wire_type(), 5);
        let MessageObject::I32(view) = value else {
            panic!("Value must be I32")
        };
        assert_eq!(view.get_float(), -13.37);
    }

    fn impl_complex_test(data: WireData) {
        // get 1: -13.37.i32
        let (view, remainder) = Field::from(data).unwrap();
        assert!(!remainder.is_empty());
        assert_eq!(view.get_field_id(), 1);
        assert_eq!(view.into_i32().unwrap().get_float(), -13.37);

        // get 2: !{ ... }
        let (view, remainder) = Field::from(remainder).unwrap();
        assert!(!remainder.is_empty());
        assert_eq!(view.get_field_id(), 2);

        let group = view.into_group().unwrap();
        assert_eq!(group.get_fields().len(), 2);
        let child_1 = &group.get_fields()[0];
        let child_2 = &group.get_fields()[1];

        assert_eq!(child_1.get_field_id(), 1);
        assert_eq!(child_2.get_field_id(), 2);

        assert_eq!(child_1.as_i64().unwrap().get_double(), 13.37);
        assert_eq!(child_2.as_len().unwrap().as_str().unwrap(), "hello, world!");

        // get 3: { ... }
        let (view, remainder) = Field::from(remainder).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(view.get_field_id(), 3);

        let len = view.into_len().unwrap();

        // get 405: 10101
        let (inner_view, remainder) = Field::from(len.get_data()).unwrap();
        assert!(!remainder.is_empty());
        assert_eq!(inner_view.get_field_id(), 405);
        assert_eq!(inner_view.into_varint().unwrap().get(), 10101);

        // get 32: -5z
        let (inner_view, remainder) = Field::from(remainder).unwrap();
        assert!(!remainder.is_empty());
        assert_eq!(inner_view.get_field_id(), 32);
        assert_eq!(inner_view.into_varint().unwrap().as_proto_sint64(), -5);

        // get 61: { ... }
        let (inner_view, remainder) = Field::from(remainder).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(inner_view.get_field_id(), 61);

        let inner_len = inner_view.into_len().unwrap();

        let mut inner_message = inner_len.into_message().into_iter();
        let repeat_value = inner_message.next().unwrap();
        assert_eq!(repeat_value.get_field_id(), 1);
        assert_eq!(repeat_value.into_len().unwrap().as_str().unwrap(), "hello");
        let repeat_value = inner_message.next().unwrap();
        assert_eq!(repeat_value.get_field_id(), 1);
        assert_eq!(repeat_value.into_len().unwrap().as_str().unwrap(), ",");
        let repeat_value = inner_message.next().unwrap();
        assert_eq!(repeat_value.get_field_id(), 1);
        assert_eq!(repeat_value.into_len().unwrap().as_str().unwrap(), " ");
        let repeat_value = inner_message.next().unwrap();
        assert_eq!(repeat_value.get_field_id(), 1);
        assert_eq!(repeat_value.into_len().unwrap().as_str().unwrap(), "world!");
        assert!(inner_message.next().is_none());
    }

    #[test]
    fn test_complex() {
        /* Test based off the following protoscope:
        1: -13.37i32
        2: !{
                1: 13.37
                2: {"hello, world!"}
        }
        3: {
                405: 10101
                32: -5z
                61: {
                        1: {"hello"}
                        1: {","}
                        1: {" "}
                        1: {"world!"}
                }
        }
                    */
        let data = WireData::new(vec![
            0b00001101, 0b10000101, 0b11101011, 0b01010101, 0b11000001, 0b00010011, 0b00001001,
            0b00111101, 0b00001010, 0b11010111, 0b10100011, 0b01110000, 0b10111101, 0b00101010,
            0b01000000, 0b00010010, 0b00001101, 0b01101000, 0b01100101, 0b01101100, 0b01101100,
            0b01101111, 0b00101100, 0b00100000, 0b01110111, 0b01101111, 0b01110010, 0b01101100,
            0b01100100, 0b00100001, 0b00010100, 0b00011010, 0b00011111, 0b10101000, 0b00011001,
            0b11110101, 0b01001110, 0b10000000, 0b00000010, 0b00001001, 0b11101010, 0b00000011,
            0b00010101, 0b00001010, 0b00000101, 0b01101000, 0b01100101, 0b01101100, 0b01101100,
            0b01101111, 0b00001010, 0b00000001, 0b00101100, 0b00001010, 0b00000001, 0b00100000,
            0b00001010, 0b00000110, 0b01110111, 0b01101111, 0b01110010, 0b01101100, 0b01100100,
            0b00100001,
        ]);

        impl_complex_test(data);
    }
}
