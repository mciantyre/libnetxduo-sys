Delete previously created packet pool

### Description

This service deletes a previously created packet pool. NetX Duo checks for any threads currently suspended on packets in the packet pool and clears the suspension.

### Parameters

- *pool_ptr*: Packet pool control block pointer.

### Return Values 

- **NX_SUCCESS** (0x00) Successful packet pool delete.
- **NX_PTR_ERROR** (0x07) Invalid pool pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Threads

### Preemption Possible

Yes

