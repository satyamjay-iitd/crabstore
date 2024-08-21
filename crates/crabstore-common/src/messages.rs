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
    ConnectResponseMT,
    CreateRequestMT,
    CreateResponseMT,
    OidReserveRequestMT,
    OidReserveResponseMT,
    OidSealRequestMT,
    OidSealResponseMT,
}

#[derive(Debug)]
pub enum Messages {
    ConnectRequest(messages::ConnectRequest),
    ConnectResponse(messages::ConnectResponse),
    CreateRequest(messages::CreateRequest),
    CreateResponse(messages::CreateResponse),
    OidReserveRequest(messages::OidReserveRequest),
    OidReserveResponse(messages::OidReserveResponse),
    OidSealRequest(messages::OidSealRequest),
    OidSealResponse(messages::OidSealResponse),
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
        if src.len() < 10 {
            return Ok(None); // Not enough data yet, return None to indicate we need more data
        }

        // Read the message type from the buffer
        let message_type = src.get_u16_le();
        let _message_size = src.get_u64_le();

        // Based on the message type, decode the appropriate Protobuf message
        let message = match message_type {
            0 => {
                let cr = messages::ConnectRequest::decode(src);
                match cr {
                    Ok(cr) => Some(Messages::ConnectRequest(cr)),
                    Err(_) => None,
                }
            }
            1 => {
                let cr = messages::ConnectResponse::decode(src);
                match cr {
                    Ok(cr) => Some(Messages::ConnectResponse(cr)),
                    Err(_) => None,
                }
            }
            2 => {
                let cr = messages::CreateRequest::decode(src);
                match cr {
                    Ok(cr) => Some(Messages::CreateRequest(cr)),
                    Err(_) => None,
                }
            }
            3 => {
                let cr = messages::CreateResponse::decode(src);
                match cr {
                    Ok(cr) => Some(Messages::CreateResponse(cr)),
                    Err(_) => None,
                }
            }
            4 => {
                let cr = messages::OidReserveRequest::decode(src);
                match cr {
                    Ok(cr) => Some(Messages::OidReserveRequest(cr)),
                    Err(_) => None,
                }
            }
            5 => {
                let cr = messages::OidReserveResponse::decode(src);
                match cr {
                    Ok(cr) => Some(Messages::OidReserveResponse(cr)),
                    Err(_) => None,
                }
            }
            6 => {
                let cr = messages::OidSealRequest::decode(src);
                match cr {
                    Ok(cr) => Some(Messages::OidSealRequest(cr)),
                    Err(_) => None,
                }
            }
            7 => {
                let cr = messages::OidSealResponse::decode(src);
                match cr {
                    Ok(cr) => Some(Messages::OidSealResponse(cr)),
                    Err(_) => None,
                }
            }
            _ => {
                // Unknown message type
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Unknown message type",
                ));
            }
        };

        Ok(message)
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
                let message_type = MessageType::ConnectRequestMT as u16;
                dst.put_u16_le(message_type);
                dst.put_u64_le(messages::ConnectRequest::encoded_len(&cr) as u64);

                cr.encode(dst)?;
                Ok(())
            }
            Messages::ConnectResponse(cr) => {
                let message_type = MessageType::ConnectResponseMT as u16;
                dst.put_u16_le(message_type);
                dst.put_u64_le(messages::ConnectResponse::encoded_len(&cr) as u64);

                cr.encode(dst)?;
                Ok(())
            }
            Messages::CreateRequest(cr) => {
                let message_type = MessageType::CreateRequestMT as u16;
                dst.put_u16_le(message_type);
                dst.put_u64_le(messages::CreateRequest::encoded_len(&cr) as u64);

                cr.encode(dst)?;
                Ok(())
            }
            Messages::CreateResponse(cr) => {
                let message_type = MessageType::CreateResponseMT as u16;
                dst.put_u16_le(message_type);
                dst.put_u64_le(messages::CreateResponse::encoded_len(&cr) as u64);

                cr.encode(dst)?;
                Ok(())
            }
            Messages::OidReserveRequest(cr) => {
                let message_type = MessageType::OidReserveRequestMT as u16;
                dst.put_u16_le(message_type);
                dst.put_u64_le(messages::OidReserveRequest::encoded_len(&cr) as u64);

                cr.encode(dst)?;
                Ok(())
            }
            Messages::OidReserveResponse(cr) => {
                let message_type = MessageType::OidReserveResponseMT as u16;
                dst.put_u16_le(message_type);
                dst.put_u64_le(messages::OidReserveResponse::encoded_len(&cr) as u64);

                cr.encode(dst)?;
                Ok(())
            }
            Messages::OidSealRequest(cr) => {
                let message_type = MessageType::OidSealRequestMT as u16;
                dst.put_u16_le(message_type);
                dst.put_u64_le(messages::OidSealRequest::encoded_len(&cr) as u64);

                cr.encode(dst)?;
                Ok(())
            }
            Messages::OidSealResponse(cr) => {
                let message_type = MessageType::OidSealResponseMT as u16;
                dst.put_u16_le(message_type);
                dst.put_u64_le(messages::OidSealResponse::encoded_len(&cr) as u64);

                cr.encode(dst)?;
                Ok(())
            }
        }
    }
}
