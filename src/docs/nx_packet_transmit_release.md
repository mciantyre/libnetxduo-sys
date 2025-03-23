Release a transmitted packet

### Description

For non-TCP packets, this service releases a transmitted packet, including any additional packets chained to the specified packet. If another thread is blocked on packet allocation, it is given the packet and resumed. For a transmitted TCP packet, the packet is marked as being transmitted but not released till the packet is acknowledged. This service is typically called from the application's network driver after a packet is transmitted.

**Warning:** *The network driver should remove the physical media header and adjust the length of the packet before calling this service*.

### Parameters

- *packet_ptr*: Packet pointer.

### Return Values 

- **NX_SUCCESS** (0x00) Successful transmit packet release.
- **NX_PTR_ERROR** (0x07) Invalid packet pointer.
- **NX_UNDERFLOW** (0x02) Prepend pointer is less than payload start.
- **NX_OVERFLOW** (0x03) Append pointer is greater than payload end.

### Allowed From

Initialization, threads, timers, Application network drivers (including ISRs)

### Preemption Possible

Yes

