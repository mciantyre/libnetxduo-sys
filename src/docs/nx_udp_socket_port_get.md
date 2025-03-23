Pick up port number bound to UDP socket

### Description

This service retrieves the port number associated with the socket, which is useful to find the port allocated by NetX Duo in situations where the NX_ANY_PORT was specified at the time the socket was bound.

### Parameters

- *socket_ptr*: Pointer to previously created UDP socket instance.
- *port_ptr*: Pointer to destination for the return port number. Valid port numbers are (1- 0xFFFF).

### Return Values 

- **NX_SUCCESS** (0x00) Successful socket bind.
- **NX_NOT_BOUND** (0x24) This socket is not bound to a port.
- **NX_PTR_ERROR** (0x07) Invalid socket pointer or port return pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Threads

### Preemption Possible

No

