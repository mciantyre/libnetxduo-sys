Retrieve network interface parameters

### Description

This service retrieves information on network parameters for the specified network interface. All data are retrieved in host byte order. 

> **Warning:** *ip_ptr must point to a valid NetX Duo IP structure. The specified interface, if not the primary interface, must be previously attached to the IP instance*.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *interface_index*: Index specifying network interface.
- *interface_name*: Pointer to the buffer that holds the name of the network interface.
- *ip_address*: Pointer to the destination for the IP address of the interface.
- *network_mask*: Pointer to destination for network mask.
- *mtu_size*: Pointer to destination for maximum transfer unit for this interface.
- *physical_address_msw*: Pointer to destination for top 16 bits of the device MAC address.
- *physical_address_lsw*: Pointer to destination for lower 32 bits of the device MAC address.

### Return Values   

- **NX_SUCCESS** (0x00) Interface information has been obtained.
- **NX_PTR_ERROR** (0x07) Invalid pointer input.
- **NX_INVALID_INTERFACE** (0x4C) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Service is not called from system initialization or thread context.

### Allowed From

Initialization, threads

### Preemption Possible

No

