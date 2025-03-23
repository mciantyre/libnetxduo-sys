Retrieve data from packet

### Description

This service copies data from the supplied packet into the supplied buffer. The actual number of bytes copied is returned in the destination pointed to by **bytes_copied**.

Note that this service does not change internal state of the packet. The data being retrieved is still available in the packet. 

> **Caution:** *The destination buffer must be large enough to hold the packet's contents. If not, memory will be corrupted causing unpredictable results*.

### Parameters

- *packet_ptr*: Pointer to the source packet.
- *buffer_start*: Pointer to the start of the buffer area.
- *bytes_copied*: Pointer to the destination for the number of bytes copied.

### Return Values

- **NX_SUCCESS** (0x00) Successful packet data retrieve.
- **NX_INVALID_PACKET** (0x12) Invalid packet.
- **NX_PTR_ERROR** (0x07) Invalid packet, buffer start, or bytes copied pointer.

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

No

