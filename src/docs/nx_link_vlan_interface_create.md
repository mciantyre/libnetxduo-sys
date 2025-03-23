Create a VLAN interface.

### Description

This function creates a VLAN interface and bind to parent interface. Any packet received from parent interface will be dispatched to right interface according to the match of VLAN ID.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *interface_name*: Name of the interface.
- *ip_address*: IP address of the interface.
- *network_mask*: Network mask of the interface.
- *vlan_tag*: VLAN tag of the interface.
- *parent_interface_index*: Index of the parent interface.
- *interface_index_ptr*: Pointer to store the index of the interface.

### Return Values

**NX_SUCCESS** (0x00) Successful packet send.
**NX_DUPLICATED_ENTRY** (0x4D) Interface is duplicated.
**NX_NO_MORE_ENTRIES** (0x4F) No more entries.
**NX_INVALID_PARAMETERS** (0x47) Invalid parameters.

### Preemption Possible

No

