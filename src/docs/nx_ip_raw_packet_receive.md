Receive raw IP packet

### Description

This service receives a raw IP packet from the specified IP instance. If there are IP packets on the raw packet receive queue, the first (oldest) packet is returned to the caller. Otherwise, if no packets are available, the caller may suspend as specified by the wait option.

> **Caution:** *If NX_SUCCESS, is returned, the application is responsible for releasing the received packet when it is no longer needed*.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *packet_ptr*: Pointer to pointer to place the received raw IP packet in.
- *wait_option*: Defines how the service behaves if packets are not available. The wait options are defined as follows:
   - **NX_NO_WAIT** (0x00000000)
   - **NX_WAIT_FOREVER** (0xFFFFFFFF)
   - **timeout value in ticks** (0x00000001 through 0xFFFFFFFE)

### Return Values  

- **NX_SUCCESS** (0x00) Successful IP raw packet receive.
- **NX_NO_PACKET** (0x01) No packet was available.
- **NX_WAIT_ABORTED** (0x1A) Requested suspension was aborted by a call to tx_thread_wait_abort.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.
- **NX_PTR_ERROR** (0x07) Invalid IP or return packet pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service

### Allowed From

Threads

### Preemption Possible

No

