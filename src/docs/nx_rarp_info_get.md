Retrieve information about RARP activities

### Description

This service retrieves information about RARP activities for the specified IP instance.

> **Important:** *If a destination pointer is NX_NULL, that particular information is not returned to the caller*.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *rarp_requests_sent*: Pointer to destination for the total number of RARP requests sent.
- *rarp_responses_received*: Pointer to destination for the total number of RARP responses received.
- *rarp_invalid_messages*: Pointer to destination of the total number of invalid messages.

### Return Values

- **NX_SUCCESS** (0x00) Successful RARP information retrieval.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Initialization, threads

### Preemption Possible

No

