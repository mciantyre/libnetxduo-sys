pub(crate) mod control_blocks {
    use crate::control_blocks::*;
    use crate::tx::*;

    #[repr(C)]
    struct NX_DHCP_INTERFACE_RECORD {
        nx_dhcp_record_valid: UCHAR,
        nx_dhcp_state: UCHAR,
        nx_dhcp_user_option: UCHAR,
        reserved: UCHAR,
        nx_dhcp_xid: ULONG,
        nx_dhcp_seconds: ULONG,
        nx_dhcp_ip_address: ULONG,
        nx_dhcp_gateway_address: ULONG,
        nx_dhcp_server_ip: ULONG,
        nx_dhcp_network_mask: ULONG,
        nx_dhcp_interface_index: UINT,
        nx_dhcp_timeout: ULONG,
        nx_dhcp_rtr_interval: ULONG,
        nx_dhcp_lease_remain_time: ULONG,
        nx_dhcp_lease_time: ULONG,
        nx_dhcp_renewal_time: ULONG,
        nx_dhcp_rebind_time: ULONG,
        nx_dhcp_renewal_remain_time: ULONG,
        nx_dhcp_rebind_remain_time: ULONG,
        nx_dhcp_clear_broadcast: UINT,
        nx_dhcp_skip_discovery: UINT,
        nx_dhcp_options_buffer: [UCHAR; 312usize],
        nx_dhcp_options_size: UINT,
        nx_dhcp_internal_errors: ULONG,
        nx_dhcp_discoveries_sent: ULONG,
        nx_dhcp_offers_received: ULONG,
        nx_dhcp_requests_sent: ULONG,
        nx_dhcp_acks_received: ULONG,
        nx_dhcp_nacks_received: ULONG,
        nx_dhcp_releases_sent: ULONG,
        nx_dhcp_declines_sent: ULONG,
        nx_dhcp_force_renewal_rec: ULONG,
        nx_dhcp_informs_sent: ULONG,
        nx_dhcp_inform_responses: ULONG,
    }

    #[repr(C)]
    pub struct NX_DHCP {
        nx_dhcp_id: ULONG,
        nx_dhcp_name: *mut CHAR,
        nx_dhcp_ip_ptr: *mut NX_IP,
        nx_dhcp_pool: NX_PACKET_POOL,
        #[cfg(not(nx_dhcp_client_user_create_packet_pool))]
        nx_dhcp_pool_area: [UCHAR; 2960usize],
        #[cfg(not(nx_dhcp_client_user_create_packet_pool))]
        nx_dhcp_packet_pool_ptr: *mut NX_PACKET_POOL,
        nx_dhcp_socket: NX_UDP_SOCKET,
        nx_dhcp_thread: TX_THREAD,
        nx_dhcp_thread_stack: [UCHAR; 4096usize],
        nx_dhcp_mutex: TX_MUTEX,
        nx_dhcp_events: TX_EVENT_FLAGS_GROUP,
        nx_dhcp_timer: TX_TIMER,
        nx_dhcp_interface_record: [NX_DHCP_INTERFACE_RECORD; 1usize],
        nx_dhcp_user_request_parameter: [UCHAR; 4usize],
        nx_dhcp_user_request_parameter_size: UINT,
        nx_dhcp_state_change_callback:
            ::core::option::Option<unsafe extern "C" fn(dhcp_ptr: *mut NX_DHCP, new_state: UCHAR)>,
        nx_dhcp_interface_state_change_callback: ::core::option::Option<
            unsafe extern "C" fn(dhcp_ptr: *mut NX_DHCP, iface_index: UINT, new_state: UCHAR),
        >,
        nx_dhcp_user_option_add: ::core::option::Option<
            unsafe extern "C" fn(
                dhcp_ptr: *mut NX_DHCP,
                iface_index: UINT,
                message_type: UINT,
                user_option_ptr: *mut UCHAR,
                user_option_length: *mut UINT,
            ) -> UINT,
        >,
        nx_dhcp_created_next: *mut NX_DHCP,
        nx_dhcp_reserved_ptr: *mut ::core::ffi::c_void,
    }
}

pub(crate) mod error_checked {
    use super::control_blocks::*;
    use crate::control_blocks::*;
    use crate::tx::*;

