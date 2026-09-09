/* SPDX-License-Identifier: GPL-2.0+ */
/* comedi_usb.h
 * header file for USB Comedi drivers
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1997-2000 David A. Schleef <ds@schleef.org>
 */

// Dependencies supplied by the Linux USB and Comedi device headers.

#[allow(non_camel_case_types)]
pub enum usb_interface {}
#[allow(non_camel_case_types)]
pub enum usb_device {}
#[allow(non_camel_case_types)]
pub enum comedi_device {}
#[allow(non_camel_case_types)]
pub enum comedi_driver {}
#[allow(non_camel_case_types)]
pub enum usb_driver {}

extern "C" {
    pub fn comedi_to_usb_interface(dev: *mut comedi_device) -> *mut usb_interface;
    pub fn comedi_to_usb_dev(dev: *mut comedi_device) -> *mut usb_device;

    pub fn comedi_usb_auto_config(
        intf: *mut usb_interface,
        driver: *mut comedi_driver,
        context: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    pub fn comedi_usb_auto_unconfig(intf: *mut usb_interface);

    pub fn comedi_usb_driver_register(
        comedi_driver: *mut comedi_driver,
        usb_driver: *mut usb_driver,
    ) -> ::core::ffi::c_int;
    pub fn comedi_usb_driver_unregister(
        comedi_driver: *mut comedi_driver,
        usb_driver: *mut usb_driver,
    );
}

/**
 * module_comedi_usb_driver() - Helper macro for registering a comedi USB driver
 * @__comedi_driver: comedi_driver struct
 * @__usb_driver: usb_driver struct
 *
 * Helper macro for comedi USB drivers which do not do anything special
 * in module init/exit. This eliminates a lot of boilerplate. Each
 * module may only use this macro once, and calling it replaces
 * module_init() and module_exit()
 */
#[macro_export]
macro_rules! module_comedi_usb_driver {
    ($__comedi_driver:expr, $__usb_driver:expr) => {
        module_driver!(
            $__comedi_driver,
            $crate::comedi_usb_driver_register,
            $crate::comedi_usb_driver_unregister,
            &mut ($__usb_driver)
        )
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
