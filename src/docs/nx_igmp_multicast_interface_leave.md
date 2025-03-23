Leave specified multicast group via an interface

### Description

This service leaves the specified multicast group via a specified network interface. An internal counter is maintained to keep track of the number of times the same group has been a member of. After leaving the multicast group, the IGMP component will send out proper membership report, and may leave the group if there are no members from this node. To leave an IPv4 multicast group without sending IGMP group membership report, application shall use the service ***nx_ipv4_multicast_interface_leave***.

### Parameters 

- *ip_ptr*: Pointer to previously created IP instance.
- *group_address*: Class D IP multicast group address to leave. The IP address is in host byte order.
- *interface_index*: Index of the Interface attached to the NetX Duo instance.

### Return Values 

- **NX_SUCCESS** (0x00) Successful multicast group join.
- **NX_ENTRY_NOT_FOUND** (0x16) The specified multicast group address cannot be found in the local multicast table.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_INVALID_INTERFACE** (0x4C) Device index points to an invalid network interface.
- **NX_IP_ADDRESS_ERROR** (0x21) Multicast group address provided is not a valid class D address.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) IP multicast support is not enabled.

### Allowed From

Threads

### Preemption Possible

No

