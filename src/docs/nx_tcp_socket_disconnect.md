Disconnect client and server socket connections

### Description

This service disconnects an established client or server socket connection. A disconnect of a server socket should be followed by an unaccept request, while a client socket that is disconnected is left in a state ready for another connection request. If the disconnect process cannot finish immediately, the service suspends according to the supplied wait option.

### Parameters

- *socket_ptr** Pointer to previously connected client or server socket instance.
- *wait_option** Defines how the service behaves while the disconnection is in progress. The wait options are defined as follows:
	- **NX_NO_WAIT** (0x00000000)
	- **NX_WAIT_FOREVER** (0xFFFFFFFF)
	- **timeout value in ticks** (0x00000001 through 0xFFFFFFFE)

### Return Values 

- **NX_SUCCESS** (0x00) Successful socket disconnect.
- **NX_NOT_CONNECTED** (0x38) Specified socket is not connected.
- **NX_IN_PROGRESS** (0x37) Disconnect is in progress, no wait was specified.
- **NX_WAIT_ABORTED** (0x1A) Requested suspension was aborted by a call to tx_thread_wait_abort.
- **NX_PTR_ERROR** (0x07) Invalid socket pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Threads

### Preemption Possible

Yes 

