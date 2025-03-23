Accept TCP connection

### Description

This service accepts (or prepares to accept) a TCP client socket connection request for a port that was previously set up for listening. This service may be called immediately after the application calls the listen or re-listen service or after the listen callback routine is called when the client connection is actually present. If a connection cannot not be established right away, the service suspends according to the supplied wait option.

> **Warning:** *The application must call ***nx_tcp_server_socket_unaccept*** after the connection is no longer needed to remove the server socket's binding to the server port*.

> **Important:** *Application callback routines are called from within the IP's helper thread*.

### Parameters

- *socket_ptr*: Pointer to the TCP server socket control block.
- *wait_option*: Defines how the service behaves while the connection is being established. The wait options are defined as follows:
	- **NX_NO_WAIT** (0x00000000)
	- **NX_WAIT_FOREVER** (0xFFFFFFFF)
	- **timeout value in ticks** (0x00000001 through 0xFFFFFFFE)

### Return Values  

- **NX_SUCCESS** (0x00) Successful TCP server socket accept (passive connect).
- **NX_NOT_LISTEN_STATE** (0x36) The server socket supplied is not in a listen state.
- **NX_IN_PROGRESS** (0x37) No wait was specified, the connection attempt is in progress.
- **NX_WAIT_ABORTED** (0x1A) Requested suspension was aborted by a call to tx_thread_wait_abort.
- **NX_PTR_ERROR** (0x07) Socket pointer error.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Initialization, threads

### Preemption Possible

No

