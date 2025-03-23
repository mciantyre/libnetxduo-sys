Configure whether address mapping is needed

### Description

This service configures whether IP address to MAC address mapping is needed for the specified network interface. This service is typically called from the interface device driver to notify the IP stack whether the underlying interface requires IP address to layer two (MAC) address mapping.

### Parameters

- *ip_ptr*: IP control block pointer
- *interface_index*: Index to the network interface
- *mapping_needed*: NX_TRUE -- address mapping needed NX_FALSE -- address mapping not needed

### Return Values

- **NX_SUCCESS** (0x00) Successful configure
- **NX_INVALID_INTERFACE** (0x4C) Device index is not valid
- **NX_PTR_ERROR** (0x07) Invalid IP control block pointer
- **NX_CALLER_ERROR** (0x11) Service is not called from system initialization or thread context.

### Allowed From

Thread

### Preemption Possible

No

