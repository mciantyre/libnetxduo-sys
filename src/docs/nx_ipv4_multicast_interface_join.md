Join IP instance to specified multicast group via an interface

### Description

This service joins an IP instance to the specified multicast group via a specified network interface. Once the IP instance joins a multicast group, the IP receive logic starts to forward data packets from the give multicast group to the upper layer. Note that this service joins a multicast group without sending IGMP reports.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *group_address*: Class D IP multicast group address to join in host byte order.
- *interface_index*: Index of the Interface attached to the NetX Duo instance.

### Return Values

- **NX_SUCCESS** (0x00) Successful multicast group join.
- **NX_NO_MORE_ENTRIES** (0x17) No more multicast groups can be joined, maximum exceeded.
- **NX_PTR_ERROR** (0x07) Invalid pointer to IP instance, or the IP instance is invalid
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_EANABLED** (0x14) IGMP is not enabled in this IP instance
- **NX_IP_ADDRESS_ERROR** (0x21) Multicast group address provided is not a valid class D address.
- **NX_INVALID_INTERFACE** (0x4C) Device index points to an invalid network interface.

### Allowed From

Threads

### Preemption Possible

No

