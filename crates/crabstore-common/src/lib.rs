use prost::bytes::{Buf, BufMut};
use prost::Message;
use std::fmt;
use std::fmt::Write;
use std::hash::{Hash, Hasher};
use std::io;
use tokio_util::codec::{Decoder, Encoder};

// Include the `items` module, which is generated from items.proto.
pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/message.rs"));
}

#[repr(u16)]
enum MessageType {
    CreateRequestMT = 0,
    CreateResponseMT = 1,
}

#[derive(Debug)]
pub enum Messages {
    CreateRequest(messages::CreateRequest),
    CreateResponse(messages::CreateResponse),
}

pub struct MessageCodec {}

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
                let cr = messages::CreateRequest::decode(src)?;
                Messages::CreateRequest(cr)
            }
            1 => {
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

pub const UNIQUE_ID_SIZE: usize = 20; // or whatever kUniqueIDSize is in your C++ code

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ObjectId {
    id: [u8; UNIQUE_ID_SIZE],
}

impl ObjectId {
    pub fn from_binary(binary: &[u8]) -> Self {
        let mut id = [0u8; UNIQUE_ID_SIZE];
        id.copy_from_slice(&binary[..UNIQUE_ID_SIZE]);
        ObjectId { id }
    }

    pub fn data(&self) -> &[u8] {
        &self.id
    }

    pub fn mutable_data(&mut self) -> &mut [u8] {
        &mut self.id
    }

    pub fn binary(&self) -> Vec<u8> {
        self.id.to_vec()
    }

    pub fn hex(&self) -> String {
        self.id.iter().fold(String::new(), |mut output, b| {
            let _ = write!(output, "{b:02x}");
            output
        })
    }

    pub fn hash(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();
        self.id.hash(&mut hasher);
        hasher.finish()
    }

    pub fn size() -> usize {
        UNIQUE_ID_SIZE
    }
}

impl fmt::Debug for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UniqueID({})", self.hex())
    }
}

impl Hash for ObjectId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
