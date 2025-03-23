Retrieve a DHCP option from last server response

### Description

This service retrieves the specified DHCP option from the DHCP options buffer on the first interface enabled for DHCP found on the DHCP Client record. If successful, the option data is copied into the specified buffer.

### Input Parameters

- *dhcp_ptr*: Pointer to previously created DHCP instance.  
- *request_option*: DHCP option, as specified by the RFCs. See the NX_DHCP_OPTION option in *nxd_dhcp_client.h*.
- *destination_ptr*: Pointer to the destination for the response string.  
- *destination_size*: Pointer to the size of the destination and on return, the destination to place the number of bytes returned.

### Return Values

- **NX_SUCCESS**: (0x00) Successful option retrieval.  
- **NX_DHCP_NOT_BOUND**: (0x94) DHCP Client not bound.
- **NX_DHCP_NO_INTERFACES_ENABLED**: (0xA5) No interfaces enabled for DHCP
- **NX_DHCP_DEST_TO_SMALL**: (0x95) Destination is too small to hold response.
- **NX_DHCP_PARSE_ERROR**: (0x97) Option not found in Server response.
- NX_PTR_ERROR: (0x16) Invalid input pointer.
- NX_CALLER_ERROR: (0x11) Invalid caller of this service.

### Allowed From

Threads

