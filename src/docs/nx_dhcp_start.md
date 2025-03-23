Start DHCP processing

### Description

This service starts DHCP processing on all interfaces enabled for DHCP. By default the primary interface is enabled for DHCP when the application calls ***nx_dhcp_create***.

To verify when the IP instance is bound to an IP address on the DHCP Client interface, use *nx_ip_status_check* to see confirm the IP address is valid.

If there are other interfaces already running DHCP, this service will not affect them.

To start DHCP on a specific interface when multiple interfaces are enabled, use the ***nx_dhcp_interface_start*** service.

### Input Parameters

- *dhcp_ptr*: Pointer to previously created DHCP instance.

### Return Values

- **NX_SUCCESS**: (0x00) Successful DHCP start.  
- **NX_DHCP_ALREADY_STARTED**: (0x93) DHCP already started.
- NX_PTR_ERROR: (0x16) Invalid DHCP pointer.
- NX_CALLER_ERROR: (0x11) Invalid caller of service.

### Allowed From

Threads

