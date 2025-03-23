Retrieve information about TCP activities

### Description

This service retrieves information about TCP activities for the specified IP instance.

> **Important:** *If a destination pointer is NX_NULL, that particular information is not returned to the caller*.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *tcp_packets_sent*: Pointer to destination for the total number of TCP packets sent.
- *tcp_bytes_sent*: Pointer to destination for the total number of TCP bytes sent.
- *tcp_packets_received*: Pointer to destination of the total number of TCP packets received.
- *tcp_bytes_received*: Pointer to destination of the total number of TCP bytes received.
- *tcp_invalid_packets*: Pointer to destination of the total number of invalid TCP packets.
- *tcp_receive_packets_dropped*: Pointer to destination of the total number of TCP receive packets dropped.
- *tcp_checksum_errors*: Pointer to destination of the total number of TCP packets with checksum errors.
- *tcp_connections*: Pointer to destination of the total number of TCP connections.
- *tcp_disconnections*: Pointer to destination of the total number of TCP disconnections.
- *tcp_connections_dropped*: Pointer to destination of the total number of TCP connections dropped.
- *tcp_retransmit_packets*: Pointer to destination of the total number of TCP packets retransmitted.

### Return Values 

- **NX_SUCCESS** (0x00) Successful TCP information retrieval.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Initialization, threads

### Preemption Possible

No

