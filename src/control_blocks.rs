use crate::tx::*;
use core::ffi::c_void;

#[allow(unused_imports)]
pub use crate::addons::control_blocks::*;

#[repr(C)]
struct ND_CACHE_ENTRY {
    nx_nd_cache_dest_ip: [ULONG; 4usize],
    nx_nd_cache_mac_addr: [UCHAR; 6usize],
    nx_nd_cache_reserved1: UCHAR,
    nx_nd_cache_reserved2: UCHAR,
    nx_nd_cache_num_solicit: UCHAR,
    nx_nd_cache_nd_status: UCHAR,
    nx_nd_cache_packet_waiting_queue_length: UCHAR,
    nx_nd_cache_is_static: UCHAR,
    nx_nd_cache_timer_tick: ULONG,
    nx_nd_cache_interface_ptr: *mut NX_INTERFACE,
    nx_nd_cache_is_router: *mut NX_IPV6_DEFAULT_ROUTER_ENTRY,
    nx_nd_cache_packet_waiting_head: *mut NX_PACKET,
    nx_nd_cache_packet_waiting_tail: *mut NX_PACKET,
    nx_nd_cache_outgoing_address: *mut NXD_IPV6_ADDRESS,
}

#[repr(C)]
struct NX_IPV6_DESTINATION_ENTRY {
    nx_ipv6_destination_entry_valid: ULONG,
    nx_ipv6_destination_entry_destination_address: [ULONG; 4usize],
    nx_ipv6_destination_entry_next_hop: [ULONG; 4usize],
    nx_ipv6_destination_entry_nd_entry: *mut ND_CACHE_ENTRY,
}

#[repr(C)]
pub struct NX_IPV6_PREFIX_ENTRY {
    nx_ipv6_prefix_entry_network_address: [ULONG; 4usize],
    nx_ipv6_prefix_entry_prefix_length: ULONG,
    nx_ipv6_prefix_entry_valid_lifetime: ULONG,
    nx_ipv6_prefix_entry_prev: *mut NX_IPV6_PREFIX_ENTRY,
    nx_ipv6_prefix_entry_next: *mut NX_IPV6_PREFIX_ENTRY,
}

#[repr(C)]
pub struct NX_IPV6_DEFAULT_ROUTER_ENTRY {
    nx_ipv6_default_router_entry_flag: UCHAR,
    nx_ipv6_default_router_entry_reserved: UCHAR,
    nx_ipv6_default_router_entry_life_time: USHORT,
    nx_ipv6_default_router_entry_router_address: [ULONG; 4usize],
    nx_ipv6_default_router_entry_interface_ptr: *mut NX_INTERFACE,
    nx_ipv6_default_router_entry_neighbor_cache_ptr: *mut ND_CACHE_ENTRY,
}

#[repr(C)]
pub struct NXD_ADDRESS {
    pub nxd_ip_version: ULONG,
    pub nxd_ip_address: [u64; 4usize],
}

#[repr(C)]
pub struct NX_PACKET {
    nx_packet_pool_owner: *mut NX_PACKET_POOL,
    nx_packet_next: *mut NX_PACKET,
    nx_packet_prepend_ptr: *mut UCHAR,
    nx_packet_append_ptr: *mut UCHAR,
    nx_packet_data_start: *mut UCHAR,
    nx_packet_data_end: *mut UCHAR,
    nx_packet_last: *mut NX_PACKET,
    nx_packet_queue_next: *mut NX_PACKET,
    nx_packet_union_next: *mut c_void,
    nx_packet_length: ULONG,
    nx_packet_reassembly_time: ULONG,
    nx_packet_option_state: UCHAR,
    nx_packet_destination_header: UCHAR,
    nx_packet_option_offset: USHORT,
    nx_packet_ip_version: UCHAR,
    nx_packet_identical_copy: UCHAR,
    nx_packet_ip_header_length: UCHAR,
    nx_packet_reserved: UCHAR,
    nx_packet_address: *mut c_void,
    nx_packet_ip_header: *mut UCHAR,
    #[cfg(nx_enable_interface_capability)]
    nx_packet_interface_capability_flag: ULONG,
}

impl NX_PACKET {
    #[inline(always)]
    pub const fn pool_owner(&self) -> *mut NX_PACKET_POOL {
        self.nx_packet_pool_owner
    }

    #[inline(always)]
    pub const fn queue_next(&self) -> *mut NX_PACKET {
        self.nx_packet_queue_next
    }

