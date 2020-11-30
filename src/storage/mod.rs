//! Storage-related module.

mod mem;

pub use mem::IntegratedStorage;

use thiserror::Error;

/// Results produced by the storage component.
pub type Result<T> = std::result::Result<T, StorageError>;

/// Errors produced by the storage component.
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("key not found: {key}")]
    KeyNotFound { key: String },
    #[error("malformed key: {key}, reason: {reason}")]
    MalformedKey { key: String, reason: String },
}

/// Storage defines common behaviour expected from storage backends.
#[async_trait::async_trait]
pub trait Storage {
    type Key;
    type Value;

    /// Read a key-value pair from the store.
    async fn read<K, V>(&self, key: K) -> Result<V>
    where
        K: Into<Self::Key> + Send,
        V: From<Self::Value>;

    /// Put a key-value pair in the store. This will create the key if non-existent.
    async fn put<K, V>(&self, key: K, value: V) -> Result<()>
    where
        K: Into<Self::Key> + Send,
        V: Into<Self::Value> + Send;

    /// Delete a key from the store.
    async fn delete<K>(&self, key: K) -> Result<()>
    where
        K: Into<Self::Key> + Send;

    /// List keys under a prefix.
    async fn list<'l, K, L>(&self, key: K) -> Result<L>
    where
        K: Into<Self::Key> + Send,
        L: From<&'l [Self::Key]>,
        Self::Key: 'l,
    {
        // how this should be supposed to work is not currently thought out.
        todo!();
    }
}
