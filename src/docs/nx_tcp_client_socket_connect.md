Connect client TCP socket

### Description

This service connects the previously created and bound TCP client socket to the specified server's port. Valid TCP server ports range from 0 through 0xFFFF. If the connection does not complete immediately, the service suspends according to the supplied wait option.

### Parameters

- *socket_ptr*: Pointer to previously created TCP socket instance.
- *server_ip*: Server's IP address.
- *server_port*: Server port number to connect to (1 through 0xFFFF).
- *wait_option*: Defines how the service behaves while the connection is being established. The wait options are defined as follows:
	- **NX_NO_WAIT** (0x00000000)
	- **NX_WAIT_FOREVER** (0xFFFFFFFF)
	- **timeout value in ticks** (0x00000001 through 0xFFFFFFFE)

### Return Values 

- **NX_SUCCESS** (0x00) Successful socket connect.
- **NX_NOT_BOUND** (0x24) Socket is not bound.
- **NX_NOT_CLOSED** (0x35) Socket is not in a closed state.
- **NX_IN_PROGRESS** (0x37) No wait was specified, the connection attempt is in progress.
- **NX_INVALID_INTERFACE** (0x4C) Invalid interface supplied.
- **NX_WAIT_ABORTED** (0x1A) Requested suspension was aborted by a call to tx_thread_wait_abort.
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid server IP address.
- **NX_INVALID_PORT** (0x46) Invalid port.
- **NX_PTR_ERROR** (0x07) Invalid socket pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Threads

### Preemption Possible

No