    unsafe extern "C" {
        #[link_name = "_nxe_dhcp_create"]
        #[doc = include_str!("../docs/nx_dhcp_create.md")]
        pub fn nx_dhcp_create(
            dhcp_ptr: *mut NX_DHCP,
            ip_ptr: *mut NX_IP,
            name_ptr: *mut CHAR,
        ) -> UINT;

        #[link_name = "_nxe_dhcp_packet_pool_set"]
        #[doc = include_str!("../docs/nx_dhcp_packet_pool_set.md")]
        pub fn nx_dhcp_packet_pool_set(
            dhcp_ptr: *mut NX_DHCP,
            packet_pool_ptr: *mut NX_PACKET_POOL,
        ) -> UINT;

        #[link_name = "_nxe_dhcp_request_client_ip"]
        #[doc = include_str!("../docs/nx_dhcp_request_client_ip.md")]
        pub fn nx_dhcp_request_client_ip(
            dhcp_ptr: *mut NX_DHCP,
            client_ip_address: ULONG,
            skip_discover_message: UINT,
        ) -> UINT;

        #[link_name = "_nxe_dhcp_delete"]
        #[doc = include_str!("../docs/nx_dhcp_delete.md")]
        pub fn nx_dhcp_delete(dhcp_ptr: *mut NX_DHCP) -> UINT;

        #[link_name = "_nxe_dhcp_decline"]
        #[doc = include_str!("../docs/nx_dhcp_decline.md")]
        pub fn nx_dhcp_decline(dhcp_ptr: *mut NX_DHCP) -> UINT;

        #[link_name = "_nxe_dhcp_force_renew"]
        #[doc = include_str!("../docs/nx_dhcp_force_renew.md")]
        pub fn nx_dhcp_force_renew(dhcp_ptr: *mut NX_DHCP) -> UINT;

        #[link_name = "_nxe_dhcp_release"]
        #[doc = include_str!("../docs/nx_dhcp_release.md")]
        pub fn nx_dhcp_release(dhcp_ptr: *mut NX_DHCP) -> UINT;

        #[link_name = "_nxe_dhcp_start"]
        #[doc = include_str!("../docs/nx_dhcp_start.md")]
        pub fn nx_dhcp_start(dhcp_ptr: *mut NX_DHCP) -> UINT;

        #[link_name = "_nxe_dhcp_stop"]
        #[doc = include_str!("../docs/nx_dhcp_stop.md")]
        pub fn nx_dhcp_stop(dhcp_ptr: *mut NX_DHCP) -> UINT;

        #[link_name = "_nxe_dhcp_server_address_get"]
        #[doc = include_str!("../docs/nx_dhcp_server_address_get.md")]
        pub fn nx_dhcp_server_address_get(
            dhcp_ptr: *mut NX_DHCP,
            server_address: *mut ULONG,
        ) -> UINT;

        #[link_name = "_nxe_dhcp_state_change_notify"]
        #[doc = include_str!("../docs/nx_dhcp_state_change_notify.md")]
        pub fn nx_dhcp_state_change_notify(
            dhcp_ptr: *mut NX_DHCP,
            dhcp_state_change_notify: ::core::option::Option<
                unsafe extern "C" fn(dhcp_ptr: *mut NX_DHCP, new_state: UCHAR),
            >,
        ) -> UINT;

        #[link_name = "_nxe_dhcp_user_option_request"]
        pub fn nx_dhcp_user_option_request(dhcp_ptr: *mut NX_DHCP, option_code: UINT) -> UINT;

        #[link_name = "_nxe_dhcp_user_option_retrieve"]
        #[doc = include_str!("../docs/nx_dhcp_user_option_retrieve.md")]
        pub fn nx_dhcp_user_option_retrieve(
            dhcp_ptr: *mut NX_DHCP,
            request_option: UINT,
            destination_ptr: *mut UCHAR,
            destination_size: *mut UINT,
        ) -> UINT;

        #[link_name = "_nxe_dhcp_user_option_convert"]
        #[doc = include_str!("../docs/nx_dhcp_user_option_convert.md")]
        pub fn nx_dhcp_user_option_convert(source_ptr: *mut UCHAR) -> ULONG;

        #[link_name = "_nxe_dhcp_user_option_add_callback_set"]
        #[doc = include_str!("../docs/nx_dhcp_user_option_add_callback_set.md")]
        pub fn nx_dhcp_user_option_add_callback_set(
            dhcp_ptr: *mut NX_DHCP,
            dhcp_user_option_add: ::core::option::Option<
                unsafe extern "C" fn(
                    dhcp_ptr: *mut NX_DHCP,
                    iface_index: UINT,
                    message_type: UINT,
                    user_option_ptr: *mut UCHAR,
                    user_option_length: *mut UINT,
                ) -> UINT,
            >,
        ) -> UINT;

        #[link_name = "_nxe_dhcp_reinitialize"]
        #[doc = include_str!("../docs/nx_dhcp_reinitialize.md")]
        pub fn nx_dhcp_reinitialize(dhcp_ptr: *mut NX_DHCP) -> UINT;

        #[link_name = "_nxe_dhcp_send_request"]
        #[doc = include_str!("../docs/nx_dhcp_send_request.md")]
        pub fn nx_dhcp_send_request(dhcp_ptr: *mut NX_DHCP, dhcp_message_type: UINT) -> UINT;

        #[link_name = "_nxe_dhcp_set_interface_index"]
        #[doc = include_str!("../docs/nx_dhcp_set_interface_index.md")]
        pub fn nx_dhcp_set_interface_index(dhcp_ptr: *mut NX_DHCP, interface_index: UINT) -> UINT;

        #[link_name = "_nxe_dhcp_clear_broadcast_flag"]
        #[doc = include_str!("../docs/nx_dhcp_clear_broadcast_flag.md")]
        pub fn nx_dhcp_clear_broadcast_flag(dhcp_ptr: *mut NX_DHCP, clear_flag: UINT) -> UINT;

        #[link_name = "_nxe_dhcp_interface_clear_broadcast_flag"]
        #[doc = include_str!("../docs/nx_dhcp_interface_clear_broadcast_flag.md")]
        pub fn nx_dhcp_interface_clear_broadcast_flag(
            dhcp_ptr: *mut NX_DHCP,
            iface_index: UINT,
            clear_flag: UINT,
        ) -> UINT;

        #[link_name = "_nxe_dhcp_interface_enable"]
        #[doc = include_str!("../docs/nx_dhcp_interface_enable.md")]
        pub fn nx_dhcp_interface_enable(dhcp_ptr: *mut NX_DHCP, iface_index: UINT) -> UINT;

        #[link_name = "_nxe_dhcp_interface_disable"]
        #[doc = include_str!("../docs/nx_dhcp_interface_disable.md")]
        pub fn nx_dhcp_interface_disable(dhcp_ptr: *mut NX_DHCP, iface_index: UINT) -> UINT;

        #[link_name = "_nxe_dhcp_interface_decline"]
        #[doc = include_str!("../docs/nx_dhcp_interface_decline.md")]
        pub fn nx_dhcp_interface_decline(dhcp_ptr: *mut NX_DHCP, iface_index: UINT) -> UINT;

        #[link_name = "_nxe_dhcp_interface_force_renew"]
        #[doc = include_str!("../docs/nx_dhcp_interface_force_renew.md")]
        pub fn nx_dhcp_interface_force_renew(dhcp_ptr: *mut NX_DHCP, iface_index: UINT) -> UINT;

        #[link_name = "_nxe_dhcp_interface_release"]
        #[doc = include_str!("../docs/nx_dhcp_interface_release.md")]
        pub fn nx_dhcp_interface_release(dhcp_ptr: *mut NX_DHCP, iface_index: UINT) -> UINT;

        #[link_name = "_nxe_dhcp_interface_reinitialize"]
        #[doc = include_str!("../docs/nx_dhcp_interface_reinitialize.md")]
        pub fn nx_dhcp_interface_reinitialize(dhcp_ptr: *mut NX_DHCP, iface_index: UINT) -> UINT;

        #[link_name = "_nxe_dhcp_interface_request_client_ip"]
        #[doc = include_str!("../docs/nx_dhcp_interface_request_client_ip.md")]
        pub fn nx_dhcp_interface_request_client_ip(
            dhcp_ptr: *mut NX_DHCP,
            iface_index: UINT,
            client_ip_address: ULONG,
            skip_discover_message: UINT,
        ) -> UINT;

        #[link_name = "_nxe_dhcp_interface_start"]
        #[doc = include_str!("../docs/nx_dhcp_interface_start.md")]
        pub fn nx_dhcp_interface_start(dhcp_ptr: *mut NX_DHCP, iface_index: UINT) -> UINT;

        #[link_name = "_nxe_dhcp_interface_stop"]
        #[doc = include_str!("../docs/nx_dhcp_interface_stop.md")]
        pub fn nx_dhcp_interface_stop(dhcp_ptr: *mut NX_DHCP, iface_index: UINT) -> UINT;

        #[link_name = "_nxe_dhcp_interface_send_request"]
        #[doc = include_str!("../docs/nx_dhcp_interface_send_request.md")]
        pub fn nx_dhcp_interface_send_request(
            dhcp_ptr: *mut NX_DHCP,
            iface_index: UINT,
            dhcp_message_type: UINT,
        ) -> UINT;

        #[link_name = "_nxe_dhcp_interface_server_address_get"]
        #[doc = include_str!("../docs/nx_dhcp_interface_server_address_get.md")]
        pub fn nx_dhcp_interface_server_address_get(
            dhcp_ptr: *mut NX_DHCP,
            iface_index: UINT,
            server_address: *mut ULONG,
        ) -> UINT;

        #[link_name = "_nxe_dhcp_interface_state_change_notify"]
        #[doc = include_str!("../docs/nx_dhcp_interface_state_change_notify.md")]
        pub fn nx_dhcp_interface_state_change_notify(
            dhcp_ptr: *mut NX_DHCP,
            dhcp_interface_state_change_notify: ::core::option::Option<
                unsafe extern "C" fn(dhcp_ptr: *mut NX_DHCP, iface_index: UINT, new_state: UCHAR),
            >,
        ) -> UINT;

        #[link_name = "_nxe_dhcp_interface_user_option_retrieve"]
        #[doc = include_str!("../docs/nx_dhcp_interface_user_option_retrieve.md")]
        pub fn nx_dhcp_interface_user_option_retrieve(
            dhcp_ptr: *mut NX_DHCP,
            iface_index: UINT,
            option_request: UINT,
            destination_ptr: *mut UCHAR,
            destination_size: *mut UINT,
        ) -> UINT;
    }
}

