Release previously allocated packet

### Description

This service releases a packet, including any additional packets chained to the specified packet. If another thread is blocked on packet allocation, it is given the packet and resumed.

> NOTE  
> *The application must prevent releasing a packet more than once, because doing so will cause unpredictable results*.

### Parameters

- *packet_ptr*: Packet pointer.

### Return Values 

- **NX_SUCCESS** (0x00) Successful packet release.
- **NX_PTR_ERROR** (0x07) Invalid packet pointer.
- **NX_UNDERFLOW** (0x02) Prepend pointer is less than payload start.
- **NX_OVERFLOW** (0x03) Append pointer is greater than payload end.

### Allowed From

Initialization, threads, timers, and ISRs (application network drivers)

### Preemption Possible

Yes

