Release IP address on the specified interface

### Description

This service releases the IP address obtained from a DHCP server on the specified interface and reinitializes the DHCP Client. The DHCP Client can be restarted by calling ***nx_dhcp_start***.

### Input Parameters

- *dhcp_ptr*: Pointer to previously created DHCP instance.

### Return Values

- **NX_SUCCESS**: (0x00) Successful DHCP release.
- **NX_DHCP_INTERFACE_NOT_ENABLED**: (0xA4) Interface not enabled for DHCP
- **NX_DHCP_NOT_BOUND**: (0x94) The IP address has not been leased so it can't be released.
- NX_PTR_ERROR: (0x16) Invalid DHCP pointer
- NX_CALLER_ERROR: (0x11) Invalid caller of this service.
- NX_INVALID_INTERFACE: (0x4C) Invalid network interface

### Allowed From

Threads

