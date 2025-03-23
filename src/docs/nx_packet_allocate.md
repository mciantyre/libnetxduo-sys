Allocate packet from specified pool

### Description

This service allocates a packet from the specified pool and adjusts the prepend pointer in the packet according to the type of packet specified. If no packet is available, the service suspends according to the supplied wait option.

### Parameters

- *pool_ptr*: Pointer to previously created packet pool.
- *packet_ptr*: Pointer to the Pointer of the allocated packet Pointer.
- *packet_type*: Defines the type of packet requested. See "Packet Pools" on page 63 in Chapter 3 for a list of supported packet types.
- *wait_option*: Defines the wait time in ticks if there are no packets available in the packet pool. The wait options are defined as follows:
  - **NX_NO_WAIT** (0x00000000)
  - **NX_WAIT_FOREVER** (0xFFFFFFFF)
  - **timeout value in ticks** (0x00000001 through 0xFFFFFFFE)

### Return Values 

- **NX_SUCCESS** (0x00) Successful packet allocate.
- **NX_NO_PACKET** (0x01) No packet available.
- **NX_WAIT_ABORTED** (0x1A) Requested suspension was aborted by a call to tx_thread_wait_abort.
- **NX_INVALID_PARAMETERS** (0x4D) Packet size cannot support protocol.
- **NX_OPTION_ERROR** (0x0A) Invalid packet type.
- **NX_PTR_ERROR** (0x07) Invalid pool or packet return pointer.
- **NX_CALLER_ERROR** (0x11) Invalid wait option from nonthread.

### Allowed From

Initialization, threads, timers, and ISRs (application network drivers). Wait option must be *NX_NO_WAIT* when used in ISR or in timer context.

### Preemption Possible

No

