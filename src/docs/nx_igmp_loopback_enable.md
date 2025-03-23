Enable IGMP loopback

### Description

This service enables IGMP loopback for all subsequent multicast groups joined.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.

### Return Values  

- **NX_SUCCESS** (0x00) Successful IGMP loopback disable.
- **NX_NOT_ENABLED** (0x14) IGMP is not enabled.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Caller is not a thread or initialization.

### Allowed From

Initialization, threads

### Preemption Possible

No

