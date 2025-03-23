Set network interface for DHCP instance

### Description

This service sets the network interface for the DHCP instance to connect to the DHCP Server on when running DHCP Client configured for a single network interface.

By default the DHCP Client runs on the primary interface. To run DHCP on a secondary service, use this service to set the secondary interface as the DHCP Client interface. The application must previously register the specified interface to the IP instance using the ***nx_ip_interface_attach*** service.

> **Note:** This service is intended for applications that intend to run the DHCP Client on only one interface. To run DHCP on multiple interfaces see ***nx_dhcp_interface_enable*** for more details.

### Input Parameters

- *dhcp_ptr*: Pointer to DHCP control block.  
- *index*: Index of device network interface

### Return Values

- **NX_SUCCESS**: (0x00) Interface is successfully set.
- **NX_INVALID_INTERFACE**: (0x4C) Invalid network interface
- **NX_DHCP_INTERFACE_ALREADY_ENABLED**: (0xA3) Interface enabled for DHCP
- **NX_DHCP_NO_RECORDS_AVAILABLE**: (0xA7) No record available for another 
- NX_PTR_ERROR: (0x16) Invalid DHCP pointer

### Allowed From

Threads

