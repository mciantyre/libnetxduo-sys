Retrieve information about UDP activities

### Description

This service retrieves information about UDP activities for the specified IP instance.

> **Important:** *If a destination pointer is NX_NULL, that particular information is not returned to the caller*.

### Parameters

- *ip_ptr** Pointer to previously created IP instance.
- *udp_packets_sent** Pointer to destination for the total number of UDP packets sent.
- *udp_bytes_sent** Pointer to destination for the total number of UDP bytes sent.
- *udp_packets_received** Pointer to destination of the total number of UDP packets received.
- *udp_bytes_received** Pointer to destination of the total number of UDP bytes received.
- *udp_invalid_packets** Pointer to destination of the total number of invalid UDP packets.
- *udp_receive_packets_dropped** Pointer to destination of the total number of UDP receive packets dropped.
- *udp_checksum_errors** Pointer to destination of the total number of UDP packets with checksum errors.

### Return Values

- **NX_SUCCESS** (0x00) Successful UDP information retrieval.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Initialization, threads, and timers

### Preemption Possible

No

