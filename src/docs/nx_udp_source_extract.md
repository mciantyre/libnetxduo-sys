Extract IP and sending port from UDP datagram

### Description

This service extracts the sender's IP and port number from the IP and UDP headers of the supplied UDP datagram. Note that the service ***nxd_udp_source_extract*** works with packets from either IPv4 or IPv6 network.

### Parameters

- *packet_ptr*: UDP datagram packet pointer.
- *ip_address*: Valid pointer to the return IP address variable.
- *port*: Valid pointer to the return port variable.

### Return Values 

- **NX_SUCCESS** (0x00) Successful source IP/port extraction.
- **NX_INVALID_PACKET** (0x12) The supplied packet is invalid.
- **NX_PTR_ERROR** (0x07) Invalid packet or IP or port destination.

### Allowed From

Initialization, threads, timers, ISR

### Preemption Possible

No

### Description

This service enables both ICMPv4 and ICMPv6 services and can only be called after the IP instance has been created. The service can be enabled either before or after IPv6 is enabled (see *nxd_ipv6_enable*). ICMPv4 services include Echo Request/Reply. ICMPv6 services include Echo Request/Reply, Neighbor Discovery, Duplicate Address Detection, Router Discovery, and Stateless Address Auto-configuration. The IPv4 equivalent in NetX is *nx_icmp_enable*.

> NOTE 
> *If the IPv6 address is manually configured prior to enabling ICMPv6, the manually configured IPv6 is not subject to Duplicate Address Detection process*.

*nx_icmp_enable* starts ICMP services for IPv4 operations only. Applications using ICMPv6 services must use *nxd_icmp_enable* instead of *nx_icmp_enable*.

To utilize IPv6 router solicitation and IPv6 stateless auto-address configuration, ICMPv6 must be enabled.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance

### Return Values 

- **NX_SUCCESS** (0x00) ICMP services are successfully enabled
- **NX_PTR_ERROR** (0x07) Invalid IP pointer
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service

### Allowed From

Initialization, Threads

### Preemption Possible

No

### Description

This service sends out an ICMP Echo Request packet through an appropriate physical interface and waits for an Echo Reply from the destination host. NetX Duo determines the appropriate interface, based on the destination address, to send the ping message . Applications shall use the service ***nxd_icmp_source_ping*** to specify the physical interface and precise source IP address to use for packet transmission.

The IP instance must have been created, and the ICMPv4/ICMPv6 services must be enabled (see ***nxd_icmp_enable***).

> **Warning:** If NX_SUCCESS is returned, the application is responsible for releasing the received packet after it is no longer needed.

### Parameters

- *ip_ptr*: Pointer to IP instance
- *ip_address*: Destination IP address to ping, in host byte order
- *data_ptr*: Pointer to ping packet data area
- *data_size*: Number of bytes of ping data
- *response_ptr*: Pointer to response packet Pointer
- *wait_option*: Time to wait for a reply. The wait options are defined as follows:
	- **NX_NO_WAIT** (0x00000000)
	- **timeout value in ticks** (0x00000001 through 
	- **NX_WAIT_FOREVER** 0xFFFFFFFE)

### Return Values

- **NX_SUCCESS** (0x00) Successful sent and received ping
- **NX_NOT_SUPPORTED** (0x4B) IPv6 is not enabled
- **NX_OVERFLOW** (0x03) Ping data exceeds packet payload
- **NX_NO_RESPONSE** (0x29) Destination host did not respond
- **NX_WAIT_ABORTED** (0x1A) Requested suspension was aborted by tx_thread_wait_abort
- **NX_NO_INTERFACE_ADDRESS** (0x50) No suitable outgoing interface can be found.
- **NX_PTR_ERROR** (0x07) Invalid IP or response pointer
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service
- **NX_NOT_ENABLED** (0x14) IP or ICMP component is not enabled
- **NX_IP_ADDRESS_ERROR** (0x21) Input IP address is invalid

### Allowed From

Threads

### Preemption Possible

No

### Description

This service sends out an ICMP Echo Request packet using the specified index of an IPv4 or IPv6 address, and through the network interface the source address is associated with, and waits for an Echo Reply from the destination host. This service works with both IPv4 and IPv6 addresses. The parameter *address_index* indicates the source IPv4 or IPv6 address to use. For IPv4 address, the *address_index* is the same index to the attached network interface. For IPv6, the *address_index* indicates the entry in the IPv6 address table.

The IP instance must have been created, and the ICMPv4 and ICMPv6 services must be enabled (see *nxd_icmp_enable*).

