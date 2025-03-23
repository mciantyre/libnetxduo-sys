Send DHCP message to Server on a specific interface

### Description

This service sends a message to the DHCP server on the specified interface if that interface is enabled for DHCP. To send a RELEASE or DECLINE message, the application must use the ***nx_dhcp\[_interface\]_release*** or ***nx_dhcp_interface_decline*** services respectively.

The DHCP Client must be started to use this service except for sending the DHCP INFORM REQUEST message type.

This service is not intended for the host application to 'drive' the DHCP Client state machine.

### Input Parameters

- *dhcp_ptr*: Pointer to DHCP control block.
- *Interface_index*: Index of interface to send message on
- *dhcp_message_type*: Message request (defined in nxd_dhcp_client.h)

### Return Values

- **NX_SUCCESS**: (0x00) DHCP message sent  
- **NX_DHCP_NOT_STARTED**: (0x96) Invalid interface index
- **NX_DHCP_INVALID_MESSAGE**: (0x9B) Invalid message type to send
- **NX_DHCP_INTERFACE_NOT_ENABLED**: (0xA4) Interface not enabled for DHCP
- NX_PTR_ERROR: (0x16) Invalid DHCP pointer
- NX_CALLER_ERROR: (0x11) Invalid caller of this service.
- NX_INVALID_INTERFACE: (0x4C) Invalid network interface

### Allowed From

Threads

