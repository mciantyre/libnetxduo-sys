Retrieves number of bytes available for retrieval

### Description

This service obtains the number of bytes available for retrieval in the specified TCP socket. Note that the TCP socket must already be connected.

### Parameters

- *socket_ptr*: Pointer to previously created and connected TCP socket.
- *bytes_available*: Pointer to destination for bytes available.

### Return Values

- **NX_SUCCESS** (0x00) Service executes successfully. Number of bytes available for read is returned to the caller.
- **NX_NOT_CONNECTED** (0x38) Socket is not in a connected state.
- **NX_PTR_ERROR** (0x07) Invalid pointers.
- **NX_NOT_ENABLED** (0x14) TCP is not enabled.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Threads

### Preemption Possible

No

