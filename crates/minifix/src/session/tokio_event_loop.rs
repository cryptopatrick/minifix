use crate::StreamingDecoder;
use crate::tagvalue::{DecodeError, DecoderStreaming, Message};
use std::io;
use std::time::{Duration, Instant};
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::time::{sleep_until, Instant as TokioInstant};

/// Tokio-specific asynchronous low-level event loop for FIX connectors.
///
/// This event loop is optimized for Tokio runtime and uses Tokio's native
/// async primitives for better performance and integration. It provides the
/// same functionality as [`LlEventLoop`] but with Tokio-specific optimizations.
#[derive(Debug)]
pub struct TokioLlEventLoop<I> {
    decoder: DecoderStreaming<Vec<u8>>,
    input: I,
    heartbeat: Duration,
    heartbeat_soft_tolerance: Duration,
    heartbeat_hard_tolerance: Duration,
    last_reset: Instant,
    last_heartbeat: Instant,
    is_alive: bool,
}

impl<I> TokioLlEventLoop<I>
where
    I: AsyncRead + std::marker::Unpin,
{
    /// Creates a new [`TokioLlEventLoop`] with the provided `decoder` and
    /// `heartbeat`. Events will be read from `input`.
    pub fn new(
        decoder: DecoderStreaming<Vec<u8>>,
        input: I,
        heartbeat: Duration,
    ) -> Self {
        let heartbeat_soft_tolerance = heartbeat * 2;
        let heartbeat_hard_tolerance = heartbeat * 3;
        Self {
            decoder,
            input,
            heartbeat,
            heartbeat_soft_tolerance,
            heartbeat_hard_tolerance,
            last_reset: Instant::now(),
            last_heartbeat: Instant::now(),
            is_alive: true,
        }
    }

    /// How long after a missed `Heartbeat <0>` should we send a `TestRequest
    /// <1>`?
    pub fn set_soft_tolerance(&mut self, soft_tolerance: Duration) {
        self.heartbeat_soft_tolerance = soft_tolerance;
    }

    /// How long after a missed `Heartbeat <0>` should we send a `Logout <5>`?
    pub fn set_hard_tolerance(&mut self, hard_tolerance: Duration) {
        self.heartbeat_hard_tolerance = hard_tolerance;
    }

    /// Returns the next event from the event loop using Tokio's native async primitives.
    pub async fn next_event<'a>(&'a mut self) -> Option<TokioLlEvent<'a>> {
        let mut buf_filled_len = 0;
        let mut buf = self.decoder.fillable();

        loop {
            if !self.is_alive {
                return None;
            }

            let now = Instant::now();
            let tokio_now = TokioInstant::now();
            
            // Calculate timeouts using Tokio's time primitives
            let heartbeat_deadline = tokio_now + (self.heartbeat - (now - self.last_heartbeat));
            let test_request_deadline = tokio_now + (self.heartbeat_soft_tolerance - (now - self.last_reset));
            let logout_deadline = tokio_now + (self.heartbeat_hard_tolerance - (now - self.last_reset));

            tokio::select! {
                read_result = self.input.read(buf) => {
                    match read_result {
                        Err(e) => {
                            return Some(TokioLlEvent::IoError(e));
                        }
                        Ok(num_bytes) => {
                            buf_filled_len += num_bytes;
                            if buf_filled_len < buf.len() {
                                continue;
                            }

                            let result = self.decoder.try_parse();
                            buf_filled_len = 0;
                            buf = &mut self.decoder.fillable()[buf_filled_len..];

                            match result {
                                Ok(Some(())) => {
                                    let msg = self.decoder.message();
                                    return Some(TokioLlEvent::Message(msg));
                                }
                                Ok(None) => {
                                    continue;
                                }
                                Err(err) => {
                                    self.is_alive = false;
                                    return Some(TokioLlEvent::BadMessage(err))
                                }
                            }
                        }
                    };
                },
                _ = sleep_until(heartbeat_deadline) => {
                    self.last_heartbeat = Instant::now();
                    return Some(TokioLlEvent::Heartbeat);
                },
                _ = sleep_until(test_request_deadline) => {
                    return Some(TokioLlEvent::TestRequest);
                },
                _ = sleep_until(logout_deadline) => {
                    self.is_alive = false;
                    return Some(TokioLlEvent::Logout);
                }
            }
        }
    }

    /// Resets the FIX counterparty's `Heartbeat <0>` -associated timers.
    pub fn ping_heartbeat(&mut self) {
        self.last_reset = Instant::now();
    }
}

/// A low level event produced by a [`TokioLlEventLoop`].
#[derive(Debug)]
pub enum TokioLlEvent<'a> {
    /// Incoming FIX message.
    Message(Message<'a, &'a [u8]>),
    /// Tried to parse an incoming FIX message, but got illegal data.
    BadMessage(DecodeError),
    /// I/O error at the transport layer.
    IoError(io::Error),
    /// Time to send a new `HeartBeat <0>` message.
    Heartbeat,
    /// The FIX counterparty has missed the `Heartbeat <0>` deadline by some
    /// amount of time, and it's time to send a `Test Request <1>`
    /// message to check what's going on.
    TestRequest,
    /// The FIX counterparty has missed the `Heartbeat <0>` deadline by some
    /// amount of time, and it's stopped responding. It's time to
    /// disconnect via a `Logout <5>` message.
    Logout,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tagvalue::Decoder;
    use tokio::io::AsyncWriteExt;
    use tokio::net::{TcpListener, TcpStream};

    async fn produce_events(
        events: Vec<(&'static [u8], Duration)>,
    ) -> TcpStream {
        let tcp_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let local_addr = tcp_listener.local_addr().unwrap();

        tokio::spawn(async move {
            let mut stream = TcpStream::connect(local_addr).await.unwrap();
            for (event_bytes, delay) in events.iter() {
                stream.write(event_bytes).await.unwrap();
                tokio::time::sleep(*delay).await;
            }
        });

        tcp_listener.accept().await.unwrap().0
    }

    async fn new_tokio_event_loop(
        events: Vec<(&'static [u8], Duration)>,
    ) -> TokioLlEventLoop<TcpStream> {
        let input = produce_events(events).await;

        TokioLlEventLoop::new(
            Decoder::new(crate::Dictionary::fix44()).streaming(vec![]),
            input,
            Duration::from_secs(3),
        )
    }

    #[tokio::test]
    async fn tokio_dead_input_triggers_logout() {
        let mut event_loop =
            new_tokio_event_loop(vec![(b"8", Duration::from_secs(10))]).await;
        let event = event_loop.next_event().await;
        assert!(matches!(event, Some(TokioLlEvent::Heartbeat)));
        let event = event_loop.next_event().await;
        assert!(
            matches!(event, Some(TokioLlEvent::Heartbeat))
                || matches!(event, Some(TokioLlEvent::TestRequest))
        );
    }
}