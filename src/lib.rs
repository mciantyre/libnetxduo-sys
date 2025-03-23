#![no_std]
#![allow(
    // Follow NetX Duo naming conventions.
    non_camel_case_types,
    non_snake_case,
    // Safety docs aren't available in the standard
    // documentation. Since it's a call into a C
    // library, it's understood to have arbitrary
    // unsafety.
    clippy::missing_safety_doc,
)]

use libthreadx_sys as tx;

#[cfg(feature = "filex")]
use libfilex_sys as _;

mod control_blocks;
pub use control_blocks::*;

pub mod error_checked;
pub mod unchecked;

unsafe extern "C" {
    #[doc = include_str!("docs/nx_system_initialize.md")]
    #[link_name = "_nx_system_initialize"]
    pub fn nx_system_initialize();
}

//
// API returns
//

pub const NX_SUCCESS: tx::UINT = 0x00;
pub const NX_NO_PACKET: tx::UINT = 0x01;
pub const NX_UNDERFLOW: tx::UINT = 0x02;
pub const NX_OVERFLOW: tx::UINT = 0x03;
pub const NX_NO_MAPPING: tx::UINT = 0x04;
pub const NX_DELETED: tx::UINT = 0x05;
pub const NX_POOL_ERROR: tx::UINT = 0x06;
pub const NX_PTR_ERROR: tx::UINT = 0x07;
pub const NX_WAIT_ERROR: tx::UINT = 0x08;
pub const NX_SIZE_ERROR: tx::UINT = 0x09;
pub const NX_OPTION_ERROR: tx::UINT = 0x0a;
pub const NX_DELETE_ERROR: tx::UINT = 0x10;
pub const NX_CALLER_ERROR: tx::UINT = 0x11;
pub const NX_INVALID_PACKET: tx::UINT = 0x12;
pub const NX_INVALID_SOCKET: tx::UINT = 0x13;
pub const NX_NOT_ENABLED: tx::UINT = 0x14;
pub const NX_ALREADY_ENABLED: tx::UINT = 0x15;
pub const NX_ENTRY_NOT_FOUND: tx::UINT = 0x16;
pub const NX_NO_MORE_ENTRIES: tx::UINT = 0x17;
pub const NX_ARP_TIMER_ERROR: tx::UINT = 0x18;
pub const NX_RESERVED_CODE0: tx::UINT = 0x19;
pub const NX_WAIT_ABORTED: tx::UINT = 0x1A;
pub const NX_IP_INTERNAL_ERROR: tx::UINT = 0x20;
pub const NX_IP_ADDRESS_ERROR: tx::UINT = 0x21;
pub const NX_ALREADY_BOUND: tx::UINT = 0x22;
pub const NX_PORT_UNAVAILABLE: tx::UINT = 0x23;
pub const NX_NOT_BOUND: tx::UINT = 0x24;
pub const NX_RESERVED_CODE1: tx::UINT = 0x25;
pub const NX_SOCKET_UNBOUND: tx::UINT = 0x26;
pub const NX_NOT_CREATED: tx::UINT = 0x27;
pub const NX_SOCKETS_BOUND: tx::UINT = 0x28;
pub const NX_NO_RESPONSE: tx::UINT = 0x29;
pub const NX_POOL_DELETED: tx::UINT = 0x30;
pub const NX_ALREADY_RELEASED: tx::UINT = 0x31;
pub const NX_RESERVED_CODE2: tx::UINT = 0x32;
pub const NX_MAX_LISTEN: tx::UINT = 0x33;
pub const NX_DUPLICATE_LISTEN: tx::UINT = 0x34;
pub const NX_NOT_CLOSED: tx::UINT = 0x35;
pub const NX_NOT_LISTEN_STATE: tx::UINT = 0x36;
pub const NX_IN_PROGRESS: tx::UINT = 0x37;
pub const NX_NOT_CONNECTED: tx::UINT = 0x38;
pub const NX_WINDOW_OVERFLOW: tx::UINT = 0x39;
pub const NX_ALREADY_SUSPENDED: tx::UINT = 0x40;
pub const NX_DISCONNECT_FAILED: tx::UINT = 0x41;
pub const NX_STILL_BOUND: tx::UINT = 0x42;
pub const NX_NOT_SUCCESSFUL: tx::UINT = 0x43;
pub const NX_UNHANDLED_COMMAND: tx::UINT = 0x44;
pub const NX_NO_FREE_PORTS: tx::UINT = 0x45;
pub const NX_INVALID_PORT: tx::UINT = 0x46;
pub const NX_INVALID_RELISTEN: tx::UINT = 0x47;
pub const NX_CONNECTION_PENDING: tx::UINT = 0x48;
pub const NX_TX_QUEUE_DEPTH: tx::UINT = 0x49;
pub const NX_NOT_IMPLEMENTED: tx::UINT = 0x4A;
pub const NX_NOT_SUPPORTED: tx::UINT = 0x4B;
pub const NX_INVALID_INTERFACE: tx::UINT = 0x4C;
pub const NX_INVALID_PARAMETERS: tx::UINT = 0x4D;
pub const NX_NOT_FOUND: tx::UINT = 0x4E;
pub const NX_CANNOT_START: tx::UINT = 0x4F;
pub const NX_NO_INTERFACE_ADDRESS: tx::UINT = 0x50;
pub const NX_INVALID_MTU_DATA: tx::UINT = 0x51;
pub const NX_DUPLICATED_ENTRY: tx::UINT = 0x52;
pub const NX_PACKET_OFFSET_ERROR: tx::UINT = 0x53;
pub const NX_OPTION_HEADER_ERROR: tx::UINT = 0x54;
pub const NX_CONTINUE: tx::UINT = 0x55;
pub const NX_TCPIP_OFFLOAD_ERROR: tx::UINT = 0x56;