> **Caution:** *If NX_SUCCESS is returned, the application is responsible for releasing the received packet after it is no longer needed*.

### Parameters

- *ip_ptr*: Pointer to IP instance
- *ip_address*: Destination IP address to ping, in host byte order
- *address_index*: Indicates the IP address to use as source address
- *data_ptr*: Pointer to ping packet data area
- *data_size*: Number of bytes of ping data
- *response_ptr*: Pointer to response packet pointer
- *wait_option*: Time to wait for a reply. The wait options are defined as follows:
	- **NX_NO_WAIT** (0x00000000)
	- **timeout value in ticks** (0x00000001 through 0xFFFFFFFE
	- **NX_WAIT_FOREVER** 0xFFFFFFFF)

### Return Values 

- **NX_SUCCESS** (0x00) Successful sent and received ping
- **NX_NOT_SUPPORTED** (0x4B) IPv6 is not enabled
- **NX_OVERFLOW** (0x03) Ping data exceeds packet payload
- **NX_NO_RESPONSE** (0x29) Destination host did not respond
- **NX_WAIT_ABORTED** (0x1A) Requested suspension was aborted by tx_thread_wait_abort
- **NX_NO_INTERFACE_ADDRESS** (0x50) No suitable outgoing interface can be found
- **NX_PTR_ERROR** (0x07) Invalid IP or response pointer
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service
- **NX_NOT_ENABLED** (0x14) IP or ICMP component is not enabled
- **NX_IP_ADDRESS_ERROR** (0x21) Input IP address is invalid

### Allowed From

Threads

### Preemption Possible

No

### Description

This service sets the ICMPv6 Router Advertisement flag change callback function. The user-supplied callback function is invoked when NetX Duo receives a router advertisement message.

### Parameters

- *ip_ptr*: Pointer to IP instance
- *ra_callback*: User-supplied callback function

### Return Values  

- **NX_SUCCESS** (0x00) Successful set the RA flag callback function
- **NX_NOT_SUPPORTED** (0x4B) IPv6 is not enabled
- **NX_PTR_ERROR** (0x07) Invalid IP
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service

### Allowed From

Initialization, threads

### Preemption Possible

No

### Description

This service sends a raw IPv4 or IPv6 packet (no transport-layer protocol headers). On a multihome system, if the system is unable to determine an appropriate interface (for example, if the destination IP address is IPv4 broadcast, multicast or IPv6 multicast address), the primary device is selected. The service ***nxd_ip_raw_packet_source_send*** can be used to specify an outgoing interface. The NetX equivalent is ***nx_ip_raw_packet_send*.**

The IP instance must be previously created and raw IP packet handling must be enabled using the ***nx_ip_raw_packet_enable*** service.

> **Warning:** *Unless an error is returned, the application should not release the packet after this call. Doing so will cause unpredictable results because the network driver will also try to release the packet after 
transmission*.

### Parameters

- *ip_ptr*: Pointer to the previously created IP instance
- *packet_ptr*: Pointer to packet to transmit
- *destination_ip*: Pointer to destination address
- *protocol*: Packet protocol stored to the IP header
- *ttl*: Value for TTL or hop limit
- *tos*: Value for TOS or traffic class and flow label

### Return Value 

- **NX_SUCCESS** (0x00) Raw IP packet successfully sent
- **NX_NO_INTERFACE_ADDRESS** (0x50) No suitable outgoing interface can be found
- **NX_NOT_ENABLED** (0x14) Raw IP handling not enabled
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid IPv4 or IPv6 address
- **NX_UNDERFLOW** (0x02) Not enough room for IPv4 or IPv6 header in the packet
- **NX_OVERFLOW** (0x03) Packet append pointer is invalid
- **NX_PTR_ERROR** (0x07) Invalid IP pointer or packet pointer
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service
- **NX_INVALID_PARAMETERS** (0x4D) Not valid IPv6 address input

### Allowed From

Threads

### Preemption Possible

No

### Description

This service sends a raw IPv4 or IPv6 packet using the specified IPv4 or IPv6 address as source address. This service is typically used on a multihome system, if the system is unable to determine an appropriate interface (for example, if the destination IP address is IPv4 broadcast, multicast or IPv6 multicast address). The parameter *address_index* allows the application to specify the source address to use when sending this raw packet.

The IP instance must be previously created and raw IP packet handling must be enabled using the ***nx_ip_raw_packet_enable**: service.

> **Warning:** *Unless an error is returned, the application should not release the packet after this call. Doing so will cause unpredictable results because the network driver will also try to release the packet after 
transmission*.

