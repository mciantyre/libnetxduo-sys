Set requested IP address for DHCP instance

### Description

This service sets the IP address for the DHCP Client to request from the DHCP Server on the first interface enabled for DHCP in the DHCP Client record. If the *skip_discover_message* flag is set, the DHCP Client skips the Discover message and sends a Request message.

To set the request for a specific IP for DHCP messages on a specific interface, use the *nx_dhcp_interface_request_client_ip* service.

### Input Parameters

- *dhcp_ptr*: Pointer to DHCP control block.  
- *client_ip_address*: IP address to request from DHCP server
- *skip_discover_message**:
    - If true, DHCP Client sends Request message
    - If false, it sends the Discover message.

### Return Values

- **NX_SUCCESS**: (0x00) Requested IP address is set.
- NX_PTR_ERROR: (0x16) Invalid DHCP pointer
- NX_DHCP_INVALID_IP_REQUEST: (0x9D) NULL IP address requested

### Allowed From

Threads

