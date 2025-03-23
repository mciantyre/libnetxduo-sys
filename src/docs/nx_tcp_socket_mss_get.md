Get MSS of socket

### Description

This service retrieves the specified socket's local Maximum Segment Size (MSS).

### Parameters

- *socket_ptr** Pointer to previously created socket.
- *mss** Destination for returning MSS.

### Return Values  

- **NX_SUCCESS** (0x00) Successful MSS get.
- **NX_PTR_ERROR** (0x07) Invalid socket or MSS destination pointer.
- **NX_NOT_ENABLED** (0x14) TCP is not enabled.
- **NX_CALLER_ERROR** (0x11) Caller is not a thread or initialization.

### Allowed From

Initialization and threads

### Preemption Possible

No

