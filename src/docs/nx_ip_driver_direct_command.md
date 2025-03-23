Issue command to network driver

### Description

This service provides a direct interface to the application's primary network interface driver specified during the ***nx_ip_create*** call. Application-specific commands can be used providing their numeric value is greater than or equal to **NX_LINK_USER_COMMAND**.

> **Important:** *To issue command for the secondary device, use the **nx_ip_driver_interface_direct_command** service*.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *command*: Numeric command code. Standard commands are defined as follows:
	- **NX_LINK_GET_STATUS** (10)
	- **NX_LINK_GET_SPEED** (11)
	- **NX_LINK_GET_DUPLEX_TYPE** (12)
	- **NX_LINK_GET_ERROR_COUNT** (13)
	- **NX_LINK_GET_RX_COUNT** (14)
	- **NX_LINK_GET_TX_COUNT** (15)
	- **NX_LINK_GET_ALLOC_ERRORS** (16)
	- **NX_LINK_USER_COMMAND** (50)
- *return_value_ptr*: Pointer to return variable in the caller.

### Return Values  

- **NX_SUCCESS** (0x00) Successful network driver direct command.
- **NX_UNHANDLED_COMMAND** (0x44) Unhandled or unimplemented network driver command.
- **NX_PTR_ERROR** (0x07) Invalid IP or return value pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_INVALID_INTERFACE** (0x4C) Invalid interface index.

### Allowed From

Threads

### Preemption Possible

No

