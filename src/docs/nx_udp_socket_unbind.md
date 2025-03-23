Unbind UDP socket from UDP port

### Description

This service releases the binding between the UDP socket and a UDP port. Any received packets stored in the receive queue are released as part of the unbind operation.

If there are other threads waiting to bind another socket to the unbound port, the first suspended thread is then bound to the newly unbound port.

### Parameters

- *socket_ptr*: Pointer to previously created UDP socket instance.

### Return Values 

- **NX_SUCCESS** (0x00) Successful socket unbind.
- **NX_NOT_BOUND** (0x24) Socket was not bound to any port.
- **NX_PTR_ERROR** (0x07) Invalid socket pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Threads

### Preemption Possible

Yes

