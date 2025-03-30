# protowire
Utilities for inspecting and interacting with the protobuf wire format

# philosophy
protowire intends to allow efficient viewing and mutation of the protobuf wireformat. Parsing and encoding costs should only be paid for the parts of the message actually interacted with by the code. In principal most objects in protowire are wrappers around the `WireData` type which holds raw protobuf wire format data. There are a few exceptions where this is not possible (such as support for the deprecated protobuf groups in `Group`).

protowire could potentially be used as pure-Rust protobuf kernel, where static types could be generated (a la prost-build or similar) to interact with protowire objects under the hood, but for now protowire is more suited for low level viewing and manipulation of the protobuf wire format.

# examples
Check the tests in `src/lib.rs` to see example usecases
