Enable Internet Group Management Protocol (IGMP)

### Description

This service enables the IGMP component on the specified IP instance. The IGMP component is responsible for providing support for IP multicast group management operations.

### Parameters 

- *ip_ptr*: Pointer to previously created IP instance.

### Return Values

- **NX_SUCCESS** (0x00) Successful IGMP enable.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_ALREADY_ENABLED** (0x15) This component has already been enabled.

### Allowed From

Initialization, threads

### Preemption Possible

No

