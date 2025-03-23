Configure an auxiliary packet pool

### Description

This service configures an auxiliary packet pool in the IP instance. For a memory-constrained system, the user may increase memory efficiency by creating the default packet pool with packet size of MTU, and creating an auxiliary packet pool with smaller packet size for the IP thread to transmit small packets with. The recommended packet size for the auxiliary pool is 256 bytes, assuming IPv6 and IPsec are both enabled.

By default the IP instance does not accept the auxiliary packet pool. To enable this feature, *NX_DUAL_PACKET_POOL_ENABLE* must be defined when compiling the NetX Duo library.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *aux_pool*: The auxiliary packet pool to be configured for the IP instance.

### Return Values 

- **NX_SUCCESS** (0x00) Successful IP address set.
- **NX_NOT_SUPPORTED** (0x4B) The dual packet pool feature is not compiled in the library.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer or pool pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Initialization, threads

### Preemption Possible  

No