### Parameters

- *ip_ptr*: IP instance pointer
- *packet_ptr*: Pointer to packet to send
- *destination_ip*: Destination IP address
- *address_index*: Index to the IPv4 or IPv6 addresses to use as source address.
- *protocol*: Value for the protocol field
- *ttl*: Value for ttl or hop limit
- *tos*: Value for tos or traffic class and flow label

### Return Values 

- **NX_SUCCESS** (0x00) Packet is sent successfully
- **NX_UNDERFLOW** (0x02) Not enough room for IPv4 or IPv6 header in the packet
- **NX_OVERFLOW** (0x03) Packet append pointer is invalid
- **NX_PTR_ERROR** (0x07) Invalid pointer to IP control block, packet, or destination_ip
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service
- **NX_NOT_ENABLED** (0x14) Raw processing not enabled
- **NX_IP_ADDRESS_ERROR** (0x21) Address error
- **NX_INVALID_INTERFACE** (0x4C) Invalid interface index
- **NX_INVALID_PARAMETERS** (0x4D) Not valid IPv6 address input

### Allowed From

Thread

### Preemption Possible

No

### Description

This service registers an application callback routine that NetX Duo calls whenever the IPv6 Address is changed.

This service is available if the NetX Duo library is built is the option ***NX_ENABLE_IPV6_ADDRESS_CHANGE_NOTIFY*** defined.

### Parameters 

- *ip_ptr*: IP control block pointer
- *ip_address_change_notify*: Application callback function

### Return Values  

- **NX_SUCCESS** (0x00) Successful set
- **NX_NOT_SUPPORTED** (0x4B) IPv6 address change notify feature is not built into the NetX Duo library
- **NX_PTR_ERROR** (0x07) Invalid IP control block pointer
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service
- **NX_NOT_ENABLED** (0x14) IPv6 address change notify is not compiled

### Allowed From

Thread

### Preemption Possible

No

### Description

This service deletes the IPv6 address at the specified index in the IPv6 address table of the specified IP instance. There is no NetX equivalent.

### Parameters 

- *ip_ptr*: Pointer to the previously created IP instance
- *address_index*: Index to IP instance IPv6 address table

### Return Values

- **NX_SUCCESS** (0x00) Address successfully deleted
- **NX_NOT_SUPPORTED** (0x4B) IPv6 feature is not built into the NetX Duo library
- **NX_NO_INTERFACE_ADDRESS** (0x50) No suitable outgoing interface can be found
- **NX_PTR_ERROR** (0x07) Invalid IP pointer
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service

### Allowed From

Initialization, Threads

### Preemption Possible

No

### Description

This service retrieves the IPv6 address and prefix at the specified index in the address table of the specified IP instance. The index of the physical interface the IPv6 address is associated with is returned in the *interface_index* pointer. The NetX equivalent services are ***nx_ip_address_get*** and ***nx_ip_interface_address_get***.

### Parameters 

- *ip_ptr*: Pointer to the previously created IP instance
- *address_index*: Index to IP instance address table
- *ip_address*: Pointer to the address to set
- *prefix_length*: Length of the address prefix (subnet mask)
- *interface_index*: Pointer to the index of the interface

### Return Values

- **NX_SUCCESS** (0x00) IPv6 is successfully enabled
- **NX_NOT_SUPPORTED** (0x4B) IPv6 feature is not built into the NetX Duo library.
- **NX_NO_INTERFACE_ADDRESS** (0x50) No interface address, or invalid address_index
- **NX_PTR_ERROR** (0x07) Invalid IP pointer
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service

### Allowed From

Initialization, Threads

### Preemption Possible

No

### Description

This service sets the supplied IPv6 address and prefix to the specified IP instance. If the *address_index* argument is not null, the index into the IPv6 address table where the address is inserted is returned. The NetX equivalent services are ***nx_ip_address_set**: and ***nx_ip_interface_address_set***.

### Parameters  

- *ip_ptr*: Pointer to the previously created IP instance
- *interface_index*: Index to the interface the IPv6 address is associated with
- *ip_address*: Pointer to the address to set
- *prefix_length*: Length of the address prefix (subnet mask)
- *address_index*: Pointer to the index into the address table where the address is inserted

### Return Values

- **NX_SUCCESS** (0x00) IPv6 is successfully enabled
- **NX_NO_MORE_ENTRIES** (0x15) IP address table is full
- **NX_NOT_SUPPORTED** (0x4B) IPv6 feature is not built into the NetX Duo library.
- **NX_DUPLICATED_ENTRY** (0x52) The supplied IP address is already used on this IP instance
- **NX_PTR_ERROR** (0x07) Invalid IP pointer
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid IPv6 address
- **NX_INVALID_INTERFACE** (0x4C) Interface points to an invalid network interface

