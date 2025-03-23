Bind UDP socket to UDP port

### Description

This service binds the previously created UDP socket to the specified UDP port. Valid UDP sockets range from 0 through 0xFFFF. If the requested port number is bound to another socket, this service waits for specified period of time for the socket to unbind from the port number.

### Parameters

- *socket_ptr** Pointer to previously created UDP socket instance.
- *port** Port number to bind to** (1 through 0xFFFF). If port number is NX_ANY_PORT** (0x0000), the IP instance will search for the next free port and use that for the binding.
- *wait_option** Defines how the service behaves if the port is already bound to another socket. The wait options are defined as follows:
	- **NX_NO_WAIT** (0x00000000)
	- **NX_WAIT_FOREVER** (0xFFFFFFFF)
	- **timeout value in ticks** (0x00000001 through 0xFFFFFFFE)

### Return Values  

- **NX_SUCCESS** (0x00) Successful socket bind.
- **NX_ALREADY_BOUND** (0x22) This socket is already bound to another port.
- **NX_PORT_UNAVAILABLE** (0x23) Port is already bound to a different socket.
- **NX_NO_FREE_PORTS** (0x45) No free port.
- **NX_WAIT_ABORTED** (0x1A) Requested suspension was aborted by a call to tx_thread_wait_abort.
- **NX_INVALID_PORT** (0x46) Invalid port specified.
- **NX_PTR_ERROR** (0x07) Invalid socket pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Threads

### Preemption Possible

No

