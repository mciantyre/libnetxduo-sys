Enable UDP component of NetX Duo

### Description

This service enables the User Datagram Protocol (UDP) component of NetX Duo. After enabled, UDP datagrams may be sent and received by the application.

### Parameters 

- *ip_ptr** Pointer to previously created IP instance.

### Return Values

- **NX_SUCCESS** (0x00) Successful UDP enable.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_ALREADY_ENABLED** (0x15) This component has already been enabled.

### Allowed From

Initialization, threads, timers

### Preemption Possible

No

