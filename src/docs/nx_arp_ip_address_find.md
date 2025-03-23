Locate IP address given a physical address

### Description

This service attempts to find an IP address in the ARP cache that is associated with the supplied physical address.

### Parameters 

- *ip_ptr*: Pointer to previously created IP instance.
- *ip_address*: Pointer to return IP address, if one is found that has been mapped.
- *physical_msw*: Top 16 bits (47-32) of the physical address to search for.
- *physical_lsw*: Lower 32 bits (31-0) of the physical address to search for.

### Return Values

- **NX_SUCCESS** (0x00) Successful ARP IP address find 
- **NX_ENTRY_NOT_FOUND** (0x16) Mapping was not found in the ARP cache.
- **NX_PTR_ERROR** (0x07) Invalid IP or memory pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.
- **NX_INVALID_PARAMETERS** (0x4D) Physical_msw and physical_lsw are both 0.

### Allowed From

Threads

### Preemption Possible

No

