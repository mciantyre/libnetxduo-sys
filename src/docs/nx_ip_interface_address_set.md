Set interface IP address and network mask

### Description

This service sets the IPv4 address and network mask for the specified IP interface. To configure IPv6 interface address, application shall use the service ***nxd_ipv6_address_set***. 
 
> **Warning:** *The specified interface must be previously attached to the IP instance*. 

### Parameters 

- *ip_ptr*: Pointer to previously created IP instance.
- *interface_index*: Index of the interface attached to the NetX Duo instance.
- *ip_address*: New network interface IP address.
- *network_mask*: New interface network mask.

### Return Values

- **NX_SUCCESS** (0x00) Successful IP address set.
- **NX_INVALID_INTERFACE** (0x4C) Specified network interface is invalid.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_PTR_ERROR** (0x07) Invalid pointers.
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid IP address

### Allowed From

Initialization, threads

### Preemption Possible

No

