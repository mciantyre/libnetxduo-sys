Clear the DHCP client network parameters

### Description

This service clears the host application network parameters (IP address, network address and network mask), and clears the DHCP Client state on all interfaces enabled for DHCP. It is used in combination with *nx_dhcp_stop* and ***nx_dhcp_start*** to 'restart' the DHCP state machine:

```c
nx_dhcp_stop(&my_dhcp);
nx_dhcp_reinitialize(&my_dhcp);
nx_dhcp_start(&my_dhcp);
```

To reinitialize the DHCP Client on a specific interface when multiple interfaces are enabled for DHCP, use the ***nx_dhcp_interface_reinitialize*** service.

### Input Parameters

- *dhcp_ptr*: Pointer to previously created DHCP instance.

### Return Values

- **NX_SUCCESS**: (0x00) DHCP successfully reinitialized 
- NX_PTR_ERROR: (0x16) Invalid DHCP pointer

### Allowed From

Threads

