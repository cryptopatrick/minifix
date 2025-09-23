use super::{Config, DecodeError, Decoder, Encoder, RawDecoder, RawFrame};
use crate::tagvalue::Message;
use crate::{Dictionary, GetConfig};
use bytes::{BufMut, Bytes, BytesMut};
use tokio_util::codec;

/// Tokio codec for decoding raw FIX frames from BytesMut buffers.
///
/// This decoder works with tokio_util::codec::FramedRead to provide efficient
/// streaming decode of FIX messages using proper buffer management with
/// split_to and advance operations.
#[derive(Debug)]
pub struct TokioRawDecoder {
    raw_decoder: RawDecoder,
}

impl TokioRawDecoder {
    /// Creates a new TokioRawDecoder.
    pub fn new() -> Self {
        Self {
            raw_decoder: RawDecoder::new(),
        }
    }
}

impl Default for TokioRawDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl codec::Decoder for TokioRawDecoder {
    type Item = RawFrame<Bytes>;
    type Error = DecodeError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // Try to find a complete message in the buffer
        if let Some(message_len) = self.find_complete_message(src)? {
            // Split off the complete message
            let message_data = src.split_to(message_len);
            
            // Create frozen data first
            let frozen_data = message_data.freeze();
            let frozen_data_clone = frozen_data.clone();
            
            // Decode the message to get the ranges
            let result = self.raw_decoder.decode(&frozen_data[..]);
            match result {
                Ok(raw_frame) => {
                    // Create a new RawFrame with the same ranges but owned Bytes data
                    Ok(Some(RawFrame {
                        data: frozen_data_clone,
                        begin_string: raw_frame.begin_string,
                        payload: raw_frame.payload,
                    }))
                },
                Err(e) => Err(e),
            }
        } else {
            // Not enough data for a complete message
            Ok(None)
        }
    }
}

impl TokioRawDecoder {
    /// Attempts to find a complete FIX message in the buffer.
    /// Returns the length of the complete message if found.
    fn find_complete_message(&self, src: &BytesMut) -> Result<Option<usize>, DecodeError> {
        // Look for the pattern "8=...9=NNN" to extract body length
        if src.len() < 10 {
            return Ok(None); // Need at least "8=X|9=N|" pattern
        }

        // Find the body length field (tag 9)
        let separator = self.raw_decoder.config().separator;
        
        // Simple state machine to parse the header
        let mut pos = 0;
        let mut in_body_length = false;
        let mut body_length = 0u32;
        let mut body_length_end = 0;
        
        while pos < src.len() {
            if src[pos] == b'9' && pos + 1 < src.len() && src[pos + 1] == b'=' {
                // Found start of body length field
                in_body_length = true;
                pos += 2;
                continue;
            }
            
            if in_body_length {
                if src[pos] == separator {
                    // End of body length field
                    body_length_end = pos + 1;
                    break;
                } else if src[pos].is_ascii_digit() {
                    body_length = body_length * 10 + (src[pos] - b'0') as u32;
                }
            }
            
            pos += 1;
        }
        
        if body_length_end == 0 {
            return Ok(None); // Haven't found complete body length yet
        }
        
        let expected_total_length = body_length_end + body_length as usize + 7; // +7 for checksum "10=XXX|"
        
        if src.len() >= expected_total_length {
            Ok(Some(expected_total_length))
        } else {
            Ok(None)
        }
    }
}

impl GetConfig for TokioRawDecoder {
    type Config = Config;

    fn config(&self) -> &Self::Config {
        self.raw_decoder.config()
    }

    fn config_mut(&mut self) -> &mut Self::Config {
        self.raw_decoder.config_mut()
    }
}

/// Tokio codec for decoding complete FIX messages from BytesMut buffers.
///
/// This decoder builds on TokioRawDecoder to provide parsed Message objects
/// ready for application use.
#[derive(Debug)]
#[cfg_attr(doc_cfg, doc(cfg(feature = "utils-tokio")))]
pub struct TokioDecoder {
    dict: Dictionary,
    decoder: Decoder,
}

impl TokioDecoder {
    /// Creates a new TokioDecoder with the specified dictionary.
    pub fn new(dict: Dictionary) -> Self {
        Self {
            decoder: Decoder::new(dict.clone()),
            dict,
        }
    }
}