    #[inline(always)]
    pub const fn set_queue_next(&mut self, queue_next: *mut NX_PACKET) {
        self.nx_packet_queue_next = queue_next;
    }

    #[inline(always)]
    pub const fn data_start(&self) -> *mut UCHAR {
        self.nx_packet_data_start
    }

    #[inline(always)]
    pub const fn data_end(&self) -> *mut UCHAR {
        self.nx_packet_data_end
    }

    #[inline(always)]
    pub const fn prepend_ptr(&self) -> *mut UCHAR {
        self.nx_packet_prepend_ptr
    }

    #[inline(always)]
    pub const fn set_prepend_ptr(&mut self, prepend_ptr: *mut UCHAR) {
        self.nx_packet_prepend_ptr = prepend_ptr
    }

    #[inline(always)]
    pub const fn append_ptr(&self) -> *mut UCHAR {
        self.nx_packet_append_ptr
    }

    #[inline(always)]
    pub const fn set_append_ptr(&mut self, append_ptr: *mut UCHAR) {
        self.nx_packet_append_ptr = append_ptr
    }

    #[inline(always)]
    pub const fn packet_next(&self) -> *mut NX_PACKET {
        self.nx_packet_next
    }

    #[inline(always)]
    pub const fn set_packet_next(&mut self, packet_next: *mut NX_PACKET) {
        self.nx_packet_next = packet_next;
    }

    #[inline(always)]
    pub const fn packet_last(&self) -> *mut NX_PACKET {
        self.nx_packet_last
    }

    #[inline(always)]
    pub const fn set_packet_last(&mut self, packet_last: *mut NX_PACKET) {
        self.nx_packet_last = packet_last;
    }

    #[inline(always)]
    pub const fn set_ip_interface(&mut self, ip_interface: *mut NX_INTERFACE) {
        self.nx_packet_address = ip_interface.cast()
    }

    #[inline(always)]
    pub const fn packet_length(&self) -> ULONG {
        self.nx_packet_length
    }

    #[inline(always)]
    pub const fn set_packet_length(&mut self, packet_length: ULONG) {
        self.nx_packet_length = packet_length
    }
}

#[repr(C)]
pub struct NX_PACKET_POOL {
    nx_packet_pool_id: ULONG,
    nx_packet_pool_name: *mut CHAR,
    nx_packet_pool_available: ULONG,
    nx_packet_pool_total: ULONG,
    nx_packet_pool_empty_requests: ULONG,
    nx_packet_pool_empty_suspensions: ULONG,
    nx_packet_pool_invalid_releases: ULONG,
    nx_packet_pool_available_list: *mut NX_PACKET,
    nx_packet_pool_start: *mut CHAR,
    nx_packet_pool_size: ULONG,
    nx_packet_pool_payload_size: ULONG,
    nx_packet_pool_suspension_list: *mut TX_THREAD,
    nx_packet_pool_suspended_count: ULONG,
    nx_packet_pool_created_next: *mut NX_PACKET_POOL,
    nx_packet_pool_created_previous: *mut NX_PACKET_POOL,
}

impl NX_PACKET_POOL {
    #[inline(always)]
    pub const fn id(&self) -> u32 {
        self.nx_packet_pool_id
    }
}

#[repr(C)]
pub struct NX_ARP {
    nx_arp_route_static: UINT,
    nx_arp_entry_next_update: UINT,
    nx_arp_retries: UINT,
    nx_arp_pool_next: *mut NX_ARP,
    nx_arp_pool_previous: *mut NX_ARP,
    nx_arp_active_next: *mut NX_ARP,
    nx_arp_active_previous: *mut NX_ARP,
    nx_arp_active_list_head: *mut *mut NX_ARP,
    nx_arp_ip_address: ULONG,
    nx_arp_physical_address_msw: ULONG,
    nx_arp_physical_address_lsw: ULONG,
    nx_arp_ip_interface: *mut NX_INTERFACE,
    nx_arp_packets_waiting: *mut NX_PACKET,
}

