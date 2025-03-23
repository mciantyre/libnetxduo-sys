Enable the specified interface to run DHCP 

### Description

This service enables the specified interface for running DHCP. By default the primary interface is enabled for DHCP Client. At this point, DHCP can be started on this interface either by calling ***nx_dhcp_interface_start*** or to start DHCP on all enabled interfaces ***nx_dhcp_start***.

> **Note:** The application must first register this interface with the IP instance, using *nx_ip_interface_attach.*

Further, there must be an available DHCP Client interface 'record' to add this interface to the list of enabled interfaces. By default NX_DHCP_CLIENT_MAX_RECORDS is defined to 1. Set this option to the maximum number of interfaces expected to run DHCP Client simultaneously. Typically NX_DHCP_CLIENT_MAX_RECORDS will equal NX_MAX_PHYSICAL_INTERFACES; however, if a device has more physical interfaces than it expects to run DHCP Client, it can save memory by setting NX_DHCP_CLIENT_MAX_RECORDS to less than that number. There is not a one to one mapping of physical interfaces with DHCP Client interface records.

The difference between this service and ***nx_dhcp_set_interface_index*** is the latter sets only a single interface to run DHCP whereas this service simply adds the specified interface to the list of Client interfaces enabled for DHCP.

To disable an interface for DHCP, the application can call the ***nx_dhcp_interface_disable*** service.

### Input Parameters

- *dhcp_ptr*: Pointer to DHCP control block.  
- *interface_index*: Index of interface to enable DHCP on

### Return Values

- **NX_SUCCESS**: (0x00) Successful DHCP enable
- **NX_DHCP_NO_RECORDS_AVAILABLE**: (0xA7) No record available for another Interface to be enabled for DHCP
- **NX_DHCP_INTERFACE_ALREADY_ENABLED**: (0xA3) Interface enabled for DHCP
- NX_PTR_ERROR: (0x16) Invalid IP or DHCP pointer
- NX_INVALID_INTERFACE: (0x4C) Invalid network interface

### Allowed From

Threads, Initialization

