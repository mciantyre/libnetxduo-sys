Delete an ARP entry

### Description

This service removes an ARP entry for the given IP address from its IP internal ARP table.

### Parameters  

- *ip_ptr*: Pointer to previously created IP instance.
- *ip_address*: ARP entry with the specified IP address should be deleted.

### Return Values

- **NX_SUCCESS** (0x00) Successful ARP enable.
- **NX_ENTRY_NOT_FOUND** (0x16) No entry with the specified IP address can be found.
- **NX_PTR_ERROR** (0x07) Invalid IP or cache memory pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_IP_ADDRESS_ERROR** (0x21) Specified IP address is invalid.

### Allowed From  
Initialization, threads

### Preemption Possible  
No

