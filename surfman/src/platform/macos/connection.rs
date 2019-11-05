// surfman/surfman/src/platform/src/macos/connection.rs
//
//! Represents the connection to the Core Graphics window server.
//! 
//! This is a no-op, because the system APIs automatically manage the global window server
//! connection.

use crate::Error;
use super::adapter::Adapter;

#[cfg(feature = "sm-winit")]
use winit::Window;

/// A no-op connection.
#[derive(Clone)]
pub struct Connection;

impl Connection {
    /// Connects to the default display.
    #[inline]
    pub fn new() -> Result<Connection, Error> {
        Ok(Connection)
    }

    /// Returns the "best" adapter on this system.
    #[inline]
    pub fn create_adapter(&self) -> Result<Adapter, Error> {
        self.create_hardware_adapter()
    }

    /// Returns the "best" hardware adapter on this system.
    #[inline]
    pub fn create_hardware_adapter(&self) -> Result<Adapter, Error> {
        Ok(Adapter)
    }

    /// Returns the "best" software adapter on this system.
    ///
    /// The macOS backend has no software support, so this returns an error.
    #[inline]
    pub fn create_software_adapter(&self) -> Result<Adapter, Error> {
        Err(Error::NoSoftwareAdapters)
    }

    #[cfg(feature = "sm-winit")]
    pub fn from_winit_window(_: &Window) -> Result<Connection, Error> {
        Connection::new()
    }
}
