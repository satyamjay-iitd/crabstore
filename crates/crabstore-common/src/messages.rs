use prost::bytes::{Buf, BufMut};
use prost::Message;
use std::io;
use tokio_util::codec::{Decoder, Encoder};

// Include the `items` module, which is generated from items.proto.
pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/message.rs"));
}

#[repr(u16)]
enum MessageType {
    ConnectRequestMT = 0,
    ConnectResponseMT = 1,
    CreateRequestMT = 2,
    CreateResponseMT = 3,
}

#[derive(Debug)]
pub enum Messages {
    ConnectRequest(messages::ConnectRequest),
    ConnectResponse(messages::ConnectResponse),
    CreateRequest(messages::CreateRequest),
    CreateResponse(messages::CreateResponse),
}

pub struct MessageCodec;

impl Decoder for MessageCodec {
    type Item = Messages;
    type Error = io::Error;

    fn decode(
        &mut self,
        src: &mut prost::bytes::BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        // First, check if we have enough data to read the message type
        if src.len() < 2 {
            return Ok(None); // Not enough data yet, return None to indicate we need more data
        }

        // Read the message type from the buffer
        let message_type = src.get_u16_le();

        // Based on the message type, decode the appropriate Protobuf message
        let message = match message_type {
            0 => {
                // Decode a CreateRequest message
                let cr = messages::ConnectRequest::decode(src)?;
                Messages::ConnectRequest(cr)
            }
            1 => {
                // Decode a CreateRequest message
                let cr = messages::ConnectResponse::decode(src)?;
                Messages::ConnectResponse(cr)
            }
            2 => {
                // Decode a CreateRequest message
                let cr = messages::CreateRequest::decode(src)?;
                Messages::CreateRequest(cr)
            }
            3 => {
                // Decode a CreateResponse message
                let cr = messages::CreateResponse::decode(src)?;
                Messages::CreateResponse(cr)
            }
            _ => {
                // Unknown message type
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Unknown message type",
                ));
            }
        };

        Ok(Some(message))
    }
}

impl Encoder<Messages> for MessageCodec {
    type Error = io::Error;

    fn encode(
        &mut self,
        item: Messages,
        dst: &mut prost::bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        match item {
            Messages::ConnectRequest(cr) => {
                // Encode the message type as u16
                let message_type = MessageType::ConnectRequestMT as u16;
                dst.put_u16_le(message_type);

                // Encode the Protobuf message into the buffer
                cr.encode(dst)?;

                Ok(())
            }
            Messages::ConnectResponse(cr) => {
                // Encode the message type as u16
                let message_type = MessageType::ConnectResponseMT as u16;
                dst.put_u16_le(message_type);

                // Encode the Protobuf message into the buffer
                cr.encode(dst)?;

                Ok(())
            }
            Messages::CreateRequest(cr) => {
                // Encode the message type as u16
                let message_type = MessageType::CreateRequestMT as u16;
                dst.put_u16_le(message_type);

                // Encode the Protobuf message into the buffer
                cr.encode(dst)?;

                Ok(())
            }
            Messages::CreateResponse(cr) => {
                // Encode the message type as u16
                let message_type = MessageType::CreateResponseMT as u16;
                dst.put_u16_le(message_type);

                // Encode the Protobuf message into the buffer
                cr.encode(dst)?;

                Ok(())
            }
        }
    }
}
