Copy packet

### Description

This service copies the information in the supplied packet to one or more new packets that are allocated from the supplied packet pool. If successful, the pointer to the new packet is returned in destination pointed to by **new_packet_ptr**.

### Parameters

- *packet_ptr*: Pointer to the source packet.
- *new_packet_ptr*: Pointer to the destination of where to return the pointer to the new copy of the packet.
- *pool_ptr*: Pointer to the previously created packet pool that is used to allocate one or more packets for the copy.
- *wait_option*: Defines how the service waits if there are no packets available. The wait options are defined as follows:
	- **NX_NO_WAIT** (0x00000000)
	- **NX_WAIT_FOREVER** (0xFFFFFFFF)
	- **timeout value in ticks** (0x00000001 through 0xFFFFFFFE)

### Return Values  

- **NX_SUCCESS** (0x00) Successful packet copy.
- **NX_NO_PACKET** (0x01) Packet not available for copy.
- **NX_INVALID_PACKET** (0x12) Empty source packet or copy failed.
- **NX_WAIT_ABORTED** (0x1A) Requested suspension was aborted by a call to tx_thread_wait_abort.
- **NX_INVALID_PARAMETERS** (0x4D) Packet size cannot support protocol.
- **NX_PTR_ERROR** (0x07) Invalid pool, packet, or destination pointer.
- **NX_UNDERFLOW** (0x02) Invalid packet prepend pointer.
- **NX_OVERFLOW** (0x03) Invalid packet append pointer.
- **NX_CALLER_ERROR** (0x11) A wait option was specified in initialization or in an ISR.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

No

