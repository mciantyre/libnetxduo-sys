Attach network interface to IP instance

### Description

This service adds a physical network interface to the IP interface. Note the IP instance is created with the primary interface so each additional interface is secondary to the primary interface. The total number of network interfaces attached to the IP instance (including the primary interface) cannot exceed **NX_MAX_PHYSICAL_INTERFACES**.

If the IP thread has not been running yet, the secondary interfaces will be initialized as part of the IP thread startup process that initializes all physical interfaces.

If the IP thread is not running yet, the secondary interface is initialized as part of the ***nx_ip_interface_attach*** service.

> **Warning:** *ip_ptr must point to a valid NetX Duo IP structure. **NX_MAX_PHYSICAL_INTERFACES** must be configured for the number of network interfaces for the IP instance. The default value is one*.

### Parameters 

- *ip_ptr*: Pointer to previously created IP instance.
- *interface_name*: Pointer to interface name string.
- *ip_address*: Device IP address in host byte order.
- *network_mask*: Device network mask in host byte order.
- *ip_link_driver*: Ethernet driver for the interface.

### Return Values  

- **NX_SUCCESS** (0x00) Entry is added to static routing table.
- **NX_NO_MORE_ENTRIES** (0x17) Max number of interfaces. NX_MAX_PHYSICAL_INTERFACES is exceeded. If IPv6 is enabled, this error may also indicate that the driver may not have enough resource to handle IPv6 multicast operations.
- **NX_DUPLICATED_ENTRY** (0x52) The supplied IP address is already used on this IP instance.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.
- **NX_PTR_ERROR** (0x07) Invalid pointer input.
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid IP address input.

### Allowed From

Initialization, threads

### Preemption Possible

No

