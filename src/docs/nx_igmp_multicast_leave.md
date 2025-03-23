Cause IP instance to leave specified multicast group

### Description

This service causes an IP instance to leave the specified multicast group, if the number of leave requests matches the number of join requests. Otherwise, the internal join count is simply decremented. To leave an IPv4 multicast group without sending IGMP group membership report, application shall use the service ***nx_ipv4_multicast_interface_leave***.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *group_address*: Multicast group to leave.

### Return Values  

- **NX_SUCCESS** (0x00) Successful multicast group join.
- **NX_ENTRY_NOT_FOUND** (0x16) Previous join request was not found.
- **NX_INVALID_INTERFACE** (0x4C) Device index points to an invalid network interface.
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid IP group address.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Threads

### Preemption Possible

No

