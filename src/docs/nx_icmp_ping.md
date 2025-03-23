Send ping request to specified IP address

### Description

This service sends a ping request to the specified IP address and waits for the specified amount of time for a ping response message. If no response is received, an error is returned. Otherwise, the entire response message is returned in the variable pointed to by response_ptr.

To send a ping request to an IPv6 destination, applications shall use the ***nxd_icmp_ping*** or ***nxd_icmp_source_ping*** service.

> **Warning:** *If NX_SUCCESS is returned, the application is responsible for releasing the received packet after it is no longer needed*.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *ip_address*: IP address, in host byte order, to ping.
- *data*: Pointer to data area for ping message.
- *data_size*: Number of bytes in the ping data
- *response_ptr*: Pointer to packet pointer to return the ping response message in.
- *wait_option*: Defines the number of ThreadX timer ticks to wait for a ping response. The wait options are defined as follows:

| Wait Option            | Value                           |
| -----------------------|---------------------------------|
| NX_NO_WAIT             | (0x00000000)                    |
| timeout value in ticks | (0x00000001 through 0xFFFFFFFE) |
| NX_WAIT_FOREVER        | 0xFFFFFFFF                      |

### Return Values 

- **NX_SUCCESS** (0x00) Successful ping. Response message pointer was placed in the variable pointed to by response_ptr.
- **NX_NO_PACKET** (0x01) Unable to allocate a ping request packet.
- **NX_OVERFLOW** (0x03) Specified data area exceeds the default packet size for this IP instance.
- **NX_NO_RESPONSE** (0x29) Requested IP did not respond.
- **NX_WAIT_ABORTED** (0x1A) Requested suspension was aborted by a call to tx_thread_wait_abort.
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid IP address.
- **NX_PTR_ERROR** (0x07) Invalid IP or response pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Threads

### Preemption Possible

No

