Set DHCP state change callback function

### Description

This service registers the specified callback function dhcp_state_change_notify for notifying an application of DHCP state changes. The callback function supplies the state the DHCP Client has transitioned into.

Following are values associated with the various DHCP states:

- **NX_DHCP_STATE_BOOT**: 1
- **NX_DHCP_STATE_INIT**: 2
- **NX_DHCP_STATE_SELECTING**: 3
- **NX_DHCP_STATE_REQUESTING**: 4
- **NX_DHCP_STATE_BOUND**: 5
- **NX_DHCP_STATE_RENEWING**: 6
- **NX_DHCP_STATE_REBINDING**: 7
- **NX_DHCP_STATE_FORCERENEW**: 8
- **NX_DHCP_STATE_ADDRESS_PROBING**: 9

### Input Parameters

- *dhcp_ptr*: Pointer to previously created DHCP instance.
- *dhcp_state_change_notify*: State change callback function pointer

### Return Values

- **NX_SUCCESS**: (0x00) Successful callback set.  
- NX_PTR_ERROR: (0x16) Invalid DHCP pointer.
- NX_CALLER_ERROR: (0x11) Invalid caller of service.

### Allowed From

Threads, Initialization

