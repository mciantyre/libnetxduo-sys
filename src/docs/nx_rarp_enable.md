Enable Reverse Address Resolution Protocol (RARP)

### Description

This service enables the RARP component of NetX Duo for the specific IP instance. The RARP components searches through all attached network interfaces for zero IP address. A zero IP address indicates the
interface does not have IP address assignment yet. RARP attempts to resolve the IP address by enabling RARP process on that interface.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.

### Return Values

- **NX_SUCCESS** (0x00) Successful RARP enable.
- **NX_IP_ADDRESS_ERROR** (0x21) IP address is already valid.
- **NX_ALREADY_ENABLED** (0x15) RARP was already enabled.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Initialization, threads, timers

### Preemption Possible

No