#[repr(C)]
pub struct NX_UDP_SOCKET {
    nx_udp_socket_id: ULONG,
    nx_udp_socket_name: *mut CHAR,
    nx_udp_socket_port: UINT,
    nx_udp_socket_ip_ptr: *mut NX_IP,
    nx_udp_socket_packets_sent: ULONG,
    nx_udp_socket_bytes_sent: ULONG,
    nx_udp_socket_packets_received: ULONG,
    nx_udp_socket_bytes_received: ULONG,
    nx_udp_socket_invalid_packets: ULONG,
    nx_udp_socket_packets_dropped: ULONG,
    nx_udp_socket_checksum_errors: ULONG,
    nx_udp_socket_type_of_service: ULONG,
    nx_udp_socket_time_to_live: UINT,
    nx_udp_socket_fragment_enable: ULONG,
    nx_udp_socket_disable_checksum: UCHAR,
    nx_udp_socket_reserved: [UCHAR; 3usize],
    nx_udp_socket_receive_count: ULONG,
    nx_udp_socket_queue_maximum: ULONG,
    nx_udp_socket_receive_head: *mut NX_PACKET,
    nx_udp_socket_receive_tail: *mut NX_PACKET,
    nx_udp_socket_bound_next: *mut NX_UDP_SOCKET,
    nx_udp_socket_bound_previous: *mut NX_UDP_SOCKET,
    nx_udp_socket_bind_in_progress: *mut TX_THREAD,
    nx_udp_socket_receive_suspension_list: *mut TX_THREAD,
    nx_udp_socket_receive_suspended_count: ULONG,
    nx_udp_socket_bind_suspension_list: *mut TX_THREAD,
    nx_udp_socket_bind_suspended_count: ULONG,
    nx_udp_socket_created_next: *mut NX_UDP_SOCKET,
    nx_udp_socket_created_previous: *mut NX_UDP_SOCKET,
    nx_udp_receive_callback:
        ::core::option::Option<unsafe extern "C" fn(socket_ptr: *mut NX_UDP_SOCKET)>,
    nx_udp_socket_reserved_ptr: *mut ::core::ffi::c_void,
}

impl NX_UDP_SOCKET {
    #[inline(always)]
    pub const fn id(&self) -> u32 {
        self.nx_udp_socket_id
    }
}

