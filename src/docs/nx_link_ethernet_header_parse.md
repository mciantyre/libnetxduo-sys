Parse an Ethernet header.

### Description

This function parses Ethernet packet and return each file of header.

### Parameters

- *packet_ptr*: Pointer to the packet to parse.
- *destination_msb*: Pointer to store the destination MAC address MSB.
- *destination_lsb*: Pointer to store the destination MAC address LSB.
- *source_msb*: Pointer to store the source MAC address MSB.
- *source_lsb*: Pointer to store the source MAC address LSB.
- *ether_type*: Pointer to store the Ethernet type.
- *vlan_tag*: Pointer to store the VLAN tag.
- *vlan_tag_valid*: Pointer to store the VLAN tag valid.
- *header_size*: Pointer to store the header size.

### Return Values

- **NX_SUCCESS** (0x00) Successful packet send.

### Preemption Possible

No