### Allowed From

Initialization, Threads

### Preemption Possible

No

### Description

This service adds an IPv6 default router on the specified physical interface to the default router table. The equivalent NetX IPv4 service is ***nx_ip_gateway_address_set***.

*router_address* must point to a valid IPv6 address, and the router must be directly accessible from the specified physical interface.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance
- *router_address*: Pointer to the default router address, in host byte order
- *router_lifetime*: Default router life time, in seconds. Valid values are:
	- **0xFFFF:** No time out
	- **0-0xFFFE:** Timeout value, in seconds
- *index_index*: Pointer to the valid memory location for the network index index through which the router can be reached

### Return Values  

- **NX_SUCCESS** (0x00) Default router is successfully added
- **NX_NO_MORE_ENTRIES** (0x17) No more entries available in the default router table.
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid IPv6 address
- **NX_NOT_SUPPORTED** (0x4B) IPv6 feature is not built into the NetX Duo library.
- **NX_INVALID_PARAMETERS** (0x4D) Not valid IPv6 address input
- **NX_PTR_ERROR** (0x07) Invalid IP instance or storage space
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service
- **NX_INVALID_INTERFACE** (0x4C) Invalid router interface index

### Allowed From

Initialization, Threads

### Preemption Possible

No

### Description

This service deletes an IPv6 default router from the default router table. The equivalent NetX IPv4 service is ***nx_ip_gateway_address_clear***.

### Restrictions

The IP instance has been created. *router_address* must point to valid information.

### Parameters

- *ip_ptr*: Pointer to a previously created IP instance
- *router_address*: Pointer to the IPv6 default gateway address

### Return Values 

- **NX_SUCCESS** (0x00) Router successfully deleted
- **NX_NOT_SUPPORTED** (0x4B) IPv6 feature is not built into the NetX Duo library.
- **NX_NOT_FOUND** (0x4E) The router entry cannot be found
- **NX_PTR_ERROR** (0x07) Invalid IP instance or storage space
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service
- **NX_INVALID_PARAMETERS** (0x82) Invalid non pointer input

### Allowed From

Initialization, Threads

### Preemption Possible

No

### Description

This service retrieves a router entry from the default IPv6 routing table that is attached to a specified network device.

### Parameters

- *ip_ptr*: IP control block pointer
- *interface_index*: Index of the network interface
- *entry_index*: Entry Index
- *router_addr*: Router IPv6 Address
- *router_lifetime*: Pointer to router life time
- *prefix_length*: Pointer to prefix length
- *configuration_method*: Pointer to the information on how the entry was configured

### Return Values  

- **NX_SUCCESS** (0x00) Successful get
- **NX_NOT_FOUND** (0x4E) Router entry not found
- **NX_INVALID_INTERFACE** (0x4C) Interface index is not valid
- **NX_PTR_ERROR** (0x07) Invalid IP control block pointer
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service

### Allowed From

Initialization, threads

### Preemption Possible

No

### Description

This service retrieves an IPv6 default router address, lifetime and prefix length on the specified physical interface from the default router table. The equivalent NetX IPv4 service is ***nx_ip_gateway_address_get*.**

*router_address* must point to a valid NXD_ADDRESS structure, so this service can fill in the IPv6 address of the default router.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance
- *interface_index*: The index to the network interface that the router is accessible through
- *router_address*: Pointer to the storage space for the return value of the default router address, in host byte order.
- *router_lifetime*: Pointer to the router lifetime
- *prefix_length*: Pointer to the router address prefix length

### Return Values 

- **NX_SUCCESS** (0x00) Default router is successfully added
- **NX_NOT_SUPPORTED** (0x4B) IPv6 feature is not built into the NetX Duo library.
- **NX_NOT_FOUND** (0x4E) Default router not found
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service
- **NX_INVALID_INTERFACE** (0x4C) Invalid router interface index
- **NX_PTR_ERROR** (0x07) Invalid IP instance or storage space

### Allowed From

Initialization, Threads

### Preemption Possible

No

### Description

This service retrieves the number of IPv6 default routers configured on a given network interface.

### Parameters

- *ip_ptr*: IP control block pointer
- *interface_index*: Index of the network interface
- *num_entries*: Destination for number of IPv6 routers on a specified network device

### Return Values 

