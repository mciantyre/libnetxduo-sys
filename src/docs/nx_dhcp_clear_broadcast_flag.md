Set the DHCP broadcast flag

### Description

This service sets or clears the broadcast flag the DHCP message header for all interfaces enabled for DHCP. For some DHCP messages (e.g. DISCOVER) the broadcast flag is set to broadcast because the Client does not have an IP address.

clear_flag values:

- **NX_TRUE**: broadcast flag is cleared (request unicast response)
- **NX_FALSE**: broadcast flag is set (request broadcast response)

This service is intended for DHCP Clients that must go through a router to get to the DHCP Server, where the router rejects forwarding broadcast messages.

### Input Parameters

- *dhcp_ptr*: Pointer to DHCP control block  
- *clear_flag*: Value to set the broadcast flag to

### Return Values

- **NX_SUCCESS**: (0x00) Successfully set flag
- NX_PTR_ERROR: (0x16) Invalid IP or DHCP pointer

### Allowed From

Threads, Initialization

