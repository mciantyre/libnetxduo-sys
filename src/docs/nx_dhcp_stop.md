Stops DHCP processing

### Description

This service stops DHCP processing on all interfaces that have started DHCP processing. If there are no interfaces processing DHCP, this service will suspend the DHCP Client thread, and inactivate the DHCP Client timer.

To stop DHCP on a specific interface if multiple interfaces are enabled for DHCP, use the *nx_dhcp_interface_stop* service.

### Input Parameters

- *dhcp_ptr*: Pointer to previously created DHCP instance.

### Return Values

- **NX_SUCCESS**: (0x00) Successful DHCP stop
- **NX_DHCP_NOT_STARTED**: (0x96) The DHCP instance not started.
- NX_PTR_ERROR: (0x16) Invalid DHCP pointer.
- NX_CALLER_ERROR: (0x11) Invalid caller of service.

### Allowed From

Threads

