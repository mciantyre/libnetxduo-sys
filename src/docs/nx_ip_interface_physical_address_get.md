Get the physical address of a network device

### Description

This service retrieves the physical address of a network interface from the IP instance.

### Parameters

- *ip_ptr*: IP control block pointer
- *interface_index*: Index of the network interface
- *physical_msw*: Pointer to destination for top 16 bits of the device MAC address
- *physical_lsw*: Pointer to destination for lower 32 bits of the device MAC address

### Return Values

- **NX_SUCCESS** (0x00) Successful get
- **NX_INVALID_INTERFACE** (0x4C) Interface index is not valid
- **NX_PTR_ERROR** (0x07) Invalid IP control block pointer or physical address pointer
- **NX_CALLER_ERROR** (0x11) Service is not called from system initialization or thread context.

### Allowed From

Initialization, threads

### Preemption Possible

No

