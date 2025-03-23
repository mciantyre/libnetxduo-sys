Get the DHCP Client's DHCP server IP address on the specified interface

### Description

This service retrieves the DHCP Client DHCP server IP address on the specified interface if that interface is enabled for DHCP. The DHCP Client must be in the Bound state. After starting the DHCP Client on that interface, the host application can either use the ***nx_ip_status_check*** service to verify the IP address is set, or it can use the DHCP Client state change callback and query the DHCP Client state is NX_DHCP_STATE_BOUND. See ***nx_dhcp_state_change_notify*** for more details about setting the state change callback function.

### Input Parameters

- *dhcp_ptr*: Pointer to DHCP control block.
- *Interface_index*: Index of interface to obtain IP address
- *server_address*: Pointer to server IP address

### Return Values

- **NX_SUCCESS**: (0x00) DHCP server address returned
- **NX_DHCP_NO_INTERFACES_ENABLED**: (0xA5) No interfaces enabled for DHCP
- **NX_DHCP_NOT_BOUND**: (0x94) DHCP Client not bound
- NX_PTR_ERROR: (0x16) Invalid DHCP pointer
- NX_CALLER_ERROR: (0x11) Invalid caller of this service.
- NX_INVALID_INTERFACE: (0x4C) Invalid network interface

### Allowed From

Threads

