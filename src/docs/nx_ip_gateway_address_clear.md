Clear the IPv4 gateway address

### Description

This service clears the IPv4 gateway address configured in the instance. To clear an IPv6 default outer from the IP instance, applications shall use the service ***nxd_ipv6_default_router_delete.***

### Parameters

- *ip_ptr*: IP control block pointer

### Return Values  

- **NX_SUCCESS** (0x00) Successfully cleared the IP gateway address.
- **NX_PTR_ERROR** (0x07) Invalid IP control block
- **NX_CALLER_ERROR** (0x11) Service is not called from system initialization or thread context.

### Allowed From

Initialization, threads

### Preemption Possible

No

