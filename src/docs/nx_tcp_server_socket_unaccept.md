Remove socket association with listening port

### Description

This service removes the association between this server socket and the specified server port. The application must call this service after a disconnection or after an unsuccessful accept call.

### Parameters

- *socket_ptr*: Pointer to previously setup server socket instance.

### Return Values  

- **NX_SUCCESS** (0x00) Successful server socket unaccept.
- **NX_NOT_LISTEN_STATE** (0x36) Server socket is in an improper state, and is probably not disconnected.
- **NX_PTR_ERROR** (0x07) Invalid socket pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Threads

### Preemption Possible

No