- **NX_SUCCESS** (0x00) Successful get
- **NX_NOT_SUPPORTED** (0x4B) IPv6 feature is not built into the NetX Duo library.
- **NX_INVALID_INTERFACE** (0x4C) Device index value is not valid
- **NX_PTR_ERROR** (0x07) Invalid IP control block pointer or num_entries pointer

### Allowed From

Thread

### Preemption Possible

No

### Description

This service disables the IPv6 for the specified IP instance. It also clears the default router table, ND cache and IPv6 address table, leaves the all multicast groups, and resets the router solicitation variables. This service has no effect if IPv6 is not enabled.

### Parameters

- *ip_ptr*: IP instance pointer

### Return Values  

- **NX_SUCCESS** (0x00) Successful disable
- **NX_NOT_SUPPORTED** (0x4B) IPv6 feature is not built into the NetX Duo library.
- **NX_PTR_ERROR** (0x07) Invalid IP control block pointer
- **NX_NOT_SUPPORT** (0x4B) IPv6 module is not compiled
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service

### Allowed From

Initialization, threads

### Preemption Possible

No

### Description

This service enables IPv6 services. When the IPv6 services are enabled, the IP instance joins the all-node multicast group (FF02::1). This service does not set the link local address or global address. Applications should use *nxd_ipv6_address_set* to configure the device network addresses. There is no NetX equivalent.

### Parameters

- *ip_ptr*: Pointer to the previously created IP instance

### Return Values  

- **NX_SUCCESS** (0x00) IPv6 is successfully enabled
- **NX_ALREADY_ENABLED** (0x15) IPv6 is already enabled
- **NX_NOT_SUPPORTED** (0x4B) IPv6 feature is not built into the NetX Duo library.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service

### Allowed From

Initialization, Threads

### Preemption Possible

No

### Description

This service allows an application to join a specific IPv6 multicast address on a specific network interface. The link driver is notified to add the multicast address. This service is available if the NetX Duo library is built with the option ***NX_ENABLE_IPV6_MULTICAST*** defined.

### Parameters  

- *ip_ptr*: IP instance pointer
- *group_address*: IPv6 multicast address
- *interface_index*: The index to the network interface associated with the multicast group

### Return Values  

- **NX_SUCCESS** (0x00) Successfully enables receiving on IPv6 multicast address
- **NX_NO_MORE_ENTRIES** (0x17) No more entries in the IPv6 multicast table.
- **NX_OVERFLOW** (0x03) No more group addresses available in the device driver
- **NX_NOT_SUPPORTED** (0x4B) IPv6 feature or IPv6 multicast feature is not built into the NetX Duo library
- **NX_PTR_ERROR** (0x07) Invalid IP control block pointer
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid IPv6 address
- **NX_INVALID_INTERFACE** (0x4C) Interface index is not valid

### Allowed From

Threads

### Preemption Possible

No

### Description

This service removes the
specific IPv6 multicast address from the specific network device. The link driver is also notified of the removal of the IPv6 multicast address.This service is available if the NetX Duo library is built with the option **NX_ENABLE_IPV6_MULTICAST** defined.

### Parameters  

- *ip_ptr*: IP instance pointer
- *group_address*: IPv6 multicast address
- *interface_index*: The index to the network interface associated with group

### Return Values 

- **NX_SUCCESS** (0x00) Successful multicast leave
- **NX_ENTRY_NOT_FOUND** (0x16) Entry not found
- **NX_NOT_SUPPORTED** (0x4B) IPv6 feature or IPv6 multicast feature is not built into the NetX Duo library
- **NX_PTR_ERROR** (0x07) Invalid IP control block pointer
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid IPv6 address
- **NX_INVALID_INTERFACE** (0x4C) Interface index is not valid

### Allowed From

Threads

### Preemption Possible

No

### Description

This service disables the IPv6 stateless address auto configuration feature on a specified network device. It has no effect if the IPv6 address has been configured.

This service is available if the NetX Duo library is built with the option **NX_IPV6_STATELESS_AUTOCONFIG_CONTROL** defined.

### Parameters

- *ip_ptr*: IP instance pointer
- *interface_index*: The index to the network interface that the IPv6 stateless address autoconfiguration should be disabled.

### Return Values 

- **NX_SUCCESS** (0x00) Successfully disables stateless address autoconfigure feature.
- **NX_NOT_SUPPORTED** (0x4B) IPv6 feature or IPv6 stateless address autoconfig control feature is not built into the NetX Duo library
- **NX_INVALID_INTERFACE** (0x4C) Interface index is not valid
- **NX_PTR_ERROR** (0x07) Invalid IP control block pointer
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service

