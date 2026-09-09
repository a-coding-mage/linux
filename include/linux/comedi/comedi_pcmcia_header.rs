/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * comedi_pcmcia.h
 * header file for Comedi PCMCIA drivers
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1997-2000 David A. Schleef <ds@schleef.org>
 */

// Dependencies supplied by the PCMCIA and Comedi headers.

#[repr(C)]
pub struct pcmcia_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pcmcia_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct comedi_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct comedi_driver {
    _private: [u8; 0],
}

extern "C" {
    pub fn comedi_to_pcmcia_dev(dev: *mut comedi_device) -> *mut pcmcia_device;

    pub fn comedi_pcmcia_enable(
        dev: *mut comedi_device,
        conf_check: Option<unsafe extern "C" fn(
            p_dev: *mut pcmcia_device,
            priv_data: *mut core::ffi::c_void,
        ) -> core::ffi::c_int>,
    ) -> core::ffi::c_int;

    pub fn comedi_pcmcia_disable(dev: *mut comedi_device);

    pub fn comedi_pcmcia_auto_config(
        link: *mut pcmcia_device,
        driver: *mut comedi_driver,
    ) -> core::ffi::c_int;

    pub fn comedi_pcmcia_auto_unconfig(link: *mut pcmcia_device);

    pub fn comedi_pcmcia_driver_register(
        comedi_driver: *mut comedi_driver,
        pcmcia_driver: *mut pcmcia_driver,
    ) -> core::ffi::c_int;

    pub fn comedi_pcmcia_driver_unregister(
        comedi_driver: *mut comedi_driver,
        pcmcia_driver: *mut pcmcia_driver,
    );
}

/**
 * Helper macro for registering a comedi PCMCIA driver.
 *
 * Each module may only use this macro once; calling it replaces module_init()
 * and module_exit().
 *
 * The `module_driver` macro is supplied by the kernel module infrastructure.
 */
#[macro_export]
macro_rules! module_comedi_pcmcia_driver {
    ($__comedi_driver:expr, $__pcmcia_driver:expr) => {
        module_driver!(
            $__comedi_driver,
            $crate::comedi_pcmcia_driver_register,
            $crate::comedi_pcmcia_driver_unregister,
            &mut ($__pcmcia_driver)
        );
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
