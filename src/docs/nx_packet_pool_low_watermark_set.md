Set packet pool low watermark

### Description

This service configures the low watermark for the specified packet pool. Once the low watermark value is set, TCP or UDP will not queue up the received packets if the number of available packets in the packet pool is less than the packet pool's low watermark, preventing the packet pool from being starved of packets. This service is available if the NetX Duo library is built with the option **NX_ENABLE_LOW_WATERMARK** defined.

### Parameters

- *pool_ptr*: Pointer to packet pool control block.
- *low_watermark*: Low watermark value to be configured

### Return Values 

- **NX_SUCCESS** (0x00) Successfully set the low watermark value.
- **NX_NOT_SUPPORTED** (0x4B) The low watermark feature is not built into NetX Duo.
- **NX_PTR_ERROR** (0x07) Invalid pool pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Threads

### Preemption Possible

No

