Append data to end of packet

### Description

This service appends data to the end of the specified packet. The supplied data area is copied into the packet. If there is not enough memory available, and the chained packet feature is enabled, one or more packets will be allocated to satisfy the request. If the chained packet feature is not enabled, *NX_SIZE_ERROR* is returned.

### Parameters

- *packet_ptr*: Packet pointer.
- *data_start*: Pointer to the start of the user's data area to append to the packet.
- *data_size*: Size of user's data area.
- *pool_ptr*: Pointer to packet pool from which to allocate another packet if there is not enough room in the current packet.
- *wait_option*: Defines how the service behaves if there are no packets available. The wait options are defined as follows:
	- **NX_NO_WAIT** (0x00000000)
	- **NX_WAIT_FOREVER** (0xFFFFFFFF)
	- **timeout value in ticks** (0x00000001 through 0xFFFFFFFE)

### Return Values

- **NX_SUCCESS** (0x00) Successful packet append.
- **NX_NO_PACKET** (0x01) No packet available.
- **NX_WAIT_ABORTED** (0x1A) Requested suspension was aborted by a call to tx_thread_wait_abort.
- **NX_INVALID_PARAMETERS** (0x4D) Packet size cannot support protocol.
- **NX_UNDERFLOW** (0x02) Prepend pointer is less than payload start.
- **NX_OVERFLOW** (0x03) Append pointer is greater than payload end.
- **NX_PTR_ERROR** (0x07) Invalid pool, packet, or data Pointer.
- **NX_SIZE_ERROR** (0x09) Invalid data size.
- **NX_CALLER_ERROR** (0x11) Invalid wait option from nonthread.

### Allowed From

Initialization, threads, timers, and ISRs (application network drivers)

### Preemption Possible

No

