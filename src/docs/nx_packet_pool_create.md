Create packet pool in specified memory area

### Description

This service creates a packet pool of the specified packet size in the memory area supplied by the user.

### Parameters

- *pool_ptr*: Pointer to packet pool control block.
- *name*: Pointer to application's name for the packet pool.
- *payload_size*: Number of bytes in each packet in the pool. This value must be at least 40 bytes and must also be evenly divisible by 4.
- *memory_ptr*: Pointer to the memory area to place the packet pool in. The pointer should be aligned on an ULONG boundary.
- *memory_size*: Size of the pool memory area.

### Return Values 

- **NX_SUCCESS** (0x00) Successful packet pool create.
- **NX_PTR_ERROR** (0x07) Invalid pool or memory pointer.
- **NX_SIZE_ERROR** (0x09) Invalid block or memory size.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Initialization, threads

### Preemption Possible

No

