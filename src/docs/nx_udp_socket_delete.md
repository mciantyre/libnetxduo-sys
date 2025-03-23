Delete UDP socket

### Description

This service deletes a previously created UDP socket. If the socket was bound to a port, the socket must be unbound first.

### Parameters

- *socket_ptr*: Pointer to previously created UDP socket instance.

### Return Values 

- **NX_SUCCESS** (0x00) Successful socket delete.
- **NX_STILL_BOUND** (0x42) Socket is still bound.
- **NX_PTR_ERROR** (0x07) Invalid socket pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Threads

### Preemption Possible

No

