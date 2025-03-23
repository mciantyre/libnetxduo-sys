use crate::tx::*;
use crate::*;

#[allow(unused_imports)]
pub use crate::addons::unchecked::*;

unsafe extern "C" {
    #[doc = include_str!("docs/nx_arp_dynamic_entries_invalidate.md")]
    #[link_name = "_nx_arp_dynamic_entries_invalidate"]
    pub fn nx_arp_dynamic_entries_invalidate(ip_ptr: *mut NX_IP) -> UINT;

    #[doc = include_str!("docs/nx_arp_dynamic_entry_set.md")]
    #[link_name = "_nx_arp_dynamic_entry_set"]
    pub fn nx_arp_dynamic_entry_set(
        ip_ptr: *mut NX_IP,
        ip_address: ULONG,
        physical_msw: ULONG,
        physical_lsw: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_arp_enable.md")]
    #[link_name = "_nx_arp_enable"]
    pub fn nx_arp_enable(
        ip_ptr: *mut NX_IP,
        arp_cache_memory: *mut ::core::ffi::c_void,
        arp_cache_size: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_arp_entry_delete.md")]
    #[link_name = "_nx_arp_entry_delete"]
    pub fn nx_arp_entry_delete(ip_ptr: *mut NX_IP, ip_address: ULONG) -> UINT;

    #[doc = include_str!("docs/nx_arp_gratuitous_send.md")]
    #[link_name = "_nx_arp_gratuitous_send"]
    pub fn nx_arp_gratuitous_send(
        ip_ptr: *mut NX_IP,
        response_handler: ::core::option::Option<
            unsafe extern "C" fn(ip_ptr: *mut NX_IP, packet_ptr: *mut NX_PACKET),
        >,
    ) -> UINT;

    #[doc = include_str!("docs/nx_arp_hardware_address_find.md")]
    #[link_name = "_nx_arp_hardware_address_find"]
    pub fn nx_arp_hardware_address_find(
        ip_ptr: *mut NX_IP,
        ip_address: ULONG,
        physical_msw: *mut ULONG,
        physical_lsw: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_arp_info_get.md")]
    #[link_name = "_nx_arp_info_get"]
    pub fn nx_arp_info_get(
        ip_ptr: *mut NX_IP,
        arp_requests_sent: *mut ULONG,
        arp_requests_received: *mut ULONG,
        arp_responses_sent: *mut ULONG,
        arp_responses_received: *mut ULONG,
        arp_dynamic_entries: *mut ULONG,
        arp_static_entries: *mut ULONG,
        arp_aged_entries: *mut ULONG,
        arp_invalid_messages: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_arp_ip_address_find.md")]
    #[link_name = "_nx_arp_ip_address_find"]
    pub fn nx_arp_ip_address_find(
        ip_ptr: *mut NX_IP,
        ip_address: *mut ULONG,
        physical_msw: ULONG,
        physical_lsw: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_arp_static_entries_delete.md")]
    #[link_name = "_nx_arp_static_entries_delete"]
    pub fn nx_arp_static_entries_delete(ip_ptr: *mut NX_IP) -> UINT;

    #[doc = include_str!("docs/nx_arp_static_entry_create.md")]
    #[link_name = "_nx_arp_static_entry_create"]
    pub fn nx_arp_static_entry_create(
        ip_ptr: *mut NX_IP,
        ip_address: ULONG,
        physical_msw: ULONG,
        physical_lsw: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_arp_static_entry_delete.md")]
    #[link_name = "_nx_arp_static_entry_delete"]
    pub fn nx_arp_static_entry_delete(
        ip_ptr: *mut NX_IP,
        ip_address: ULONG,
        physical_msw: ULONG,
        physical_lsw: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_icmp_enable.md")]
    #[link_name = "_nx_icmp_enable"]
    pub fn nx_icmp_enable(ip_ptr: *mut NX_IP) -> UINT;

    #[doc = include_str!("docs/nx_icmp_info_get.md")]
    #[link_name = "_nx_icmp_info_get"]
    pub fn nx_icmp_info_get(
        ip_ptr: *mut NX_IP,
        pings_sent: *mut ULONG,
        ping_timeouts: *mut ULONG,
        ping_threads_suspended: *mut ULONG,
        ping_responses_received: *mut ULONG,
        icmp_checksum_errors: *mut ULONG,
        icmp_unhandled_messages: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_icmp_ping.md")]
    #[link_name = "_nx_icmp_ping"]
    pub fn nx_icmp_ping(
        ip_ptr: *mut NX_IP,
        ip_address: ULONG,
        data: *mut CHAR,
        data_size: ULONG,
        response_ptr: *mut *mut NX_PACKET,
        wait_option: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_igmp_enable.md")]
    #[link_name = "_nx_igmp_enable"]
    pub fn nx_igmp_enable(ip_ptr: *mut NX_IP) -> UINT;

    #[doc = include_str!("docs/nx_igmp_info_get.md")]
    #[link_name = "_nx_igmp_info_get"]
    pub fn nx_igmp_info_get(
        ip_ptr: *mut NX_IP,
        igmp_reports_sent: *mut ULONG,
        igmp_queries_received: *mut ULONG,
        igmp_checksum_errors: *mut ULONG,
        current_groups_joined: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_igmp_loopback_disable.md")]
    #[link_name = "_nx_igmp_loopback_disable"]
    pub fn nx_igmp_loopback_disable(ip_ptr: *mut NX_IP) -> UINT;

    #[doc = include_str!("docs/nx_igmp_loopback_enable.md")]
    #[link_name = "_nx_igmp_loopback_enable"]
    pub fn nx_igmp_loopback_enable(ip_ptr: *mut NX_IP) -> UINT;

    #[doc = include_str!("docs/nx_igmp_multicast_interface_join.md")]
    #[link_name = "_nx_igmp_multicast_interface_join"]
    pub fn nx_igmp_multicast_interface_join(
        ip_ptr: *mut NX_IP,
        group_address: ULONG,
        interface_index: UINT,
    ) -> UINT;

    #[doc = include_str!("docs/nx_igmp_multicast_interface_leave.md")]
    #[link_name = "_nx_igmp_multicast_interface_leave"]
    pub fn nx_igmp_multicast_interface_leave(
        ip_ptr: *mut NX_IP,
        group_address: ULONG,
        interface_index: UINT,
    ) -> UINT;

    #[doc = include_str!("docs/nx_igmp_multicast_join.md")]
    #[link_name = "_nx_igmp_multicast_join"]
    pub fn nx_igmp_multicast_join(ip_ptr: *mut NX_IP, group_address: ULONG) -> UINT;

    #[doc = include_str!("docs/nx_igmp_multicast_leave.md")]
    #[link_name = "_nx_igmp_multicast_leave"]
    pub fn nx_igmp_multicast_leave(ip_ptr: *mut NX_IP, group_address: ULONG) -> UINT;

    #[doc = include_str!("docs/nx_ip_address_change_notify.md")]
    #[link_name = "_nx_ip_address_change_notify"]
    pub fn nx_ip_address_change_notify(
        ip_ptr: *mut NX_IP,
        ip_address_change_notify: ::core::option::Option<
            unsafe extern "C" fn(arg1: *mut NX_IP, arg2: *mut ::core::ffi::c_void),
        >,
        additional_info: *mut ::core::ffi::c_void,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_address_get.md")]
    #[link_name = "_nx_ip_address_get"]
    pub fn nx_ip_address_get(
        ip_ptr: *mut NX_IP,
        ip_address: *mut ULONG,
        network_mask: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_address_set.md")]
    #[link_name = "_nx_ip_address_set"]
    pub fn nx_ip_address_set(ip_ptr: *mut NX_IP, ip_address: ULONG, network_mask: ULONG) -> UINT;

    #[doc = include_str!("docs/nx_ip_auxiliary_packet_pool_set.md")]
    #[link_name = "_nx_ip_auxiliary_packet_pool_set"]
    pub fn nx_ip_auxiliary_packet_pool_set(
        ip_ptr: *mut NX_IP,
        auxiliary_pool: *mut NX_PACKET_POOL,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_create.md")]
    #[link_name = "_nx_ip_create"]
    pub fn nx_ip_create(
        ip_ptr: *mut NX_IP,
        name: *mut CHAR,
        ip_address: ULONG,
        network_mask: ULONG,
        default_pool: *mut NX_PACKET_POOL,
        ip_link_driver: ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP_DRIVER)>,
        memory_ptr: *mut ::core::ffi::c_void,
        memory_size: ULONG,
        priority: UINT,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_delete.md")]
    #[link_name = "_nx_ip_delete"]
    pub fn nx_ip_delete(ip_ptr: *mut NX_IP) -> UINT;

    #[doc = include_str!("docs/nx_ip_driver_direct_command.md")]
    #[link_name = "_nx_ip_driver_direct_command"]
    pub fn nx_ip_driver_direct_command(
        ip_ptr: *mut NX_IP,
        command: UINT,
        return_value_ptr: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_driver_interface_direct_command.md")]
    #[link_name = "_nx_ip_driver_interface_direct_command"]
    pub fn nx_ip_driver_interface_direct_command(
        ip_ptr: *mut NX_IP,
        command: UINT,
        interface_index: UINT,
        return_value_ptr: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_forwarding_disable.md")]
    #[link_name = "_nx_ip_forwarding_disable"]
    pub fn nx_ip_forwarding_disable(ip_ptr: *mut NX_IP) -> UINT;

    #[doc = include_str!("docs/nx_ip_forwarding_enable.md")]
    #[link_name = "_nx_ip_forwarding_enable"]
    pub fn nx_ip_forwarding_enable(ip_ptr: *mut NX_IP) -> UINT;

    #[doc = include_str!("docs/nx_ip_fragment_disable.md")]
    #[link_name = "_nx_ip_fragment_disable"]
    pub fn nx_ip_fragment_disable(ip_ptr: *mut NX_IP) -> UINT;

    #[doc = include_str!("docs/nx_ip_fragment_enable.md")]
    #[link_name = "_nx_ip_fragment_enable"]
    pub fn nx_ip_fragment_enable(ip_ptr: *mut NX_IP) -> UINT;

    #[doc = include_str!("docs/nx_ip_gateway_address_clear.md")]
    #[link_name = "_nx_ip_gateway_address_clear"]
    pub fn nx_ip_gateway_address_clear(ip_ptr: *mut NX_IP) -> UINT;

    #[doc = include_str!("docs/nx_ip_gateway_address_get.md")]
    #[link_name = "_nx_ip_gateway_address_get"]
    pub fn nx_ip_gateway_address_get(ip_ptr: *mut NX_IP, ip_address: *mut ULONG) -> UINT;

    #[doc = include_str!("docs/nx_ip_gateway_address_set.md")]
    #[link_name = "_nx_ip_gateway_address_set"]
    pub fn nx_ip_gateway_address_set(ip_ptr: *mut NX_IP, ip_address: ULONG) -> UINT;

    #[doc = include_str!("docs/nx_ip_info_get.md")]
    #[link_name = "_nx_ip_info_get"]
    pub fn nx_ip_info_get(
        ip_ptr: *mut NX_IP,
        ip_total_packets_sent: *mut ULONG,
        ip_total_bytes_sent: *mut ULONG,
        ip_total_packets_received: *mut ULONG,
        ip_total_bytes_received: *mut ULONG,
        ip_invalid_packets: *mut ULONG,
        ip_receive_packets_dropped: *mut ULONG,
        ip_receive_checksum_errors: *mut ULONG,
        ip_send_packets_dropped: *mut ULONG,
        ip_total_fragments_sent: *mut ULONG,
        ip_total_fragments_received: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_interface_address_get.md")]
    #[link_name = "_nx_ip_interface_address_get"]
    pub fn nx_ip_interface_address_get(
        ip_ptr: *mut NX_IP,
        interface_index: UINT,
        ip_address: *mut ULONG,
        network_mask: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_interface_address_mapping_configure.md")]
    #[link_name = "_nx_ip_interface_address_mapping_configure"]
    pub fn nx_ip_interface_address_mapping_configure(
        ip_ptr: *mut NX_IP,
        interface_index: UINT,
        mapping_needed: UINT,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_interface_address_set.md")]
    #[link_name = "_nx_ip_interface_address_set"]
    pub fn nx_ip_interface_address_set(
        ip_ptr: *mut NX_IP,
        interface_index: UINT,
        ip_address: ULONG,
        network_mask: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_interface_attach.md")]
    #[link_name = "_nx_ip_interface_attach"]
    pub fn nx_ip_interface_attach(
        ip_ptr: *mut NX_IP,
        interface_name: *mut CHAR,
        ip_address: ULONG,
        network_mask: ULONG,
        ip_link_driver: ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP_DRIVER)>,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_interface_capability_get.md")]
    #[link_name = "_nx_ip_interface_capability_get"]
    pub fn nx_ip_interface_capability_get(
        ip_ptr: *mut NX_IP,
        interface_index: UINT,
        interface_capability_flag: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_interface_capability_set.md")]
    #[link_name = "_nx_ip_interface_capability_set"]
    pub fn nx_ip_interface_capability_set(
        ip_ptr: *mut NX_IP,
        interface_index: UINT,
        interface_capability_flag: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_interface_detach.md")]
    #[link_name = "_nx_ip_interface_detach"]
    pub fn nx_ip_interface_detach(ip_ptr: *mut NX_IP, index: UINT) -> UINT;

    #[doc = include_str!("docs/nx_ip_interface_info_get.md")]
    #[link_name = "_nx_ip_interface_info_get"]
    pub fn nx_ip_interface_info_get(
        ip_ptr: *mut NX_IP,
        interface_index: UINT,
        interface_name: *mut *mut CHAR,
        ip_address: *mut ULONG,
        network_mask: *mut ULONG,
        mtu_size: *mut ULONG,
        physical_address_msw: *mut ULONG,
        physical_address_lsw: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_interface_mtu_set.md")]
    #[link_name = "_nx_ip_interface_mtu_set"]
    pub fn nx_ip_interface_mtu_set(
        ip_ptr: *mut NX_IP,
        interface_index: UINT,
        mtu_size: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_interface_physical_address_get.md")]
    #[link_name = "_nx_ip_interface_physical_address_get"]
    pub fn nx_ip_interface_physical_address_get(
        ip_ptr: *mut NX_IP,
        interface_index: UINT,
        physical_msw: *mut ULONG,
        physical_lsw: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_interface_physical_address_set.md")]
    #[link_name = "_nx_ip_interface_physical_address_set"]
    pub fn nx_ip_interface_physical_address_set(
        ip_ptr: *mut NX_IP,
        interface_index: UINT,
        physical_msw: ULONG,
        physical_lsw: ULONG,
        update_driver: UINT,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_interface_status_check.md")]
    #[link_name = "_nx_ip_interface_status_check"]
    pub fn nx_ip_interface_status_check(
        ip_ptr: *mut NX_IP,
        interface_index: UINT,
        needed_status: ULONG,
        actual_status: *mut ULONG,
        wait_option: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_link_status_change_notify_set.md")]
    #[link_name = "_nx_ip_link_status_change_notify_set"]
    pub fn nx_ip_link_status_change_notify_set(
        ip_ptr: *mut NX_IP,
        link_status_change_notify: ::core::option::Option<
            unsafe extern "C" fn(ip_ptr: *mut NX_IP, interface_index: UINT, link_up: UINT),
        >,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_max_payload_size_find.md")]
    #[link_name = "_nx_ip_max_payload_size_find"]
    pub fn nx_ip_max_payload_size_find(
        ip_ptr: *mut NX_IP,
        dest_address: *mut NXD_ADDRESS,
        if_index: UINT,
        src_port: UINT,
        dest_port: UINT,
        protocol: ULONG,
        start_offset_ptr: *mut ULONG,
        payload_length_ptr: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_status_check.md")]
    #[link_name = "_nx_ip_status_check"]
    pub fn nx_ip_status_check(
        ip_ptr: *mut NX_IP,
        needed_status: ULONG,
        actual_status: *mut ULONG,
        wait_option: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_static_route_add.md")]
    #[link_name = "_nx_ip_static_route_add"]
    pub fn nx_ip_static_route_add(
        ip_ptr: *mut NX_IP,
        network_address: ULONG,
        net_mask: ULONG,
        next_hop: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_static_route_delete.md")]
    #[link_name = "_nx_ip_static_route_delete"]
    pub fn nx_ip_static_route_delete(
        ip_ptr: *mut NX_IP,
        network_address: ULONG,
        net_mask: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ipv4_multicast_interface_join.md")]
    #[link_name = "_nx_ipv4_multicast_interface_join"]
    pub fn nx_ipv4_multicast_interface_join(
        ip_ptr: *mut NX_IP,
        group_address: ULONG,
        interface_index: UINT,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ipv4_multicast_interface_leave.md")]
    #[link_name = "_nx_ipv4_multicast_interface_leave"]
    pub fn nx_ipv4_multicast_interface_leave(
        ip_ptr: *mut NX_IP,
        group_address: ULONG,
        interface_index: UINT,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_raw_packet_disable.md")]
    #[link_name = "_nx_ip_raw_packet_disable"]
    pub fn nx_ip_raw_packet_disable(ip_ptr: *mut NX_IP) -> UINT;

    #[doc = include_str!("docs/nx_ip_raw_packet_enable.md")]
    #[link_name = "_nx_ip_raw_packet_enable"]
    pub fn nx_ip_raw_packet_enable(ip_ptr: *mut NX_IP) -> UINT;

    #[doc = include_str!("docs/nx_ip_raw_packet_filter_set.md")]
    #[link_name = "_nx_ip_raw_packet_filter_set"]
    pub fn nx_ip_raw_packet_filter_set(
        ip_ptr: *mut NX_IP,
        raw_packet_filter: ::core::option::Option<
            unsafe extern "C" fn(arg1: *mut NX_IP, arg2: ULONG, arg3: *mut NX_PACKET) -> UINT,
        >,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_raw_packet_receive.md")]
    #[link_name = "_nx_ip_raw_packet_receive"]
    pub fn nx_ip_raw_packet_receive(
        ip_ptr: *mut NX_IP,
        packet_ptr: *mut *mut NX_PACKET,
        wait_option: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_raw_packet_send.md")]
    #[link_name = "_nx_ip_raw_packet_send"]
    pub fn nx_ip_raw_packet_send(
        ip_ptr: *mut NX_IP,
        packet_ptr: *mut NX_PACKET,
        destination_ip: ULONG,
        type_of_service: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_raw_packet_source_send.md")]
    #[link_name = "_nx_ip_raw_packet_source_send"]
    pub fn nx_ip_raw_packet_source_send(
        ip_ptr: *mut NX_IP,
        packet_ptr: *mut NX_PACKET,
        destination_ip: ULONG,
        address_index: UINT,
        type_of_service: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_ip_raw_receive_queue_max_set.md")]
    #[link_name = "_nx_ip_raw_receive_queue_max_set"]
    pub fn nx_ip_raw_receive_queue_max_set(ip_ptr: *mut NX_IP, queue_max: ULONG) -> UINT;

    #[doc = include_str!("docs/nx_packet_allocate.md")]
    #[link_name = "_nx_packet_allocate"]
    pub fn nx_packet_allocate(
        pool_ptr: *mut NX_PACKET_POOL,
        packet_ptr: *mut *mut NX_PACKET,
        packet_type: ULONG,
        wait_option: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_packet_copy.md")]
    #[link_name = "_nx_packet_copy"]
    pub fn nx_packet_copy(
        packet_ptr: *mut NX_PACKET,
        new_packet_ptr: *mut *mut NX_PACKET,
        pool_ptr: *mut NX_PACKET_POOL,
        wait_option: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_packet_data_append.md")]
    #[link_name = "_nx_packet_data_append"]
    pub fn nx_packet_data_append(
        packet_ptr: *mut NX_PACKET,
        data_start: *mut ::core::ffi::c_void,
        data_size: ULONG,
        pool_ptr: *mut NX_PACKET_POOL,
        wait_option: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_packet_data_extract_offset.md")]
    #[link_name = "_nx_packet_data_extract_offset"]
    pub fn nx_packet_data_extract_offset(
        packet_ptr: *mut NX_PACKET,
        offset: ULONG,
        buffer_start: *mut ::core::ffi::c_void,
        buffer_length: ULONG,
        bytes_copied: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_packet_data_retrieve.md")]
    #[link_name = "_nx_packet_data_retrieve"]
    pub fn nx_packet_data_retrieve(
        packet_ptr: *mut NX_PACKET,
        buffer_start: *mut ::core::ffi::c_void,
        bytes_copied: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_packet_length_get.md")]
    #[link_name = "_nx_packet_length_get"]
    pub fn nx_packet_length_get(packet_ptr: *mut NX_PACKET, length: *mut ULONG) -> UINT;

    #[doc = include_str!("docs/nx_packet_pool_create.md")]
    #[link_name = "_nx_packet_pool_create"]
    pub fn nx_packet_pool_create(
        pool_ptr: *mut NX_PACKET_POOL,
        name: *mut CHAR,
        payload_size: ULONG,
        memory_ptr: *mut ::core::ffi::c_void,
        memory_size: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_packet_pool_delete.md")]
    #[link_name = "_nx_packet_pool_delete"]
    pub fn nx_packet_pool_delete(pool_ptr: *mut NX_PACKET_POOL) -> UINT;

    #[doc = include_str!("docs/nx_packet_pool_info_get.md")]
    #[link_name = "_nx_packet_pool_info_get"]
    pub fn nx_packet_pool_info_get(
        pool_ptr: *mut NX_PACKET_POOL,
        total_packets: *mut ULONG,
        free_packets: *mut ULONG,
        empty_pool_requests: *mut ULONG,
        empty_pool_suspensions: *mut ULONG,
        invalid_packet_releases: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_packet_pool_low_watermark_set.md")]
    #[link_name = "_nx_packet_pool_low_watermark_set"]
    pub fn nx_packet_pool_low_watermark_set(
        pool_ptr: *mut NX_PACKET_POOL,
        low_water_mark: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_packet_release.md")]
    #[link_name = "_nx_packet_release"]
    pub fn nx_packet_release(packet_ptr: *mut NX_PACKET) -> UINT;

    #[doc = include_str!("docs/nx_packet_transmit_release.md")]
    #[link_name = "_nx_packet_transmit_release"]
    pub fn nx_packet_transmit_release(packet_ptr: *mut NX_PACKET) -> UINT;

    /// This service isn't documented in rtos-docs.
    #[link_name = "_nx_packet_vlan_priority_set"]
    pub fn nx_packet_vlan_priority_set(packet_ptr: *mut NX_PACKET, vlan_priority: UINT) -> UINT;

    #[doc = include_str!("docs/nx_rarp_disable.md")]
    #[link_name = "_nx_rarp_disable"]
    pub fn nx_rarp_disable(ip_ptr: *mut NX_IP) -> UINT;

    #[doc = include_str!("docs/nx_rarp_enable.md")]
    #[link_name = "_nx_rarp_enable"]
    pub fn nx_rarp_enable(ip_ptr: *mut NX_IP) -> UINT;

    #[doc = include_str!("docs/nx_rarp_info_get.md")]
    #[link_name = "_nx_rarp_info_get"]
    pub fn nx_rarp_info_get(
        ip_ptr: *mut NX_IP,
        rarp_requests_sent: *mut ULONG,
        rarp_responses_received: *mut ULONG,
        rarp_invalid_messages: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_tcp_client_socket_bind.md")]
    #[link_name = "_nx_tcp_client_socket_bind"]
    pub fn nx_tcp_client_socket_bind(
        socket_ptr: *mut NX_TCP_SOCKET,
        port: UINT,
        wait_option: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_tcp_client_socket_connect.md")]
    #[link_name = "_nx_tcp_client_socket_connect"]
    pub fn nx_tcp_client_socket_connect(
        socket_ptr: *mut NX_TCP_SOCKET,
        server_ip: ULONG,
        server_port: UINT,
        wait_option: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_tcp_client_socket_port_get.md")]
    #[link_name = "_nx_tcp_client_socket_port_get"]
    pub fn nx_tcp_client_socket_port_get(
        socket_ptr: *mut NX_TCP_SOCKET,
        port_ptr: *mut UINT,
    ) -> UINT;

    #[doc = include_str!("docs/nx_tcp_client_socket_unbind.md")]
    #[link_name = "_nx_tcp_client_socket_unbind"]
    pub fn nx_tcp_client_socket_unbind(socket_ptr: *mut NX_TCP_SOCKET) -> UINT;

    #[doc = include_str!("docs/nx_tcp_enable.md")]
    #[link_name = "_nx_tcp_enable"]
    pub fn nx_tcp_enable(ip_ptr: *mut NX_IP) -> UINT;

    #[doc = include_str!("docs/nx_tcp_free_port_find.md")]
    #[link_name = "_nx_tcp_free_port_find"]
    pub fn nx_tcp_free_port_find(ip_ptr: *mut NX_IP, port: UINT, free_port_ptr: *mut UINT) -> UINT;

    #[doc = include_str!("docs/nx_tcp_info_get.md")]
    #[link_name = "_nx_tcp_info_get"]
    pub fn nx_tcp_info_get(
        ip_ptr: *mut NX_IP,
        tcp_packets_sent: *mut ULONG,
        tcp_bytes_sent: *mut ULONG,
        tcp_packets_received: *mut ULONG,
        tcp_bytes_received: *mut ULONG,
        tcp_invalid_packets: *mut ULONG,
        tcp_receive_packets_dropped: *mut ULONG,
        tcp_checksum_errors: *mut ULONG,
        tcp_connections: *mut ULONG,
        tcp_disconnections: *mut ULONG,
        tcp_connections_dropped: *mut ULONG,
        tcp_retransmit_packets: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_tcp_server_socket_accept.md")]
    #[link_name = "_nx_tcp_server_socket_accept"]
    pub fn nx_tcp_server_socket_accept(socket_ptr: *mut NX_TCP_SOCKET, wait_option: ULONG) -> UINT;

    #[doc = include_str!("docs/nx_tcp_server_socket_listen.md")]
    #[link_name = "_nx_tcp_server_socket_listen"]
    pub fn nx_tcp_server_socket_listen(
        ip_ptr: *mut NX_IP,
        port: UINT,
        socket_ptr: *mut NX_TCP_SOCKET,
        listen_queue_size: UINT,
        tcp_listen_callback: ::core::option::Option<
            unsafe extern "C" fn(socket_ptr: *mut NX_TCP_SOCKET, port: UINT),
        >,
    ) -> UINT;

    #[doc = include_str!("docs/nx_tcp_server_socket_relisten.md")]
    #[link_name = "_nx_tcp_server_socket_relisten"]
    pub fn nx_tcp_server_socket_relisten(
        ip_ptr: *mut NX_IP,
        port: UINT,
        socket_ptr: *mut NX_TCP_SOCKET,
    ) -> UINT;

    #[doc = include_str!("docs/nx_tcp_server_socket_unaccept.md")]
    #[link_name = "_nx_tcp_server_socket_unaccept"]
    pub fn nx_tcp_server_socket_unaccept(socket_ptr: *mut NX_TCP_SOCKET) -> UINT;

    #[doc = include_str!("docs/nx_tcp_server_socket_unlisten.md")]
    #[link_name = "_nx_tcp_server_socket_unlisten"]
    pub fn nx_tcp_server_socket_unlisten(ip_ptr: *mut NX_IP, port: UINT) -> UINT;

    #[doc = include_str!("docs/nx_tcp_socket_bytes_available.md")]
    #[link_name = "_nx_tcp_socket_bytes_available"]
    pub fn nx_tcp_socket_bytes_available(
        socket_ptr: *mut NX_TCP_SOCKET,
        bytes_available: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_tcp_socket_create.md")]
    #[link_name = "_nx_tcp_socket_create"]
    pub fn nx_tcp_socket_create(
        ip_ptr: *mut NX_IP,
        socket_ptr: *mut NX_TCP_SOCKET,
        name: *mut CHAR,
        type_of_service: ULONG,
        fragment: ULONG,
        time_to_live: UINT,
        window_size: ULONG,
        tcp_urgent_data_callback: ::core::option::Option<
            unsafe extern "C" fn(socket_ptr: *mut NX_TCP_SOCKET),
        >,
        tcp_disconnect_callback: ::core::option::Option<
            unsafe extern "C" fn(socket_ptr: *mut NX_TCP_SOCKET),
        >,
    ) -> UINT;

    #[doc = include_str!("docs/nx_tcp_socket_delete.md")]
    #[link_name = "_nx_tcp_socket_delete"]
    pub fn nx_tcp_socket_delete(socket_ptr: *mut NX_TCP_SOCKET) -> UINT;

    #[doc = include_str!("docs/nx_tcp_socket_disconnect.md")]
    #[link_name = "_nx_tcp_socket_disconnect"]
    pub fn nx_tcp_socket_disconnect(socket_ptr: *mut NX_TCP_SOCKET, wait_option: ULONG) -> UINT;

    #[doc = include_str!("docs/nx_tcp_socket_disconnect_complete_notify.md")]
    #[link_name = "_nx_tcp_socket_disconnect_complete_notify"]
    pub fn nx_tcp_socket_disconnect_complete_notify(
        socket_ptr: *mut NX_TCP_SOCKET,
        tcp_disconnect_complete_notify: ::core::option::Option<
            unsafe extern "C" fn(arg1: *mut NX_TCP_SOCKET),
        >,
    ) -> UINT;

    #[doc = include_str!("docs/nx_tcp_socket_establish_notify.md")]
    #[link_name = "_nx_tcp_socket_establish_notify"]
    pub fn nx_tcp_socket_establish_notify(
        socket_ptr: *mut NX_TCP_SOCKET,
        tcp_establish_notify: ::core::option::Option<
            unsafe extern "C" fn(arg1: *mut NX_TCP_SOCKET),
        >,
    ) -> UINT;

    #[doc = include_str!("docs/nx_tcp_socket_info_get.md")]
    #[link_name = "_nx_tcp_socket_info_get"]
    pub fn nx_tcp_socket_info_get(
        socket_ptr: *mut NX_TCP_SOCKET,
        tcp_packets_sent: *mut ULONG,
        tcp_bytes_sent: *mut ULONG,
        tcp_packets_received: *mut ULONG,
        tcp_bytes_received: *mut ULONG,
        tcp_retransmit_packets: *mut ULONG,
        tcp_packets_queued: *mut ULONG,
        tcp_checksum_errors: *mut ULONG,
        tcp_socket_state: *mut ULONG,
        tcp_transmit_queue_depth: *mut ULONG,
        tcp_transmit_window: *mut ULONG,
        tcp_receive_window: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_tcp_socket_mss_get.md")]
    #[link_name = "_nx_tcp_socket_mss_get"]
    pub fn nx_tcp_socket_mss_get(socket_ptr: *mut NX_TCP_SOCKET, mss: *mut ULONG) -> UINT;

    #[doc = include_str!("docs/nx_tcp_socket_mss_peer_get.md")]
    #[link_name = "_nx_tcp_socket_mss_peer_get"]
    pub fn nx_tcp_socket_mss_peer_get(socket_ptr: *mut NX_TCP_SOCKET, peer_mss: *mut ULONG)
    -> UINT;

    #[doc = include_str!("docs/nx_tcp_socket_mss_set.md")]
    #[link_name = "_nx_tcp_socket_mss_set"]
    pub fn nx_tcp_socket_mss_set(socket_ptr: *mut NX_TCP_SOCKET, mss: ULONG) -> UINT;

    #[doc = include_str!("docs/nx_tcp_socket_peer_info_get.md")]
    #[link_name = "_nx_tcp_socket_peer_info_get"]
    pub fn nx_tcp_socket_peer_info_get(
        socket_ptr: *mut NX_TCP_SOCKET,
        peer_ip_address: *mut ULONG,
        peer_port: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_tcp_socket_queue_depth_notify_set.md")]
    #[link_name = "_nx_tcp_socket_queue_depth_notify_set"]
    pub fn nx_tcp_socket_queue_depth_notify_set(
        socket_ptr: *mut NX_TCP_SOCKET,
        tcp_socket_queue_depth_notify: ::core::option::Option<
            unsafe extern "C" fn(arg1: *mut NX_TCP_SOCKET),
        >,
    ) -> UINT;

    #[doc = include_str!("docs/nx_tcp_socket_receive.md")]
    #[link_name = "_nx_tcp_socket_receive"]
    pub fn nx_tcp_socket_receive(
        socket_ptr: *mut NX_TCP_SOCKET,
        packet_ptr: *mut *mut NX_PACKET,
        wait_option: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_tcp_socket_receive_notify.md")]
    #[link_name = "_nx_tcp_socket_receive_notify"]
    pub fn nx_tcp_socket_receive_notify(
        socket_ptr: *mut NX_TCP_SOCKET,
        tcp_receive_notify: ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_TCP_SOCKET)>,
    ) -> UINT;

    /// This service isn't documented in rtos-docs.
    #[link_name = "_nx_tcp_socket_receive_queue_max_set"]
    pub fn nx_tcp_socket_receive_queue_max_set(
        socket_ptr: *mut NX_TCP_SOCKET,
        receive_queue_maximum: UINT,
    ) -> UINT;

    #[doc = include_str!("docs/nx_tcp_socket_send.md")]
    #[link_name = "_nx_tcp_socket_send"]
    pub fn nx_tcp_socket_send(
        socket_ptr: *mut NX_TCP_SOCKET,
        packet_ptr: *mut NX_PACKET,
        wait_option: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_tcp_socket_state_wait.md")]
    #[link_name = "_nx_tcp_socket_state_wait"]
    pub fn nx_tcp_socket_state_wait(
        socket_ptr: *mut NX_TCP_SOCKET,
        desired_state: UINT,
        wait_option: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_tcp_socket_timed_wait_callback.md")]
    #[link_name = "_nx_tcp_socket_timed_wait_callback"]
    pub fn nx_tcp_socket_timed_wait_callback(
        socket_ptr: *mut NX_TCP_SOCKET,
        tcp_timed_wait_callback: ::core::option::Option<
            unsafe extern "C" fn(arg1: *mut NX_TCP_SOCKET),
        >,
    ) -> UINT;

    #[doc = include_str!("docs/nx_tcp_socket_transmit_configure.md")]
    #[link_name = "_nx_tcp_socket_transmit_configure"]
    pub fn nx_tcp_socket_transmit_configure(
        socket_ptr: *mut NX_TCP_SOCKET,
        max_queue_depth: ULONG,
        timeout: ULONG,
        max_retries: ULONG,
        timeout_shift: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_tcp_socket_window_update_notify_set.md")]
    #[link_name = "_nx_tcp_socket_window_update_notify_set"]
    pub fn nx_tcp_socket_window_update_notify_set(
        socket_ptr: *mut NX_TCP_SOCKET,
        tcp_window_update_notify: ::core::option::Option<
            unsafe extern "C" fn(arg1: *mut NX_TCP_SOCKET),
        >,
    ) -> UINT;

    /// This service isn't documented in rtos-docs.
    #[link_name = "_nx_tcp_socket_vlan_priority_set"]
    pub fn nx_tcp_socket_vlan_priority_set(
        socket_ptr: *mut NX_TCP_SOCKET,
        vlan_priority: UINT,
    ) -> UINT;

    #[doc = include_str!("docs/nx_udp_enable.md")]
    #[link_name = "_nx_udp_enable"]
    pub fn nx_udp_enable(ip_ptr: *mut NX_IP) -> UINT;

    #[doc = include_str!("docs/nx_udp_free_port_find.md")]
    #[link_name = "_nx_udp_free_port_find"]
    pub fn nx_udp_free_port_find(ip_ptr: *mut NX_IP, port: UINT, free_port_ptr: *mut UINT) -> UINT;

    #[doc = include_str!("docs/nx_udp_info_get.md")]
    #[link_name = "_nx_udp_info_get"]
    pub fn nx_udp_info_get(
        ip_ptr: *mut NX_IP,
        udp_packets_sent: *mut ULONG,
        udp_bytes_sent: *mut ULONG,
        udp_packets_received: *mut ULONG,
        udp_bytes_received: *mut ULONG,
        udp_invalid_packets: *mut ULONG,
        udp_receive_packets_dropped: *mut ULONG,
        udp_checksum_errors: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_udp_packet_info_extract.md")]
    #[link_name = "_nx_udp_packet_info_extract"]
    pub fn nx_udp_packet_info_extract(
        packet_ptr: *mut NX_PACKET,
        ip_address: *mut ULONG,
        protocol: *mut UINT,
        port: *mut UINT,
        interface_index: *mut UINT,
    ) -> UINT;

    #[doc = include_str!("docs/nx_udp_socket_bind.md")]
    #[link_name = "_nx_udp_socket_bind"]
    pub fn nx_udp_socket_bind(
        socket_ptr: *mut NX_UDP_SOCKET,
        port: UINT,
        wait_option: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_udp_socket_bytes_available.md")]
    #[link_name = "_nx_udp_socket_bytes_available"]
    pub fn nx_udp_socket_bytes_available(
        socket_ptr: *mut NX_UDP_SOCKET,
        bytes_available: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_udp_socket_checksum_disable.md")]
    #[link_name = "_nx_udp_socket_checksum_disable"]
    pub fn nx_udp_socket_checksum_disable(socket_ptr: *mut NX_UDP_SOCKET) -> UINT;

    #[doc = include_str!("docs/nx_udp_socket_checksum_enable.md")]
    #[link_name = "_nx_udp_socket_checksum_enable"]
    pub fn nx_udp_socket_checksum_enable(socket_ptr: *mut NX_UDP_SOCKET) -> UINT;

    #[doc = include_str!("docs/nx_udp_socket_create.md")]
    #[link_name = "_nx_udp_socket_create"]
    pub fn nx_udp_socket_create(
        ip_ptr: *mut NX_IP,
        socket_ptr: *mut NX_UDP_SOCKET,
        name: *mut CHAR,
        type_of_service: ULONG,
        fragment: ULONG,
        time_to_live: UINT,
        queue_maximum: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_udp_socket_delete.md")]
    #[link_name = "_nx_udp_socket_delete"]
    pub fn nx_udp_socket_delete(socket_ptr: *mut NX_UDP_SOCKET) -> UINT;

    #[doc = include_str!("docs/nx_udp_socket_info_get.md")]
    #[link_name = "_nx_udp_socket_info_get"]
    pub fn nx_udp_socket_info_get(
        socket_ptr: *mut NX_UDP_SOCKET,
        udp_packets_sent: *mut ULONG,
        udp_bytes_sent: *mut ULONG,
        udp_packets_received: *mut ULONG,
        udp_bytes_received: *mut ULONG,
        udp_packets_queued: *mut ULONG,
        udp_receive_packets_dropped: *mut ULONG,
        udp_checksum_errors: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_udp_socket_port_get.md")]
    #[link_name = "_nx_udp_socket_port_get"]
    pub fn nx_udp_socket_port_get(socket_ptr: *mut NX_UDP_SOCKET, port_ptr: *mut UINT) -> UINT;

    #[doc = include_str!("docs/nx_udp_socket_receive.md")]
    #[link_name = "_nx_udp_socket_receive"]
    pub fn nx_udp_socket_receive(
        socket_ptr: *mut NX_UDP_SOCKET,
        packet_ptr: *mut *mut NX_PACKET,
        wait_option: ULONG,
    ) -> UINT;

    #[doc = include_str!("docs/nx_udp_socket_receive_notify.md")]
    #[link_name = "_nx_udp_socket_receive_notify"]
    pub fn nx_udp_socket_receive_notify(
        socket_ptr: *mut NX_UDP_SOCKET,
        udp_receive_notify: ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_UDP_SOCKET)>,
    ) -> UINT;

    #[doc = include_str!("docs/nx_udp_socket_unbind.md")]
    #[link_name = "_nx_udp_socket_unbind"]
    pub fn nx_udp_socket_unbind(socket_ptr: *mut NX_UDP_SOCKET) -> UINT;

    #[doc = include_str!("docs/nx_udp_source_extract.md")]
    #[link_name = "_nx_udp_source_extract"]
    pub fn nx_udp_source_extract(
        packet_ptr: *mut NX_PACKET,
        ip_address: *mut ULONG,
        port: *mut UINT,
    ) -> UINT;

    /// This service isn't documented in rtos-docs.
    #[link_name = "_nx_udp_socket_vlan_priority_set"]
    pub fn nx_udp_socket_vlan_priority_set(
        socket_ptr: *mut NX_UDP_SOCKET,
        vlan_priority: UINT,
    ) -> UINT;

    #[doc = include_str!("docs/nx_udp_socket_send.md")]
    #[link_name = "_nx_udp_socket_send"]
    pub fn nx_udp_socket_send(
        socket_ptr: *mut NX_UDP_SOCKET,
        packet_ptr: *mut NX_PACKET,
        ip_address: ULONG,
        port: UINT,
    ) -> UINT;

    #[doc = include_str!("docs/nx_udp_socket_source_send.md")]
    #[link_name = "_nx_udp_socket_source_send"]
    pub fn nx_udp_socket_source_send(
        socket_ptr: *mut NX_UDP_SOCKET,
        packet_ptr: *mut NX_PACKET,
        ip_address: ULONG,
        port: UINT,
        address_index: UINT,
    ) -> UINT;
}

unsafe extern "C" {
    pub fn _nx_ip_driver_deferred_enable(
        ip_ptr: *mut NX_IP,
        driver_deferred_packet_handler: ::core::option::Option<
            unsafe extern "C" fn(ip_ptr: *mut NX_IP, packet_ptr: *mut NX_PACKET),
        >,
    );

    pub fn _nx_ip_driver_deferred_receive(ip_ptr: *mut NX_IP, packet_ptr: *mut NX_PACKET);

    pub fn _nx_ip_driver_deferred_processing(ip_ptr: *mut NX_IP);

    pub fn _nx_ip_packet_deferred_receive(ip_ptr: *mut NX_IP, packet_ptr: *mut NX_PACKET);

    pub fn _nx_arp_packet_deferred_receive(ip_ptr: *mut NX_IP, packet_ptr: *mut NX_PACKET);

    pub fn _nx_rarp_packet_deferred_receive(ip_ptr: *mut NX_IP, packet_ptr: *mut NX_PACKET);

    pub fn _nx_ip_packet_receive(ip_ptr: *mut NX_IP, packet_ptr: *mut NX_PACKET);

    pub fn _nx_ip_driver_link_status_event(ip_ptr: *mut NX_IP, interface_index: UINT);
}