#[repr(C)]
pub struct NX_TCP_SOCKET {
    nx_tcp_socket_id: ULONG,
    nx_tcp_socket_name: *mut CHAR,
    nx_tcp_socket_client_type: UINT,
    nx_tcp_socket_port: UINT,
    nx_tcp_socket_mss: ULONG,
    nx_tcp_socket_connect_ip: NXD_ADDRESS,
    nx_tcp_socket_connect_port: UINT,
    nx_tcp_socket_connect_mss: ULONG,
    nx_tcp_socket_peer_mss: ULONG,
    nx_tcp_socket_connect_interface: *mut NX_INTERFACE,
    nx_tcp_socket_next_hop_address: ULONG,
    nx_tcp_socket_connect_mss2: ULONG,
    nx_tcp_socket_tx_slow_start_threshold: ULONG,
    nx_tcp_socket_state: UINT,
    nx_tcp_socket_tx_sequence: ULONG,
    nx_tcp_socket_rx_sequence: ULONG,
    nx_tcp_socket_rx_sequence_acked: ULONG,
    nx_tcp_socket_delayed_ack_timeout: ULONG,
    nx_tcp_socket_fin_sequence: ULONG,
    nx_tcp_socket_fin_received: USHORT,
    nx_tcp_socket_fin_acked: USHORT,
    nx_tcp_socket_tx_window_advertised: ULONG,
    nx_tcp_socket_tx_window_congestion: ULONG,
    nx_tcp_socket_tx_outstanding_bytes: ULONG,
    nx_tcp_socket_tx_sequence_recover: ULONG,
    nx_tcp_socket_previous_highest_ack: ULONG,
    nx_tcp_socket_ack_n_packet_counter: ULONG,
    nx_tcp_socket_duplicated_ack_received: UINT,
    nx_tcp_socket_rx_window_default: ULONG,
    nx_tcp_socket_rx_window_current: ULONG,
    nx_tcp_socket_rx_window_last_sent: ULONG,
    nx_tcp_socket_packets_sent: ULONG,
    nx_tcp_socket_bytes_sent: ULONG,
    nx_tcp_socket_packets_received: ULONG,
    nx_tcp_socket_bytes_received: ULONG,
    nx_tcp_socket_retransmit_packets: ULONG,
    nx_tcp_socket_checksum_errors: ULONG,
    nx_tcp_socket_zero_window_probe_failure: ULONG,
    nx_tcp_socket_zero_window_probe_sequence: ULONG,
    nx_tcp_socket_zero_window_probe_has_data: UCHAR,
    nx_tcp_socket_zero_window_probe_data: UCHAR,
    nx_tcp_socket_fast_recovery: UCHAR,
    nx_tcp_socket_reserved: UCHAR,
    nx_tcp_socket_ip_ptr: *mut NX_IP,
    nx_tcp_socket_type_of_service: ULONG,
    nx_tcp_socket_time_to_live: UINT,
    nx_tcp_socket_fragment_enable: ULONG,
    nx_tcp_socket_receive_queue_count: ULONG,
    nx_tcp_socket_receive_queue_head: *mut NX_PACKET,
    nx_tcp_socket_receive_queue_tail: *mut NX_PACKET,
    nx_tcp_socket_transmit_queue_maximum: ULONG,
    nx_tcp_socket_transmit_sent_count: ULONG,
    nx_tcp_socket_transmit_sent_head: *mut NX_PACKET,
    nx_tcp_socket_transmit_sent_tail: *mut NX_PACKET,
    nx_tcp_socket_timeout: ULONG,
    nx_tcp_socket_timeout_rate: ULONG,
    nx_tcp_socket_timeout_retries: ULONG,
    nx_tcp_socket_timeout_max_retries: ULONG,
    nx_tcp_socket_timeout_shift: UCHAR,
    nx_tcp_socket_reserved2: [UCHAR; 3usize],
    nx_tcp_socket_bound_next: *mut NX_TCP_SOCKET,
    nx_tcp_socket_bound_previous: *mut NX_TCP_SOCKET,
    nx_tcp_socket_bind_in_progress: *mut TX_THREAD,
    nx_tcp_socket_receive_suspension_list: *mut TX_THREAD,
    nx_tcp_socket_receive_suspended_count: ULONG,
    nx_tcp_socket_transmit_suspension_list: *mut TX_THREAD,
    nx_tcp_socket_transmit_suspended_count: ULONG,
    nx_tcp_socket_connect_suspended_thread: *mut TX_THREAD,
    nx_tcp_socket_disconnect_suspended_thread: *mut TX_THREAD,
    nx_tcp_socket_bind_suspension_list: *mut TX_THREAD,
    nx_tcp_socket_bind_suspended_count: ULONG,
    nx_tcp_socket_created_next: *mut NX_TCP_SOCKET,
    nx_tcp_socket_created_previous: *mut NX_TCP_SOCKET,
    nx_tcp_urgent_data_callback:
        ::core::option::Option<unsafe extern "C" fn(socket_ptr: *mut NX_TCP_SOCKET)>,
    nx_tcp_disconnect_callback:
        ::core::option::Option<unsafe extern "C" fn(socket_ptr: *mut NX_TCP_SOCKET)>,
    nx_tcp_receive_callback:
        ::core::option::Option<unsafe extern "C" fn(socket_ptr: *mut NX_TCP_SOCKET)>,
    nx_tcp_socket_window_update_notify:
        ::core::option::Option<unsafe extern "C" fn(socket_ptr: *mut NX_TCP_SOCKET)>,
    nx_tcp_socket_reserved_ptr: *mut ::core::ffi::c_void,
    nx_tcp_socket_transmit_queue_maximum_default: ULONG,
    nx_tcp_socket_ipv6_addr: *mut NXD_IPV6_ADDRESS,
}

#[repr(C)]
pub struct NX_TCP_LISTEN {
    nx_tcp_listen_port: UINT,
    nx_tcp_listen_callback:
        ::core::option::Option<unsafe extern "C" fn(socket_ptr: *mut NX_TCP_SOCKET, port: UINT)>,
    nx_tcp_listen_socket_ptr: *mut NX_TCP_SOCKET,
    nx_tcp_listen_queue_maximum: ULONG,
    nx_tcp_listen_queue_current: ULONG,
    nx_tcp_listen_queue_head: *mut NX_PACKET,
    nx_tcp_listen_queue_tail: *mut NX_PACKET,
    nx_tcp_listen_next: *mut NX_TCP_LISTEN,
    nx_tcp_listen_previous: *mut NX_TCP_LISTEN,
}

#[repr(C)]
pub struct NXD_IPV6_ADDRESS {
    pub nxd_ipv6_address_valid: UCHAR,
    pub nxd_ipv6_address_type: UCHAR,
    pub nxd_ipv6_address_state: UCHAR,
    pub nxd_ipv6_address_prefix_length: UCHAR,
    pub nxd_ipv6_address_attached: *mut NX_INTERFACE,
    pub nxd_ipv6_address: [ULONG; 4usize],
    pub nxd_ipv6_address_next: *mut NXD_IPV6_ADDRESS,
    pub nxd_ipv6_address_DupAddrDetectTransmit: UCHAR,
    pub nxd_ipv6_address_ConfigurationMethod: UCHAR,
    pub nxd_ipv6_address_index: UCHAR,
    pub reserved: UCHAR,
}

