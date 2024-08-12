use std::io;

use tokio_util::codec::{Decoder, Encoder};

// Include the `items` module, which is generated from items.proto.
pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/message.rs"));
}

#[repr(u16)]
enum MessageType {
    CreateRequest = 0,
    CreateResponse = 1,
}

enum Message {
    CreateRequest(messages::CreateRequest),
    CreateResponse(messages::CreateResponse),
}

struct MessageCodec {}

impl Decoder for MessageCodec {
    type Item = Message;
    type Error = io::Error;

    fn decode(
        &mut self,
        src: &mut prost::bytes::BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        todo!()
    }
}

impl Encoder<Message> for MessageCodec {
    type Error = io::Error;

    fn encode(
        &mut self,
        item: Message,
        dst: &mut prost::bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
