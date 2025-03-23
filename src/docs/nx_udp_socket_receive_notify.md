Notify application of each received packet

### Description 

This service sets the receive notify function pointer to the callback function specified by the  application. This callback function is then called whenever a packet is received on the socket. If a NX_NULL pointer is supplied, the receive notify function is disabled.

### Parameters

- *socket_ptr*: Pointer to the UDP socket.
- *udp_receive_notify*: Application callback function pointer that is called when a packet is received on the socket.

### Return Values 

- **NX_SUCCESS** (0x00) Successfully set socket receive notify function.
- **NX_PTR_ERROR** (0x07) Invalid socket pointer.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

No