### Allowed From

Initialization, threads

### Preemption Possible

No

### Description

This service enables the IPv6 stateless address auto configuration feature on a specified network device.

This service is available if the NetX Duo library is built with the option **NX_IPV6_STATELESS_AUTOCONFIG_CONTROL** defined.

### Parameters 

- *ip_ptr*: IP instance pointer
- *interface_index*: The index to the network interface that the IPv6 stateless address autoconfiguration should be enabled.

### Return Values 

- **NX_SUCCESS** (0x00) Successfully enables stateless address autoconfig feature.
- **NX_ALREADY_ENABLED** (0x15) Already enabled
- **NX_NOT_SUPPORTED** (0x4B) IPv6 feature or IPv6 stateless address autoconfig control feature is not built into the NetX Duo library
- **NX_INVALID_INTERFACE** (0x4C) Interface index is not valid
- **NX_PTR_ERROR** (0x07) Invalid IP control block pointer
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service

### Allowed From

Initialization, threads

### Preemption Possible

No

### Description

This service deletes an IPv6 neighbor discovery cache entry for the supplied IP address. The equivalent NetX IPv4 function is ***nx_arp_static_entry_delete***.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance
- *ip_address*: Pointer to IPv6 address to delete, in host byte order

### Return Values

- **NX_SUCCESS** (0x00) Successfully deleted the address
- **NX_ENTRY_NOT_FOUND** (0x16) Address not found in the IPv6 neighbor cache
- **NX_NOT_SUPPORTED** (0x4B) IPv6 feature is not built into the NetX Duo library
- **NX_PTR_ERROR** (0x07) Invalid IP instance or storage space
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service

### Allowed From

Initialization, threads

### Preemption Possible

No

### Description

This service adds an entry to the neighbor discovery cache for the specified IP address *ip_address* mapped to the hardware MAC address on the specified network interface index (interface_index). The equivalent NetX IPv4 service is ***nx_arp_static_entry_create***.

### Parameters 

- *ip_ptr*: Pointer to previously created IP instance
- *dest_ip*: Pointer to IPv6 address instance
- *interface_index*: Index specifying physical network interface where the destination IPv6 address can be reached 
- *mac*: Pointer to hardware address.

### Return Values 

- **NX_SUCCESS** (0x00) Entry successfully added
- **NX_NOT_SUCCESSFUL** (0x43) Invalid cache or no neighbor cache entries available
- **NX_NOT_SUPPORTED** (0x4B) IPv6 feature is not built into the NetX Duo library
- **NX_PTR_ERROR** (0x07) Invalid IP instance or storage space
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service
- **NX_INVALID_INTERFACE** (0x4C) Invalid interface index value.

### Allowed From

Initialization, Threads

### Preemption Possible

No

### Description

This service attempts to find a physical hardware address in the IPv6 neighbor discovery cache that is associated with the supplied IPv6 address. The index of the network interface through which the neighbor can be reached is also returned in the parameter *interface_index.* The equivalent NetX IPv4 service is ***nx_arp_hardware_address_find***.

### Parameters 

- *ip_ptr*: Pointer to previously created IP instance
- *ip_address*: Pointer to IP address to find, host byte order
- *physical_msw*: Pointer to the top 16 bits (47-32) of the physical address, in host byte order 
- *physical_lsw*: Pointer to the lower 32 bits (31-0) of the physical address in host byte order
- *interface_index*: Pointer to the valid memory location for the interface index specifying the network device on which the IPv6 address can be reached.

### Return Values

- **NX_SUCCESS** (0x00) Successfully found the address
- **NX_ENTRY_NOT_FOUND** (0x16) Mapping not in the neighbor cache
- **NX_NOT_SUPPORTED** (0x4B) IPv6 feature is not built into the NetX Duo library
- **NX_INVALID_PARAMETERS** (0x4D) The supplied IP address is not version 6.
- **NX_PTR_ERROR** (0x07) Invalid IP instance or storage space
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service

### Allowed From

Threads

### Preemption Possible

No

### Description

This service invalidates the entire IPv6 neighbor discovery cache. This service can be invoked either before or after ICMPv6 has been enabled. This service is not applicable to IPv4 connectivity, so there
is no NetX equivalent service.

### Parameters

- *ip_ptr*: Pointer to IP instance

### Return Values

- **NX_SUCCESS** (0x00) Cache successfully invalidated
- **NX_NOT_SUPPORTED** (0x4B) IPv6 feature is not built into the NetX Duo library
- **NX_PTR_ERROR** (0x07) Invalid IP instance
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service

