//! In-memory storage for registrant.

use super::*;

use std::collections::BTreeMap;
use std::sync::Mutex;

use uuid::Uuid;

pub type Map<K, V> = Mutex<BTreeMap<K, V>>;

/// Integrated storage(in-memory) for registrant.
#[derive(Default)]
pub struct IntegratedStorage {
    /// the registry sub-storage
    registry: Map<Uuid, String>,
}

impl IntegratedStorage {
    fn read_impl<'k>(&self, key: &'k str) -> Result<String> {
        // fixme 30/11/2020: do not get a lock unless needed.
        // fixme 30/11/2020: this is a kinda clunky implementation.
        let registry = self.registry.lock().unwrap();

        // reject any read ops that do not read from under the registry prefix.
        if !key.starts_with("registry") {
            return Err(StorageError::KeyNotFound { key: key.into() });
        }

        let key_split: Vec<&str> = key.split("/").collect();
        // reject any read ops that try to read more than registry/<uuid>.
        if key_split.len() > 2 {
            return Err(StorageError::KeyNotFound { key: key.into() });
        }
        // if the entire registry is queried, return a comma-separated list of registry entries.
        if key_split.len() == 1 {
            return Ok(registry
                .keys()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(","));
        }
        // attempt to convert key segment 1 into an uuid.
        let key_uuid = match Uuid::parse_str(key_split[1]) {
            Ok(value) => value,
            Err(e) => {
                return Err(StorageError::MalformedKey {
                    key: key.into(),
                    reason: format!("failed to convert key segment 1 into uuid: {}", e),
                })
            }
        };
        // attempt to find the given value for the uuid.
        match registry.get(&key_uuid) {
            Some(value) => Ok(value.clone()),
            None => Err(StorageError::KeyNotFound { key: key.into() }),
        }
    }

    fn put_impl<'k>(&self, key: &'k str, value: String) -> Result<()> {
        if !key.starts_with("registry") {
            return Err(StorageError::KeyNotFound { key: key.into() });
        }
        let key_split: Vec<&str> = key.split("/").collect();
        // reject any read ops that try to read more than registry/<uuid>.
        if key_split.len() != 2 {
            return Err(StorageError::KeyNotFound { key: key.into() });
        }
        // attempt to convert key segment 1 into an uuid.
        let key_uuid = match Uuid::parse_str(key_split[1]) {
            Ok(value) => value,
            Err(e) => {
                return Err(StorageError::MalformedKey {
                    key: key.into(),
                    reason: format!("failed to convert key segment 1 into uuid: {}", e),
                })
            }
        };
        // insert the value
        // fixme 30/11/2020: do not get a lock unless needed.
        // fixme 30/11/2020: this is a kinda clunky implementation.
        let mut registry = self.registry.lock().unwrap();
        registry.insert(key_uuid, value);
        Ok(())
    }
}

#[async_trait::async_trait]
impl Storage for IntegratedStorage {
    type Key = String;
    type Value = String;

    /// Read a key-value pair from the store.
    async fn read<K, V>(&self, key: K) -> Result<V>
    where
        K: Into<Self::Key> + Send,
        V: From<Self::Value>,
    {
        Ok(self.read_impl(key.into().as_str())?.into())
    }

    /// Put a key-value pair in the store. This will create the key if non-existent.
    async fn put<K, V>(&self, key: K, value: V) -> Result<()>
    where
        K: Into<Self::Key> + Send,
        V: Into<Self::Value> + Send,
    {
        self.put_impl(key.into().as_str(), value.into())
    }

    /// Delete a key from the store.
    async fn delete<K>(&self, key: K) -> Result<()>
    where
        K: Into<Self::Key> + Send,
    {
        // todo 30/11/2020: this has been left unimplemented as the MVP does not have a delete
        // feature.
        todo!();
    }

    /// List keys under a prefix.
    async fn list<'l, K, L>(&self, key: K) -> Result<L>
    where
        K: Into<Self::Key> + Send,
        L: From<&'l [Self::Key]>,
        Self::Key: 'l,
    {
        // todo 30/11/2020: this has been left unimplemented as the MVP does not have a list
        // feature.
        todo!();
    }
}
