Wait for TCP socket to enter specific state

### Description

This service waits for the socket to enter the desired state. If the socket is not in the desired state, the service suspends according to the supplied wait option.

### Parameters

- *socket_ptr** Pointer to previously connected TCP socket instance.
- *desired_state** Desired TCP state. Valid TCP socket states are defined as follows:
	- **NX_TCP_CLOSED** (0x01)
	- **NX_TCP_LISTEN_STATE** (0x02)
	- **NX_TCP_SYN_SENT** (0x03)
	- **NX_TCP_SYN_RECEIVED** (0x04)
	- **NX_TCP_ESTABLISHED** (0x05)
	- **NX_TCP_CLOSE_WAIT** (0x06)
	- **NX_TCP_FIN_WAIT_1** (0x07)
	- **NX_TCP_FIN_WAIT_2** (0x08)
	- **NX_TCP_CLOSING** (0x09)
	- **NX_TCP_TIMED_WAIT** (0x0A)
	- **NX_TCP_LAST_ACK** (0x0B)
- *wait_option** Defines how the service behaves if the requested state is not present. The wait options are defined as follows:
	- **NX_NO_WAIT** (0x00000000)
	- **timeout value in ticks** (0x00000001 through 0xFFFFFFFF)

### Return Values  

- **NX_SUCCESS** (0x00) Successful state wait.
- **NX_PTR_ERROR** (0x07) Invalid socket pointer.
- **NX_NOT_SUCCESSFUL** (0x43) State not present within the specified wait time.
- **NX_WAIT_ABORTED** (0x1A) Requested suspension was aborted by a call to tx_thread_wait_abort.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.
- **NX_OPTION_ERROR** (0x0A) The desired socket state is invalid.

### Allowed From

Threads

### Preemption Possible

No