### Allowed From

Threads

### Preemption Possible

No

### Description

This service attempts to find an IPv6 address in the IPv6 neighbor discovery cache that is associated with the supplied physical address. The index of the network interface through which the neighbor can be reached is also returned. The equivalent NetX IPv4 service is ***nx_arp_ip_address_find***.

### Parameters 

- *ip_ptr*: Pointer to previously created IP instance
- *ip_address*: Pointer to valid NXD_ADDRESS structure
- *physical_msw*: Top 16 bits (47-32) of the physical address to find, host byte order
- *physical_lsw*: Lower 32 bits (31-0) of the physical address to find, host byte order
- *interface_index*: Pointer to the network device index through which the IPv6 address can be reached

### Return Values

- **NX_SUCCESS** (0x00) Successfully found the address
- **NX_ENTRY_NOT_FOUND** (0x16) Physical address not found in the neighbor cache
- **NX_NOT_SUPPORTED** (0x4B) IPv6 feature is not built into the NetX Duo library
- **NX_PTR_ERROR** (0x07) Invalid IP instance or storage space
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service 
- **NX_INVALID_PARAMETERS** (0x4D) MAC address is zero.

### Allowed From

Threads

### Preemption Possible

No

### Description

This service makes TCP connection using a previously created TCP client socket to the specified server's port. This service works on either IPv4 or IPv6 networks. Valid TCP server ports range from 0 through 0xFFFF. NetX Duo determines the appropriate physical interface based on the server IP address. The NetX IPv4 equivalent is ***nx_tcp_client_socket_connect***.

The socket must have been bound to a local port.

### Parameters

- *socket_ptr*: Pointer to previously created TCP socket
- *server_ip*: Pointer to IPv4 or IPv6 destination address, in host byte order
- *server_port*: Server port number to connect to (1 through 0xFFFF), in host byte order
- *wait_option*: Wait option while the connection is being established. The wait options are defined as follows:
	- **NX_NO_WAIT** (0x00000000)
	- **NX_WAIT_FOREVER** (0xFFFFFFFF)
	- **timeout value in ticks** (0x00000001 through 0xFFFFFFFE)

### Return Values

- **NX_SUCCESS** (0x00) Successful socket connect
- **NX_WAIT_ABORTED** (0x1A) Requested suspension was aborted by a call to tx_thread_wait_abort
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid server IPv4 or IPv6 address
- **NX_NOT_BOUND** (0x24) Socket is not bound
- **NX_NOT_CLOSED** (0x35) Socket is not in a closed state
- **NX_IN_PROGRESS** (0x37) No wait was specified, connection attempt is in progress
- **NX_INVALID_INTERFACE** (0x4C) Invalid interface index.
- **NX_NO_INTERFACE_ADDRESS** (0x50) The network interface does not have valid IPv6 address
- **NX_NOT_ENABLED** (0x14) TCP not enabled
- **NX_INVALID_PORT** (0x46) Invalid port
- **NX_PTR_ERROR** (0x07) Invalid socket pointer
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service
- **NX_NOT_CONNECTED** (0x38) Connection fails.

### Allowed From

Threads

### Preemption Possible

No

### Description

This service retrieves peer IP address and port information for the connected TCP socket over either IPv4 or IPv6 network. The equivalent NetX IPv4 service is ***nx_tcp_socket_peer_info_get***.

Note that *socket_ptr* must point to a TCP socket that is already in the connected state.

### Parameters

- *socket_ptr*: Pointer to TCP socket connected to peer host
- *peer_ip_address*: Pointer to IPv4 or IPv6 peer address. The returned IP address is in host byte order.
- *peer_port*: Pointer to peer port number. The returned port number is in host byte order.

### Return Values

- **NX_SUCCESS** (0x00) Socket information successfully retrieved
- **NX_NOT_CONNECTED** (0x38) Socket not connected to peer
- **NX_NOT_ENABLED** (0x14) TCP not enabled
- **NX_PTR_ERROR** (0x07) Invalid pointer input
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service

### Allowed From

Threads

### Preemption Possible

No

### Description

This service extracts network parameters from a packet received on either IPv4 or IPv6 UDP networks. The NetX equivalent service is ***nx_udp_packet_info_extract.***

### Parameters

- *packet_ptr*: Pointer to packet.
- *ip_address*: Pointer to sender IP address.
- *protocol*: Pointer to protocol to be returned.
- *port*: Pointer to sender's port number.
- *interface_index*: Pointer to the index of the network interface from which this packet is received

