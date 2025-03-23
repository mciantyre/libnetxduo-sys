Set IPv4 address and network mask

### Description

This service sets IPv4 address and network mask for the primary network interface.

> **Important:** *To set IP address and network mask for the secondary device, use the service **nx_ip_interface_address_set***.

### Parameters 

- *ip_ptr*: Pointer to previously created IP instance.
- *ip_address*: New IP address.
- *network_mask*: New network mask.

### Return Values

- **NX_SUCCESS** (0x00) Successful IP address set.
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid IP address.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Initialization, threads

### Preemption Possible

No

