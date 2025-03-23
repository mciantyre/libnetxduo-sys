Send raw IP packet

### Description

This service sends a raw IPv4 packet to the destination IP address. Note that this routine returns immediately, and it is therefore not known whether the IP packet has actually been sent. The network driver will be responsible for releasing the packet when the transmission is complete.

For a multihome system, NetX Duo uses the destination IP address to find an appropriate network interface and uses the IP address of the interface as the source address. If the destination IP address is broadcast or multicast, the first valid interface is used. Applications use the ***nx_ip_raw_packet_source_send*** in this case.

To send a raw IPv6 packet, application shall use the service ***nxd_ip_raw_packet_send,*** or ***nxd_ip_raw_packet_source_send.***

> **Warning:** *Unless an error is returned, the application should not release the packet after this call. Doing so will cause unpredictable results because the network driver will release the packet after transmission*.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *packet_ptr*: Pointer to the raw IP packet to send.
- *destination_ip*: Destination IP address, which can be a specific host IP address, a network broadcast, an internal loop-back, or a multicast address.
- *type_of_service*: Defines the type of service for the transmission, legal values are as follows:
	- **NX_IP_NORMAL** (0x00000000)
	- **NX_IP_MIN_DELAY** (0x00100000)
	- **NX_IP_MAX_DATA** (0x00080000)
	- **NX_IP_MAX_RELIABLE** (0x00040000)
	- **NX_IP_MIN_COST** (0x00020000)

### Return Values  

- **NX_SUCCESS** (0x00) Successful IP raw packet send initiated.
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid IP address.
- **NX_NOT_ENABLED** (0x14) Raw IP feature is not enabled.
- **NX_OPTION_ERROR** (0x0A) Invalid type of service.
- **NX_UNDERFLOW** (0x02) Not enough room to prepend an IP header on the packet.
- **NX_OVERFLOW** (0x03) Packet append pointer is invalid.
- **NX_PTR_ERROR** (0x07) Invalid IP or packet pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Threads

### Preemption Possible

No

