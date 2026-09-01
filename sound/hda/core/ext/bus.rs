// SPDX-License-Identifier: GPL-2.0-only
/*
 *  hdac-ext-bus.c - HD-audio extended core bus functions.
 *
 *  Copyright (C) 2014-2015 Intel Corp
 *  Author: Jeeja KP <jeeja.kp@intel.com>
 *  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

// C dependencies:
// #include <linux/module.h>
// #include <linux/slab.h>
// #include <linux/io.h>
// #include <sound/hdaudio_ext.h>
//
// MODULE_DESCRIPTION("HDA extended core");
// MODULE_LICENSE("GPL v2");

use core::ffi::c_int;

extern "C" {
    static mut snd_hda_bus_type: bus_type;

    fn snd_hdac_bus_init(
        bus: *mut hdac_bus,
        dev: *mut device,
        ops: *const hdac_bus_ops,
    ) -> c_int;
    fn snd_hdac_bus_exit(bus: *mut hdac_bus);
    fn snd_hdac_device_unregister(codec: *mut hdac_device);
    fn put_device(dev: *mut device);
    fn list_empty(head: *const list_head) -> c_int;
    fn WARN_ON(condition: c_int) -> c_int;
    fn drv_to_hdac_driver(drv: *mut device_driver) -> *mut hdac_driver;
    fn dev_to_hdac_dev(dev: *mut device) -> *mut hdac_device;
    fn driver_register(drv: *mut device_driver) -> c_int;
    fn driver_unregister(drv: *mut device_driver);

    /*
     * Rust translation of list_for_each_entry_safe(codec, __codec,
     * &bus->codec_list, list).  The concrete list traversal primitive is
     * supplied by the surrounding kernel/Rust binding layer.
     */
    fn hdac_codec_list_for_each_entry_safe(
        head: *mut list_head,
        cb: unsafe extern "C" fn(codec: *mut hdac_device, data: *mut core::ffi::c_void),
        data: *mut core::ffi::c_void,
    );
}

pub const HDA_DEV_ASOC: c_int = 1;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct bus_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hdac_bus_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hdac_ext_bus_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub bus: *mut bus_type,
    pub probe: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(dev: *mut device)>,
}

#[repr(C)]
pub struct device {
    pub driver: *mut device_driver,
}

#[repr(C)]
pub struct hdac_bus {
    pub ext_ops: *const hdac_ext_bus_ops,
    pub idx: c_int,
    pub cmd_dma_state: bool,
    pub hlink_list: list_head,
    pub codec_list: list_head,
}

#[repr(C)]
pub struct hdac_device {
    pub dev: device,
    pub list: list_head,
}

#[repr(C)]
pub struct hdac_driver {
    pub type_: c_int,
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(hdev: *mut hdac_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(hdev: *mut hdac_device) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(hdev: *mut hdac_device)>,
}

/**
 * snd_hdac_ext_bus_init - initialize a HD-audio extended bus
 * @bus: the pointer to HDAC bus object
 * @dev: device pointer
 * @ops: bus verb operators
 * @ext_ops: operators used for ASoC HDA codec drivers
 *
 * Returns 0 if successful, or a negative error code.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_bus_init(
    bus: *mut hdac_bus,
    dev: *mut device,
    ops: *const hdac_bus_ops,
    ext_ops: *const hdac_ext_bus_ops,
) -> c_int {
    let ret: c_int;

    ret = snd_hdac_bus_init(bus, dev, ops);
    if ret < 0 {
        return ret;
    }

    (*bus).ext_ops = ext_ops;
    /* FIXME:
     * Currently only one bus is supported, if there is device with more
     * buses, bus->idx should be greater than 0, but there needs to be a
     * reliable way to always assign same number.
     */
    (*bus).idx = 0;
    (*bus).cmd_dma_state = true;

    0
}

// EXPORT_SYMBOL_GPL(snd_hdac_ext_bus_init);

/**
 * snd_hdac_ext_bus_exit - clean up a HD-audio extended bus
 * @bus: the pointer to HDAC bus object
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_bus_exit(bus: *mut hdac_bus) {
    snd_hdac_bus_exit(bus);
    WARN_ON((list_empty(&(*bus).hlink_list) == 0) as c_int);
}

// EXPORT_SYMBOL_GPL(snd_hdac_ext_bus_exit);

/**
 * snd_hdac_ext_bus_device_remove - remove HD-audio extended codec base devices
 *
 * @bus: the pointer to HDAC bus object
 */
unsafe extern "C" fn snd_hdac_ext_bus_device_remove_one(
    codec: *mut hdac_device,
    _data: *mut core::ffi::c_void,
) {
    snd_hdac_device_unregister(codec);
    put_device(&mut (*codec).dev);
}

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_bus_device_remove(bus: *mut hdac_bus) {
    /*
     * we need to remove all the codec devices objects created in the
     * snd_hdac_ext_bus_device_init
     */
    hdac_codec_list_for_each_entry_safe(
        &mut (*bus).codec_list,
        snd_hdac_ext_bus_device_remove_one,
        core::ptr::null_mut(),
    );
}

// EXPORT_SYMBOL_GPL(snd_hdac_ext_bus_device_remove);
// #define dev_to_hdac(dev) (container_of((dev), struct hdac_device, dev))

#[inline]
unsafe fn get_hdrv(dev: *mut device) -> *mut hdac_driver {
    let hdrv: *mut hdac_driver = drv_to_hdac_driver((*dev).driver);
    hdrv
}

#[inline]
unsafe fn get_hdev(dev: *mut device) -> *mut hdac_device {
    let hdev: *mut hdac_device = dev_to_hdac_dev(dev);
    hdev
}

unsafe extern "C" fn hda_ext_drv_probe(dev: *mut device) -> c_int {
    ((*get_hdrv(dev)).probe.unwrap())(get_hdev(dev))
}

unsafe extern "C" fn hdac_ext_drv_remove(dev: *mut device) -> c_int {
    ((*get_hdrv(dev)).remove.unwrap())(get_hdev(dev))
}

unsafe extern "C" fn hdac_ext_drv_shutdown(dev: *mut device) {
    ((*get_hdrv(dev)).shutdown.unwrap())(get_hdev(dev));
}

/**
 * snd_hda_ext_driver_register - register a driver for ext hda devices
 *
 * @drv: ext hda driver structure
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hda_ext_driver_register(drv: *mut hdac_driver) -> c_int {
    (*drv).type_ = HDA_DEV_ASOC;
    (*drv).driver.bus = &mut snd_hda_bus_type;
    /* we use default match */

    if (*drv).probe.is_some() {
        (*drv).driver.probe = Some(hda_ext_drv_probe);
    }
    if (*drv).remove.is_some() {
        (*drv).driver.remove = Some(hdac_ext_drv_remove);
    }
    if (*drv).shutdown.is_some() {
        (*drv).driver.shutdown = Some(hdac_ext_drv_shutdown);
    }

    driver_register(&mut (*drv).driver)
}

// EXPORT_SYMBOL_GPL(snd_hda_ext_driver_register);

/**
 * snd_hda_ext_driver_unregister - unregister a driver for ext hda devices
 *
 * @drv: ext hda driver structure
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hda_ext_driver_unregister(drv: *mut hdac_driver) {
    driver_unregister(&mut (*drv).driver);
}

// EXPORT_SYMBOL_GPL(snd_hda_ext_driver_unregister);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
