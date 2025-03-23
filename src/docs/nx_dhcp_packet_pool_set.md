Set the DHCP Client packet pool

### Description

This service allows the application to create the DHCP Client packet pool by passing in a pointer to a previously created packet pool in this service call. To use this feature, the host application must define **NX_DHCP_CLIENT_USER_CREATE_PACKET_POOL**. When defined, the *nx_dhcp_create* service will not create the Client's packet pool. 

> **Note:** The application is recommended to use the default values for the DHCP client packet pool payload, defined as NX_DHCP_PACKET_PAYLOAD in *nxd_dhcp_client.h* when creating the packet pool.

### Input Parameters

- *dhcp_ptr*: Pointer to DHCP control block.  
- *packet_pool_ptr*: Pointer to previously created packet pool

### Return Values

- **NX_SUCCESS**: (0x00) DHCP Client packet pool is set
- **NX_NOT_ENABLED**: (0x14) Service is not enabled
- NX_PTR_ERROR: (0x16) Invalid DHCP pointer
- NX_DHCP_INVALID_PAYLOAD: (0x9C) Payload is too small

### Allowed From

Application code

