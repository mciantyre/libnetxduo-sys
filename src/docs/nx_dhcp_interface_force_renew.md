Send a force renew message on the specified interface

### Description

This service enables the host application to send a force renew message on the input interface as long as that interface has been enabled for DHCP (see ***nx_dhcp_interface_enable***). The DHCP Client on the specified interface must be in a BOUND state. This function sets the state to RENEW such that the DHCP Client will try to renew before the T1 timeout expires.

### Input Parameters

- *dhcp_ptr*: Pointer to previously created DHCP instance.

### Return Values

- **NX_SUCCESS**: (0x00) Successfully sent force renew.  
- **NX_DHCP_INTERFACE_NOT_ENABLED**: (0xA4) Interface not enabled for DHCP
- NX_PTR_ERROR: (0x16) Invalid IP or DHCP pointer
- NX_INVALID_INTERFACE: (0x4C) Invalid network interface

### Allowed From

Threads

