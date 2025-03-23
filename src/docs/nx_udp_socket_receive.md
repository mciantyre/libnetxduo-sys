Receive datagram from UDP socket

### Description

This service receives an UDP datagram from the specified socket. If no datagram is queued on the specified socket, the caller suspends based on the supplied wait option.

> **Caution:** *If NX_SUCCESS is returned, the application is responsible for releasing the received packet when it is no longer needed*.

### Parameters

- *socket_ptr*: Pointer to previously created UDP socket instance.
- *packet_ptr*: Pointer to UDP datagram packet pointer.
- *wait_option*: Defines how the service behaves if a datagram is not currently queued on this socket. The wait options are defined as follows:
	- **NX_NO_WAIT** (0x00000000)
	- **NX_WAIT_FOREVER** (0xFFFFFFFF)
	- **timeout value in ticks** (0x00000001 through 0xFFFFFFFE)

### Return Values  

- **NX_SUCCESS** (0x00) Successful socket receive.
- **NX_NOT_BOUND** (0x24) Socket was not bound to any port.
- **NX_NO_PACKET** (0x01) There was no UDP datagram to receive.
- **NX_WAIT_ABORTED** (0x1A) Requested suspension was aborted by a call to tx_thread_wait_abort.
- **NX_PTR_ERROR** (0x07) Invalid socket or packet return pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Threads

### Preemption Possible

No

