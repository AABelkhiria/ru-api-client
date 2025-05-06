// Declare modules within this crate
pub mod client;
pub mod errors;

// Re-export the main components for users of this crate
pub use client::ApiClient;
pub use errors::ApiClientError;