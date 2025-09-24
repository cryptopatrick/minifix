//! FIX message processing between counterparties.
//!
//! ![](https://www.fixtrading.org/wp-content/uploads/2020/06/fixsessionlayerconceptualsimple.png)
//!
//! The above is a conceptual view of the FIX Session layer, complete with its
//! state machine and transitions between initiator and acceptor.

/// Session backend implementations and application interfaces.
pub mod backends;
mod config;
//mod connection; FIXME
mod environment;
mod errs;
mod event_loop;
mod heartbeat_rule;
mod resend_request_range;
mod seq_numbers;

/// Tokio-based FIX connection implementation.
#[cfg(feature = "utils-tokio")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "utils-tokio")))]
pub mod tokio_connection;

/// Tokio-based FIX event loop implementation.
#[cfg(feature = "utils-tokio")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "utils-tokio")))]
pub mod tokio_event_loop;

use crate::tagvalue::Message;
use crate::{FieldType, SetField};
pub use config::{Config, Configure};
// pub use connection::*; FIXME
pub use environment::Environment;
pub use event_loop::*;
pub use heartbeat_rule::HeartbeatRule;
pub use resend_request_range::ResendRequestRange;
pub use seq_numbers::{SeqNumberError, SeqNumbers};
use std::ops::Range;

/// The owner of a [`FixConnection`]. It can react to events, store incoming
/// messages, send messages, etc..
pub trait Backend: Clone {
    /// The type of errors that can arise during a [`FixConnection`].
    type Error: for<'a> FieldType<'a>;

    /// Returns the sender company ID for this session.
    fn sender_comp_id(&self) -> &[u8];
    /// Returns the target company ID for this session.
    fn target_comp_id(&self) -> &[u8];

    /// Returns the message encoding type, if specified.
    fn message_encoding(&self) -> Option<&[u8]> {
        None
    }

    /// Sets the sender and target company IDs on a message.
    fn set_sender_and_target(&self, msg: &mut impl SetField<u32>) {
        msg.set(49, self.sender_comp_id());
        msg.set(56, self.target_comp_id());
    }

    /// Returns the FIX session environment (Production/Test).
    fn environment(&self) -> Environment {
        Environment::Production { allow_test: false }
    }

    /// Called when a heartbeat message should be sent.
    fn on_heartbeat_is_due(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Callback for processing incoming FIX application messages.
    fn on_inbound_app_message(
        &mut self,
        message: Message<&[u8]>,
    ) -> Result<(), Self::Error>;

    /// Callback for post-processing outbound FIX messages.
    fn on_outbound_message(
        &mut self,
        message: &[u8],
    ) -> Result<(), Self::Error>;

    /// Callback for processing incoming FIX messages.
    fn on_inbound_message(
        &mut self,
        message: Message<&[u8]>,
        is_app: bool,
    ) -> Result<(), Self::Error> {
        if is_app { self.on_inbound_app_message(message) } else { Ok(()) }
    }

    /// Callback for processing `ResendRequest` messages.
    fn on_resend_request(
        &mut self,
        range: Range<u64>,
    ) -> Result<(), Self::Error>;

    /// Callback for additional logic to execute after a valid [`FixConnection`]
    /// is established with the counterparty.
    fn on_successful_handshake(&mut self) -> Result<(), Self::Error>;

    /// Fetches pending messages from the transport layer.
    fn fetch_messages(&mut self) -> Result<&[&[u8]], Self::Error>;

    /// Returns the next pending message, if any.
    fn pending_message(&mut self) -> Option<&[u8]>;
}

/// FIX session state tracking.
///
/// Tracks sequence numbers and session information for FIX protocol sessions.
#[derive(Debug, Clone)]
pub struct State {
    #[allow(dead_code)] // TODO: Implement sequence number tracking
    next_inbound: u64,
    #[allow(dead_code)] // TODO: Implement sequence number tracking
    next_outbound: u64,
}

/// Message sequence number counter for FIX sessions.
///
/// Handles incrementing and tracking of FIX message sequence numbers.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MsgSeqNumCounter(u64);

impl MsgSeqNumCounter {
    /// Starting sequence number (0).
    pub const START: Self = Self(0);

    /// Get the next sequence number and increment the counter.
    pub fn next(&mut self) -> u64 {
        self.0 += 1;
        self.0
    }

    /// Get the expected next sequence number without incrementing.
    pub fn expected(&self) -> u64 {
        self.0 + 1
    }
}

impl Iterator for MsgSeqNumCounter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        Some(MsgSeqNumCounter::next(self))
    }
}

// FIXME
#[derive(Debug)]
/// FIX protocol connection handler.
///
/// Manages the connection state and message flow for FIX sessions.
pub struct FixConnection;
