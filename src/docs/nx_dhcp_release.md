Release Leased IP address

### Description

This service releases the IP address obtained from a DHCP server by sending the RELEASE message to that server. It then reinitializes the DHCP Client. This service is applied to all interfaces enabled for DHCP.

The application can restart the DHCP Client by calling ***nx_dhcp_start***.

To release an address back to the DHCP server on a specific interface, use the *nx_dhcp_interface_release* service

### Input Parameters

- *dhcp_ptr*: Pointer to previously created DHCP instance.

### Return Values

- **NX_SUCCESS**: (0x00) Successful DHCP release.  
- **NX_DHCP_NOT_BOUND**: (0x94) The IP address has not been leased so it can't be released.
- NX_PTR_ERROR: (0x16) Invalid DHCP pointer.
- NX_CALLER_ERROR: (0x11) Invalid caller of this service.

### Allowed From

Threads

