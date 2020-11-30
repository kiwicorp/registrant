//! Registrant sandhog registry module.

use crate::storage::Storage;

use uuid::Uuid;

/// A trait describing the behaviour of a registry.
#[async_trait::async_trait]
pub trait Registry {
    /// Register a sandhog in the registry.
    async fn register<'f, U, S>(&self, uuid: U, hostname: S) -> anyhow::Result<()>
    where
        U: Into<Uuid> + Send,
        S: Into<&'f str> + Send;

    /// Lookup a sandhog in the registry.
    async fn lookup() {
        unimplemented!();
    }

    /// Deregister a sandhog from the registry.
    async fn deregister() {
        unimplemented!();
    }
}

/// Registry implementation.
#[derive(Default)]
pub struct RegistryImpl {
    storage: crate::storage::IntegratedStorage,
}

impl RegistryImpl {
    async fn register_impl(&self, uuid: Uuid, hostname: &str) -> anyhow::Result<()> {
        self.storage
            .put(format!("registry/{}", uuid), hostname)
            .await?;
        println!("Registered sandhog with uuid {} and hostname {}", uuid, hostname);
        Ok(())
    }
}

#[async_trait::async_trait]
impl Registry for RegistryImpl {
    async fn register<'f, U, S>(&self, uuid: U, hostname: S) -> anyhow::Result<()>
    where
        U: Into<Uuid> + Send,
        S: Into<&'f str> + Send,
    {
        self.register_impl(uuid.into(), hostname.into()).await
    }
}
