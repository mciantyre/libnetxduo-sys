Enable Address Resolution Protocol (ARP)

### Description

This service initializes the ARP component of NetX Duo for the specific IP instance. ARP initialization includes setting up the ARP cache and various ARP processing routines necessary for sending and receiving ARP messages.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *arp_cache_memory*: Pointer to memory area to place ARP cache.
- *arp_cache_size*: Each ARP entry is 52 bytes, the total number of ARP entries is, therefore, the size divided by 52.

### Return Values

- **NX_SUCCESS** (0x00) Successful ARP enable.
- **NX_PTR_ERROR** (0x07) Invalid IP or cache memory pointer.
- **NX_SIZE_ERROR** (0x09) User supplied ARP cache memory is too small.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_ALREADY_ENABLED** (0x15) This component has already been enabled.

### Allowed From   
Initialization, threads

### Preemption Possible  
No

