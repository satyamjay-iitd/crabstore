// Include the `items` module, which is generated from items.proto.
pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/snazzy.items.rs"));
}
