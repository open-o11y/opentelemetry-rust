//! # OpenTelemetry Tracer Provider Interface
//!
//! ### Obtaining a Tracer
//!
//! New `Tracer` instances can be created via a `TracerProvider` and its `get_tracer`
//! method. This method expects an Into<String> argument:
//!
//! - `name` (required): This name must identify the instrumentation library (also
//!   referred to as integration, e.g. `io.opentelemetry.contrib.mongodb`) and *not*
//!   the instrumented library.
//!   In case an invalid name (empty string) is specified, a working
//!   default Tracer implementation as a fallback is returned rather than returning
//!   None or throwing an exception.
//!   A library, implementing the OpenTelemetry API *may* also ignore this name and
//!   return a default instance for all calls, if it does not support "named"
//!   functionality (e.g. an implementation which is not even observability-related).
//!   A TracerProvider could also return a no-op Tracer here if application owners configure
//!   the SDK to suppress telemetry produced by this library.
//!
//! Implementations might require the user to specify configuration properties at
//! `TracerProvider` creation time, or rely on external configurations.
use crate::trace::{TraceResult, Tracer};
use std::fmt;

/// A struct which contains parameters to pass to `TracerProvider`'s `get_tracer` method
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct TracerConfig {
    /// The name of the instrumentation library, e.g. `io.opentelemetry.contrib.mongodb`
    pub name: &'static str,
    /// The version of the instrumentation library, e.g. 1.0.0
    pub version: Option<&'static str>,
}

impl TracerConfig {
    /// Specify the name of the `Tracer`
    pub fn with_name(mut self, name: &'static str) -> Self {
        self.name = name;
        self
    }
    /// Specify the version of the `Tracer`
    pub fn with_version(mut self, version: &'static str) -> Self {
        self.version = Some(version);
        self
    }
}
/// Default `Tracer` configuration
pub fn tracer_config() -> TracerConfig {
    TracerConfig::default()
}

/// An interface to create `Tracer` instances.
pub trait TracerProvider: fmt::Debug + 'static {
    /// The `Tracer` type that this `TracerProvider` will return.
    type Tracer: Tracer;

    /// Creates a named tracer instance of `Self::Tracer`.
    /// If the name is an empty string then provider uses default name.
    fn get_tracer(&self, config: &TracerConfig) -> Self::Tracer;

    /// Force flush all remaining spans in span processors and return results.
    fn force_flush(&self) -> Vec<TraceResult<()>>;
}
