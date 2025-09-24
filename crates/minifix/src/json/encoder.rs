use crate::FieldType;
use crate::dict::IsFieldDefinition;

/// A codec for the JSON encoding type.
#[derive(Debug, Clone)]
pub struct Encoder {
    buffer: Vec<u8>,
    has_message: bool,
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Encoder {
    /// Constructs a fresh JSON encoder with an empty internal buffer.
    pub fn new() -> Self {
        Self { buffer: Vec::new(), has_message: false }
    }

    /// Begins a new FIX message and returns the initial typestate builder.
    pub fn start_message(&mut self) -> encoder_states::Initial {
        self.buffer.clear();
        self.has_message = true;
        encoder_states::Initial { encoder: self }
    }
}

/// Typestates for the JSON [`Encoder`].
pub mod encoder_states {
    use super::*;

    /// Typestate produced by [`Encoder::start_message`].
    #[derive(Debug)]
    #[must_use]
    pub struct Initial<'a> {
        /// Mutable reference to the underlying encoder state machine.
        pub encoder: &'a mut Encoder,
    }

    impl<'a> Initial<'a> {
        /// Starts populating the standard header section of the message.
        pub fn with_header(self) -> StdHeader<'a> {
            self.encoder.buffer.extend_from_slice(br#"{"StandardHeader":{"#);
            StdHeader { encoder: self.encoder }
        }
    }

    trait EncoderStateAtTopLevel
    where
        Self: Sized,
    {
        fn encoder_mut(&mut self) -> &mut Encoder;

        /// Adds a `field` with a `value` to the current message.
        fn set<'a, T, F>(mut self, field: &F, value: T) -> Self
        where
            T: FieldType<'a>,
            F: IsFieldDefinition,
        {
            debug_assert!(field.name().is_ascii());
            let encoder = self.encoder_mut();
            encoder.buffer.extend_from_slice(br#"""#);
            field.name().as_bytes().serialize(&mut encoder.buffer);
            encoder.buffer.extend_from_slice(br#"":""#);
            value.serialize(&mut encoder.buffer);
            encoder.buffer.extend_from_slice(br#"""#);
            self
        }
    }

    /// Typestate produced by [`Initial::with_header`].
    #[derive(Debug)]
    #[must_use]
    pub struct StdHeader<'a> {
        encoder: &'a mut Encoder,
    }

    impl<'a> StdHeader<'a> {
        /// Switches from populating the standard header to encoding the body.
        pub fn with_body(self) -> Body<'a> {
            self.encoder.buffer.extend_from_slice(br#"},"Body":{"#);
            Body { encoder: self.encoder }
        }

        /// Adds a header field to the JSON message.
        pub fn set<T, F>(self, field: &F, value: T) -> Self
        where
            T: FieldType<'a>,
            F: IsFieldDefinition,
        {
            EncoderStateAtTopLevel::set(self, field, value)
        }
    }

    impl<'a> EncoderStateAtTopLevel for StdHeader<'a> {
        fn encoder_mut(&mut self) -> &mut Encoder {
            self.encoder
        }
    }

    /// Typestate produced by [`StdHeader::with_body`].
    #[derive(Debug)]
    #[must_use]
    pub struct Body<'a> {
        encoder: &'a mut Encoder,
    }

    impl<'a> Body<'a> {
        /// Switches from the message body to encoding the standard trailer.
        pub fn with_trailer(self) -> StdTrailer<'a> {
            self.encoder.buffer.extend_from_slice(br#"},"StandardTrailer":{"#);
            StdTrailer { encoder: self.encoder }
        }

        /// Adds a body field to the JSON message.
        pub fn set<T, F>(self, field: &F, value: T) -> Self
        where
            T: FieldType<'a>,
            F: IsFieldDefinition,
        {
            EncoderStateAtTopLevel::set(self, field, value)
        }
    }

    impl<'a> EncoderStateAtTopLevel for Body<'a> {
        fn encoder_mut(&mut self) -> &mut Encoder {
            self.encoder
        }
    }

    /// Typestate produced by [`Body::with_trailer`].
    #[derive(Debug)]
    #[must_use]
    pub struct StdTrailer<'a> {
        encoder: &'a mut Encoder,
    }

    impl<'a> StdTrailer<'a> {
        /// Finalizes the JSON message and returns the encoded string slice.
        pub fn done(self) -> &'a str {
            self.encoder.buffer.extend_from_slice(b"}}");
            std::str::from_utf8(&self.encoder.buffer[..]).unwrap()
        }

        /// Adds a trailer field to the JSON message.
        pub fn set<T, F>(self, field: &F, value: T) -> Self
        where
            T: FieldType<'a>,
            F: IsFieldDefinition,
        {
            EncoderStateAtTopLevel::set(self, field, value)
        }
    }

    impl<'a> EncoderStateAtTopLevel for StdTrailer<'a> {
        fn encoder_mut(&mut self) -> &mut Encoder {
            self.encoder
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_message_is_valid_json() {
        let mut encoder = Encoder::new();
        let message = encoder
            .start_message()
            .with_header()
            .with_body()
            .with_trailer()
            .done();
        let json = serde_json::from_str::<serde_json::Value>(message);
        assert!(json.is_ok());
    }
}
