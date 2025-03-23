Set DHCP state change callback function on the specified interface

### Description

This service registers the specified callback function for notifying an application of DHCP state changes. The callback function input arguments are the interface index and the state the DHCP Client has transitioned to on that interface.

For more information about state change functions, see *nx_dhcp_state_change_notify*().

### Input Parameters

- *dhcp_ptr*: Pointer to previously created DHCP instance.
- *dhcp_interface_state_change_notify*: Application callback function pointer

### Return Values

- **NX_SUCCESS**: (0x00) Successful callback set.  
- NX_PTR_ERROR: (0x16) Invalid DHCP pointer.

### Allowed From

Threads, Initialization

