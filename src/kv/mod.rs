//! Module for defining the KV store required by registrant.

mod mem;

use async_trait::async_trait;

use serde::Deserialize;
use serde::Serialize;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum KVStoreError<'s> {
    #[error("key not found: {0}")]
    KeyNotFound(&'s str),
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// A key-value storage interface.
#[async_trait]
pub trait KVStore {
    /// Read a key-value pair from the store.
    async fn read<'f, 'v, K, V>(&self, key: K) -> Result<V>
    where
        K: Into<&'f str> + Send,
        V: From<&'v [u8]>;

    /// Put a key-value pair in the store. This will create the key if non-existent.
    async fn put<'f, K, V>(&self, key: K, value: V) -> Result<()>
    where
        K: Into<&'f str> + Send,
        V: Into<&'f [u8]> + Send;

    /// Delete a key from the store.
    async fn delete<'f, K>(&self, key: K) -> Result<()>
    where
        K: Into<&'f str> + Send;

    /// List keys under a prefix.
    async fn list<'f, 'l, K, L>(&self, key: K) -> Result<L>
    where
        K: Into<&'f str> + Send,
        L: From<&'l [&'l str]>;
}

/// Key-value storage interface extensions.
#[async_trait(?Send)]
pub trait KVStoreExt {
    /// Read a key-value pair from the store.
    async fn read_value<'f, 'v, K, V>(&self, key: K) -> Result<V>
    where
        Self: KVStore,
        K: Into<&'f str>,
        V: Deserialize<'v>,
    {
        unimplemented!();
    }

    /// Put a key-value pair in the store. This will create the key if non-existent.
    async fn put_value<'f, K, V>(&self, key: K, value: V) -> Result<()>
    where
        Self: KVStore,
        K: Into<&'f str>,
        V: Serialize,
    {
        Ok(())
    }
}
