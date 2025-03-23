Set callback function for adding user supplied options

### Description

This service registers the specified callback function for adding user supplied options.

If the callback function specified, the applications can add user supplied options into the packet by iface_index and message_type.

> **Note:** In user's routine. Applications must follow the DHCP options format when add user supplied options. The total size of user options must be less or equal to user_option_length, and update the user_option_length as real options length. Return NX_TRUE if add options successfully, else return NX_FALSE.

### Input Parameters

- *dhcp_ptr*: Pointer to previously created DHCP instance.
- *dhcp_user_option_add*: Pointer to user option add function.

### Return Values

- **NX_SUCCESS**: (0x00) Successful callback set.
- NX_PTR_ERROR: (0x16) Invalid pointer.

### Allowed From

Threads

