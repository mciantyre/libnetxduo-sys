Retrieve a DHCP option from last server response on the specified interface

### Description

This service retrieves the specified DHCP option from the DHCP options buffer on the specified interface, if that interface is enabled for DHCP. If successful, the option data is copied into the specified buffer.

### Input Parameters

- *dhcp_ptr*: Pointer to previously created DHCP instance.
- *Interface_index*: Index on which to retrieve the specified option
- *request_option*: DHCP option, as specified by the RFCs. See the NX_DHCP_OPTION option: in *nxd_dhcp_client.h*.  
- *destination_ptr*: Pointer to the destination for the response string.  
- *destination_size*: Pointer to the size of the destination and on return, the destination to place the number of bytes returned.

### Return Values

- **NX_SUCCESS**: (0x00) Successful option retrieval.  
- **NX_DHCP_NOT_BOUND**: (0x94) IP address not assigned
- **NX_DHCP_DEST_TO_SMALL**: (0x95) Buffer is too small
- **NX_DHCP_PARSE_ERROR**: (0x97) DHCP Option not found in Server response.
- NX_PTR_ERROR: (0x16) Invalid DHCP pointer.
- NX_CALLER_ERROR: (0x11) Invalid caller of service.
- NX_INVALID_INTERFACE: (0x4C) Invalid network interface

### Allowed From

Threads

