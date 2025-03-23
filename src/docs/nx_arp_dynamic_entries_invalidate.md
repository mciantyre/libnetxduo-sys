Invalidate all dynamic entries in the ARP cache

### Description
This service invalidates all dynamic ARP entries currently in the ARP cache.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.

### Return Values

- **NX_SUCCESS** (0x00) Successful ARP cache invalidate.
- **NX_NOT_ENABLED** (0x14) ARP is not enabled.
- **NX_PTR_ERROR** (0x07) Invalid IP address.
- **NX_CALLER_ERROR** (0x11) Caller is not a thread.

### Allowed From
Threads

### Preemption Possible
No

