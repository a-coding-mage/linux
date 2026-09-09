// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi_pcmcia.c
 * Comedi PCMCIA driver specific functions.
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1997-2000 David A. Schleef <ds@schleef.org>
 */

// Dependencies supplied by the surrounding kernel/comedi Rust bindings.

extern "C" {
    fn to_pcmcia_dev(dev: *mut device) -> *mut pcmcia_device;
    fn pcmcia_request_io(link: *mut pcmcia_device) -> i32;
    fn pcmcia_loop_config(
        link: *mut pcmcia_device,
        conf_check: Option<unsafe extern "C" fn(*mut pcmcia_device, *mut core::ffi::c_void) -> i32>,
        priv_data: *mut core::ffi::c_void,
    ) -> i32;
    fn pcmcia_enable_device(link: *mut pcmcia_device) -> i32;
    fn pcmcia_disable_device(link: *mut pcmcia_device);
    fn comedi_auto_config(dev: *mut device, driver: *mut comedi_driver, context: i32) -> i32;
    fn comedi_auto_unconfig(dev: *mut device);
    fn comedi_driver_register(driver: *mut comedi_driver) -> i32;
    fn comedi_driver_unregister(driver: *mut comedi_driver);
    fn pcmcia_register_driver(driver: *mut pcmcia_driver) -> i32;
    fn pcmcia_unregister_driver(driver: *mut pcmcia_driver);
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pcmcia_device {
    pub dev: device,
    pub config_index: u32,
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
pub struct pcmcia_driver {
    _private: [u8; 0],
}

pub unsafe extern "C" fn comedi_to_pcmcia_dev(dev: *mut comedi_device) -> *mut pcmcia_device {
    if !(*dev).hw_dev.is_null() {
        to_pcmcia_dev((*dev).hw_dev)
    } else {
        core::ptr::null_mut()
    }
}

unsafe extern "C" fn comedi_pcmcia_conf_check(
    link: *mut pcmcia_device,
    _priv_data: *mut core::ffi::c_void,
) -> i32 {
    if (*link).config_index == 0 {
        return -22; // -EINVAL
    }

    pcmcia_request_io(link)
}

pub unsafe extern "C" fn comedi_pcmcia_enable(
    dev: *mut comedi_device,
    mut conf_check: Option<unsafe extern "C" fn(*mut pcmcia_device, *mut core::ffi::c_void) -> i32>,
) -> i32 {
    let link = comedi_to_pcmcia_dev(dev);

    if link.is_null() {
        return -19; // -ENODEV
    }

    if conf_check.is_none() {
        conf_check = Some(comedi_pcmcia_conf_check);
    }

    let ret = pcmcia_loop_config(link, conf_check, core::ptr::null_mut());
    if ret != 0 {
        return ret;
    }

    pcmcia_enable_device(link)
}

pub unsafe extern "C" fn comedi_pcmcia_disable(dev: *mut comedi_device) {
    let link = comedi_to_pcmcia_dev(dev);

    if !link.is_null() {
        pcmcia_disable_device(link);
    }
}

pub unsafe extern "C" fn comedi_pcmcia_auto_config(
    link: *mut pcmcia_device,
    driver: *mut comedi_driver,
) -> i32 {
    comedi_auto_config(&mut (*link).dev, driver, 0)
}

pub unsafe extern "C" fn comedi_pcmcia_auto_unconfig(link: *mut pcmcia_device) {
    comedi_auto_unconfig(&mut (*link).dev);
}

pub unsafe extern "C" fn comedi_pcmcia_driver_register(
    comedi_driver: *mut comedi_driver,
    pcmcia_driver: *mut pcmcia_driver,
) -> i32 {
    let mut ret = comedi_driver_register(comedi_driver);
    if ret < 0 {
        return ret;
    }

    ret = pcmcia_register_driver(pcmcia_driver);
    if ret < 0 {
        comedi_driver_unregister(comedi_driver);
        return ret;
    }

    0
}

pub unsafe extern "C" fn comedi_pcmcia_driver_unregister(
    comedi_driver: *mut comedi_driver,
    pcmcia_driver: *mut pcmcia_driver,
) {
    pcmcia_unregister_driver(pcmcia_driver);
    comedi_driver_unregister(comedi_driver);
}

// EXPORT_SYMBOL_GPL(comedi_to_pcmcia_dev);
// EXPORT_SYMBOL_GPL(comedi_pcmcia_enable);
// EXPORT_SYMBOL_GPL(comedi_pcmcia_disable);
// EXPORT_SYMBOL_GPL(comedi_pcmcia_auto_config);
// EXPORT_SYMBOL_GPL(comedi_pcmcia_auto_unconfig);
// EXPORT_SYMBOL_GPL(comedi_pcmcia_driver_register);
// EXPORT_SYMBOL_GPL(comedi_pcmcia_driver_unregister);
// MODULE_AUTHOR("https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi PCMCIA interface module");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