### Return Values 

- **NX_SUCCESS** (0x00) Packet interface data successfully extracted.
- **NX_INVALID_PACKET** (0x12) Packet is neither IPv4 or IPv6.
- **NX_PTR_ERROR** (0x07) Invalid pointer input
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service

### Allowed From

Threads

### Preemption Possible

No

### Description

This service sends a UDP datagram through a previously created and bound UDP socket for either IPv4 or IPv6 networks. NetX Duo finds a suitable local IP address as source address based on the destination IP address. To specify a specific interface and source IP address, the application should use the **nxd_udp_socket_source_send** service.

Note that this service returns immediately regardless of whether the UDP datagram was successfully sent. The NetX (IPv4) equivalent service is ***nx_udp_socket_send***.

The socket must be bound to a local port.

> **Warning:** *Unless an error is returned, the application should not release the packet after this call. Doing so will cause unpredictable results because the network driver will also try to release the packet after 
transmission*.

### Parameters

- *socket_ptr*: Pointer to previously created UDP socket instance
- *packet_ptr*: UDP datagram packet pointer
- *ip_address*: Pointer to destination IPv4 or IPv6 address
- *port*: Valid destination port number between 1 and 0xFFFF), in host byte order

### Return Values

- **NX_SUCCESS** (0x00) Successful UDP socket send
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid server IPv4 or IPv6 address
- **NX_NOT_BOUND** (0x24) Socket not bound to any port
- **NX_NO_INTERFACE_ADDRESS** (0x50) No suitable outgoing interface can be found.
- **NX_UNDERFLOW** (0x02) Not enough room for UDP header in the packet
- **NX_OVERFLOW** (0x03) Packet append pointer is invalid
- **NX_PTR_ERROR** (0x07) Invalid socket pointer, address pointer, or packet pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service
- **NX_NOT_ENABLED** (0x14) UDP has not been enabled
- **NX_INVALID_PORT** (0x46) Port number is not within a valid range

### Allowed From

Threads

### Preemption Possible

No

### Description

This service sends a UDP datagram through a previously created and bound UDP socket for either IPv4 or IPv6 networks. The parameter *address_index* specifies the source IP address to use for the outgoing packet. Note that the function returns immediately regardless of whether the UDP datagram was successfully sent.

The socket must be bound to a local port.

The NetX (IPv4) equivalent service is ***nx_udp_socket_source_send***.

> **Warning:** *Unless an error is returned, the application should not release the packet after this call. Doing so will cause unpredictable results because the network driver will also try to release the packet after 
transmission*.

### Parameters

- *socket_ptr*: Pointer to previously created UDP socket instance
- *packet_ptr*: UDP datagram packet pointer
- *ip_address*: Pointer to destination IPv4 or IPv6 address port Valid destination port number between 1 and 0xFFFF), in host byte order
- *address_index*: Index specifying the source address to use for the packet

### Return Values

- **NX_SUCCESS** (0x00) Successful UDP socket send
- **NX_IP_ADDRESS_ERROR** (0x21) Invalid server IPv4 or IPv6 address
- **NX_NOT_BOUND** (0x24) Socket not bound to any port
- **NX_NO_INTERFACE_ADDRESS** (0x50) No suitable outgoing interface can be found.
- **NX_NOT_FOUND** (0x4E) No suitable interface can be found
- **NX_PTR_ERROR** (0x07) Invalid socket pointer, address, or packet pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service
- **NX_NOT_ENABLED** (0x14) UDP has not been enabled
- **NX_INVALID_PORT** (0x46) Port number is not within valid range.
- **NX_INVALID_INTERFACE** (0x4C) Specified network interface is valid
- **NX_UNDERFLOW** (0x02) Not enough room for UDP header in the packet
- **NX_OVERFLOW** (0x03) Packet append pointer is invalid

### Allowed From

Threads

### Preemption Possible

No

### Description

This service extracts the source IP address and port number from a UDP packet received through the host UDP socket. The NetX (IPv4) equivalent is ***nx_udp_source_extract***.

### Parameters

- *packet_ptr*: Pointer to received UDP packet
- *ip_address*: Pointer to NXD_ADDRESS structure to store packet source IP address
- *port*: Pointer to UDP socket port number

### Return Values

- **NX_SUCCESS** (0x00) Successful source extract
- **NX_INVALID_PACKET** (0x12) Packet is not valid
- **NX_PTR_ERROR** (0x07) Invalid socket pointer
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service

### Allowed From

Threads

### Preemption Possible

No

