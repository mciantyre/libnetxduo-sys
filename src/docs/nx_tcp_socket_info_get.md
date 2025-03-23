Retrieve information about TCP socket activities

### Description

This service retrieves information about TCP socket activities for the specified TCP socket instance.

> NOTE  
> *If a destination pointer is NX_NULL, that particular information is not returned to the caller*.

### Parameters

- *socket_ptr** Pointer to previously created TCP socket instance.
- *tcp_packets_sent** Pointer to destination for the total number of TCP packets sent on socket.
- *tcp_bytes_sent** Pointer to destination for the total number of TCP bytes sent on socket.
- *tcp_packets_received** Pointer to destination of the total number of TCP packets received on socket.
- *tcp_bytes_received** Pointer to destination of the total number of TCP bytes received on socket.
- *tcp_retransmit_packets** Pointer to destination of the total number of TCP packet retransmissions.
- *tcp_packets_queued** Pointer to destination of the total number of queued TCP packets on socket.
- *tcp_checksum_errors** Pointer to destination of the total number of TCP packets with checksum errors on socket.
- *tcp_socket_state** Pointer to destination of the socket's current state.
- *tcp_transmit_queue_depth** Pointer to destination of the total number of transmit packets still queued waiting for ACK.
- *tcp_transmit_window** Pointer to destination of the current transmit window size.
- *tcp_receive_window** Pointer to destination of the current receive window size.

### Return Values  

- **NX_SUCCESS** (0x00) Successful TCP socket information retrieval.
- **NX_PTR_ERROR** (0x07) Invalid socket pointer. 
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Initialization, threads

### Preemption Possible

No

