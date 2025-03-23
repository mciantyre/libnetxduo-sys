Receive data from TCP socket

### Description

This service receives TCP data from the specified socket. If no data is queued on the specified socket, the caller suspends based on the supplied wait option.

> **Caution:** *If NX_SUCCESS is returned, the application is responsible for releasing the received packet when it is no longer needed*.

### Parameters

- *socket_ptr** Pointer to previously created TCP socket instance.
- *packet_ptr** Pointer to TCP packet pointer.
- *wait_option** Defines how the service behaves if do data are currently queued on this socket. The wait options are defined as follows:
	- **NX_NO_WAIT** (0x00000000)
	- **NX_WAIT_FOREVER** (0xFFFFFFFF)
	- **timeout value in ticks** (0x00000001 through 0xFFFFFFFE)

### Return Values  

- **NX_SUCCESS** (0x00) Successful socket data receive.
- **NX_NOT_BOUND** (0x24) Socket is not bound yet.
- **NX_NO_PACKET** (0x01) No data received.
- **NX_WAIT_ABORTED** (0x1A) Requested suspension was aborted by a call to tx_thread_wait_abort.
- **NX_NOT_CONNECTED** (0x38) The socket is no longer connected.
- **NX_PTR_ERROR** (0x07) Invalid socket or return packet pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Threads

### Preemption Possible

No

