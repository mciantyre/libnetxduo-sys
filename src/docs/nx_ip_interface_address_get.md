Retrieve interface IP address

### Description

This service retrieves the IPv4 address of a specified network interface. To retrieve IPv6 address, application shall use the service ***nxd_ipv6_address_get***

> **Caution:** *The specified device, if not the primary device, must be previously attached to the IP instance*.

### Parameters 

- *ip_ptr*: Pointer to previously created IP instance.
- *interface_index*: Interface index, the same value as the index to the network interface attached to the IP instance.
- *ip_address*: Pointer to destination for the device interface IP address.
- *network_mask*: Pointer to destination for the device interface network mask.

### Return Values

- **NX_SUCCESS** (0x00) Successful IP address get.
- **NX_INVALID_INTERFACE** (0x4C) Specified network interface is invalid.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.

### Allowed From

Initialization, threads

### Preemption Possible

No