pub const NX_LINK_PACKET_SEND: crate::tx::UINT = 0;
pub const NX_LINK_INITIALIZE: crate::tx::UINT = 1;
pub const NX_LINK_ENABLE: crate::tx::UINT = 2;
pub const NX_LINK_DISABLE: crate::tx::UINT = 3;
pub const NX_LINK_PACKET_BROADCAST: crate::tx::UINT = 4;
pub const NX_LINK_ARP_SEND: crate::tx::UINT = 5;
pub const NX_LINK_ARP_RESPONSE_SEND: crate::tx::UINT = 6;
pub const NX_LINK_RARP_SEND: crate::tx::UINT = 7;
pub const NX_LINK_MULTICAST_JOIN: crate::tx::UINT = 8;
pub const NX_LINK_MULTICAST_LEAVE: crate::tx::UINT = 9;
pub const NX_LINK_GET_STATUS: crate::tx::UINT = 10;
pub const NX_LINK_GET_SPEED: crate::tx::UINT = 11;
pub const NX_LINK_GET_DUPLEX_TYPE: crate::tx::UINT = 12;
pub const NX_LINK_GET_ERROR_COUNT: crate::tx::UINT = 13;
pub const NX_LINK_GET_RX_COUNT: crate::tx::UINT = 14;
pub const NX_LINK_GET_TX_COUNT: crate::tx::UINT = 15;
pub const NX_LINK_GET_ALLOC_ERRORS: crate::tx::UINT = 16;
pub const NX_LINK_UNINITIALIZE: crate::tx::UINT = 17;
pub const NX_LINK_DEFERRED_PROCESSING: crate::tx::UINT = 18;
pub const NX_LINK_INTERFACE_ATTACH: crate::tx::UINT = 19;
pub const NX_LINK_SET_PHYSICAL_ADDRESS: crate::tx::UINT = 20;
pub const NX_INTERFACE_CAPABILITY_GET: crate::tx::UINT = 21;
pub const NX_INTERFACE_CAPABILITY_SET: crate::tx::UINT = 22;
pub const NX_LINK_INTERFACE_DETACH: crate::tx::UINT = 23;
pub const NX_LINK_FACTORY_ADDRESS_GET: crate::tx::UINT = 24;
pub const NX_LINK_RX_ENABLE: crate::tx::UINT = 25;
pub const NX_LINK_RX_DISABLE: crate::tx::UINT = 26;
pub const NX_LINK_6LOWPAN_COMMAND: crate::tx::UINT = 27;
pub const NX_LINK_GET_INTERFACE_TYPE: crate::tx::UINT = 28;
pub const NX_LINK_RAW_PACKET_SEND: crate::tx::UINT = 29;
pub const NX_LINK_USER_COMMAND: crate::tx::UINT = 50;

