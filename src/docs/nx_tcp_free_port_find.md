Find next available TCP port

### Description

This service attempts to locate a free TCP port (unbound) starting from the application supplied port. The search logic will wrap around if the search happens to reach the maximum port value of 0xFFFF. If  the search is successful, the free port is returned in the variable pointed to by *free_port_ptr*.

> **Warning:** *This service can be called from another thread and have the same port returned. To prevent this race condition, the application may wish to place this service and the actual client socket bind under the protection of a mutex*.

### Parameters  

- *ip_ptr*: Pointer to previously created IP instance.
- *port*: Port number to start search at (1 through 0xFFFF).
- *free_port_ptr*: Pointer to the destination free port return value.

### Return Values  

- **NX_SUCCESS** (0x00) Successful free port find.
- **NX_NO_FREE_PORTS** (0x45) No free ports found.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.
- **NX_INVALID_PORT** (0x46) The specified port number is invalid.

### Allowed From

Threads

### Preemption Possible

No

