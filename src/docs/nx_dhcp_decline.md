Decline IP address from DHCP Server

### Description

This service declines an IP address leased from the DHCP server on all interfaces enabled for DHCP. If **NX_DHCP_CLIENT_SEND_ ARP_PROBE** is defined, the DHCP Client will send a DECLINE message if it detects that the IP address is already in use. See **ARP Probes** in Chapter One for more information on ARP probe configuration in the NetX Duo DHCP Client.

The application can use this service to decline its IP address if it discovers the address is in use by other means.

This service reinitializes the DHCP Client to that it can be restarted by calling ***nx_dhcp_start***.

### Input Parameters

- *dhcp_ptr*: Pointer to previously created DHCP instance.

### Return Values

- **NX_SUCCESS**: (0x00) Decline successfully sent  
- **NX_DHCP_NOT_BOUND**: (0x94) DHCP Client not bound
- NX_PTR_ERROR: (0x16) Invalid DHCP pointer
- NX_CALLER_ERROR: (0x11) Invalid caller of this service

### Allowed From

Threads

