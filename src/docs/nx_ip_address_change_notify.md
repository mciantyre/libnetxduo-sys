Notify application if IP address changes

### Description

This service registers an application notification function that is called whenever the IPv4 address is changed.

### Parameters

- *ip_ptr*: Pointer to previously created IP instance.
- *change_notify*: Pointer to IP change notification function. If this parameter is NX_NULL, IP address change notification is disabled.
- *additional_info*: Pointer to optional additional information that is also supplied to the notification function when the IP address is changed.

### Return Values  

- **NX_SUCCESS** (0x00) Successful IP address change notification.
- **NX_PTR_ERROR** (0x07) Invalid IP pointer.
- **NX_CALLER_ERROR** (0x11) Invalid caller of this service.

### Allowed From

Initialization, threads

### Preemption Possible

No

