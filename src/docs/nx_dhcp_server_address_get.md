Get the DHCP Client's DHCP server IP address

### Description

This service retrieves the DHCP Client DHCP server IP address on the first interface enabled for DHCP found in the DHCP Client record. The caller can only use this service after the DHCP Client is bound to an IP address assigned by the DHCP Server. The host application can use the ***nx_ip_status_check*** service to verify IP address is set, or it can use the ***nx_dhcp_state_change_notify*** and query the DHCP Client state is **NX_DHCP_STATE_BOUND**. See *nx_dhcp_state_change_notify* for more details about setting the state change callback function.

To find the DHCP server on a specific interface when multiple interfaces are enabled for DHCP Client, use the *nx_dhcp_interface_server_address_get* service

### Input Parameters

- *dhcp_ptr*: Pointer to DHCP control block.
- *server_address*: Pointer to server IP address

### Return Values

- **NX_SUCCESS**: (0x00) DHCP server address returned
- **NX_DHCP_NO_INTERFACES_ENABLED**: (0xA5) No interfaces enabled for DHCP
- NX_PTR_ERROR: (0x16) Invalid input pointer
- NX_CALLER_ERROR: (0x11) Invalid caller of this service.

### Allowed From

Threads

