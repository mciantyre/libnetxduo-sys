Get interface hardware capability

### Description

This service retrieves the capability flag from the specified network interface. To use this service, the NetX Duo library must be built with the option ***NX_ENABLE_INTERFACE_CAPABILITY*** enabled.

### Parameters

- *ip_ptr*: IP control block pointer
- *interface_index*: Index of the network interface
- *interface_capability_flag*: Pointer to memory space for the capability flag

### Return Values

- **NX_SUCCESS** (0x00) Successfully obtained interface capability information.
- **NX_NOT_SUPPORTED** (0x4B) Interface capability feature is not supported in this build.
- **NX_INVALID_INTERFACE** (0x4C) Interface index is not valid
- **NX_PTR_ERROR** (0x07) Invalid IP control block pointer or Invalid capability flag pointer
- **NX_CALLER_ERROR** (0x11) Service is not called from system initialization or thread context.

### Allowed From

Initialization, threads

### Preemption Possible

No

