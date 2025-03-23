Send a raw packet.

### Description

This function sends out a link packet with layer 2 header already constructed or raw packet. 

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *interface_index*: Index of the network interface to send the packet.
- *packet_ptr*: Pointer to the packet to send.

### Return Values

- **NX_SUCCESS** (0x00) Successful packet send.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_INVALID_INTERFACE** (0x4C) Device index points to an invalid network interface.

### Preemption Possible

No

