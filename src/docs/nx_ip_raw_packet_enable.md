Enable raw packet processing

### Description

This service enables transmission and reception of raw IP packets for this IP instance. Incoming TCP, UDP, ICMP, and IGMP packets are still processed by NetX Duo. Packets with unknown upper layer protocol types are processed by raw packet reception routine.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.

### Return Values 

- **NX_SUCCESS** (0x00) Successful IP raw packet enable.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Initialization, threads

### Preemption Possible

No

