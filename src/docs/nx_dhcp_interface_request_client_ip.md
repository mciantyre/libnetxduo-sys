Set requested IP address for DHCP instance on specified interface

### Description

This service sets the IP address for the DHCP Client to request from the DHCP Server on the specified interface, if that interface is enabled for DHCP (see *nx_dhcp_interface_enable*). If the ***skip_discover_message*** flag is set, the DHCP Client skips the Discover message and sends a Request message.

### Input Parameters

- *dhcp_ptr*: Pointer to DHCP control block.
- *Interface_index*: Index of interface to request IP address on
- *client_ip_address*: IP address to request from DHCP server
- *skip_discover_message*: If true, DHCP Client sends Request message; else it sends the Discover message.

### Return Values

- **NX_SUCCESS**: (0x00) Requested IP address is set.
- **NX_DHCP_INTERFACE_NOT_ENABLED**: (0xA4) Interface not enabled for DHCP
- NX_PTR_ERROR: (0x16) Invalid IP or DHCP pointer
- NX_INVALID_INTERFACE: (0x4C) Invalid network interface

### Allowed From

Threads

