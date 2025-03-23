Set Gateway IP address

### Description

This service sets the IPv4 gateway IP address. All out-of-network traffic are routed to this gateway for transmission. The gateway must be directly accessible through one of the network interfaces. To configure IPv6 gateway address, use the service ***nxd_ipv6_default_router_add.*** 

### Parameters 

- *ip_ptr*: Pointer to previously created IP instance.
- *ip_address*: IP address of the gateway.

### Return Values

- **NX_SUCCESS** (0x00) Successful Gateway IP address set.
- **NX_PTR_ERROR** (0x07) Invalid IP instance pointer.
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid IP address.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Initialization, thread

### Preemption Possible

No

