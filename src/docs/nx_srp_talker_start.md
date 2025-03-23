Start SRP talker.

### Description

This function start SRP talker, it sets event callback functions and register domain, Vlan, stream request.

### Parameters

- *srp_ptr*:                       Pointer to SRP instance.
- *event_callback*:                callback invoked by application to monitor the SRP process.
- *stream_id*:                     stream id of talker advertised.


### Return Values

- **NX_SUCCESS** (0x00) Successful start
- **NX_INVALID_PARAMETERS** (0x4D) Invalid parameter
- **NX_MSRP_EVENT_NOT_SUPPORTED** (0x06) unsupported event
- **NX_MSRP_ATTRIBUTE_FIND_ERROR** (0x09) not found attribute


### Allowed From

Threads

### Preemption Possible

No

