Disable the specified interface to run DHCP 

### Description

This service disables the specified interface for running DHCP. It reinitializes the DHCP Client on this interface.

To restart the DHCP Client the application must re-enable the interface using *nx_dhcp_interface_enable* and restart DHCP by calling *nx_dhcp_interface_start*.

### Input Parameters

- *dhcp_ptr*: Pointer to DHCP control block.  
- *interface_index*: Index of interface to disable DHCP on

### Return Values

- **NX_SUCCESS**: (0x00) Successful DHCP create
- **NX_DHCP_INTERFACE_NOT_ENABLED**: (0xA4) Interface not enabled for DHCP
- NX_PTR_ERROR: (0x16) Invalid IP or DHCP pointer
- NX_CALLER_ERROR: (0x11) Invalid caller of this service
- NX_INVALID_INTERFACE: (0x4C) Invalid network interface

### Allowed From

Threads

