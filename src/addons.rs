//! These submodules are enabled with Cargo features.

#[cfg(feature = "dhcp-client")]
mod dhcp_client;

pub(crate) mod control_blocks {
    #[cfg(feature = "dhcp-client")]
    pub use super::dhcp_client::control_blocks::*;
}

pub(crate) mod error_checked {
    #[cfg(feature = "dhcp-client")]
    pub use super::dhcp_client::error_checked::*;
}

pub(crate) mod unchecked {
    #[cfg(feature = "dhcp-client")]
    pub use super::dhcp_client::unchecked::*;
}
