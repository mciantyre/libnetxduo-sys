Send raw IP packet through specified network interface

### Description

This service sends a raw IP packet to the destination IP address using the specified local IPv4 address as the source address, and through the associated network interface. Note that this routine returns immediately, and it is, therefore, not known if the IP packet has actually been sent. The network driver will be responsible for releasing the packet when the transmission is complete. This service differs from other services in that there is no way of knowing if the packet was actually sent. It could get lost on the Internet.

> **Caution:** *Note that raw IP processing must be enabled*.

> **Warning:** *This service is similar to **nx_ip_raw_packet_send,** except that this service allows an application to send raw IPv4 packet from a specified physical interfaces*.

### Parameters  

- *ip_ptr*: Pointer to previously created IP task.
- *packet_ptr*: Pointer to packet to transmit.
- *destination_ip*: IP address to send packet.
- *address_index*: Index of the address of the interface to send packet out on.
- *type_of_service*: Type of service for packet.

### Return Values  

- **NX_SUCCESS** (0x00) Packet successfully transmitted.
- **NX_IP_ADDRESS_ERROR** (0x21) No suitable outgoing interface available.
- **NX_NOT_ENABLED** (0x14) Raw IP packet processing not enabled.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_PTR_ERROR** (0x07) Invalid pointer input.
- **NX_OPTION_ERROR** (0x0A) Invalid type of service specified.
- **NX_OVERFLOW** (0x03) Invalid packet prepend pointer.
- **NX_UNDERFLOW** (0x02) Invalid packet prepend pointer.
- **NX_INVALID_INTERFACE** (0x4C) Invalid interface index specified.

### Allowed From

Threads

### Preemption Possible

No

