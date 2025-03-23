Send data through a TCP socket

### Description

This service sends TCP data through a previously connected TCP socket. If the receiver's last advertised window size is less than this request, the service optionally suspends based on the wait option specified. This service guarantees that no packet data larger than MSS is sent to the IP layer. 

> **Warning:** *Unless an error is returned, the application should not release the packet after this call. Doing so will cause unpredictable results because the network driver will also try to release the packet after 
transmission*.

### Parameters

- *socket_ptr** Pointer to previously connected TCP socket instance.
- *packet_ptr** TCP data packet pointer.
- *wait_option** Defines how the service behaves if the request is greater than the window size of the receiver. The wait options are defined as follows:
	- **NX_NO_WAIT** (0x00000000)
	- **NX_WAIT_FOREVER** (0xFFFFFFFF)
	- **timeout value in ticks** (0x00000001 through 0xFFFFFFFE)

### Return Values

- **NX_SUCCESS** (0x00) Successful socket send.
- **NX_NOT_BOUND** (0x24) Socket was not bound to any port.
- **NX_NO_INTERFACE_ADDRESS** (0x50) No suitable outgoing interface found.
- **NX_NOT_CONNECTED** (0x38) Socket is no longer connected.
- **NX_WINDOW_OVERFLOW** (0x39) Request is greater than receiver's advertised window size in bytes.
- **NX_WAIT_ABORTED** (0x1A) Requested suspension was aborted by a call to tx_thread_wait_abort.
- **NX_INVALID_PACKET** (0x12) Packet is not allocated.
- **NX_TX_QUEUE_DEPTH** (0x49) Maximum transmit queue depth has been reached.
- **NX_OVERFLOW** (0x03) Packet append pointer is invalid.
- **NX_PTR_ERROR** (0x07) Invalid socket pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.
- **NX_UNDERFLOW** (0x02) Packet prepend pointer is invalid.

### Allowed From

Threads

### Preemption Possible

No

