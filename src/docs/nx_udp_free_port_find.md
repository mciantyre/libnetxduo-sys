Find next available UDP port

### Description

This service looks for a free UDP port (unbound) starting from the application supplied port number. The search logic will wrap around if the search reaches the maximum port value of 0xFFFF. If the search is successful, the free port is returned in the variable pointed to by free_port_ptr.

> **Warning:** *This service can be called from another thread and can have the same port returned. To prevent this race condition, the application may wish to place this service and the actual socket bind under the protection of a mutex*.

### Parameters

- *ip_ptr** Pointer to previously created IP instance.
- *port** Port number to start search (1 through 0xFFFF).
- *free_port_ptr** Pointer to the destination free port return variable.

### Return Values  

- **NX_SUCCESS** (0x00) Successful free port find.
- **NX_NO_FREE_PORTS** (0x45) No free ports found.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.
- **NX_INVALID_PORT** (0x46) Specified port number is invalid.

### Allowed From

Threads

### Preemption Possible

No

