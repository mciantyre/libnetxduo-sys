Extract network parameters from UDP packet

### Description

This service extracts network parameters, such as IPv4 address, peer port number, protocol type (this service always returns UDP type) from a packet received on an incoming interface. To obtain information on a packet coming from IPv4 or IPv6 network, application shall use the service ***nxd_udp_packet_info_extract.***

### Parameters

- *packet_ptr** Pointer to packet.
- *ip_address** Pointer to sender IP address.
- *protocol** Pointer to protocol (UDP).
- *port** Pointer to sender's port number.
- *interface_index** Pointer to receiving interface index.

### Return Values  

- **NX_SUCCESS** (0x00) Packet interface data successfully extracted.
- **NX_INVALID_PACKET** (0x12) Packet does not contain IPv4 frame.
- **NX_PTR_ERROR** (0x07) Invalid pointer input
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Threads

### Preemption Possible

No

