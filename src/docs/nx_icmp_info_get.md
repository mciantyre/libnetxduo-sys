Retrieve information about ICMP activities

### Description

This service retrieves information about ICMP activities for the specified IP instance.

> NOTE  
> *If a destination pointer is NX_NULL, that particular information is not returned to the caller*.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *pings_sent*: Pointer to destination for the total number of pings sent.
- *ping_timeouts*: Pointer to destination for the total number of ping timeouts.
- *ping_threads_suspended*: Pointer to destination of the total number of threads suspended on ping requests.
- *ping_responses_received*: Pointer to destination of the total number of ping responses received.
- *icmp_checksum_errors*: Pointer to destination of the total number of ICMP checksum errors.
- *icmp_unhandled_messages*: Pointer to destination of the total number of un-handled ICMP messages.

### Return Values

- **NX_SUCCESS** (0x00) Successful ICMP information retrieval.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Initialization, threads

### Preemption Possible

No

