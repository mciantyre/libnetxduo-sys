Set the MTU value of a network interface

### Description

This service is used by the device driver to configure the IP MTU value for the specified network interface.

### Parameters

- *ip_ptr*: IP control block pointer
- *interface_index*: Index to the network interface
- *mtu_size*: IP MTU size

### Return Values  

- **NX_SUCCESS** (0x00) Successfully set MTU value
- **NX_INVALID_INTERFACE** (0x4C) Interface index is not valid
- **NX_PTR_ERROR** (0x07) Invalid IP control block pointer
- **NX_CALLER_ERROR** (0x11) Service is not called from system initialization or thread context.

### Allowed From

Initialization, threads

### Preemption Possible

No

