Delete previously created IP instance

### Description

This service deletes a previously created IP instance and releases all of the system resources owned by the IP instance.

### Parameters 

- *ip_ptr*: Pointer to previously created IP instance.

### Return Values

- **NX_SUCCESS** (0x00) Successful IP deletion.
- **NX_SOCKETS_BOUND** (0x28) This IP instance still has UDP or TCP sockets bound to it. All sockets must be unbound and deleted prior to deleting the IP instance.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Threads

### Preemption Possible

Yes

