Set dynamic ARP entry

### Description

This service allocates a dynamic entry from the ARP cache and sets up the specified IP to physical address mapping. If a zero physical address is specified, an actual ARP request is sent to the network in order to have the physical address resolved. Also note that this entry will be removed if ARP aging is active or if the ARP cache is exhausted and this is the least recently used ARP entry.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *ip_address*: IP address to map.
- *physical_msw*: Top 16 bits (47-32) of the physical address.
- *physical_lsw*: Lower 32 bits (31-0) of the physical address.

### Return Values

- **NX_SUCCESS** (0x00) Successful ARP dynamic entry set.
- **NX_NO_MORE_ENTRIES** (0x17) No more ARP entries are available in the ARP cache.
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid IP address.
- **NX_PTR_ERROR** (0x07) Invalid IP instance pointer.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Threads

### Preemption Possible

No

