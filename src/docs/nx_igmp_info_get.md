Retrieve information about IGMP activities

### Description

This service retrieves information about IGMP activities for the specified IP instance.

> **Important:** *If a destination pointer is NX_NULL, that particular information is not returned to the caller*.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *igmp_reports_sent*: Pointer to destination for the total number of ICMP reports sent.
- *igmp_queries_received*: Pointer to destination for the total number of queries received by multicast router.
- *igmp_checksum_errors*: Pointer to destination of the total number of IGMP checksum errors on receive packets.
- *current_groups_joined*: Pointer to destination of the current number of groups joined through this IP instance.

### Return Values  

- **NX_SUCCESS** (0x00) Successful IGMP information retrieval.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Initialization, threads

### Preemption Possible

No

