Install TCP disconnect complete notify callback function
 
### Description

This service registers a callback function which is invoked after a socket disconnect operation is completed. The TCP socket disconnect complete callback function is available if NetX Duo is built with the option ***NX_ENABLE_EXTENDED_NOTIFY_SUPPORT*** defined.

### Parameters

- *socket_ptr** Pointer to previously connected client or server socket instance.
- *tcp_disconnect_complete_notify** The callback function to be installed.

### Return Values  

- **NX_SUCCESS** (0x00) Successfully registered the callback function.
- **NX_NOT_SUPPORTED** (0x4B) The extended notify feature is not built into the NetX Duo library NX_PTR_ERROR** (0x07) Invalid socket pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) TCP feature is not enabled.

### Allowed From

Initialization, threads

### Preemption Possible

No

