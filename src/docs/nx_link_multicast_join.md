Join a multicast group.

### Description

This function handles the request to join the specified multicast group on a specified network device. 

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *interface_index*: Index of the network interface to join the multicast group.
- *physical_address_msw*: Top 16 bits (47-32) of the multicast address to join.
- *physical_address_lsw*: Lower 32 bits (31-0) of the multicast address to join.

### Return Values
- **NX_SUCCESS** (0x00) Successful multicast group join.
- **NX_INVALID_INTERFACE** (0x4C) Device index points to an invalid network interface.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.

### Preemption Possible

No

