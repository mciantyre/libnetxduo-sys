Add a packet receive callback.

### Description

This function adds a receive callback function to specified interface. Multiple callbacks callback functions can be added to each interface. They will be invoked one by one until the packet is consumed. Only packet matching registered packet_type will be passed to callback function. NX_LINK_PACKET_TYPE_ALL can be used to handle all types except TCP/IP ones. 

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *interface_index*: Index of the network interface to add the callback.
- *queue_ptr*: Pointer to the receive queue.
- *packet_type*: Type of the packet to receive.
- *callback_ptr*: Pointer to the callback function.
- *context*: Pointer to the context.

### Return Values

- **NX_SUCCESS** (0x00) Successful packet send.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_INVALID_INTERFACE** (0x4C) Device index points to an invalid network interface.

### Preemption Possible

No

