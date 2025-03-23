Compute maximum packet data payload

### Description

This service finds the maximum application payload size that will not require IP fragmentation to reach the destination; e.g., payload is at or below the local interface MTU size. (or the Path MTU value obtained via IPv6 Path MTU discovery). IP header and upper application header size (TCP or UDP) are subtracted from the total payload. If NetX Duo IPsec Security Policy applies to this end-point, the IPsec headers (ESP/AH) and associated overhead, such as Initial Vector, are also subtracted from the MTU. This service is applicable for both IPv4 and IPv6 packets.

The parameter *if_index* specifies the interface to use for sending out the packet. For a multihome system, the caller needs to specify the *if_index* parameter if the destination is a broadcast (IPv4 only), multicast, or IPv6 link-local address.

This service returns two values to the caller:

1) start_offset_ptr: This is the location after the TCP/UDP/IP/IPsec headers;
2) payload_length_ptr: the amount of data application may transfer without exceeding MTU.

There is no equivalent NetX service.

### Restrictions 

The IP instance must be previously created.

### Parameters

- *ip_ptr*: Pointer to IP instance
- *dest_address*: Pointer to packet destination address
- *if_index*: Indicates the index of the interface to use
- *src_port*: Source port number
- *dest_port*: Destination port number
- *protocol*: Upper layer protocol to be used
- *start_offset_ptr*: Pointer to the start of data for maximum packet payload
- *payload_length_ptr*: Pointer to payload size excluding headers

### Return Values

- **NX_SUCCESS** (0x00) Payload successfully computed
- **NX_INVALID_INTERFACE** (0x4C) Interface index is invalid, or the interface is not valid.
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid IP address.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer, or invalid destination address
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid address supplied 
- **NX_NOT_SUPPORTED** (0x4B) Invalid protocol (not UDP or TCP)
- **NX_CALLER_ERROR** (0x11) Service is not called from system initialization or thread context.

### Allowed From

Initialization, threads

### Preemption Possible

No

