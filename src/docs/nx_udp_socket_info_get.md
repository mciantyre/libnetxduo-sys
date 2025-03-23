Retrieve information about UDP socket activities

### Description

This service retrieves information about UDP socket activities for the specified UDP socket instance.

> **Important:** *If a destination pointer is NX_NULL, that particular information is not returned to the caller*.

### Parameters

- *socket_ptr*: Pointer to previously created UDP socket instance.
- *udp_packets_sent*: Pointer to destination for the total number of UDP packets sent on socket.
- *udp_bytes_sent*: Pointer to destination for the total number of UDP bytes sent on socket.
- *udp_packets_received*: Pointer to destination of the total number of UDP packets received on socket.
- *udp_bytes_received*: Pointer to destination of the total number of UDP bytes received on socket.
- *udp_packets_queued*: Pointer to destination of the total number of queued UDP packets on socket.
- *udp_receive_packets_dropped*: Pointer to destination of the total number of UDP receive packets dropped for socket due to queue size being exceeded.
- *udp_checksum_errors*: Pointer to destination of the total number of UDP packets with checksum errors on socket.

### Return Values 

- **NX_SUCCESS** (0x00) Successful UDP socket information retrieval.
- **NX_PTR_ERROR** (0x07) Invalid socket pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Initialization, threads, and timers

### Preemption Possible

No

