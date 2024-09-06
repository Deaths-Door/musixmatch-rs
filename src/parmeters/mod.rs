mod filter;
mod group;
mod other;
mod query;
mod sorting;

pub use filter::*;
pub use group::*;
pub use other::*;
pub use query::*;
pub use sorting::*;

#[derive(Default)]
pub(crate) struct Parameters<'a>(
    std::collections::HashMap<&'a str, api_request_utils::serde_json::Value>,
);

/// Represents an error that occurs when a value is out of range.
#[derive(Debug)]
pub struct OutOfRangeError<T> {
    pub(super) message: &'static str,
    pub(super) value: T,
}

impl<T> OutOfRangeError<T> {
    /// Creates a new `OutOfRangeError` with the specified value and message.
    ///
    /// # Arguments
    ///
    /// * `value`: The value that was found to be out of range.
    /// * `message`: The error message associated with the out-of-range condition.
    ///
    /// # Returns
    ///
    /// A new `OutOfRangeError` instance.
    pub const fn new(value: T, message: &'static str) -> Self {
        Self { message, value }
    }
}

impl<'a> std::ops::Deref for Parameters<'a> {
    type Target = std::collections::HashMap<&'a str, api_request_utils::serde_json::Value>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> std::ops::DerefMut for Parameters<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
