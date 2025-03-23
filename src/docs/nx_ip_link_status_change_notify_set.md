Set the link status change notify callback function

### Description

This service configures the link status change notify callback function. The user-supplied *link_status_change_notify* routine is invoked when either the primary or secondary interface status is changed (such as IP address is changed.) If *link_status_change_notify* is NULL, the link status change notify callback feature is disabled.

### Parameters

- *ip_ptr*: IP control block pointer
- *link_status_change_notify*: User-supplied callback function to be called upon a change to the physical interface.

### Return Values

- **NX_SUCCESS** (0x00) Successful set
- **NX_PTR_ERROR** (0x07) Invalid IP control block pointer or new physical address pointer
- **NX_CALLER_ERROR** (0x11) Service is not called from system initialization or thread context.

### Allowed From

Initialization, threads

### Preemption Possible

No

