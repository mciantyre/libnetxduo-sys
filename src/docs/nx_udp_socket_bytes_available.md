Retrieves number of bytes available for retrieval

### Description

This service retrieves number of bytes available for reception in the specified UDP socket.

### Parameters

- *socket_ptr** Pointer to previously created UDP socket.
- *bytes_available** Pointer to destination for bytes available.

### Return Values 

- **NX_SUCCESS** (0x00) Successful bytes available retrieval.
- **NX_NOT_SUCCESSFUL** (0x43) Socket not bound to a port.
- **NX_PTR_ERROR** (0x07) Invalid pointers.
- **NX_NOT_ENABLED** (0x14) UDP feature is not enabled.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Threads

### Preemption Possible

No

