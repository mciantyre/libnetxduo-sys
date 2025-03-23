Enable TCP component of NetX Duo

### Description

This service enables the Transmission Control Protocol (TCP) component of NetX Duo. After enabled, TCP connections may be established by the application.

### Parameters  

- *ip_ptr*: Pointer to previously created IP instance.

### Return Values

- **NX_SUCCESS** (0x00) Successful TCP enable.
- **NX_ALREADY_ENABLED** (0x15) TCP is already enabled.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Initialization, threads, timers

### Preemption Possible

No

