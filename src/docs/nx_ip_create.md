Create an IP instance

### Description

This service creates an IP instance with the user supplied IP address and network driver. In addition, the application must supply a previously created packet pool for the IP instance to use for internal packet allocation. Note that the supplied application network driver is not called until this IP's thread executes.

### Parameters

- *ip_ptr*: Pointer to control block to create a new IP instance.
- *name*: Name of this new IP instance.
- *ip_address*: IP address for this new IP instance.
- *network_mask*: Mask to delineate the network portion of the IP address for sub-netting and super-netting uses.
- *default_pool*: Pointer to control block of previously created NetX Duo packet pool.
- *ip_network_driver*: User-supplied network driver used to send and receive IP packets.
- *memory_ptr*: Pointer to memory area for the IP helper thread's stack area.
- *memory_size*: Number of bytes in the memory area for the IP helper thread's stack.
- *priority*: Priority of IP helper thread.

### Return Values  

- **NX_SUCCESS** (0x00) Successful IP instance creation.
- **NX_NOT_IMPLEMENTED** (0x4A) NetX Duo library is configured incorrectly.
- **NX_PTR_ERROR** (0x07) Invalid IP, network driver function pointer, packet pool, or memory pointer.
- **NX_SIZE_ERROR** (0x09) The supplied stack size is too small.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_IP_ADDRESS_ERROR** (0x21) The supplied IP address is invalid.
- **NX_OPTION_ERROR** (0x21) The supplied IP thread priority is invalid.

### Allowed From

Initialization, threads

### Preemption Possible

No

