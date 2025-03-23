Retrieve information about peer TCP socket

### Description

This service retrieves peer IP address and port information for the connected TCP socket over IPv4 network. The equivalent service that also supports IPv6 network is ***nxd_tcp_socket_peer_info_get.***

### Parameters

- *socket_ptr** Pointer to previously created TCP socket.
- *peer_ip_address** Pointer to destination for peer IP address, in host byte order.
- *peer_port** Pointer to destination for peer port number, in host byte order.

### Return Values  

- **NX_SUCCESS** (0x00) Service executes successfully. Peer IP address and port number are returned to the caller.
- **NX_NOT_CONNECTED** (0x38) Socket is not in a connected state.
- **NX_PTR_ERROR** (0x07) Invalid pointers.
- **NX_NOT_ENABLED** (0x14) TCP is not enabled.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Threads

### Preemption Possible

No