pub(crate) mod unchecked {
    use super::control_blocks::*;
    use crate::control_blocks::*;
    use crate::tx::*;

    unsafe extern "C" {
        #[link_name = "_nx_dhcp_create"]
        #[doc = include_str!("../docs/nx_dhcp_create.md")]
        pub fn nx_dhcp_create(
            dhcp_ptr: *mut NX_DHCP,
            ip_ptr: *mut NX_IP,
            name_ptr: *mut CHAR,
        ) -> UINT;

        #[link_name = "_nx_dhcp_packet_pool_set"]
        #[doc = include_str!("../docs/nx_dhcp_packet_pool_set.md")]
        pub fn nx_dhcp_packet_pool_set(
            dhcp_ptr: *mut NX_DHCP,
            packet_pool_ptr: *mut NX_PACKET_POOL,
        ) -> UINT;

        #[link_name = "_nx_dhcp_request_client_ip"]
        #[doc = include_str!("../docs/nx_dhcp_request_client_ip.md")]
        pub fn nx_dhcp_request_client_ip(
            dhcp_ptr: *mut NX_DHCP,
            client_ip_address: ULONG,
            skip_discover_message: UINT,
        ) -> UINT;

        #[link_name = "_nx_dhcp_delete"]
        #[doc = include_str!("../docs/nx_dhcp_delete.md")]
        pub fn nx_dhcp_delete(dhcp_ptr: *mut NX_DHCP) -> UINT;

        #[link_name = "_nx_dhcp_decline"]
        #[doc = include_str!("../docs/nx_dhcp_decline.md")]
        pub fn nx_dhcp_decline(dhcp_ptr: *mut NX_DHCP) -> UINT;

        #[link_name = "_nx_dhcp_force_renew"]
        #[doc = include_str!("../docs/nx_dhcp_force_renew.md")]
        pub fn nx_dhcp_force_renew(dhcp_ptr: *mut NX_DHCP) -> UINT;

        #[link_name = "_nx_dhcp_release"]
        #[doc = include_str!("../docs/nx_dhcp_release.md")]
        pub fn nx_dhcp_release(dhcp_ptr: *mut NX_DHCP) -> UINT;

        #[link_name = "_nx_dhcp_start"]
        #[doc = include_str!("../docs/nx_dhcp_start.md")]
        pub fn nx_dhcp_start(dhcp_ptr: *mut NX_DHCP) -> UINT;

        #[link_name = "_nx_dhcp_stop"]
        #[doc = include_str!("../docs/nx_dhcp_stop.md")]
        pub fn nx_dhcp_stop(dhcp_ptr: *mut NX_DHCP) -> UINT;

        #[link_name = "_nx_dhcp_server_address_get"]
        #[doc = include_str!("../docs/nx_dhcp_server_address_get.md")]
        pub fn nx_dhcp_server_address_get(
            dhcp_ptr: *mut NX_DHCP,
            server_address: *mut ULONG,
        ) -> UINT;

        #[link_name = "_nx_dhcp_state_change_notify"]
        #[doc = include_str!("../docs/nx_dhcp_state_change_notify.md")]
        pub fn nx_dhcp_state_change_notify(
            dhcp_ptr: *mut NX_DHCP,
            dhcp_state_change_notify: ::core::option::Option<
                unsafe extern "C" fn(dhcp_ptr: *mut NX_DHCP, new_state: UCHAR),
            >,
        ) -> UINT;

        #[link_name = "_nx_dhcp_user_option_request"]
        pub fn nx_dhcp_user_option_request(dhcp_ptr: *mut NX_DHCP, option_code: UINT) -> UINT;

        #[link_name = "_nx_dhcp_user_option_retrieve"]
        #[doc = include_str!("../docs/nx_dhcp_user_option_retrieve.md")]
        pub fn nx_dhcp_user_option_retrieve(
            dhcp_ptr: *mut NX_DHCP,
            request_option: UINT,
            destination_ptr: *mut UCHAR,
            destination_size: *mut UINT,
        ) -> UINT;

        #[link_name = "_nx_dhcp_user_option_convert"]
        #[doc = include_str!("../docs/nx_dhcp_user_option_convert.md")]
        pub fn nx_dhcp_user_option_convert(source_ptr: *mut UCHAR) -> ULONG;

        #[link_name = "_nx_dhcp_user_option_add_callback_set"]
        #[doc = include_str!("../docs/nx_dhcp_user_option_add_callback_set.md")]
        pub fn nx_dhcp_user_option_add_callback_set(
            dhcp_ptr: *mut NX_DHCP,
            dhcp_user_option_add: ::core::option::Option<
                unsafe extern "C" fn(
                    dhcp_ptr: *mut NX_DHCP,
                    iface_index: UINT,
                    message_type: UINT,
                    user_option_ptr: *mut UCHAR,
                    user_option_length: *mut UINT,
                ) -> UINT,
            >,
        ) -> UINT;

        #[link_name = "_nx_dhcp_reinitialize"]
        #[doc = include_str!("../docs/nx_dhcp_reinitialize.md")]
        pub fn nx_dhcp_reinitialize(dhcp_ptr: *mut NX_DHCP) -> UINT;

        #[link_name = "_nx_dhcp_send_request"]
        #[doc = include_str!("../docs/nx_dhcp_send_request.md")]
        pub fn nx_dhcp_send_request(dhcp_ptr: *mut NX_DHCP, dhcp_message_type: UINT) -> UINT;

        #[link_name = "_nx_dhcp_set_interface_index"]
        #[doc = include_str!("../docs/nx_dhcp_set_interface_index.md")]
        pub fn nx_dhcp_set_interface_index(dhcp_ptr: *mut NX_DHCP, interface_index: UINT) -> UINT;

        #[link_name = "_nx_dhcp_clear_broadcast_flag"]
        #[doc = include_str!("../docs/nx_dhcp_clear_broadcast_flag.md")]
        pub fn nx_dhcp_clear_broadcast_flag(dhcp_ptr: *mut NX_DHCP, clear_flag: UINT) -> UINT;

        #[link_name = "_nx_dhcp_interface_clear_broadcast_flag"]
        #[doc = include_str!("../docs/nx_dhcp_interface_clear_broadcast_flag.md")]
        pub fn nx_dhcp_interface_clear_broadcast_flag(
            dhcp_ptr: *mut NX_DHCP,
            iface_index: UINT,
            clear_flag: UINT,
        ) -> UINT;

        #[link_name = "_nx_dhcp_interface_enable"]
        #[doc = include_str!("../docs/nx_dhcp_interface_enable.md")]
        pub fn nx_dhcp_interface_enable(dhcp_ptr: *mut NX_DHCP, iface_index: UINT) -> UINT;

        #[link_name = "_nx_dhcp_interface_disable"]
        #[doc = include_str!("../docs/nx_dhcp_interface_disable.md")]
        pub fn nx_dhcp_interface_disable(dhcp_ptr: *mut NX_DHCP, iface_index: UINT) -> UINT;

        #[link_name = "_nx_dhcp_interface_decline"]
        #[doc = include_str!("../docs/nx_dhcp_interface_decline.md")]
        pub fn nx_dhcp_interface_decline(dhcp_ptr: *mut NX_DHCP, iface_index: UINT) -> UINT;

        #[link_name = "_nx_dhcp_interface_force_renew"]
        #[doc = include_str!("../docs/nx_dhcp_interface_force_renew.md")]
        pub fn nx_dhcp_interface_force_renew(dhcp_ptr: *mut NX_DHCP, iface_index: UINT) -> UINT;

        #[link_name = "_nx_dhcp_interface_release"]
        #[doc = include_str!("../docs/nx_dhcp_interface_release.md")]
        pub fn nx_dhcp_interface_release(dhcp_ptr: *mut NX_DHCP, iface_index: UINT) -> UINT;

        #[link_name = "_nx_dhcp_interface_reinitialize"]
        #[doc = include_str!("../docs/nx_dhcp_interface_reinitialize.md")]
        pub fn nx_dhcp_interface_reinitialize(dhcp_ptr: *mut NX_DHCP, iface_index: UINT) -> UINT;

        #[link_name = "_nx_dhcp_interface_request_client_ip"]
        #[doc = include_str!("../docs/nx_dhcp_interface_request_client_ip.md")]
        pub fn nx_dhcp_interface_request_client_ip(
            dhcp_ptr: *mut NX_DHCP,
            iface_index: UINT,
            client_ip_address: ULONG,
            skip_discover_message: UINT,
        ) -> UINT;

        #[link_name = "_nx_dhcp_interface_start"]
        #[doc = include_str!("../docs/nx_dhcp_interface_start.md")]
        pub fn nx_dhcp_interface_start(dhcp_ptr: *mut NX_DHCP, iface_index: UINT) -> UINT;

        #[link_name = "_nx_dhcp_interface_stop"]
        #[doc = include_str!("../docs/nx_dhcp_interface_stop.md")]
        pub fn nx_dhcp_interface_stop(dhcp_ptr: *mut NX_DHCP, iface_index: UINT) -> UINT;

        #[link_name = "_nx_dhcp_interface_send_request"]
        #[doc = include_str!("../docs/nx_dhcp_interface_send_request.md")]
        pub fn nx_dhcp_interface_send_request(
            dhcp_ptr: *mut NX_DHCP,
            iface_index: UINT,
            dhcp_message_type: UINT,
        ) -> UINT;

        #[link_name = "_nx_dhcp_interface_server_address_get"]
        #[doc = include_str!("../docs/nx_dhcp_interface_server_address_get.md")]
        pub fn nx_dhcp_interface_server_address_get(
            dhcp_ptr: *mut NX_DHCP,
            iface_index: UINT,
            server_address: *mut ULONG,
        ) -> UINT;

        #[link_name = "_nx_dhcp_interface_state_change_notify"]
        #[doc = include_str!("../docs/nx_dhcp_interface_state_change_notify.md")]
        pub fn nx_dhcp_interface_state_change_notify(
            dhcp_ptr: *mut NX_DHCP,
            dhcp_interface_state_change_notify: ::core::option::Option<
                unsafe extern "C" fn(dhcp_ptr: *mut NX_DHCP, iface_index: UINT, new_state: UCHAR),
            >,
        ) -> UINT;

        #[link_name = "_nx_dhcp_interface_user_option_retrieve"]
        #[doc = include_str!("../docs/nx_dhcp_interface_user_option_retrieve.md")]
        pub fn nx_dhcp_interface_user_option_retrieve(
            dhcp_ptr: *mut NX_DHCP,
            iface_index: UINT,
            option_request: UINT,
            destination_ptr: *mut UCHAR,
            destination_size: *mut UINT,
        ) -> UINT;
    }
}
