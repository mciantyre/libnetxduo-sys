Notify application of window size updates

### Description

This service installs a socket window update callback routine. This routine is called automatically whenever the specified socket receives a packet indicating an increase in the window size of the remote host.

### Parameters

- *socket_ptr** Pointer to previously created TCP socket.
- *tcp_window_update_notify** Callback routine to be called when the window size changes. A value of NULL disables the window change update.

### Return Values  

- **NX_SUCCESS** (0x00) Callback routine is installed on the socket.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_PTR_ERROR** (0x07) Invalid pointers.
- **NX_NOT_ENABLED** (0x14) TCP feature is not enabled.

### Allowed From

Initialization, threads

### Preemption Possible

No

