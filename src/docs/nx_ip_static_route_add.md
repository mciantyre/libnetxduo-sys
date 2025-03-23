Add static route to the routing table

### Description

This service adds an entry to the static routing table. Note that the *next_hop* address must be directly accessible from one of the local network devices.

> **Caution:** *Note that ip_ptr must point to a valid NetX Duo IP structure and the NetX Duo library must be built with NX_ENABLE_IP_STATIC_ROUTING defined to use this service. By default NetX Duo is built without NX_ENABLE_IP_STATIC_ROUTING defined*.

### Parameters 

- *ip_ptr*: Pointer to previously created IP instance.
- *network_address*: Target network address, in host byte order 
- *net_mask*: Target network mask, in host byte order
- *next_hop*: Next hop address for the target network, in host byte order

### Return Values  

- **NX_SUCCESS** (0x00) Entry is added to the static routing table.
- **NX_OVERFLOW** (0x03) Static routing table is full.
- **NX_NOT_SUPPORTED** (0x4B) This feature is not compiled in.
- **NX_IP_ADDRESS_ERROR** (0x21) Next hop is not directly accessible via local interfaces.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_PTR_ERROR** (0x07) Invalid ip_ptr pointer.

### Allowed From

Initialization, threads

### Preemption Possible

No

