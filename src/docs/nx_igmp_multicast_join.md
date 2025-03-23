Join IP instance to specified multicast group

### Description

This service joins an IP instance to the specified multicast group. An internal counter is maintained to keep track of the number of times the same group has been joined. The driver is commanded to send an IGMP report if this is the first join request out on the network indicating the host's intention to join the group. After joining, the IGMP component will allow reception of IP packets with this group address and report to routers that this IP is a member of this multicast group. To join an IPv4 multicast group without sending IGMP group membership report, application shall use the service ***nx_ipv4_multicast_interface_join***.

> NOTE  
> *To join a multicast group on a non-primary device, use the service **nx_igmp_multicast_interface_join.***

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *group_address*: Class D IP multicast group address to join.

### Return Values 

- **NX_SUCCESS** (0x00) Successful multicast group join.
- **NX_NO_MORE_ENTRIES** (0x17) No more multicast groups can be joined, maximum exceeded.
- **NX_INVALID_INTERFACE** (0x4C) Device index points to an invalid network interface.
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid IP group address.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Threads

### Preemption Possible

No

