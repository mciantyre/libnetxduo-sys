Retrieve IPv4 address and network mask

### Description

This service retrieves IPv4 address and its subnet mask of the primary network interface.

> **Important:** *To obtain information of the secondary device, use the service **nx_ip_interface_address_get***.

### Parameters 

- *ip_ptr*: Pointer to previously created IP instance.
- *ip_address*: Pointer to destination for IP address.
- *network_mask*: Pointer to destination for network mask.

### Return Values 

- **NX_SUCCESS** (0x00) Successful IP address get.
- **NX_PTR_ERROR** (0x07) Invalid IP or return variable pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Initialization, threads

### Preemption Possible

No

