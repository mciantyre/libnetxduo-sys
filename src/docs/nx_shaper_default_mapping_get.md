Get default mapping of shaper.

### Description

This function gets the default pcp to HW queue mapping config.

### Parameters

- *interface_ptr*: Pointer to the interface instance.
- *pcp_list*: Pointer to the pcp list.
- *queue_id_list*: Pointer to the queue id list.
- *list_size*: Size of the list.

### Return Values

**NX_SUCCESS** Successfully get mapping.
**NX_INVALID_PARAMETERS** (0x47) Invalid parameters.
**NX_NOT_SUPPORTED** (0x4B) Not supported.

### Preemption Possible

No

