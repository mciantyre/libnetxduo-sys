Set the TCP transmit queue notify function

### Description

This service sets the transmit queue depth update notify function specified by the application, which is called whenever the specified socket determines that it has released packets from the transmit queue such that the queue depth is no longer exceeding its limit. If an application would be blocked on transmit due to queue depth, the callback function serves as a notification to the application that it may start transmitting again. This service is available only if the NetX Duo library is built with the option ***NX_ENABLE_TCP_QUEUE_DEPTH_UPDATE_NOTIFY*** defined.

### Parameters 

- *socket_ptr** Pointer to the socket structure 
- *tcp_socket_queue_depth_notify** The notify function to be installed

### Return Values 

- **NX_SUCCESS** (0x00) Successfully installed the notify function
- **NX_NOT_SUPPORTED** (0x4B) The TCP socket queue depth notify feature is not built into the NetX Duo library
- **NX_PTR_ERROR** (0x07) Invalid pointer to the socket control block or the notify function
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) TCP feature is not enabled.

### Allowed From

Threads

### Preemption Possible

No

