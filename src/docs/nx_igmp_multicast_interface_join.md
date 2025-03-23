Join IP instance to specified multicast group via an interface

### Description

This service joins an IP instance to the specified multicast group via a specified network interface. An internal counter is maintained to keep track of the number of times the same group has been joined. After joining the multicast group, the IGMP component will allow reception of IP packets with this group address via the specified network interface and also report to routers that this IP is a member of this multicast group. The IGMP membership join, report, and leave messages are also sent via the specified network interface. To join an IPv4 multicast group without sending IGMP group membership report, application shall use the service ***nx_ipv4_multicast_interface_join***.

### Parameters 

- *ip_ptr*: Pointer to previously created IP instance.
- *group_address*: Class D IP multicast group address to join in host byte order.
- *interface_index*: Index of the Interface attached to the NetX Duo instance.

### Return Values

- **NX_SUCCESS** (0x00) Successful multicast group join.
- **NX_NO_MORE_ENTRIES** (0x17) No more multicast groups can be joined, maximum exceeded.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_INVALID_INTERFACE** (0x4C) Device index points to an invalid network interface.
- **NX_IP_ADDRESS_ERROR** (0x21) Multicast group address provided is not a valid class D address.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) IP multicast support is not enabled.

### Allowed From

Threads

### Preemption Possible

No