#[repr(C)]
pub struct NX_INTERFACE {
    nx_interface_name: *mut CHAR,
    nx_interface_valid: UCHAR,
    nx_interface_address_mapping_needed: UCHAR,
    nx_interface_link_up: UCHAR,
    nx_interface_index: UCHAR,
    nx_interface_link_status_change: UCHAR,
    nx_interface_reserved: [UCHAR; 3usize],
    nx_interface_physical_address_msw: ULONG,
    nx_interface_physical_address_lsw: ULONG,
    nx_interface_ip_address: ULONG,
    nx_interface_ip_network_mask: ULONG,
    nx_interface_ip_network: ULONG,
    pub nxd_interface_ipv6_address_list_head: *mut NXD_IPV6_ADDRESS,
    nx_interface_ip_mtu_size: ULONG,
    nx_ipv6_rtr_solicitation_max: ULONG,
    nx_ipv6_rtr_solicitation_count: ULONG,
    nx_ipv6_rtr_solicitation_interval: ULONG,
    nx_ipv6_rtr_solicitation_timer: ULONG,
    nx_interface_additional_link_info: *mut ::core::ffi::c_void,
    nx_interface_link_driver_entry:
        ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP_DRIVER)>,
    #[cfg(nx_enable_interface_capability)]
    nx_interface_capability_flag: ULONG,
    nx_interface_arp_defend_timeout: ULONG,
    nx_interface_ip_probe_address: ULONG,
    nx_interface_ip_conflict_notify_handler: ::core::option::Option<
        unsafe extern "C" fn(arg1: *mut NX_IP, arg2: UINT, arg3: ULONG, arg4: ULONG, arg5: ULONG),
    >,
}

impl NX_INTERFACE {
    #[inline(always)]
    pub const fn index(&self) -> UCHAR {
        self.nx_interface_index
    }

    #[inline(always)]
    pub const fn link_up(&self) -> UCHAR {
        self.nx_interface_link_up
    }

    #[inline(always)]
    pub const fn set_link_up(&mut self, link_up: UCHAR) {
        self.nx_interface_link_up = link_up;
    }
}

#[repr(C)]
struct NX_IPV4_MULTICAST_ENTRY {
    nx_ipv4_multicast_join_list: ULONG,
    nx_ipv4_multicast_join_interface_list: *mut NX_INTERFACE,
    nx_ipv4_multicast_join_count: ULONG,
    nx_ipv4_multicast_update_time: ULONG,
    nx_ipv4_multicast_loopback_enable: UINT,
}

