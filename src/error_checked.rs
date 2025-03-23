use crate::tx::*;
use crate::*;

#[allow(unused_imports)]
pub use crate::addons::error_checked::*;

unsafe extern "C" {
    fn _nxe_ip_create(
        ip_ptr: *mut NX_IP,
        name: *mut CHAR,
        ip_address: ULONG,
        network_mask: ULONG,
        default_pool: *mut NX_PACKET_POOL,
        ip_link_driver: ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP_DRIVER)>,
        memory_ptr: *mut ::core::ffi::c_void,
        memory_size: ULONG,
        priority: UINT,
        ip_control_block_size: UINT,
    ) -> UINT;

    fn _nxe_packet_pool_create(
        pool_ptr: *mut NX_PACKET_POOL,
        name: *mut CHAR,
        payload_size: ULONG,
        memory_ptr: *mut ::core::ffi::c_void,
        memory_size: ULONG,
        pool_control_block_size: UINT,
    ) -> UINT;

    fn _nxe_tcp_socket_create(
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
        tcp_socket_size: UINT,
    ) -> UINT;

    fn _nxe_udp_socket_create(
        ip_ptr: *mut NX_IP,
        socket_ptr: *mut NX_UDP_SOCKET,
        name: *mut CHAR,
        type_of_service: ULONG,
        fragment: ULONG,
        time_to_live: UINT,
        queue_maximum: ULONG,
        udp_socket_size: UINT,
    ) -> UINT;
}

#[doc = include_str!("docs/nx_ip_create.md")]
#[inline(always)]
pub unsafe extern "C" fn nx_ip_create(
    ip_ptr: *mut NX_IP,
    name: *mut CHAR,
    ip_address: ULONG,
    network_mask: ULONG,
    default_pool: *mut NX_PACKET_POOL,
    ip_link_driver: ::core::option::Option<unsafe extern "C" fn(arg1: *mut NX_IP_DRIVER)>,
    memory_ptr: *mut ::core::ffi::c_void,
    memory_size: ULONG,
    priority: UINT,
) -> UINT {
    unsafe {
        _nxe_ip_create(
            ip_ptr,
            name,
            ip_address,
            network_mask,
            default_pool,
            ip_link_driver,
            memory_ptr,
            memory_size,
            priority,
            size_of::<NX_IP>() as _,
        )
    }
}

#[doc = include_str!("docs/nx_packet_pool_create.md")]
#[inline(always)]
pub unsafe extern "C" fn nx_packet_pool_create(
    pool_ptr: *mut NX_PACKET_POOL,
    name: *mut CHAR,
    payload_size: ULONG,
    memory_ptr: *mut ::core::ffi::c_void,
    memory_size: ULONG,
) -> UINT {
    unsafe {
        _nxe_packet_pool_create(
            pool_ptr,
            name,
            payload_size,
            memory_ptr,
            memory_size,
            size_of::<NX_PACKET_POOL>() as _,
        )
    }
}

#[doc = include_str!("docs/nx_tcp_socket_create.md")]
#[inline(always)]
pub unsafe extern "C" fn nx_tcp_socket_create(
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
) -> UINT {
    unsafe {
        _nxe_tcp_socket_create(
            ip_ptr,
            socket_ptr,
            name,
            type_of_service,
            fragment,
            time_to_live,
            window_size,
            tcp_urgent_data_callback,
            tcp_disconnect_callback,
            size_of::<NX_TCP_SOCKET>() as _,
        )
    }
}

#[doc = include_str!("docs/nx_udp_socket_create.md")]
#[inline(always)]
pub unsafe extern "C" fn nx_udp_socket_create(
    ip_ptr: *mut NX_IP,
    socket_ptr: *mut NX_UDP_SOCKET,
    name: *mut CHAR,
    type_of_service: ULONG,
    fragment: ULONG,
    time_to_live: UINT,
    queue_maximum: ULONG,
) -> UINT {
    unsafe {
        _nxe_udp_socket_create(
            ip_ptr,
            socket_ptr,
            name,
            type_of_service,
            fragment,
            time_to_live,
            queue_maximum,
            size_of::<NX_UDP_SOCKET>() as _,
        )
    }
}
