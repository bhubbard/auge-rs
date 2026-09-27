pub mod cleaner;
pub mod cli;
pub mod engine;
pub mod error;
pub mod formatter;
pub mod input;
pub mod types;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
