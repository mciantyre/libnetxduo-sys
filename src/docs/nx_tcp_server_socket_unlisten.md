Disable listening for client connection on TCP port

### Description

This service disables listening for a client connection request on the specified TCP port.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *port*: Number of port to disable listening (0 through 0xFFFF).

### Return Values 

- **NX_SUCCESS** (0x00) Successful TCP listen disable.
- **NX_ENTRY_NOT_FOUND** (0x16) Listening was not enabled for the specified port.
- **NX_INVALID_PORT** (0x46) Invalid port specified.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Threads

### Preemption Possible

No

