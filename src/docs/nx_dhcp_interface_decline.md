Decline IP address from DHCP Server on the specified interface

### Description

This service sends the DECLINE message to the server to decline an IP address assigned by the DHCP server. It also reinitializes the DHCP Client. See ***nx_dhcp_decline*** for more details.

### Input Parameters

- *dhcp_ptr*: Pointer to previously created DHCP instance.
- *Interface_index*: Index of interface to decline IP address

### Return Values

- **NX_SUCCESS**: (0x00) DHCP decline message sent  
- **NX_DHCP_NOT_BOUND**: (0x94) DHCP Client not bound
- **NX_DHCP_INTERFACE_NOT_ENABLED**: (0xA4) Interface not enabled for DHCP
- NX_PTR_ERROR: (0x16) Invalid DHCP pointer
- NX_CALLER_ERROR: (0x11) Invalid caller of this service.
- NX_INVALID_INTERFACE: (0x4C) Invalid network interface

### Allowed From

Threads

