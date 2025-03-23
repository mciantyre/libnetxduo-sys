Get the IPv4 gateway address

### Description

This service retrieves the IPv4 gateway address configured in the IP instance.

### Parameters

- *ip_ptr*: IP control block pointer
- *ip_address*: Pointer to the memory where the gateway address is stored

### Return Values

- **NX_SUCCESS** (0x00) Successful get 
- **NX_PTR_ERROR** (0x07) Invalid IP control block pointer or ip address pointer
- **NX_NOT_FOUND** (0x4E) Gateway address not found
- **NX_CALLER_ERROR** (0x11) Service is not called from system initialization or thread context.

### Allowed From

Initialization, threads

### Preemption Possible

No

