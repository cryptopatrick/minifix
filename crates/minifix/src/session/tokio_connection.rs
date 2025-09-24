use super::tokio_event_loop::{TokioLlEvent, TokioLlEventLoop};
use super::{Backend, Config, Configure};

#[derive(Debug, Clone)]
enum Response<'a> {
    None,
    ResetHeartbeat,
    TerminateTransport,
    Application(Message<'a, &'a [u8]>),
    OutboundBytes(&'a [u8]),
}
use crate::FieldMap;
use crate::tagvalue::Message;
use crate::tagvalue::{DecoderStreaming, Encoder};
use crate::{SetField, StreamingDecoder};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use uuid::Uuid;

const BEGIN_SEQ_NO: u32 = 7;
const BEGIN_STRING: u32 = 8;
const END_SEQ_NO: u32 = 16;
const MSG_SEQ_NUM: u32 = 34;
const MSG_TYPE: u32 = 35;
const SENDER_COMP_ID: u32 = 49;
const SENDING_TIME: u32 = 52;
const TARGET_COMP_ID: u32 = 56;
const TEXT: u32 = 58;
const ENCRYPT_METHOD: u32 = 98;
const TEST_REQ_ID: u32 = 112;

/// A Tokio-optimized FIX connection processor.
///
/// This connection type is specifically designed for Tokio runtimes and provides
/// native integration with Tokio's async I/O primitives, channels, and timers.
#[derive(Debug)]
pub struct TokioFixConnection<B, C = Config> {
    uuid: Uuid,
    config: C,
    backend: B,
    encoder: Encoder,
    buffer: Vec<u8>,
    msg_seq_num_inbound: MsgSeqNumCounter,
    msg_seq_num_outbound: MsgSeqNumCounter,
}

/// Channel-based application message handler for Tokio integration.
#[derive(Debug)]
pub struct TokioAppHandler {
    inbound_tx: mpsc::UnboundedSender<Message<'static, Vec<u8>>>,
    outbound_rx: mpsc::UnboundedReceiver<Vec<u8>>,
}

impl TokioAppHandler {
    /// Creates a new channel-based application handler.
    pub fn new() -> (Self, TokioAppChannels) {
        let (inbound_tx, inbound_rx) = mpsc::unbounded_channel();
        let (outbound_tx, outbound_rx) = mpsc::unbounded_channel();
        
        let handler = Self {
            inbound_tx,
            outbound_rx,
        };
        
        let channels = TokioAppChannels {
            inbound_rx,
            outbound_tx,
        };
        
        (handler, channels)
    }

