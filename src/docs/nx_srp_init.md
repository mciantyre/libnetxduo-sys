Initialization of SRP.

### Description

This function initialize SRP, it initializes MRP, MSRP, MVRP sequencly, and create a thread in MRP initializaton.

### Parameters

- *srp_ptr*:                             Pointer to SRP instance.
- *ip_ptr*:                              Pointer to IP instance.
- *interface_index*:                     Index of the network interface to use SRP.
- *pkt_pool_ptr*:                        pointer to Packet pool.
- *stack_ptr*:                           pointer to SRP thread Stack.
- *stack_size*:                          SRP thread Stack size .
- *priority*:                            SRP thread priority.


### Return Values

- **NX_SUCCESS** (0x00) Successful init
- **NX_INVALID_INTERFACE** (0x4C) Invalid interface index
- **NX_PTR_ERROR** (0x07) Invalid IP pointer

### Allowed From

Threads

### Preemption Possible

No