//
// Packet sizes assum NX_IPSEC_ENABLE is undefined. Values without
// version assume default build with FEATURE_NX_IPV6 defined.
//

pub const NX_IPSEC_MAX_HEADER_SIZE: crate::tx::ULONG = 0;
pub const NX_PHYSICAL_HEADER: crate::tx::ULONG = 16;

pub const NX_IPV4_PACKET: crate::tx::ULONG = NX_PHYSICAL_HEADER + 20;
pub const NX_IPV4_TCP_PACKET: crate::tx::ULONG = NX_IPV4_PACKET + 20;
pub const NX_IPV4_UDP_PACKET: crate::tx::ULONG = NX_IPV4_PACKET + 8;
pub const NX_IPV4_ICMP_PACKET: crate::tx::ULONG = NX_IPV4_PACKET;
pub const NX_IPV4_IGMP_PACKET: crate::tx::ULONG = NX_IPV4_PACKET;

pub const NX_IPV6_PACKET: crate::tx::ULONG = NX_PHYSICAL_HEADER + 40;
pub const NX_IPV6_UDP_PACKET: crate::tx::ULONG = NX_IPV6_PACKET + 8;
pub const NX_IPV6_TCP_PACKET: crate::tx::ULONG = NX_IPV6_PACKET + 20;
pub const NX_IPV6_ICMP_PACKET: crate::tx::ULONG = NX_IPV6_PACKET;
pub const NX_RECEIVE_PACKET: crate::tx::ULONG = 0;

pub const NX_IP_PACKET: crate::tx::ULONG = NX_IPV6_PACKET;
pub const NX_TCP_PACKET: crate::tx::ULONG = NX_IPV6_TCP_PACKET + NX_IPSEC_MAX_HEADER_SIZE;
pub const NX_UDP_PACKET: crate::tx::ULONG = NX_IPV6_UDP_PACKET + NX_IPSEC_MAX_HEADER_SIZE;
pub const NX_ICMP_PACKET: crate::tx::ULONG = NX_IPV6_ICMP_PACKET + NX_IPSEC_MAX_HEADER_SIZE;

//
// Object IDs.
//

pub const NX_IP_ID: u32 = 0x49502020;
pub const NX_PACKET_POOL_ID: u32 = 0x5041434B;
pub const NX_UDP_ID: u32 = 0x55445020;

//
// Interface capabilities
//

pub const NX_INTERFACE_CAPABILITY_IPV4_TX_CHECKSUM: crate::tx::ULONG = 0x00000001;
pub const NX_INTERFACE_CAPABILITY_IPV4_RX_CHECKSUM: crate::tx::ULONG = 0x00000002;
pub const NX_INTERFACE_CAPABILITY_TCP_TX_CHECKSUM: crate::tx::ULONG = 0x00000004;
pub const NX_INTERFACE_CAPABILITY_TCP_RX_CHECKSUM: crate::tx::ULONG = 0x00000008;
pub const NX_INTERFACE_CAPABILITY_UDP_TX_CHECKSUM: crate::tx::ULONG = 0x00000010;
pub const NX_INTERFACE_CAPABILITY_UDP_RX_CHECKSUM: crate::tx::ULONG = 0x00000020;
pub const NX_INTERFACE_CAPABILITY_ICMPV4_TX_CHECKSUM: crate::tx::ULONG = 0x00000040;
pub const NX_INTERFACE_CAPABILITY_ICMPV4_RX_CHECKSUM: crate::tx::ULONG = 0x00000080;
pub const NX_INTERFACE_CAPABILITY_ICMPV6_RX_CHECKSUM: crate::tx::ULONG = 0x00000100;
pub const NX_INTERFACE_CAPABILITY_ICMPV6_TX_CHECKSUM: crate::tx::ULONG = 0x00000200;
pub const NX_INTERFACE_CAPABILITY_IGMP_TX_CHECKSUM: crate::tx::ULONG = 0x00000400;
pub const NX_INTERFACE_CAPABILITY_IGMP_RX_CHECKSUM: crate::tx::ULONG = 0x00000800;

//
// Misc.
//

pub const NX_IP_TIME_TO_LIVE: u32 = 0x00000080;

mod addons;
