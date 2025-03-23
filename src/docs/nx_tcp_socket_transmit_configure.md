Configure socket's transmit parameters

### Description

This service configures various transmit parameters of the specified TCP socket.

### Parameters

- *socket_ptr** Pointer to the TCP socket.
- *max_queue_depth** Maximum number of packets allowed to be queued for transmission.
- *timeout** Number of ThreadX timer ticks an ACK is waited for before the packet is sent again.
- *max_retries** Maximum number of retries allowed.
- *timeout_shift** Value to shift the timeout for each subsequent retry. A value of 0, results in the same timeout between successive retries. A value of 1, doubles the timeout between retries.

### Return Values 

- **NX_SUCCESS** (0x00) Successful transmit socket configure.
- **NX_PTR_ERROR** (0x07) Invalid socket pointer.
- **NX_OPTION_ERROR** (0x0a) Invalid queue depth option.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) TCP feature is not enabled.

### Allowed From

Initialization, threads

### Preemption Possible

No

