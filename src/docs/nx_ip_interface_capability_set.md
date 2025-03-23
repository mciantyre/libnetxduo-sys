Set the hardware capability flag

### Description

This service is used by the network device driver to configure the capability flag for a specified network interface. To use this service, the NetX Duo library must be compiled with the option ***NX_ENABLE_INTERFACE_CAPABILITY*** defined.

### Parameters

- *ip_ptr*: IP control block pointer
- *interface_index*: Index of network interface
- *interface_capability_flag*: Capability flag for output

### Return Values

- **NX_SUCCESS** (0x00) Successfully set interface hardware capability flag.
- **NX_NOT_SUPPORTED** (0x4B) Interface capability feature is not supported in this build.
- **NX_INVALID_INTERFACE** (0x4C) Interface index is not valid 
- **NX_PTR_ERROR** (0x07) Invalid IP control block pointer 
- **NX_CALLER_ERROR** (0x11) Service is not called from system initialization or thread context.

### Allowed From

Initialization, threads

### Preemption Possible

No

