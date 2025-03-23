Disable Reverse Address Resolution Protocol (RARP)

### Description

This service disables the RARP component of NetX Duo for the specific IP instance. For a multihome  system, this service disables RARP on all interfaces.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.

### Return Values 

- **NX_SUCCESS** (0x00) Successful RARP disable.
- **NX_NOT_ENABLED** (0x14) RARP was not enabled.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Initialization, threads

### Preemption Possible

No

