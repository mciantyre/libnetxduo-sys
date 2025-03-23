Send a UDP Datagram

### Description

This service sends a UDP datagram through a previously created and bound UDP socket for IPv4 networks. NetX Duo finds a suitable local IP address as source address based on the destination IP address. To specify a specific interface and source IP address, the application should use the  **nxd_udp_socket_source_send** service.

Note that this service returns immediately regardless of whether the UDP datagram was successfully sent. The NetX Duo (IPv4/IPv6) equivalent service is ***nxd_udp_socket_send***.

The socket must be bound to a local port.

> **Warning:** *Unless an error is returned, the application should not release the packet after this call. Doing so will cause unpredictable results because the network driver will also try to release the packet after 
transmission*.

### Parameters

- *socket_ptr*: Pointer to previously created UDP socket instance
- *packet_ptr*: UDP datagram packet pointer
- *ip_address*: Destination IPv4 address
- *port*: Valid destination port number between 1 and 0xFFFF), in host byte order

### Return Values

- **NX_SUCCESS** (0x00) Successful UDP socket send
- **NX_NOT_BOUND** (0x24) Socket not bound to any port
- **NX_NO_INTERFACE_ADDRESS** (0x50) No suitable outgoing interface can be found.
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid server IP address
- **NX_UNDERFLOW** (0x02) Not enough room for UDP header in the packet
- **NX_OVERFLOW** (0x03) Packet append pointer is invalid
- **NX_PTR_ERROR** (0x07) Invalid socket pointer
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service
- **NX_NOT_ENABLED** (0x14) UDP has not been enabled
- **NX_INVALID_PORT** (0x46) Port number is not within a valid range

### Allowed From

Threads

### Preemption Possible

No

