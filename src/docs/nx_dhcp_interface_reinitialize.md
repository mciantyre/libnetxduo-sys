Clear the DHCP client network parameters on the specified interface 

### Description

This service clears the network parameters (IP address, network address and network mask) on the specified interface if that interface is enabled for DHCP (see ***nx_dhcp_interface_enable***). See ***nx_dhcp_reinitialize*** for more details.

### Input Parameters

- *dhcp_ptr*: Pointer to previously created DHCP instance
- *interface_index*: Index of interface to reinitialize

### Return Values

- **NX_SUCCESS**: (0x00) Interface successfully reinitialized
- **NX_DHCP_INTERFACE_NOT_ENABLED**: (0xA4) Interface not enabled for DHCP
- NX_PTR_ERROR: (0x16) Invalid DHCP pointer
- NX_CALLER_ERROR: (0x11) Invalid caller of this service.
- NX_INVALID_INTERFACE: (0x4C) Invalid network interface

### Allowed From

Threads

