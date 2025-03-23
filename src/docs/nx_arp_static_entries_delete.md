Delete all static ARP entries

### Description

This service deletes all static entries in the ARP cache.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.

### Return Values

- **NX_SUCCESS** (0x00) Static entries are deleted.
- **NX_PTR_ERROR** (0x07) Invalid ip_ptr pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Initialization, threads

### Preemption Possible

No

