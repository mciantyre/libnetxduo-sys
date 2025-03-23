Enable IP packet fragmenting

### Description

This service enables IPv4 and IPv6 packet fragmenting and reassembling functionality. On creation of the IP task, this service is automatically disabled.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.

### Return Values  

- **NX_SUCCESS** (0x00) Successful IP fragment enable.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) IP Fragmentation features is not compiled into NetX Duo.

### Allowed From

Initialization, threads

### Preemption Possible

No

