Send DHCP message to Server

### Description

This service sends the specified DHCP message to the DHCP server on the first interface enabled for DHCP found in the DHCP Client record. To send a RELEASE or DECLINE message, the application must use the ***nx_dhcp\[_interface\]_release*** or ***nx_dhcp_interface_decline*** services respectively.

The DHCP Client must be started to use this service except for sending the INFORM_REQUEST message type.

> **Note:** This service is not intended for the host application to 'drive' the DHCP Client state machine.

### Input Parameters

- *dhcp_ptr*: Pointer to DHCP control block.  
- *dhcp_message_type*: Message request (defined in *nxd_dhcp_client.h*)

### Return Values

- **NX_SUCCESS**: (0x00) DHCP message sent  
- **NX_DHCP_NOT_STARTED**: (0x96) Invalid interface index
- **NX_DHCP_INVALID_MESSAGE**: (0x9B) Invalid message type to send
- NX_PTR_ERROR: (0x16) Invalid pointer input

### Allowed From

Threads

