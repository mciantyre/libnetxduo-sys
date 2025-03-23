Create UDP socket

### Description

This service creates a UDP socket for the specified IP instance.

### Parameters

- *ip_ptr** Pointer to previously created IP instance.
- *socket_ptr** Pointer to new UDP socket control bloc.
- *name** Application name for this UDP socket.
- *type_of_service** Defines the type of service for the transmission, legal values are as follows:
	- **NX_IP_NORMAL** (0x00000000)
	- **NX_IP_MIN_DELAY** (0x00100000)
	- **NX_IP_MAX_DATA** (0x00080000)
	- **NX_IP_MAX_RELIABLE** (0x00040000)
	- **NX_IP_MIN_COST** (0x00020000)
- *fragment Specifies*: whether or not IP fragmenting is allowed. If NX_FRAGMENT_OKAY (0x0) is specified, IP fragmenting is allowed. If NX_DONT_FRAGMENT (0x4000) is specified, IP fragmenting is disabled.
- *time_to_live*: Specifies the 8-bit value that defines how many routers this packet can pass before being thrown away. The default value is specified by **NX_IP_TIME_TO_LIVE**.
- *queue_maximum*: Defines the maximum number of UDP datagrams that can be queued for this socket. After the queue limit is reached, for every new packet received the oldest UDP packet is released.

### Return Values 

- **NX_SUCCESS** (0x00) Successful UDP socket create.
- **NX_OPTION_ERROR** (0x0A) Invalid type-of-service, fragment, or time-to-live option.
- **NX_PTR_ERROR** (0x07) Invalid IP or socket pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_NOT_ENABLED** (0x14) This component has not been enabled.

### Allowed From

Initialization and Threads

### Preemption Possible

No

