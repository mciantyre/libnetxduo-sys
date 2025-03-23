Notify application of received packets

### Description 

This service configures the receive notify function pointer with the callback function specified by the application. This callback function is then called whenever one or more packets are received on the socket. If a NX_NULL pointer is supplied, the notify function is disabled.

### Parameters

- *socket_ptr** Pointer to the TCP socket.
- *tcp_receive_notify** Application callback function pointer that is called when one or more packets are received on the socket.

### Return Values 

- **NX_SUCCESS** (0x00) Successful socket receive notify.
- **NX_PTR_ERROR** (0x07) Invalid socket pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) TCP feature is not enabled.

### Allowed From

Initialization, threads

### Preemption Possible

No

