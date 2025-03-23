Send gratuitous ARP request

### Description

This service goes through all the physical interfaces to transmit gratuitous ARP requests as long as the interface IP address is valid. If an ARP response is subsequently received, the supplied response 
handler is called to process the response to the gratuitous ARP.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *response_handler*: Pointer to response handling function. If NX_NULL is supplied, responses are ignored.

### Return Values

- **NX_SUCCESS** (0x00) Successful gratuitous ARP send.
- **NX_NO_PACKET** (0x01) No packet available.
- **NX_NOT_ENABLED** (0x14) ARP is not enabled.
- **NX_IP_ADDRESS_ERROR** (0x21) Current IP address is invalid.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Caller is not a thread.

### Allowed From

Threads

### Preemption Possible

No

