Stop SRP talker.

### Description

This function stop SRP talker. It withdraw the domain,Vlan,stream request.

### Parameters

- *srp_ptr*:                    Pointer to SRP instance.
- *stream_id*:                  stream id of talker advertised.
- *domain*:                     domain of SRP talker.


### Return Values

- **NX_SUCCESS** (0x00) Successful stop
- **NX_INVALID_PARAMETERS** (0x4D) Invalid parameter
- **NX_MSRP_EVENT_NOT_SUPPORTED** (0x06) unsupported event
- **NX_MSRP_ATTRIBUTE_FIND_ERROR** (0x09) not found attribute

### Allowed From

Threads

### Preemption Possible

No