#[repr(C)]
pub struct NX_IP {
    nx_ip_id: ULONG,
    nx_ip_name: *mut CHAR,
    nx_ip_gateway_address: ULONG,
    nx_ip_gateway_interface: *mut NX_INTERFACE,
    nx_ipv6_address: [NXD_IPV6_ADDRESS; 4usize],
    nx_ipv6_destination_table: [NX_IPV6_DESTINATION_ENTRY; 8usize],
    nx_ipv6_nd_cache: [ND_CACHE_ENTRY; 16usize],
    nx_ipv6_destination_table_size: UINT,
    nx_ip_total_packet_send_requests: ULONG,
    nx_ip_total_packets_sent: ULONG,
    nx_ip_total_bytes_sent: ULONG,
    nx_ip_total_packets_received: ULONG,
    nx_ip_total_packets_delivered: ULONG,
    nx_ip_total_bytes_received: ULONG,
    nx_ip_packets_forwarded: ULONG,
    nx_ip_packets_reassembled: ULONG,
    nx_ip_reassembly_failures: ULONG,
    nx_ip_invalid_packets: ULONG,
    nx_ip_invalid_transmit_packets: ULONG,
    nx_ip_invalid_receive_address: ULONG,
    nx_ip_unknown_protocols_received: ULONG,
    nx_ip_transmit_resource_errors: ULONG,
    nx_ip_transmit_no_route_errors: ULONG,
    nx_ip_receive_packets_dropped: ULONG,
    nx_ip_receive_checksum_errors: ULONG,
    nx_ip_send_packets_dropped: ULONG,
    nx_ip_total_fragment_requests: ULONG,
    nx_ip_successful_fragment_requests: ULONG,
    nx_ip_fragment_failures: ULONG,
    nx_ip_total_fragments_sent: ULONG,
    nx_ip_total_fragments_received: ULONG,
    nx_ip_arp_requests_sent: ULONG,
    nx_ip_arp_requests_received: ULONG,
    nx_ip_arp_responses_sent: ULONG,
    nx_ip_arp_responses_received: ULONG,
    nx_ip_arp_aged_entries: ULONG,
    nx_ip_arp_invalid_messages: ULONG,
    nx_ip_arp_static_entries: ULONG,
    nx_ip_udp_packets_sent: ULONG,
    nx_ip_udp_bytes_sent: ULONG,
    nx_ip_udp_packets_received: ULONG,
    nx_ip_udp_bytes_received: ULONG,
    nx_ip_udp_invalid_packets: ULONG,
    nx_ip_udp_no_port_for_delivery: ULONG,
    nx_ip_udp_receive_packets_dropped: ULONG,
    nx_ip_udp_checksum_errors: ULONG,
    nx_ip_tcp_packets_sent: ULONG,
    nx_ip_tcp_bytes_sent: ULONG,
    nx_ip_tcp_packets_received: ULONG,
    nx_ip_tcp_bytes_received: ULONG,
    nx_ip_tcp_invalid_packets: ULONG,
    nx_ip_tcp_receive_packets_dropped: ULONG,
    nx_ip_tcp_checksum_errors: ULONG,
    nx_ip_tcp_connections: ULONG,
    nx_ip_tcp_passive_connections: ULONG,
    nx_ip_tcp_active_connections: ULONG,
    nx_ip_tcp_disconnections: ULONG,
    nx_ip_tcp_connections_dropped: ULONG,
    nx_ip_tcp_retransmit_packets: ULONG,
    nx_ip_tcp_resets_received: ULONG,
    nx_ip_tcp_resets_sent: ULONG,
    nx_ip_icmp_total_messages_received: ULONG,
    nx_ip_icmp_checksum_errors: ULONG,
    nx_ip_icmp_invalid_packets: ULONG,
    nx_ip_icmp_unhandled_messages: ULONG,
    nx_ip_pings_sent: ULONG,
    nx_ip_ping_timeouts: ULONG,
    nx_ip_ping_threads_suspended: ULONG,
    nx_ip_ping_responses_received: ULONG,
    nx_ip_pings_received: ULONG,
    nx_ip_pings_responded_to: ULONG,
    nx_ip_igmp_invalid_packets: ULONG,
    nx_ip_igmp_reports_sent: ULONG,
    nx_ip_igmp_queries_received: ULONG,
    nx_ip_igmp_checksum_errors: ULONG,
    nx_ip_igmp_groups_joined: ULONG,
    nx_ip_igmp_router_version: ULONG,
    nx_ip_rarp_requests_sent: ULONG,
    nx_ip_rarp_responses_received: ULONG,
    nx_ip_rarp_invalid_messages: ULONG,
    nx_ip_forward_packet_process:
        ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP, arg2: *mut NX_PACKET)>,
    nx_ip_packet_id: ULONG,
    nx_ip_default_packet_pool: *mut NX_PACKET_POOL,
    nx_ip_protection: TX_MUTEX,
    nx_ip_initialize_done: UINT,
    #[cfg(nx_driver_deferred_processing)]
    nx_ip_driver_deferred_packet_head: *mut NX_PACKET,
    #[cfg(nx_driver_deferred_processing)]
    nx_ip_driver_deferred_packet_tail: *mut NX_PACKET,
    #[cfg(nx_driver_deferred_processing)]
    nx_ip_driver_deferred_packet_handler:
        ::core::option::Option<unsafe extern "C" fn(ip: *mut NX_IP, packet: *mut NX_PACKET)>,
    nx_ip_deferred_received_packet_head: *mut NX_PACKET,
    nx_ip_deferred_received_packet_tail: *mut NX_PACKET,
    nx_ip_raw_ip_processing: ::core::option::Option<
        unsafe extern "C" fn(arg1: *mut NX_IP, arg2: ULONG, arg3: *mut NX_PACKET) -> UINT,
    >,
    nx_ip_raw_received_packet_head: *mut NX_PACKET,
    nx_ip_raw_received_packet_tail: *mut NX_PACKET,
    nx_ip_raw_received_packet_count: ULONG,
    nx_ip_raw_received_packet_max: ULONG,
    nx_ip_raw_packet_suspension_list: *mut TX_THREAD,
    nx_ip_raw_packet_suspended_count: ULONG,
    nx_ip_thread: TX_THREAD,
    nx_ip_events: TX_EVENT_FLAGS_GROUP,
    nx_ip_periodic_timer: TX_TIMER,
    nx_ip_fragment_processing:
        ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP_DRIVER)>,
    nx_ip_fragment_assembly: ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP)>,
    nx_ip_fragment_timeout_check: ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP)>,
    nx_ip_timeout_fragment: *mut NX_PACKET,
    nx_ip_received_fragment_head: *mut NX_PACKET,
    nx_ip_received_fragment_tail: *mut NX_PACKET,
    nx_ip_fragment_assembly_head: *mut NX_PACKET,
    nx_ip_fragment_assembly_tail: *mut NX_PACKET,
    nx_ip_address_change_notify: ::core::option::Option<
        unsafe extern "C" fn(arg1: *mut NX_IP, arg2: *mut ::core::ffi::c_void),
    >,
    nx_ip_address_change_notify_additional_info: *mut ::core::ffi::c_void,
    nx_ip_address_change_notify_internal: ::core::option::Option<
        unsafe extern "C" fn(arg1: *mut NX_IP, arg2: *mut ::core::ffi::c_void),
    >,
    nx_ipv4_multicast_entry: [NX_IPV4_MULTICAST_ENTRY; 7usize],
    nx_ip_igmp_global_loopback_enable: UINT,
    nx_ip_igmp_packet_receive:
        ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP, arg2: *mut NX_PACKET)>,
    nx_ip_igmp_periodic_processing: ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP)>,
    nx_ip_igmp_queue_process: ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP)>,
    nx_ip_igmp_queue_head: *mut NX_PACKET,
    nx_ip_icmp_sequence: ULONG,
    nx_ip_icmp_packet_receive:
        ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP, arg2: *mut NX_PACKET)>,
    nx_ip_icmp_queue_process: ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP)>,
    nx_ip_icmpv4_packet_process:
        ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP, arg2: *mut NX_PACKET)>,
    nx_ip_icmpv6_packet_process:
        ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP, arg2: *mut NX_PACKET)>,
    nx_nd_cache_fast_periodic_update:
        ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP)>,
    nx_nd_cache_slow_periodic_update:
        ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP)>,
    nx_icmpv6_ra_flag_callback:
        ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP, arg2: UINT)>,
    nx_ip_icmp_queue_head: *mut NX_PACKET,
    nx_ip_icmp_ping_suspension_list: *mut TX_THREAD,
    nx_ip_icmp_ping_suspended_count: ULONG,
    nx_ip_udp_port_table: [*mut NX_UDP_SOCKET; 32usize],
    nx_ip_udp_created_sockets_ptr: *mut NX_UDP_SOCKET,
    nx_ip_udp_created_sockets_count: ULONG,
    nx_ip_udp_packet_receive:
        ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP, arg2: *mut NX_PACKET)>,
    nx_ip_tcp_port_table: [*mut NX_TCP_SOCKET; 32usize],
    nx_ip_tcp_created_sockets_ptr: *mut NX_TCP_SOCKET,
    nx_ip_tcp_created_sockets_count: ULONG,
    nx_ip_tcp_packet_receive:
        ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP, arg2: *mut NX_PACKET)>,
    nx_ip_tcp_periodic_processing: ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP)>,
    nx_ip_tcp_fast_periodic_processing:
        ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP)>,
    nx_ip_tcp_queue_process: ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP)>,
    nx_ip_tcp_queue_head: *mut NX_PACKET,
    nx_ip_tcp_queue_tail: *mut NX_PACKET,
    nx_ip_tcp_received_packet_count: ULONG,
    nx_ip_tcp_server_listen_reqs: [NX_TCP_LISTEN; 10usize],
    nx_ip_tcp_available_listen_requests: *mut NX_TCP_LISTEN,
    nx_ip_tcp_active_listen_requests: *mut NX_TCP_LISTEN,
    nx_ip_fast_periodic_timer_created: UINT,
    nx_ip_fast_periodic_timer: TX_TIMER,
    nx_ip_arp_table: [*mut NX_ARP; 32usize],
    nx_ip_arp_static_list: *mut NX_ARP,
    nx_ip_arp_dynamic_list: *mut NX_ARP,
    nx_ip_arp_dynamic_active_count: ULONG,
    nx_ip_arp_deferred_received_packet_head: *mut NX_PACKET,
    nx_ip_arp_deferred_received_packet_tail: *mut NX_PACKET,
    nx_ip_arp_allocate: ::core::option::Option<
        unsafe extern "C" fn(arg1: *mut NX_IP, arg2: *mut *mut NX_ARP, arg3: UINT) -> UINT,
    >,
    nx_ip_arp_periodic_update: ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP)>,
    nx_ip_arp_queue_process: ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP)>,
    nx_ip_arp_packet_send: ::core::option::Option<
        unsafe extern "C" fn(
            arg1: *mut NX_IP,
            destination_ip: ULONG,
            nx_interface: *mut NX_INTERFACE,
        ),
    >,
    nx_ip_arp_gratuitous_response_handler:
        ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP, arg2: *mut NX_PACKET)>,
    nx_ip_arp_cache_memory: *mut NX_ARP,
    nx_ip_arp_total_entries: ULONG,
    nx_ip_rarp_periodic_update: ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP)>,
    nx_ip_rarp_queue_process: ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP)>,
    nx_ip_rarp_deferred_received_packet_head: *mut NX_PACKET,
    nx_ip_rarp_deferred_received_packet_tail: *mut NX_PACKET,
    nx_ip_created_next: *mut NX_IP,
    nx_ip_created_previous: *mut NX_IP,
    nx_ip_reserved_ptr: *mut ::core::ffi::c_void,
    nx_tcp_deferred_cleanup_check: ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP)>,
    nx_ip_interface: [NX_INTERFACE; 2usize],
    nx_ipv4_packet_receive:
        ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP, arg2: *mut NX_PACKET)>,
    nx_ipv6_default_router_table_size: USHORT,
    nx_ipv6_default_router_table: [NX_IPV6_DEFAULT_ROUTER_ENTRY; 8usize],
    nx_ipv6_default_router_table_round_robin_index: UINT,
    nx_ipv6_prefix_list_table: [NX_IPV6_PREFIX_ENTRY; 8usize],
    nx_ipv6_prefix_list_ptr: *mut NX_IPV6_PREFIX_ENTRY,
    nx_ipv6_prefix_entry_free_list: *mut NX_IPV6_PREFIX_ENTRY,
    nx_ipv6_packet_receive:
        ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP, arg2: *mut NX_PACKET)>,
    nx_ipv6_retrans_timer_ticks: ULONG,
    nx_ipv6_reachable_timer: ULONG,
    nx_ipv6_hop_limit: ULONG,
    nx_ip_link_status_change_callback:
        ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP, arg2: UINT, arg3: UINT)>,
}

