Detach the specified interface from the IP instance

### Description

This service detaches the specified IP interface from the IP instance. Once an interface is detached, all connected TCP sockets closed, and ND cache and ARP entries for this interface are removed from their
respective tables. IGMP memberships for this interface are removed.

### Parameters 

- *ip_ptr*: Pointer to previously created IP instance.
- *index*: Index of the interface to be removed.

### Return Values

- **NX_SUCCESS** (0x00) Successfully removed a physical interface.
- **NX_INVALID_INTERFACE** (0x4C) Specified network interface is invalid.
- **NX_PTR_ERROR** (0x07) Invalid pointers.

### Allowed From

Initialization, threads

### Preemption Possible

No

