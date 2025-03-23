Delete a DHCP instance

### Description

This service deletes a previously created DHCP instance.

### Input Parameters

- *dhcp_ptr*: Pointer to previously created DHCP instance.

### Return Values

- **NX_SUCCESS**: (0x00) Successful DHCP delete.
- NX_PTR_ERROR: (0x16) Invalid DHCP pointer.
- NX_CALLER_ERROR: (0x11) Invalid caller of this service.

### Allowed From

Threads

