Set the physical address for a specified network interface

### Description

This service is used by the application or a device driver to configure the physical address of the MAC address of the specified network interface. The new MAC address is applied to the control block of the interface structure. If the ***update_driver*** flag is set, a driver-level command is issued so the device driver is able to update its MAC address programmed into the Ethernet controller.

In a typical situation, this service is called from the interface device driver during initialization phase to notify the IP stack of its MAC address. In this case, the ***update_driver*** flag should not be set.

This routine can also be called from user application to reconfigure the interface MAC address at run time. In this use case, the ***update_driver*** flag should be set, so the new MAC address can be applied to the device driver.

### Parameters

- *ip_ptr*: IP control block pointer
- *interface_index*: Index to the network interface
- *physical_msw*: Pointer to destination for top 16 bits of the device MAC address
- *physical_lsw*: Pointer to destination for lower 32 bits of the device MAC address

### Return Values

- **NX_SUCCESS** (0x00) Successful set
- **NX_UNHANDLED_COMMAND** (0x4B) Command not recognized by the driver
- **NX_INVALID_INTERFACE** (0x4C) Interface index is not valid
- **NX_PTR_ERROR** (0x07) Invalid IP control block pointer
- **NX_CALLER_ERROR** (0x11) Service is not called from system initialization or thread context.

### Allowed From

Initialization, threads

### Preemption Possible

No

