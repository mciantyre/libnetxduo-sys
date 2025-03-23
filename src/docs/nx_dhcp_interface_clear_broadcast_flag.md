Set or clear the broadcast flag on the specified interface

### Description

This service enables the DHCP Client host application to set or clear the broadcast flag in DHCP Client messages to the DHCP Server on the specified interface. For more details see ***nx_dhcp_clear_broadcast_flag***.

### Input Parameters

- *dhcp_ptr*: Pointer to DHCP control block
- *interface_index*: Index of interface to set the broadcast flag
- *clear_flag*: Value to set the broadcast flag to

### Return Values

- **NX_SUCCESS**: (0x00) Successfully set flag
- **NX_DHCP_INTERFACE_NOT_ENABLED**: (0xA4) Interface not enabled for DHCP
- NX_PTR_ERROR: (0x16) Invalid IP or DHCP pointer
- NX_INVALID_INTERFACE: (0x4C) Invalid network interface

### Allowed From

Threads, Initialization

