Retrieve information about ARP activities

### Description

This service retrieves information about ARP activities for the associated IP instance.

> **Note:** *If a destination pointer is NX_NULL, that particular information is not returned to the caller*.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *arp_requests_sent*: Pointer to destination for the total ARP requests sent from this IP instance.
- *arp_requests_received*: Pointer to destination for the total ARP requests received from the network.
- *arp_responses_sent*: Pointer to destination for the total ARP responses sent from this IP instance.
- *arp_responses_received*: Pointer to the destination for the total ARP responses received from the network.
- *arp_dynamic_entries*: Pointer to the destination for the current number of dynamic ARP entries.
- *arp_static_entries*: Pointer to the destination for the current number of static ARP entries.
- *arp_aged_entries*: Pointer to the destination of the total number of ARP entries that have aged and became invalid.
- *arp_invalid_messages*: Pointer to the destination of the total invalid ARP messages received.

### Return Values

- **NX_SUCCESS** (0x00) Successful ARP information retrieval.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Threads

### Preemption Possible

No

