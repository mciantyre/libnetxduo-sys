Disable raw packet sending/receiving

### Description

This service disables transmission and reception of raw IP packets for this IP instance. If the raw packet service was previously enabled, and there are raw packets in the receive queue, this service will release any received raw packets.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.

### Return Values 

- **NX_SUCCESS** (0x00) Successful IP raw packet disable.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Initialization, threads

### Preemption Possible

No

