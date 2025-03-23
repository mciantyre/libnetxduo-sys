Get MSS of the peer TCP socket

### Description

This service retrieves the Maximum Segment Size (MSS) advertised by the peer socket.

### Parameters

- *socket_ptr** Pointer to previously created and connected socket.
- *mss** Destination for returning the MSS.

### Return Values 

- **NX_SUCCESS** (0x00) Successful peer MSS get.
- **NX_PTR_ERROR** (0x07) Invalid socket or MSS destination pointer.
- **NX_NOT_ENABLED** (0x14) TCP is not enabled.
- **NX_CALLER_ERROR** (0x11) Caller is not a thread or initialization.

### Allowed From

Threads

### Preemption Possible

No

