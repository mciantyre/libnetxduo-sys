Install callback for timed wait state

### Description

This service registers a callback function which is invoked when the TCP socket is in timed wait state. To use this service, the NetX Duo library must be built with the option ***NX_ENABLE_EXTENDED_NOTIFY*** defined.

### Parameters

- *socket_ptr** Pointer to previously connected client or server socket instance.
- *tcp_timed_wait_callback** The timed wait callback function

### Return Values  

- **NX_SUCCESS** (0x00) Successfully registers the callback function socket
- **NX_NOT_SUPPORTED** (0x4B) NetX Duo library is built without the extended notify feature enabled.
- **NX_PTR_ERROR** (0x07) Invalid socket pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) TCP feature is not enabled.

### Allowed From

Initialization, threads

### Preemption Possible

No