impl NX_IP {
    #[inline(always)]
    pub const fn id(&self) -> u32 {
        self.nx_ip_id
    }

    #[inline(always)]
    pub const fn default_packet_pool(&self) -> *mut NX_PACKET_POOL {
        self.nx_ip_default_packet_pool
    }
}

#[repr(C)]
pub struct NX_IP_DRIVER {
    nx_ip_driver_command: UINT,
    nx_ip_driver_status: UINT,
    nx_ip_driver_physical_address_msw: ULONG,
    nx_ip_driver_physical_address_lsw: ULONG,
    nx_ip_driver_packet: *mut NX_PACKET,
    nx_ip_driver_return_ptr: *mut ULONG,
    nx_ip_driver_ptr: *mut NX_IP,
    nx_ip_driver_interface: *mut NX_INTERFACE,
}

impl NX_IP_DRIVER {
    #[inline(always)]
    pub const fn command(&self) -> UINT {
        self.nx_ip_driver_command
    }

    #[inline(always)]
    pub const fn set_status(&mut self, status: UINT) {
        self.nx_ip_driver_status = status;
    }

    #[inline(always)]
    pub const fn return_ptr(&self) -> *mut ULONG {
        self.nx_ip_driver_return_ptr
    }

    #[inline(always)]
    pub const fn packet(&self) -> *mut NX_PACKET {
        self.nx_ip_driver_packet
    }

    #[inline(always)]
    pub const fn ip(&self) -> *mut NX_IP {
        self.nx_ip_driver_ptr
    }

    #[inline(always)]
    pub const fn interface(&self) -> *mut NX_INTERFACE {
        self.nx_ip_driver_interface
    }

    #[inline(always)]
    pub const fn physical_address_msw(&self) -> ULONG {
        self.nx_ip_driver_physical_address_msw
    }

    #[inline(always)]
    pub const fn physical_address_lsw(&self) -> ULONG {
        self.nx_ip_driver_physical_address_lsw
    }
}

#[repr(C)]
pub struct NX_IP_ROUTING_ENTRY_STRUCT {
    nx_ip_routing_dest_ip: ULONG,
    nx_ip_routing_net_mask: ULONG,
    nx_ip_routing_next_hop_address: ULONG,
    nx_ip_routing_entry_ip_interface: *mut NX_INTERFACE,
}
