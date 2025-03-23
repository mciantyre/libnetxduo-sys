Enable Internet Control Message Protocol (ICMP)

### Description

This service enables the ICMP component for the specified IP instance. The ICMP component is responsible for handling Internet error messages and ping requests and replies.

> **Important:** *This service only enables ICMP for IPv4 service. To enable both ICMPv4 and ICMPv6, applications shall use the **nxd_icmp_enable** service*.

### Parameters 

- *ip_ptr*: Pointer to previously created IP instance.

### Return Values  

- **NX_SUCCESS** (0x00) Successful ICMP enable.
- **NX_ALREADY_ENABLED** (0x15) ICMP is already enabled.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Initialization, threads

### Preemption Possible

No

