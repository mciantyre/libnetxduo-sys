Delete static IP to hardware mapping in ARP cache

### Description

This service finds and deletes a previously created static IP-to-physical address mapping in the ARP cache for the specified IP instance.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *ip_address*: IP address that was mapped statically.
- *physical_msw*: Top 16 bits (47 - **32) of the physical address that was mapped statically.
- *physical_lsw*: Lower 32 bits (31 - **0) of the physical address that was mapped statically.

### Return Values

- **NX_SUCCESS** (0x00) Successful ARP static entry delete.
- **NX_ENTRY_NOT_FOUND** (0x16) Static ARP entry was not found in the ARP cache.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid IP address.
- **NX_INVALID_PARAMETERS** (0x4D) Physical_msw and physical_lsw are both 0.

### Allowed From

Threads

### Preemption Possible

No

