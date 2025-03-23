Send datagram through UDP socket

### Description

This service sends a UDP datagram through a previously created and bound UDP socket through the network interface with the specified IP address as the source address. Note that service returns immediately, regardless of whether or not the UDP datagram was successfully sent. ***nxd_udp_socket_source_send*** works for both IPv4 and IPv6 networks.

> **Warning:** *Unless an error is returned, the application should not release the packet after this call. Doing so will cause unpredictable results because the network driver will also try to release the packet after 
transmission*.

### Parameters 

- *socket_ptr*: Socket to transmit the packet out on.
- *packet_ptr*: Pointer to packet to transmit.
- *ip_address*: Destination IP address to send packet.
- *port*: Destination port.
- *address_index*: Index of the address associated with the interface to send packet on.

### Return Values

- **NX_SUCCESS** (0x00) Packet successfully sent.
- **NX_NOT_BOUND** (0x24) Socket not bound to a port.
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid IP address.
- **NX_NOT_ENABLED** (0x14) UDP processing not enabled.
- **NX_PTR_ERROR** (0x07) Invalid pointer.
- **NX_OVERFLOW** (0x03) Invalid packet append pointer.
- **NX_UNDERFLOW** (0x02) Invalid packet prepend pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_INVALID_INTERFACE** (0x4C) Invalid address index.
- **NX_INVALID_PORT** (0x46) Port number exceeds maximum port number.

### Allowed From

Threads

### Preemption Possible

No

