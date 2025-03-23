Set TCP establish notify callback function

### Description

This service registers a callback function, which is called after a TCP socket makes a connection. The TCP socket establish callback function is available if NetX Duo is built with the option ***NX_ENABLE_EXTENDED_NOTIFY_SUPPORT*** defined.

### Parameters

- *socket_ptr** Pointer to previously connected client or server socket instance.
- *tcp_establish_notify** Callback function invoked after a TCP connection is established.

### Return Values 

- **NX_SUCCESS** (0x00) Successfully sets the notify function.
- **NX_NOT_SUPPORTED** (0x4B) The extended notify feature is not built into the NetX Duo library 
- **NX_PTR_ERROR** (0x07) Invalid socket pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) TCP has not been enabled by the application.

### Allowed From

Threads

### Preemption Possible

No

