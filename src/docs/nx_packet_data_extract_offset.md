Extract data from packet via an offset

### Description

This service copies data from a NetX Duo packet (or packet chain) starting at the specified offset from the packet prepend pointer of the specified size in bytes into the specified buffer. The number of bytes actually copied is returned in *bytes_copied.* This service does not remove data from the packet, nor does it adjust the prepend pointer or other internal state information.

### Parameters

- *packet_ptr*: Pointer to packet to extract
- *offset*: Offset from the current prepend pointer.
- *buffer_start*: Pointer to start of save buffer
- *buffer_length*: Number of bytes to copy
- *bytes_copied*: Number of bytes actually copied

### Return Values

- **NX_SUCCESS** (0x00) Successful packet copy
- **NX_PACKET_OFFSET_ERROR** (0x53) Invalid offset value was supplied
- **NX_PTR_ERROR** (0x07) Invalid packet pointer or buffer pointer

### Allowed From

Initialization, threads, timers, and ISRs

### Preemption Possible

No

```c
/* Extract 10 bytes from the start of the received packet buffer
   into the specified memory area. */
status = nx_packet_data_extract_offset(my_packet, 0, &data[0], 10,
                                       &bytes_copied) ;
/* If status is NX_SUCCESS, 10 bytes were successfully copied into
   the data buffer. */
```
