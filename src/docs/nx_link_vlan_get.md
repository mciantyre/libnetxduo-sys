Get VLAN tag from interface.

### Description

This function gets VLAN tag from interface, VLAN tag is comprised the PCP and VLAN ID, encoded in host byte order. The PCP is the 3 most significant bits and the VLAN ID is the 12 least significant bits. The PCP is used to prioritize the packet and the VLAN ID is used to identify the VLAN.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *interface_index*: Index of the network interface to get the VLAN tag.
- *vlan_tag*: Pointer to store the VLAN tag.

### Return Values

- **NX_SUCCESS** (0x00) Successful socket checksum disable.
- **NX_PTR_ERROR** (0x07) Invalid IP instance.
- **NX_INVALID_INTERFACE** (0x4C) Invalid interface index.
- **NX_NOT_FOUND** (0x4E) VLAN tag not found.

### Preemption Possible

No

