Get length of packet data

### Description

This service gets the length of the data in the specified packet.

### Parameters

- *packet_ptr*: Pointer to the packet.
- *length*: Destination for the packet length.

### Return Values  

- **NX_SUCCESS** (0x00) Successful packet length get.
- **NX_PTR_ERROR** (0x07) Invalid packet pointer.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

No

