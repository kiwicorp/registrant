//! Registrant sandhog registry module.

use uuid::Uuid;

/// A trait describing the behaviour of a registry.
pub trait Registry {
    /// Register a sandhog in the registry.
    fn register<'f, U, S>(&self, uuid: U, hostname: S) -> Result<(), Box<dyn std::error::Error>>
    where
        U: Into<Uuid>,
        S: Into<&'f str>;

    /// Lookup a sandhog in the registry.
    fn lookup() {
        unimplemented!();
    }

    /// Deregister a sandhog from the registry.
    fn deregister() {
        unimplemented!();
    }
}

/// Registry implementation.
pub struct RegistryImpl {}

impl Default for RegistryImpl {
    fn default() -> Self { todo!() }
}

impl RegistryImpl {
    fn register_impl(&self, uuid: Uuid, hostname: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl Registry for RegistryImpl {
    fn register<'f, U, S>(&self, uuid: U, hostname: S) -> Result<(), Box<dyn std::error::Error>>
    where
        U: Into<Uuid>,
        S: Into<&'f str>,
    {
        self.register_impl(uuid.into(), hostname.into())
    }
}
