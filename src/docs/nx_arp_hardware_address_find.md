Locate physical hardware address given an IP address

### Description

This service attempts to find a physical hardware address in the ARP cache that is associated with the supplied IP address.

### Parameters 

- *ip_ptr*: Pointer to previously created IP instance.
- *ip_address*: IP address to search for.
- *physical_msw*: Pointer to the variable for returning the top 16 bits (47-32) of the physical address.
- *physical_lsw*: Pointer to the variable for returning the lower 32 bits (31-0) of the physical address.

### Return Values

- **NX_SUCCESS** (0x00) Successful ARP hardware address find.
- **NX_ENTRY_NOT_FOUND** (0x16) Mapping was not found in the ARP cache.
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid IP address.
- **NX_PTR_ERROR** (0x07) Invalid IP or memory pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Threads

### Preemption Possible

No

