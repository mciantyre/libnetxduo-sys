Clears VLAN tag from interface.

### Description

This function clears VLAN tag from interface.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *interface_index*: Index of the network interface to clear the VLAN tag.

### Return Values

- **NX_SUCCESS** (0x00) Successful socket checksum disable.
- **NX_PTR_ERROR** (0x07) Invalid IP instance.
- **NX_INVALID_INTERFACE** (0x4C) Invalid interface index.

### Preemption Possible

No

