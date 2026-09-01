// SPDX-License-Identifier: GPL-2.0-only
/*
 * HD-audio bus
 */

// C includes translated as external dependencies:
// <linux/init.h>, <linux/device.h>, <linux/module.h>, <linux/export.h>,
// <sound/hdaudio.h>

use core::ffi::{c_char, c_int, c_uint};

// MODULE_DESCRIPTION("HD-audio bus");
// MODULE_LICENSE("GPL");

const ENOMEM: c_int = 12;

#[repr(C)]
pub struct hda_device_id {
    pub vendor_id: c_uint,
    pub rev_id: c_uint,
}

#[repr(C)]
pub struct hdac_device {
    pub vendor_id: c_uint,
    pub revision_id: c_uint,
    pub type_: c_int,
}

pub type HdacDriverMatch =
    Option<unsafe extern "C" fn(*mut hdac_device, *const hdac_driver) -> c_int>;

#[repr(C)]
pub struct hdac_driver {
    pub id_table: *const hda_device_id,
    pub type_: c_int,
    pub match_: HdacDriverMatch,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kobj_uevent_env {
    _private: [u8; 0],
}

pub type BusMatch =
    Option<unsafe extern "C" fn(*mut device, *const device_driver) -> c_int>;
pub type BusUevent =
    Option<unsafe extern "C" fn(*const device, *mut kobj_uevent_env) -> c_int>;

#[repr(C)]
pub struct bus_type {
    pub name: *const c_char,
    pub match_: BusMatch,
    pub uevent: BusUevent,
}

unsafe extern "C" {
    fn dev_to_hdac_dev(dev: *const device) -> *mut hdac_device;
    fn drv_to_hdac_driver(drv: *const device_driver) -> *const hdac_driver;
    fn snd_hdac_codec_modalias(
        hdev: *mut hdac_device,
        modalias: *mut c_char,
        len: usize,
    );
    fn add_uevent_var(
        env: *mut kobj_uevent_env,
        format: *const c_char,
        ...
    ) -> c_int;
    fn bus_register(bus: *const bus_type) -> c_int;
    fn bus_unregister(bus: *const bus_type);
}

/**
 * hdac_get_device_id - gets the hdac device id entry
 * @hdev: HD-audio core device
 * @drv: HD-audio codec driver
 *
 * Compares the hdac device vendor_id and revision_id to the hdac_device
 * driver id_table and returns the matching device id entry.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hdac_get_device_id(
    hdev: *mut hdac_device,
    drv: *const hdac_driver,
) -> *const hda_device_id {
    unsafe {
        if !(*drv).id_table.is_null() {
            let mut id: *const hda_device_id = (*drv).id_table;

            while (*id).vendor_id != 0 {
                if (*hdev).vendor_id == (*id).vendor_id
                    && ((*id).rev_id == 0 || (*id).rev_id == (*hdev).revision_id)
                {
                    return id;
                }
                id = id.add(1);
            }
        }

        core::ptr::null()
    }
}
// EXPORT_SYMBOL_GPL(hdac_get_device_id);

unsafe extern "C" fn hdac_codec_match(
    dev: *mut hdac_device,
    drv: *const hdac_driver,
) -> c_int {
    unsafe { (!hdac_get_device_id(dev, drv).is_null()) as c_int }
}

unsafe extern "C" fn hda_bus_match(
    dev: *mut device,
    drv: *const device_driver,
) -> c_int {
    unsafe {
        let hdev: *mut hdac_device = dev_to_hdac_dev(dev);
        let hdrv: *const hdac_driver = drv_to_hdac_driver(drv);

        if (*hdev).type_ != (*hdrv).type_ {
            return 0;
        }

        /*
         * if driver provided a match function use that otherwise we will
         * use hdac_codec_match function
         */
        if let Some(match_fn) = (*hdrv).match_ {
            return match_fn(hdev, hdrv);
        }
        hdac_codec_match(hdev, hdrv)
    }
}

unsafe extern "C" fn hda_uevent(
    dev: *const device,
    env: *mut kobj_uevent_env,
) -> c_int {
    unsafe {
        let mut modalias: [c_char; 32] = [0; 32];

        snd_hdac_codec_modalias(
            dev_to_hdac_dev(dev),
            modalias.as_mut_ptr(),
            core::mem::size_of_val(&modalias),
        );
        if add_uevent_var(env, c"MODALIAS=%s".as_ptr(), modalias.as_ptr()) != 0 {
            return -ENOMEM;
        }
        0
    }
}

#[unsafe(no_mangle)]
pub static snd_hda_bus_type: bus_type = bus_type {
    name: c"hdaudio".as_ptr(),
    match_: Some(hda_bus_match),
    uevent: Some(hda_uevent),
};
// EXPORT_SYMBOL_GPL(snd_hda_bus_type);

// __init
unsafe extern "C" fn hda_bus_init() -> c_int {
    unsafe { bus_register(&snd_hda_bus_type) }
}

// __exit
unsafe extern "C" fn hda_bus_exit() {
    unsafe {
        bus_unregister(&snd_hda_bus_type);
    }
}

// subsys_initcall(hda_bus_init);
// module_exit(hda_bus_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
