Set raw IP packet filter

### Description

This service configures the IP raw packet filter. The raw packet filter function, implemented by user application, allows an application to receive raw packets based on user-supplied criteria. Note that NetX Duo BSD wrapper layer uses the raw packet filter feature to handle raw socket in the BSD layer. To use this service, the NetX Duo library must be built with the option ***NX_ENABLE_IP_RAW_PACKET_FILTER*** defined.

### Parameters  

- *ip_ptr*: IP control block pointer
- *raw_packet_filter*: Function pointer of the raw packet filter

### Return Values  

- **NX_SUCCESS** (0x00) Successfully set the raw packet filter routine
- **NX_NOT_SUPPORT** (0x4B) Raw packet support is not available
- **NX_PTR_ERROR** (0x07) Invalid IP control block pointer
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Initialization, threads

### Preemption Possible

No