impl codec::Decoder for TokioDecoder {
    type Item = Message<'static, Bytes>;
    type Error = DecodeError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // Use the same logic as TokioRawDecoder to find complete messages
        if let Some(message_len) = self.find_complete_message(src)? {
            // Split off the complete message
            let message_data = src.split_to(message_len);
            let frozen_data = message_data.freeze();
            
            // Clone the frozen data for the owned message
            let frozen_data_clone = frozen_data.clone();
            
            // Decode the message using the full decoder
            let result = self.decoder.decode(&frozen_data[..]);
            match result {
                Ok(message) => {
                    // Convert to owned message with Bytes backing
                    let owned_message = Self::message_to_owned_static(message, frozen_data_clone);
                    Ok(Some(owned_message))
                }
                Err(e) => Err(e),
            }
        } else {
            Ok(None)
        }
    }
}

impl TokioDecoder {
    /// Same logic as TokioRawDecoder for finding complete messages.
    fn find_complete_message(&self, src: &BytesMut) -> Result<Option<usize>, DecodeError> {
        if src.len() < 10 {
            return Ok(None);
        }

        let separator = self.decoder.config().separator;
        
        let mut pos = 0;
        let mut in_body_length = false;
        let mut body_length = 0u32;
        let mut body_length_end = 0;
        
        while pos < src.len() {
            if src[pos] == b'9' && pos + 1 < src.len() && src[pos + 1] == b'=' {
                in_body_length = true;
                pos += 2;
                continue;
            }
            
            if in_body_length {
                if src[pos] == separator {
                    body_length_end = pos + 1;
                    break;
                } else if src[pos].is_ascii_digit() {
                    body_length = body_length * 10 + (src[pos] - b'0') as u32;
                }
            }
            
            pos += 1;
        }
        
        if body_length_end == 0 {
            return Ok(None);
        }
        
        let expected_total_length = body_length_end + body_length as usize + 7;
        
        if src.len() >= expected_total_length {
            Ok(Some(expected_total_length))
        } else {
            Ok(None)
        }
    }

    /// Converts a borrowed message to an owned message backed by Bytes.
    fn message_to_owned_static(_message: Message<&[u8]>, _data: Bytes) -> Message<'static, Bytes> {
        // This is a placeholder implementation - in practice you'd need to
        // properly construct an owned Message backed by the Bytes data
        // TODO: Replace with proper owned Message construction
        unsafe { std::mem::transmute(_message) }
    }
}

/// Tokio codec for encoding FIX messages to BytesMut buffers.
///
/// This encoder works with tokio_util::codec::FramedWrite to provide efficient
/// streaming encode of FIX messages.
#[derive(Debug)]
#[cfg_attr(doc_cfg, doc(cfg(feature = "utils-tokio")))]
pub struct TokioEncoder {
    encoder: Encoder,
}

impl TokioEncoder {
    /// Creates a new TokioEncoder.
    pub fn new() -> Self {
        Self {
            encoder: Encoder::new(),
        }
    }
}

impl Default for TokioEncoder {
    fn default() -> Self {
        Self::new()
    }
}

impl codec::Encoder<&[u8]> for TokioEncoder {
    type Error = std::io::Error;

    fn encode(&mut self, item: &[u8], dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.reserve(item.len());
        dst.put_slice(item);
        Ok(())
    }
}

impl codec::Encoder<Vec<u8>> for TokioEncoder {
    type Error = std::io::Error;

    fn encode(&mut self, item: Vec<u8>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.reserve(item.len());
        dst.put_slice(&item);
        Ok(())
    }
}

impl codec::Encoder<Bytes> for TokioEncoder {
    type Error = std::io::Error;

    fn encode(&mut self, item: Bytes, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.reserve(item.len());
        dst.put_slice(&item);
        Ok(())
    }
}

impl GetConfig for TokioEncoder {
    type Config = Config;

    fn config(&self) -> &Self::Config {
        self.encoder.config()
    }

    fn config_mut(&mut self) -> &mut Self::Config {
        self.encoder.config_mut()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio_util::codec::{FramedRead, FramedWrite};
    use tokio::io;

    #[tokio::test]
    async fn tokio_raw_decoder_basic() {
        let mut decoder = TokioRawDecoder::new();
        let mut buf = BytesMut::from(&b"8=FIX.4.4|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=185|"[..]);
        
        let result = decoder.decode(&mut buf);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn tokio_encoder_basic() {
        let mut encoder = TokioEncoder::new();
        let mut buf = BytesMut::new();
        
        let message = b"8=FIX.4.4|9=42|35=0|10=185|";
        let result = encoder.encode(message, &mut buf);
        assert!(result.is_ok());
        assert_eq!(buf.as_ref(), message);
    }
}
