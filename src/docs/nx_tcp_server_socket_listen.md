Enable listening for client connection on TCP port

### Description

This service enables listening for a client connection request on the specified TCP port. When a client connection request is received, the supplied server socket is bound to the specified port and the supplied listen callback function is called.

The listen callback routine's processing is completely up to the application. It may contain logic to wake up an application thread that subsequently performs an accept operation. If the application already has a thread suspended on accept processing for this socket, the listen callback routine may not be needed.

If the application wishes to handle additional client connections on the same port, the ***nx_tcp_server_socket_relisten*** must be called with an available socket (a socket in the CLOSED state) for the next connection. Until the re-listen service is called, additional client connections are queued. When the maximum queue depth is exceeded, the oldest connection request is dropped in favor of queuing the new connection request. The maximum queue depth is specified by this service.

> **Important:** *Application callback routines are called from the internal IP helper thread*.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *port*: Port number to listen on (1 through 0xFFFF).
- *socket_ptr*: Pointer to socket to use for the connection.
- *listen_queue_size*: Number of client connection requests that can be queued.
- *listen_callback*: Application function to call when the connection is received. If a NULL is specified, the listen callback feature is disabled.

### Return Values 

- **NX_SUCCESS** (0x00) Successful TCP port listen enable.
- **NX_MAX_LISTEN** (0x33) No more listen request structures are available. The constant NX_MAX_LISTEN_REQUESTS in ***nx_api.h*** defines how many active listen requests are possible.
- **NX_NOT_CLOSED** (0x35) The supplied server socket is not in a closed state.
- **NX_ALREADY_BOUND** (0x22) The supplied server socket is already bound to a port.
- **NX_DUPLICATE_LISTEN** (0x34) There is already an active listen request for this port.
- **NX_INVALID_PORT** (0x46) Invalid port specified.
- **NX_PTR_ERROR** (0x07) Invalid IP or socket pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Threads

### Preemption Possible

No

