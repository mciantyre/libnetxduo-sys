Send an Ethernet packet.

### Description

This function sends out a link packet with layer 3 header already constructed or raw packet. Ethernet header will be added in this function.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *interface_index*: Index of the network interface to send the packet.
- *packet_ptr*: Pointer to the packet to send.
- *physical_address_msw*: Top 16 bits (47-32) of the destination MAC address.
- *physical_address_lsw*: Lower 32 bits (31-0) of the destination MAC address.
- *packet_type*: Type of the packet to send.

### Return Values

- **NX_SUCCESS** (0x00) Successful packet send.
- **NX_INVALID_INTERFACE** (0x4C) Device index points to an invalid network interface.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.

### Preemption Possible

No

