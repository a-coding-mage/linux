// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi_usb.c
 * Comedi USB driver specific functions.
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1997-2000 David A. Schleef <ds@schleef.org>
 */

// Linux kernel and Comedi USB declarations are supplied by the surrounding
// translation unit.

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct usb_interface {
    pub dev: device,
}

#[repr(C)]
pub struct usb_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct comedi_device {
    pub hw_dev: *mut device,
}

#[repr(C)]
pub struct comedi_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct usb_driver {
    _private: [u8; 0],
}

extern "C" {
    fn to_usb_interface(dev: *mut device) -> *mut usb_interface;
    fn interface_to_usbdev(intf: *mut usb_interface) -> *mut usb_device;
    fn comedi_auto_config(dev: *mut device, driver: *mut comedi_driver, context: usize) -> i32;
    fn comedi_auto_unconfig(dev: *mut device);
    fn comedi_driver_register(driver: *mut comedi_driver) -> i32;
    fn comedi_driver_unregister(driver: *mut comedi_driver);
    fn usb_register(driver: *mut usb_driver) -> i32;
    fn usb_deregister(driver: *mut usb_driver);
}

/// comedi_to_usb_interface() - Return USB interface attached to COMEDI device
/// @dev: COMEDI device.
///
/// Assuming @dev->hw_dev is non-%NULL, it is assumed to be pointing to a
/// a &struct device embedded in a &struct usb_interface.
///
/// Return: Attached USB interface if @dev->hw_dev is non-%NULL.
/// Return %NULL if @dev->hw_dev is %NULL.
pub unsafe fn comedi_to_usb_interface(dev: *mut comedi_device) -> *mut usb_interface {
    if !(*dev).hw_dev.is_null() {
        to_usb_interface((*dev).hw_dev)
    } else {
        core::ptr::null_mut()
    }
}

/// comedi_to_usb_dev() - Return USB device attached to COMEDI device
/// @dev: COMEDI device.
///
/// Assuming @dev->hw_dev is non-%NULL, it is assumed to be pointing to a
/// a &struct device embedded in a &struct usb_interface.
///
/// Return: USB device to which the USB interface belongs if @dev->hw_dev is
/// non-%NULL.  Return %NULL if @dev->hw_dev is %NULL.
pub unsafe fn comedi_to_usb_dev(dev: *mut comedi_device) -> *mut usb_device {
    let intf = comedi_to_usb_interface(dev);

    if !intf.is_null() {
        interface_to_usbdev(intf)
    } else {
        core::ptr::null_mut()
    }
}

/// comedi_usb_auto_config() - Configure/probe a USB COMEDI driver
/// @intf: USB interface.
/// @driver: Registered COMEDI driver.
/// @context: Driver specific data, passed to comedi_auto_config().
///
/// Typically called from the usb_driver (*probe) function.  Auto-configure a
/// COMEDI device, using a pointer to the &struct device embedded in *@intf as
/// the hardware device.  The @context value gets passed through to @driver's
/// "auto_attach" handler.  The "auto_attach" handler may call
/// comedi_to_usb_interface() on the passed in COMEDI device to recover @intf.
///
/// Return: The result of calling comedi_auto_config() (%0 on success, or
/// a negative error number on failure).
pub unsafe fn comedi_usb_auto_config(
    intf: *mut usb_interface,
    driver: *mut comedi_driver,
    context: usize,
) -> i32 {
    comedi_auto_config(&mut (*intf).dev, driver, context)
}

/// comedi_usb_auto_unconfig() - Unconfigure/disconnect a USB COMEDI device
/// @intf: USB interface.
///
/// Typically called from the usb_driver (*disconnect) function.
/// Auto-unconfigure a COMEDI device attached to this USB interface, using a
/// pointer to the &struct device embedded in *@intf as the hardware device.
/// The COMEDI driver's "detach" handler will be called during unconfiguration
/// of the COMEDI device.
///
/// Note that the COMEDI device may have already been unconfigured using the
/// %COMEDI_DEVCONFIG ioctl, in which case this attempt to unconfigure it
/// again should be ignored.
pub unsafe fn comedi_usb_auto_unconfig(intf: *mut usb_interface) {
    comedi_auto_unconfig(&mut (*intf).dev);
}

/// comedi_usb_driver_register() - Register a USB COMEDI driver
pub unsafe fn comedi_usb_driver_register(
    comedi_driver: *mut comedi_driver,
    usb_driver: *mut usb_driver,
) -> i32 {
    let mut ret = comedi_driver_register(comedi_driver);
    if ret < 0 {
        return ret;
    }

    ret = usb_register(usb_driver);
    if ret < 0 {
        comedi_driver_unregister(comedi_driver);
        return ret;
    }

    0
}

/// comedi_usb_driver_unregister() - Unregister a USB COMEDI driver
pub unsafe fn comedi_usb_driver_unregister(
    comedi_driver: *mut comedi_driver,
    usb_driver: *mut usb_driver,
) {
    usb_deregister(usb_driver);
    comedi_driver_unregister(comedi_driver);
}

// EXPORT_SYMBOL_GPL(comedi_to_usb_interface);
// EXPORT_SYMBOL_GPL(comedi_to_usb_dev);
// EXPORT_SYMBOL_GPL(comedi_usb_auto_config);
// EXPORT_SYMBOL_GPL(comedi_usb_auto_unconfig);
// EXPORT_SYMBOL_GPL(comedi_usb_driver_register);
// EXPORT_SYMBOL_GPL(comedi_usb_driver_unregister);
// MODULE_AUTHOR("https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi USB interface module");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
