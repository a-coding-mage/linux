// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Linux driver model AC97 bus interface
 *
 * Author:	Nicolas Pitre
 * Created:	Jan 14, 2005
 * Copyright:	(C) MontaVista Software Inc.
 */

// C includes translated as external dependencies:
// <linux/module.h>, <linux/init.h>, <linux/device.h>, <linux/string.h>,
// <sound/ac97_codec.h>

use core::ffi::{c_char, c_int, c_uint};

pub const AC97_VENDOR_ID1: c_uint = 0x7c;
pub const AC97_VENDOR_ID2: c_uint = 0x7e;
pub const ENODEV: c_int = 19;

#[repr(C)]
pub struct snd_ac97_bus_ops {
    pub read: Option<unsafe extern "C" fn(ac97: *mut snd_ac97, reg: c_uint) -> c_uint>,
    pub reset: Option<unsafe extern "C" fn(ac97: *mut snd_ac97)>,
    pub warm_reset: Option<unsafe extern "C" fn(ac97: *mut snd_ac97)>,
}

#[repr(C)]
pub struct snd_ac97_bus {
    pub ops: *const snd_ac97_bus_ops,
}

#[repr(C)]
pub struct snd_ac97 {
    pub id: c_uint,
    pub bus: *mut snd_ac97_bus,
}

#[repr(C)]
pub struct bus_type {
    pub name: *const c_char,
}

unsafe extern "C" {
    fn bus_register(bus: *const bus_type) -> c_int;
    fn bus_unregister(bus: *const bus_type);
}

/*
 * snd_ac97_check_id() - Reads and checks the vendor ID of the device
 * @ac97: The AC97 device to check
 * @id: The ID to compare to
 * @id_mask: Mask that is applied to the device ID before comparing to @id
 *
 * If @id is 0 this function returns true if the read device vendor ID is
 * a valid ID. If @id is non 0 this functions returns true if @id
 * matches the read vendor ID. Otherwise the function returns false.
 */
unsafe fn snd_ac97_check_id(ac97: *mut snd_ac97, id: c_uint, id_mask: c_uint) -> bool {
    let read = (*(*(*ac97).bus).ops).read.expect("snd_ac97_bus_ops.read is NULL");

    (*ac97).id = read(ac97, AC97_VENDOR_ID1) << 16;
    (*ac97).id |= read(ac97, AC97_VENDOR_ID2);

    if (*ac97).id == 0x0 || (*ac97).id == 0xffffffff {
        return false;
    }

    if id != 0 && id != ((*ac97).id & id_mask) {
        return false;
    }

    true
}

/**
 * snd_ac97_reset() - Reset AC'97 device
 * @ac97: The AC'97 device to reset
 * @try_warm: Try a warm reset first
 * @id: Expected device vendor ID
 * @id_mask: Mask that is applied to the device ID before comparing to @id
 *
 * This function resets the AC'97 device. If @try_warm is true the function
 * first performs a warm reset. If @try_warm is false the function issues
 * cold reset followed by a warm reset. If @id is 0 any valid device ID
 * will be accepted, otherwise only the ID that matches @id and @id_mask
 * is accepted.
 * Returns:
 * * %1 - if warm reset is successful
 * * %0 - if cold reset and warm reset is successful
 * * %-ENODEV - if @id and @id_mask not matching
 */
#[no_mangle]
pub unsafe extern "C" fn snd_ac97_reset(
    ac97: *mut snd_ac97,
    try_warm: bool,
    id: c_uint,
    id_mask: c_uint,
) -> c_int {
    let ops = (*(*ac97).bus).ops;

    if try_warm && (*ops).warm_reset.is_some() {
        ((*ops).warm_reset.unwrap())(ac97);
        if snd_ac97_check_id(ac97, id, id_mask) {
            return 1;
        }
    }

    if (*ops).reset.is_some() {
        ((*ops).reset.unwrap())(ac97);
    }
    if (*ops).warm_reset.is_some() {
        ((*ops).warm_reset.unwrap())(ac97);
    }

    if snd_ac97_check_id(ac97, id, id_mask) {
        return 0;
    }
    -ENODEV
}
// EXPORT_SYMBOL_GPL(snd_ac97_reset);

#[no_mangle]
pub static ac97_bus_type: bus_type = bus_type {
    name: b"ac97\0".as_ptr() as *const c_char,
};
// EXPORT_SYMBOL(ac97_bus_type);

// __init
unsafe extern "C" fn ac97_bus_init() -> c_int {
    bus_register(&ac97_bus_type)
}

// subsys_initcall(ac97_bus_init);

// __exit
unsafe extern "C" fn ac97_bus_exit() {
    bus_unregister(&ac97_bus_type);
}

// module_exit(ac97_bus_exit);

// MODULE_DESCRIPTION("Legacy AC97 bus interface");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
