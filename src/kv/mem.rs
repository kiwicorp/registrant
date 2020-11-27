//! Module for an in-memory KVStore.

use super::KVStore;
use super::KVStoreExt;
use super::Result;

use std::collections::BTreeMap;

use async_trait::async_trait;

use serde_json::Value;

/// An in-memory kv store.
pub struct MemKVStore<'k> {
    inner: BTreeMap<&'k str, Value>,
}

impl<'k> MemKVStore<'k> {
    /// Create a new MemKVStore.
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    fn read_impl<'kf, 'v>(&self, key: &'kf str) -> Result<&'v [u8]> {
        Ok(&[])
    }

    fn put_impl<'kf>(&self, key: &'kf str, value: &[u8]) -> Result<()> {
        Ok(())
    }

    fn delete_impl<'kf>(&self, key: &'kf str) -> Result<()> {
        Ok(())
    }

    fn list_impl<'kf, 'l>(&self, key: &'kf str) -> Result<&'l [&'l str]> {
        Ok(&[])
    }
}

#[async_trait]
impl<'k> KVStore for MemKVStore<'k> {
    /// Read a key-value pair from the store.
    async fn read<'f, 'v, K, V>(&self, key: K) -> Result<V>
    where
        K: Into<&'f str> + Send,
        V: From<&'v [u8]>,
    {
        Ok(self.read_impl(key.into())?.into())
    }

    /// Put a key-value pair in the store. This will create the key if non-existent.
    async fn put<'f, K, V>(&self, key: K, value: V) -> Result<()>
    where
        K: Into<&'f str> + Send,
        V: Into<&'f [u8]> + Send,
    {
        Ok(self.put_impl(key.into(), value.into())?)
    }

    /// Delete a key from the store.
    async fn delete<'f, K>(&self, key: K) -> Result<()>
    where
        K: Into<&'f str> + Send,
    {
        Ok(self.delete_impl(key.into())?)
    }

    /// List keys under a prefix.
    async fn list<'f, 'l, K, L>(&self, key: K) -> Result<L>
    where
        K: Into<&'f str> + Send,
        L: From<&'l [&'l str]>,
    {
        Ok(self.list_impl(key.into())?.into())
    }
}
