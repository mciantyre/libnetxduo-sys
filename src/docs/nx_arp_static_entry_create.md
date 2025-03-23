Create static IP to hardware mapping in ARP cache

### Description

This service creates a static IP-to-physical address mapping in the ARP cache for the specified IP instance. Static ARP entries are not subject to ARP periodic updates.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *ip_address*: IP address to map.
- *physical_msw*: Top 16 bits (47-32) of the physical address to map.
- *physical_lsw*: Lower 32 bits (31-0) of the physical address to map.

### Return Values 

- **NX_SUCCESS** (0x00) Successful ARP static entry create.
- **NX_NO_MORE_ENTRIES** (0x17) No more ARP entries are available in the ARP cache.
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid IP address.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.
- **NX_INVALID_PARAMETERS** (0x4D) Physical_msw and physical_lsw are both 0.

### Allowed From

Initialization, threads

### Preemption Possible

No

