Re-listen for client connection on TCP port

### Description

This service is called after a connection has been received on a port that was setup previously for listening. The main purpose of this service is to provide a new server socket for the next client connection. If a connection request is queued, the connection will be processed immediately during this service call.

> **Important:** *The same callback routine specified by the original listen request is also called when a connection is present for this new server socket*.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *port*: Port number to re-listen on (1 through 0xFFFF).
- *socket_ptr*: Socket to use for the next client connection.

### Return Values  

- **NX_SUCCESS** (0x00) Successful TCP port re-listen.
- **NX_NOT_CLOSED** (0x35) The supplied server socket is not in a closed state.
- **NX_ALREADY_BOUND** (0x22) The supplied server socket is already bound to a port.
- **NX_INVALID_RELISTEN** (0x47) There is already a valid socket pointer for this port or the port specified does not have a listen request active.
- **NX_CONNECTION_PENDING** (0x48) Same as NX_SUCCESS, except there was a queued connection request and it was processed during this call.
- **NX_INVALID_PORT** (0x46) Invalid port specified.
- **NX_PTR_ERROR** (0x07) Invalid IP or listen callback pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Threads

### Preemption Possible

No

