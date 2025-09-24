use super::config::TokioFixConfig;
use crate::Dictionary;
use crate::session::Backend;
use crate::session::tokio_connection::{
    TokioAppChannels, TokioAppHandler, TokioFixConnection,
};
use crate::tagvalue::{Decoder, Message};
use std::error::Error;
use std::fmt;
use tokio::net::TcpStream;

/// A high-level Tokio FIX client that provides an easy-to-use interface
/// for FIX protocol communication.
///
/// This client handles the low-level details of FIX session management,
/// heartbeats, and message framing, allowing applications to focus on
/// business logic.
#[derive(Debug)]
pub struct TokioFixClient {
    config: TokioFixConfig,
    dictionary: Dictionary,
    backend: SimpleBackend,
}

impl TokioFixClient {
    /// Creates a new Tokio FIX client with the specified configuration and dictionary.
    pub fn new(config: TokioFixConfig, dictionary: Dictionary) -> Self {
        Self { config, dictionary, backend: SimpleBackend::new() }
    }

    /// Starts the FIX client using the provided TCP stream.
    ///
    /// This method will handle the complete FIX session lifecycle including
    /// logon, heartbeat management, and graceful logout.
    pub async fn start(
        &mut self,
        stream: TcpStream,
    ) -> Result<TokioAppChannels, Box<dyn Error + Send + Sync>> {
        let decoder = Decoder::new(self.dictionary.clone()).streaming(vec![]);
        let (app_handler, app_channels) = TokioAppHandler::new();

        let mut connection =
            TokioFixConnection::new(self.config.clone(), self.backend.clone());

        // Start the connection in a background task
        let _connection_handle = tokio::spawn(async move {
            connection
                .start_with_stream(stream, decoder, Some(app_handler))
                .await
        });

        // Return the app channels for the application to use
        Ok(app_channels)
    }

    /// Starts the FIX client and returns both the app channels and a handle to the connection task.
    pub async fn start_with_handle(
        &mut self,
        stream: TcpStream,
    ) -> Result<
        (
            TokioAppChannels,
            tokio::task::JoinHandle<Result<(), Box<dyn Error + Send + Sync>>>,
        ),
        Box<dyn Error + Send + Sync>,
    > {
        let decoder = Decoder::new(self.dictionary.clone()).streaming(vec![]);
        let (app_handler, app_channels) = TokioAppHandler::new();

        let mut connection =
            TokioFixConnection::new(self.config.clone(), self.backend.clone());

        // Start the connection in a background task
        let connection_handle = tokio::spawn(async move {
            connection
                .start_with_stream(stream, decoder, Some(app_handler))
                .await
        });

        Ok((app_channels, connection_handle))
    }
}

/// Simple FIX client for basic use cases.
///
/// This provides a minimal ~20 LOC interface for users to get started
/// with FIX communication using Tokio.
#[derive(Debug)]
pub struct SimpleTokioFixClient;

impl SimpleTokioFixClient {
    /// Creates a FIX client with minimal configuration.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use minifix::tokio::SimpleTokioFixClient;
    /// use tokio::net::TcpStream;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ///     let stream = TcpStream::connect("127.0.0.1:9878").await?;
    ///     let channels = SimpleTokioFixClient::connect(
    ///         stream,
    ///         "CLIENT",
    ///         "SERVER"
    ///     ).await?;
    ///
    ///     // Use channels.inbound_rx to receive messages
    ///     // Use channels.outbound_tx to send messages
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn connect(
        stream: TcpStream,
        sender_comp_id: &str,
        target_comp_id: &str,
    ) -> Result<TokioAppChannels, Box<dyn Error + Send + Sync>> {
        let config = TokioFixConfig::builder()
            .sender_comp_id(sender_comp_id)
            .target_comp_id(target_comp_id)
            .build();

        let mut client = TokioFixClient::new(config, Dictionary::fix44());
        client.start(stream).await
    }
}

/// Simple backend implementation for basic FIX session management.
#[derive(Debug, Clone)]
struct SimpleBackend {
    // Add fields as needed for session state
}

impl SimpleBackend {
    fn new() -> Self {
        Self {}
    }
}

impl Backend for SimpleBackend {
    type Error = SimpleBackendError;

    fn sender_comp_id(&self) -> &[u8] {
        b"SENDER"
    }

    fn target_comp_id(&self) -> &[u8] {
        b"TARGET"
    }

    fn on_inbound_app_message(
        &mut self,
        _message: Message<&[u8]>,
    ) -> Result<(), Self::Error> {
        // Basic implementation - just accept all messages
        Ok(())
    }

    fn on_outbound_message(
        &mut self,
        _message: &[u8],
    ) -> Result<(), Self::Error> {
        // Basic implementation - just accept all messages
        Ok(())
    }

    fn on_successful_handshake(&mut self) -> Result<(), Self::Error> {
        // Basic implementation
        Ok(())
    }

    fn on_resend_request(
        &mut self,
        _range: std::ops::Range<u64>,
    ) -> Result<(), Self::Error> {
        // Basic implementation
        Ok(())
    }

    fn fetch_messages(&mut self) -> Result<&[&[u8]], Self::Error> {
        // Return empty slice - no messages to fetch
        Ok(&[])
    }

    fn pending_message(&mut self) -> Option<&[u8]> {
        // No pending messages
        None
    }
}

/// Simple error type for the basic backend.
#[derive(Debug)]
pub struct SimpleBackendError;

// SAFETY: SimpleBackendError is a zero-sized type with no internal data,
// making it safe to send across threads and share between threads.
unsafe impl Send for SimpleBackendError {}
unsafe impl Sync for SimpleBackendError {}

impl fmt::Display for SimpleBackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Simple backend error")
    }
}

impl Error for SimpleBackendError {}

impl<'a> crate::FieldType<'a> for SimpleBackendError {
    type Error = &'static str;
    type SerializeSettings = ();

    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: crate::Buffer,
    {
        b"SimpleBackendError".serialize(buffer)
    }

    fn deserialize(_data: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(SimpleBackendError)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;

    #[test]
    fn client_creation() {
        let config = TokioFixConfig::builder()
            .sender_comp_id("TEST_SENDER")
            .target_comp_id("TEST_TARGET")
            .heartbeat_interval(Duration::from_secs(30))
            .build();

        let client = TokioFixClient::new(config, Dictionary::fix44());
        assert_eq!(client.config.sender_comp_id, "TEST_SENDER");
        assert_eq!(client.config.target_comp_id, "TEST_TARGET");
    }
}
