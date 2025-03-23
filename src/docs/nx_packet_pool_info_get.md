Retrieve information about a packet pool

### Description

This service retrieves information about the specified packet pool.

> **Important:** *If a destination pointer is NX_NULL, that particular information is not returned to the caller*.

### Parameters

- *pool_ptr*: Pointer to previously created packet pool.
- *total_packets*: Pointer to destination for the total number of packets in the pool.
- *free_packets*: Pointer to destination for the total number of currently free packets.
- *empty_pool_requests*: Pointer to destination of the total number of allocation requests when the pool was empty.
- *empty_pool_suspensions*: Pointer to destination of the total number of empty pool suspensions.
- *invalid_packet_releases*: Pointer to destination of the total number of invalid packet releases.

### Return Values 

- **NX_SUCCESS** (0x00) Successful packet pool information retrieval.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Initialization, threads, and timers

### Preemption Possible

No

