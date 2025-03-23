Set maximum raw receive queue size

### Description

This service configures the maximum depth of the IP raw packet receive queue. Note that the IP raw packet receive queue is shared with both IPv4 and IPv6 packets. When the raw packet receive queue reaches the userconfigured maximum depth, newly received raw packets are dropped. The default IP raw packet receive queue depth is 20.

### Parameters

- *ip_ptr*: IP control block pointer
- *queue_max*: New value for the queue size

### Return Values

- **NX_SUCCESS** (0x00) Successfully set raw receive queue maximum depth
- **NX_PTR_ERROR** (0x07) Invalid IP control block pointer
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Initialization and threads

### Preemption Possible

No

