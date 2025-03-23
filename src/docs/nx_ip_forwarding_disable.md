Disable IP packet forwarding

### Description

This service disables forwarding IP packets inside the NetX Duo IP component. On creation of the IP task, this service is automatically disabled.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.

### Return Values 

- **NX_SUCCESS** (0x00) Successful IP forwarding disable.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Initialization, threads, timers

### Preemption Possible  

No

