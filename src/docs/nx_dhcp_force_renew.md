Send a force renew message 

### Description

This service enables the host application to send a force renew message on all interfaces enabled for DHCP. The DHCP Client must be in a BOUND state. This function sets the state to RENEW such that the DHCP Client will try to renew before the T1 timeout expires.

To send a force renew on a specific interface when multiple interfaces are DHCP-enabled, use ***nx_dhcp_interface_force_renew***.

### Input Parameters

- *dhcp_ptr*: Pointer to previously created DHCP instance.

### Return Values

- **NX_SUCCESS**: (0x00) Successfully sent force renew.
- **NX_DHCP_NOT_BOUND**: (0x94) Client IP address not bound.  
- NX_PTR_ERROR: (0x16) Invalid DHCP pointer.
- NX_CALLER_ERROR: (0x11) Invalid caller of this service.

### Allowed From

Threads

