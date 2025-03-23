Set MSS of socket

### Description

This service sets the specified socket's Maximum Segment Size (MSS). Note the MSS value must be within the network interface IP MTU, allowing room for IP and TCP headers.

This service should be used before a TCP socket starts the connection process. If the service is used after a TCP connection is established, the new value has no effect on the connection.

### Parameters

- *socket_ptr** Pointer to previously created socket.
- *mss** Value of MSS to set.

### Return Values  

- **NX_SUCCESS** (0x00) Successful MSS set.
- **NX_SIZE_ERROR** (0x09) Specified MSS value is too large.
- **NX_NOT_CONNECTED** (0x38) TCP connection has not been established 
- **NX_PTR_ERROR** (0x07) Invalid socket pointer.
- **NX_NOT_ENABLED** (0x14) TCP is not enabled.
- **NX_CALLER_ERROR** (0x11) Caller is not a thread or initialization.

### Allowed From

Initialization and threads

### Preemption Possible

No

