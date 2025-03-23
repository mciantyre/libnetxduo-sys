Delete static route from routing table

### Description

This service deletes an entry from the static routing table.

> **Warning:** *Note that ip_ptr must point to a valid NetX Duo IP structure and the NetX Duo library must be built with NX_ENABLE_IP_STATIC_ROUTING defined to use this service. By default NetX Duo is built without NX_ENABLE_IP_STATIC_ROUTING defined*.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *network_address*: Target network address, in host byte order.
- *net_mask*: Target network mask, in host byte order.

### Return Values  

- **NX_SUCCESS** (0x00) Successful deletion from the static routing table.
- **NX_NOT_SUCCESSFUL** (0x43) Entry cannot be found in the routing table.
- **NX_NOT_SUPPORTED** (0x4B) This feature is not compiled in.
- **NX_PTR_ERROR** (0x07) Invalid ip_ptr pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Initialization, threads

### Preemption Possible

No