    /// Sends an inbound application message to the application handler.
    pub fn send_inbound(&self, message: Message<'static, Vec<u8>>) -> Result<(), mpsc::error::SendError<Message<'static, Vec<u8>>>> {
        self.inbound_tx.send(message)
    }

    /// Tries to receive an outbound message from the application.
    pub fn try_recv_outbound(&mut self) -> Result<Vec<u8>, mpsc::error::TryRecvError> {
        self.outbound_rx.try_recv()
    }
}

/// Application-side channels for handling FIX messages.
#[derive(Debug)]
pub struct TokioAppChannels {
    pub inbound_rx: mpsc::UnboundedReceiver<Message<'static, Vec<u8>>>,
    pub outbound_tx: mpsc::UnboundedSender<Vec<u8>>,
}

impl<C, B> TokioFixConnection<B, C>
where
    C: Configure,
    B: Backend,
{
    /// Creates a new Tokio FIX connection.
    pub fn new(config: C, backend: B) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            config,
            backend,
            encoder: Encoder::new(),
            buffer: Vec::new(),
            msg_seq_num_inbound: MsgSeqNumCounter::new(),
            msg_seq_num_outbound: MsgSeqNumCounter::new(),
        }
    }

    /// Starts the FIX connection using a Tokio TcpStream.
    pub async fn start_with_stream(
        &mut self, 
        stream: TcpStream,
        mut decoder: DecoderStreaming<Vec<u8>>,
        app_handler: Option<TokioAppHandler>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let (mut reader, mut writer) = stream.into_split();
        
        // Establish the FIX connection
        self.establish_connection(&mut reader, &mut writer, &mut decoder).await?;
        
        // Run the main event loop
        self.tokio_event_loop(reader, writer, decoder, app_handler).await
    }

    async fn establish_connection(
        &mut self,
        reader: &mut tokio::net::tcp::OwnedReadHalf,
        writer: &mut tokio::net::tcp::OwnedWriteHalf,
        decoder: &mut DecoderStreaming<Vec<u8>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Send logon message
        let logon = self.create_logon_message();
        writer.write_all(&logon).await?;
        self.backend.on_outbound_message(&logon).ok();

        // Wait for logon response
        loop {
            let buffer = decoder.fillable();
            reader.read_exact(buffer).await?;
            
            if let Ok(Some(())) = decoder.try_parse() {
                let logon_response = decoder.message();
                self.on_logon(logon_response);
                self.backend.on_inbound_message(logon_response, true).ok();
                decoder.clear();
                self.msg_seq_num_inbound.next();
                self.backend.on_successful_handshake().ok();
                break;
            }
        }
        
        Ok(())
    }

    async fn tokio_event_loop(
        &mut self,
        reader: tokio::net::tcp::OwnedReadHalf,
        mut writer: tokio::net::tcp::OwnedWriteHalf,
        decoder: DecoderStreaming<Vec<u8>>,
        mut app_handler: Option<TokioAppHandler>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut event_loop = TokioLlEventLoop::new(decoder, reader, self.heartbeat());
        
        loop {
            tokio::select! {
                // Handle FIX protocol events
                event = event_loop.next_event() => {
                    match event {
                        Some(TokioLlEvent::Message(msg)) => {
                            let response = self.on_inbound_message(msg);
                            match response {
                                Response::OutboundBytes(bytes) => {
                                    writer.write_all(bytes).await?;
                                    // Backend callback handled in on_inbound_message
                                }
                                Response::ResetHeartbeat => {
                                    event_loop.ping_heartbeat();
                                }
                                Response::Application(app_msg) => {
                                    if let Some(ref _handler) = app_handler {
                                        // Create owned message safely by copying the data
                                        let _owned_data = app_msg.as_bytes().to_vec();
                                        // TODO: Replace with proper owned Message construction
                                        // For now, we cannot safely convert without proper reconstruction
                                        // handler.send_inbound(owned_msg).ok();
                                        // Temporarily disabled to avoid unsafe code
                                        eprintln!("Message handling temporarily disabled due to unsafe code removal");
                                    }
                                }
                                _ => {}
                            }
                        }
                        Some(TokioLlEvent::BadMessage(_err)) => {
                            // Handle bad message
                        }
                        Some(TokioLlEvent::IoError(_)) => {
                            return Err("I/O error in FIX connection".into());
                        }
                        Some(TokioLlEvent::Heartbeat) => {
                            let _ = self.backend.on_heartbeat_is_due();
                            let heartbeat = self.create_heartbeat();
                            writer.write_all(&heartbeat).await?;
                            // Backend callback for outbound message not needed here
                        }
                        Some(TokioLlEvent::TestRequest) => {
                            // Handle test request
                        }
                        Some(TokioLlEvent::Logout) => {
                            return Ok(());
                        }
                        None => {
                            return Ok(());
                        }
                    }
                }
                
                // Handle outbound application messages
                outbound = async {
                    if let Some(ref mut handler) = app_handler {
                        handler.try_recv_outbound().ok()
                    } else {
                        None
                    }
                } => {
                    if let Some(msg_bytes) = outbound {
                        writer.write_all(&msg_bytes).await?;
                        self.on_outbound_message(&msg_bytes).ok();
                    }
                }
            }
        }
    }

    fn create_logon_message(&mut self) -> Vec<u8> {
        let begin_string = self.config.begin_string();
        let sender_comp_id = self.config.sender_comp_id();
        let target_comp_id = self.config.target_comp_id();
        let heartbeat = self.config.heartbeat().as_secs();
        let msg_seq_num = self.msg_seq_num_outbound.next();
        
        self.buffer.clear();
        let mut msg = self.encoder.start_message(begin_string, &mut self.buffer, b"A");
        msg.set(49, sender_comp_id);
        msg.set(56, target_comp_id);
        let now = chrono::Utc::now().format("%Y%m%d-%H:%M:%S%.3f").to_string();
        msg.set(52, now.as_str());
        msg.set(34, msg_seq_num);
        msg.set(98, 0);
        msg.set(108, heartbeat as u32);
        
        let (_, _) = msg.done();
        self.buffer.clone()
    }

    fn on_heartbeat_is_due(&mut self) -> &[u8] {
        let begin_string = self.config.begin_string();
        let msg_seq_num = self.msg_seq_num_outbound.next();
        let sender_comp_id = self.config.sender_comp_id();
        let target_comp_id = self.config.target_comp_id();
        
        self.buffer.clear();
        let mut msg = self.encoder.start_message(begin_string, &mut self.buffer, b"0");
        msg.set(49, sender_comp_id);
        msg.set(56, target_comp_id);
        msg.set(34, msg_seq_num);
        let now = chrono::Utc::now().format("%Y%m%d-%H:%M:%S%.3f").to_string();
        msg.set(52, now.as_str());
        
        let (_, _) = msg.done();
        &self.buffer
    }

    fn on_inbound_message<'a>(&'a mut self, msg: Message<'a, &'a [u8]>) -> Response<'a> {
        // Extract message type and dispatch
        if let Ok(msg_type) = msg.get::<&[u8]>(35) {
            match msg_type {
                b"A" => {
                    self.on_logon(msg);
                    Response::None
                }
                b"0" => {
                    self.on_heartbeat(msg);
                    Response::ResetHeartbeat
                }
                b"1" => {
                    let response = self.on_test_request(msg);
                    Response::OutboundBytes(response)
                }
                b"5" => {
                    Response::TerminateTransport
                }
                _ => {
                    Response::Application(msg)
                }
            }
        } else {
            Response::Application(msg)
        }
    }

    fn on_logon(&mut self, _msg: Message<&[u8]>) {
        // Handle logon message
    }

    fn on_heartbeat(&mut self, _msg: Message<&[u8]>) {
        // Handle heartbeat message
    }

    fn on_test_request(&mut self, msg: Message<&[u8]>) -> &[u8] {
        let test_req_id = msg.get::<&[u8]>(112).unwrap_or(b"");
        let begin_string = self.config.begin_string();
        let msg_seq_num = self.msg_seq_num_outbound.next();
        let sender_comp_id = self.config.sender_comp_id();
        let target_comp_id = self.config.target_comp_id();
        
        self.buffer.clear();
        let mut response = self.encoder.start_message(begin_string, &mut self.buffer, b"0");
        response.set(49, sender_comp_id);
        response.set(56, target_comp_id);
        response.set(34, msg_seq_num);
        let now = chrono::Utc::now().format("%Y%m%d-%H:%M:%S%.3f").to_string();
        response.set(52, now.as_str());
        response.set(112, test_req_id);
        
        let (_, _) = response.done();
        &self.buffer
    }

    fn on_outbound_message(&mut self, _message: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn set_sender_and_target(&self, msg: &mut impl crate::SetField<u32>) {
        msg.set(49, self.config.sender_comp_id());
        msg.set(56, self.config.target_comp_id());
    }

    fn set_sending_time(&self, msg: &mut impl crate::SetField<u32>) {
        // Use a timestamp format compatible with FIX
        let now = chrono::Utc::now().format("%Y%m%d-%H:%M:%S%.3f").to_string();
        msg.set(52, now.as_str());
    }

    fn heartbeat(&self) -> Duration {
        self.config.heartbeat()
    }

    fn create_heartbeat(&mut self) -> Vec<u8> {
        let begin_string = self.config.begin_string();
        let msg_seq_num = self.msg_seq_num_outbound.next();
        let sender_comp_id = self.config.sender_comp_id();
        let target_comp_id = self.config.target_comp_id();
        
        self.buffer.clear();
        let mut msg = self.encoder.start_message(begin_string, &mut self.buffer, b"0");
        msg.set(49, sender_comp_id);
        msg.set(56, target_comp_id);
        msg.set(34, msg_seq_num);
        let now = chrono::Utc::now().format("%Y%m%d-%H:%M:%S%.3f").to_string();
        msg.set(52, now.as_str());
        
        let (_, _) = msg.done();
        self.buffer.clone()
    }

    fn message_to_owned(&self, msg: Message<&[u8]>) -> Message<'static, Vec<u8>> {
        // Convert message to owned data by copying the bytes
        let _owned_data = msg.as_bytes().to_vec();
        // Note: This is still a simplified implementation. A proper solution would
        // need to reconstruct the message with the owned data and proper lifetime management.
        // For now, we'll avoid the unsafe transmute and create a new message structure.
        todo!("Implement proper message ownership conversion without unsafe transmute")
    }
}

/// Message sequence number counter for FIX sessions.
#[derive(Debug)]
pub struct MsgSeqNumCounter(u64);

impl MsgSeqNumCounter {
    pub const fn new() -> Self {
        Self(1)
    }

    pub fn next(&mut self) -> u64 {
        let current = self.0;
        self.0 += 1;
        current
    }

    pub fn expected(&self) -> u64 {
        self.0
    }
}

impl Default for MsgSeqNumCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tagvalue::Decoder;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn tokio_app_handler_channels() {
        let (handler, mut channels) = TokioAppHandler::new();
        
        // Test sending a message through the handler would work
        // (This is a basic test to ensure the channel setup works)
        assert!(channels.outbound_tx.send(vec![1, 2, 3]).is_ok());
    }
}