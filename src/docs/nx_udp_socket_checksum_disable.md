Disable checksum for UDP socket

### Description

This service disables the checksum logic for sending and receiving packets on the specified UDP socket. When the checksum logic is disabled, a value of zero is loaded into the UDP header's checksum field for all packets sent through this socket. A zero-value checksum value in the UDP header signals the  receiver that checksum is not computed for this packet.

Also note that this has no effect if **NX_DISABLE_UDP_RX_CHECKSUM** and **NX_DISABLE_UDP_TX_CHECKSUM** are defined when receiving and sending UDP packets respectively,

Note that this service has no effect on packets on the IPv6 network since UDP checksum is mandatory for IPv6.

### Parameters

- *socket_ptr** Pointer to previously created UDP socket instance.

### Return Values 

- **NX_SUCCESS** (0x00) Successful socket checksum disable.
- **NX_NOT_BOUND** (0x24) Socket is not bound.
- **NX_PTR_ERROR** (0x07) Invalid socket pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Initialization, threads, timer

### Preemption Possible

No

