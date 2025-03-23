Start DHCP processing on the specified interface

### Description

This service starts DHCP processing on the specified interface if that interface is enabled for DHCP. See ***nx_dhcp_interface_enable*** for more details about enabling an interface for DHCP. By default the primary interface is enabled for DHCP when the application calls ***nx_dhcp_create**.*

If there are no other interfaces running DHCP Client this service will start/resume the DHCP Client thread and (re)activate the DHCP Client timer.  
  
The application should use ***nx_ip_status_check*** to verify if an IP address is obtained.

### Input Parameters

- *dhcp_ptr*: Pointer to previously created DHCP instance.
- *Interface_index*: Index on which to start the DHCP Client

### Return Values

- **NX_SUCCESS**: (0x00) Successful DHCP start. 
- **NX_DHCP_ALREADY_STARTED**: (0x93) DHCP already started.
- NX_PTR_ERROR: (0x16) Invalid DHCP pointer.
- NX_CALLER_ERROR: (0x11) Invalid caller of service.
- NX_INVALID_INTERFACE: (0x4C) Invalid network interface

### Allowed From

Threads

