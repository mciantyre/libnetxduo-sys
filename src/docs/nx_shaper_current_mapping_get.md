Get current mapping of shaper.

### Description

This function gets the current pcp to HW queue mapping config.

### Parameters

- *interface_ptr*: Pointer to the interface instance.
- *pcp_list*: Pointer to the pcp list.
- *queue_id_list*: Pointer to the queue id list.
- *list_size*: Size of the list.

### Return Values

**NX_SUCCESS** (0x00) Successfully get mapping.
**NX_INVALID_PARAMETERS** (0x47) Invalid parameters.
**NX_NOT_SUPPORTED** (0x4B) Not supported.
**NX_NOT_SUCCESSFUL** (0x51) Not successful.

### Preemption Possible

No

