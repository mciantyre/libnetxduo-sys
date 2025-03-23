Retrieve information about IP activities

### Description

This service retrieves information about IP activities for the specified IP instance.

> NOTE  
> *If a destination pointer is NX_NULL, that particular information is not returned to the caller*.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *ip_total_packets_sent*: Pointer to destination for the total number of IP packets sent.
- *ip_total_bytes_sent*: Pointer to destination for the total number of bytes sent.
- *ip_total_packets_received*: Pointer to destination of the total number of IP receive packets.
- *ip_total_bytes_received*: Pointer to destination of the total number of IP bytes received.
- *ip_invalid_packets*: Pointer to destination of the total number of invalid IP packets.
- *ip_receive_packets_dropped*: Pointer to destination of the total number of receive packets dropped.
- *ip_receive_checksum_errors*: Pointer to destination of the total number of checksum errors in receive packets.
- *ip_send_packets_dropped*: Pointer to destination of the total number of send packets dropped.
- *ip_total_fragments_sent*: Pointer to destination of the total number of fragments sent.
- *ip_total_fragments_received*: Pointer to destination of the total number of fragments received.

### Return Values  

- **NX_SUCCESS** (0x00) Successful IP information retrieval.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.

### Allowed From

Initialization, threads

### Preemption Possible

No

