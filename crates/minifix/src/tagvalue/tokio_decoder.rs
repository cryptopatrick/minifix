use super::{Config, DecodeError, Decoder, RawDecoder, RawFrame};
use crate::{Dictionary, GetConfig};
use bytes::{Bytes, BytesMut};
use tokio_util::codec;

#[derive(Debug)]
pub struct TokioRawDecoder {
    dict: Dictionary,
    raw_decoder: RawDecoder,
}

impl codec::Decoder for TokioRawDecoder {
    type Item = RawFrame<Bytes>;
    type Error = DecodeError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let split = src.split();
        let result = self.raw_decoder.decode(split);
        match result {
            Ok(raw_frame) => Ok(Some(RawFrame {
                data: raw_frame.data.freeze(),
                begin_string: raw_frame.begin_string,
                payload: raw_frame.payload,
            })),
            Err(DecodeError::Invalid) => Ok(None),
            Err(e) => Err(e),
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

#[derive(Debug)]
#[cfg_attr(doc_cfg, doc(cfg(feature = "utils-tokio")))]
pub struct TokioDecoder {
    dict: Dictionary,
    raw_decoder: Decoder,
}

//impl codec::Decoder for TokioDecoder {
//    type Item = Message;
//    type Error = DecodeError;
//
//    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
//        let split = src.split();
//        let result = self.raw_decoder.decode(split);
//        match result {
//            Ok(raw_frame) => Ok(Some(raw_frame.map_data(|bytes_mut| bytes_mut.freeze()))),
//            Err(DecodeError::Invalid) => Ok(None),
//            Err(e) => Err(e),
//        }
//    }
//}
