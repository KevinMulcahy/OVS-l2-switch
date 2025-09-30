//! Common error types for dataplane.

#[derive(Debug)]
pub enum AppError {
    /// Returned when a frame is too short to parse.
    InvalidFrameLength,

    /// Placeholder for unimplemented or unexpected cases.
    NotImplemented,
}
