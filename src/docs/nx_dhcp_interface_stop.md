Stop DHCP processing on the specified interface

### Description

This service stops DHCP processing on the specified interface if DHCP is already started. If there are no other interfaces running DHCP, the DHCP thread and timer are suspended.

### Input Parameters

- *dhcp_ptr*: Pointer to previously created DHCP instance.
- *Interface_index*: Interface on which to stop DHCP processing

### Return Values

- **NX_SUCCESS**: (0x00) Successful DHCP stop
- **NX_DHCP_NOT_STARTED**: (0x96) DHCP not started.
- NX_PTR_ERROR: (0x16) Invalid DHCP pointer.
- NX_CALLER_ERROR: (0x11) Invalid caller of service.
- NX_INVALID_INTERFACE: (0x4C) Invalid network interface

### Allowed From

Threads

