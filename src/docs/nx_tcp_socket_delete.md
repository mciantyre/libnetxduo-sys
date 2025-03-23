Delete TCP socket

### Description

This service deletes a previously created TCP socket. If the socket is still bound or connected, the service returns an error code.

### Parameters

- *socket_ptr** Previously created TCP socket

### Return Values

- **NX_SUCCESS** (0x00) Successful socket delete.
- **NX_NOT_CREATED** (0x27) Socket was not created.
- **NX_STILL_BOUND** (0x42) Socket is still bound.
- **NX_PTR_ERROR** (0x07) Invalid socket pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Threads

### Preemption Possible

No

