Create a DHCP instance

### Description

This service creates a DHCP instance for the previously created IP instance. By default the primary interface is enabled for running DHCP. The name input, while not used in the NetX Duo implementation of DHCP Client, must follow RFC 1035 criteria for host names. The total length must not exceed 255 characters, the labels separate by dots must begin with a letter, and end with a letter or number, and may contain hyphens but no other non-alphanumeric character.

If the application would like to run DHCP another interface registered with the IP instance, (using *nx_ip_interface_attach*), the application can call ***nx_dhcp_set_interface_index*** to run DHCP on just that interface, or *nx_dhcp_interface_enable* to run DHCP on that interface as well. See description of these services for more details.

> **Note:** The application must make sure the DHCP Client packet pool payload can support the minimum DHCP message size specified by the RFC 2131 Section 2 (548 bytes of DHCP message data plus UDP, IP and physical network frame headers).

### Input Parameters

- *dhcp_ptr*: Pointer to DHCP control block.
- *ip_ptr*: Pointer to previously created IP instance.  
- *name_ptr*: Pointer to host name for DHCP instance.

### Return Values

- **NX_SUCCESS**: (0x00) Successful DHCP create
- **NX_DHCP_INVALID_NAME**: (0xA8) Invalid host name
- **NX_DHCP_INVALID_PAYLOAD**: (0x9C) Payload too small for DHCP message
- NX_PTR_ERROR: (0x16) Invalid IP or DHCP pointer

### Allowed From

Threads, Initialization

