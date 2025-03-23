Delete a shaper.

### Description

This function deletes a shaper from interface instance, unlink the shaper container with IP interface when there is no shaper exists.

### Parameters

*interface_ptr*: Pointer to the interface instance.
*shaper*: Pointer to the shaper.

### Return Values

**NX_SUCCESS** (0x00) Successful shaper delete.
**NX_INVALID_PARAMETERS** (0x47) Invalid parameters.
**NX_ENTRY_NOT_FOUND** (0x4A) Entry not found.

### Preemption Possible

No

