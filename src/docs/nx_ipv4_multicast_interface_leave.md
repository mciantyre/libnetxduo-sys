Leave specified multicast group via an interface

### Description

This service leaves the specified multicast group via a specified network interface. After leaving the group, this service does not trigger IGMP messages being generated.

### Parameters 

- *ip_ptr*: Pointer to previously created IP instance.
- *group_address*: Class D IP multicast group address to leave. The IP address is in host byte order.
- *interface_index*: Index of the Interface attached to the NetX Duo instance.

### Return Values  

- **NX_SUCCESS** (0x00) Successful multicast group join.
- **NX_ENTRY_NOT_FOUND** (0x16) The specified multicast group address cannot be found in the local multicast table.
- **NX_INVALID_INTERFACE** (0x4C) Device index points to an invalid network interface.
- **NX_IP_ADDRESS_ERROR** (0x21) Multicast group address provided is not a valid class D address.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_PTR_ERROR** (0x07) Invalid pointer to IP instance, or the IP instance is invalid

### Allowed From

Threads

### Preemption Possible

No

