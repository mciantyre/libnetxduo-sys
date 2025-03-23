Unbind TCP client socket from TCP port

### Description

This service releases the binding between the TCP client socket and a TCP port. If there are other threads waiting to bind another socket to the same port number, the first suspended thread is then bound to this port.

### Parameters

- *socket_ptr*: Pointer to previously created TCP socket instance.

### Return Values 

- **NX_SUCCESS** (0x00) Successful socket unbind.
- **NX_NOT_BOUND** (0x24) Socket was not bound to any port.
- **NX_NOT_CLOSED** (0x35) Socket has not been disconnected.
- **NX_PTR_ERROR** (0x07) Invalid socket pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Threads

### Preemption Possible

Yes

