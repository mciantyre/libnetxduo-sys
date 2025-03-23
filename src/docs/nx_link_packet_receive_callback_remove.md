Remove a packet receive callback.

### Description

This function removes a receive callback function to specified interface.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *interface_index*: Index of the network interface to remove the callback.
- *queue_ptr*: Pointer to the receive queue.

### Return Values

- **NX_SUCCESS** (0x00) Successful packet send.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_INVALID_INTERFACE** (0x4C) Device index points to an invalid network interface.

### Preemption Possible

No

