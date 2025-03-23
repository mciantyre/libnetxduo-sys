Check status of an IP instance

### Description

This service checks and optionally waits for the specified status of the primary network interface of a previously created IP instance. To obtain status on secondary interfaces, applications shall use the service ***nx_ip_interface_status_check.***

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *needed_status*: IP status requested, defined in bit-map form as follows:
  - **NX_IP_INITIALIZE_DONE** (0x0001)
  - **NX_IP_ADDRESS_RESOLVED** (0x0002)
  - **NX_IP_LINK_ENABLED** (0x0004)
  - **NX_IP_ARP_ENABLED** (0x0008)
  - **NX_IP_UDP_ENABLED** (0x0010)
  - **NX_IP_TCP_ENABLED** (0x0020)
  - **NX_IP_IGMP_ENABLED** (0x0040)
  - **NX_IP_RARP_COMPLETE** (0x0080)
  - **NX_IP_INTERFACE_LINK_ENABLED** (0x0100)
- *actual_status*: Pointer to destination of actual bits set.
- *wait_option*: Defines how the service behaves if the requested status bits are not available. The wait options are defined as follows:
  - **NX_NO_WAIT** 0x00000000)
  - **timeout value in ticks** (0x00000001 through 0xFFFFFFFE)
  - **NX_WAIT_FOREVER** 0xFFFFFFFF

### Return Values 

- **NX_SUCCESS** (0x00) Successful IP status check.
- **NX_NOT_SUCCESSFUL** (0x43) Status request was not satisfied within the timeout specified.
- **NX_PTR_ERROR** (0x07) IP pointer is or has become invalid, or actual status pointer is invalid.
- **NX_OPTION_ERROR** (0x0a) Invalid needed status option.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Threads

### Preemption Possible

No

