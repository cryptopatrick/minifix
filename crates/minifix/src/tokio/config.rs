use crate::session::{Configure, Environment};
use std::time::Duration;

/// Configuration for Tokio-based FIX connections.
///
/// This configuration provides a simplified interface for setting up
/// FIX connections in Tokio applications with sensible defaults.
#[derive(Debug, Clone)]
pub struct TokioFixConfig {
    pub sender_comp_id: String,
    pub target_comp_id: String,
    pub heartbeat_interval: Duration,
    pub environment: Environment,
    pub begin_string: String,
}

impl TokioFixConfig {
    /// Creates a new configuration builder.
    pub fn builder() -> TokioFixConfigBuilder {
        TokioFixConfigBuilder::new()
    }
}

impl Default for TokioFixConfig {
    fn default() -> Self {
        Self {
            sender_comp_id: "SENDER".to_string(),
            target_comp_id: "TARGET".to_string(),
            heartbeat_interval: Duration::from_secs(30),
            environment: Environment::Production { allow_test: false },
            begin_string: "FIX.4.4".to_string(),
        }
    }
}

impl Configure for TokioFixConfig {
    fn begin_string(&self) -> &[u8] {
        self.begin_string.as_bytes()
    }

    fn sender_comp_id(&self) -> &[u8] {
        self.sender_comp_id.as_bytes()
    }

    fn target_comp_id(&self) -> &[u8] {
        self.target_comp_id.as_bytes()
    }

    fn environment(&self) -> Environment {
        self.environment
    }

    fn heartbeat(&self) -> Duration {
        self.heartbeat_interval
    }
}

/// Builder for TokioFixConfig.
#[derive(Debug)]
pub struct TokioFixConfigBuilder {
    config: TokioFixConfig,
}

impl TokioFixConfigBuilder {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            config: TokioFixConfig::default(),
        }
    }

    /// Sets the sender component ID.
    pub fn sender_comp_id<S: Into<String>>(mut self, id: S) -> Self {
        self.config.sender_comp_id = id.into();
        self
    }

    /// Sets the target component ID.
    pub fn target_comp_id<S: Into<String>>(mut self, id: S) -> Self {
        self.config.target_comp_id = id.into();
        self
    }

    /// Sets the heartbeat interval.
    pub fn heartbeat_interval(mut self, interval: Duration) -> Self {
        self.config.heartbeat_interval = interval;
        self
    }

    /// Sets the FIX environment.
    pub fn environment(mut self, env: Environment) -> Self {
        self.config.environment = env;
        self
    }

    /// Sets the FIX begin string (protocol version).
    pub fn begin_string<S: Into<String>>(mut self, begin_string: S) -> Self {
        self.config.begin_string = begin_string.into();
        self
    }

    /// Builds the configuration.
    pub fn build(self) -> TokioFixConfig {
        self.config
    }
}

impl Default for TokioFixConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}