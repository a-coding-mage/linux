// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 2016 Robert Jarzmik <robert.jarzmik@free.fr>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;

#[repr(C)]
pub struct device {
    pub parent: *mut device,
    pub release: Option<unsafe extern "C" fn(dev: *mut device)>,
}

#[repr(C)]
pub struct snd_ac97 {
    pub private_data: *mut c_void,
    pub bus: *mut snd_ac97_bus,
    pub dev: device,
    pub num: c_uint,
}

#[repr(C)]
pub struct snd_ac97_bus_ops {
    pub reset: Option<unsafe extern "C" fn(ac97: *mut snd_ac97)>,
    pub warm_reset: Option<unsafe extern "C" fn(ac97: *mut snd_ac97)>,
    pub write: Option<unsafe extern "C" fn(ac97: *mut snd_ac97, reg: u16, val: u16)>,
    pub read: Option<unsafe extern "C" fn(ac97: *mut snd_ac97, reg: u16) -> u16>,
}

#[repr(C)]
pub struct snd_ac97_bus {
    pub ops: *const snd_ac97_bus_ops,
}

#[repr(C)]
pub struct ac97_controller_ops {
    pub reset: Option<unsafe extern "C" fn(actrl: *mut ac97_controller)>,
    pub warm_reset: Option<unsafe extern "C" fn(actrl: *mut ac97_controller)>,
    pub write:
        Option<unsafe extern "C" fn(actrl: *mut ac97_controller, num: c_uint, reg: u16, val: u16)>,
    pub read: Option<unsafe extern "C" fn(actrl: *mut ac97_controller, num: c_uint, reg: u16) -> u16>,
}

#[repr(C)]
pub struct ac97_controller {
    pub ops: *mut ac97_controller_ops,
}

#[repr(C)]
pub struct ac97_codec_device {
    pub dev: device,
    pub ac97_ctrl: *mut ac97_controller,
    pub num: c_uint,
    pub vendor_id: c_uint,
}

unsafe extern "C" {
    fn kfree(ptr: *const c_void);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_name(dev: *mut device, fmt: *const c_char, ...);
    fn dev_name(dev: *const device) -> *const c_char;
    fn device_register(dev: *mut device) -> c_int;
    fn put_device(dev: *mut device);
    fn device_unregister(dev: *mut device);
    fn snd_ac97_bus_scan_one(actrl: *mut ac97_controller, num: c_uint) -> c_uint;
    fn ac97_ids_match(id1: c_uint, id2: c_uint, mask: c_uint) -> bool;
}

unsafe fn err_ptr<T>(err: c_int) -> *mut T {
    err as isize as *mut T
}

unsafe fn to_ac97_t(dev: *mut device) -> *mut snd_ac97 {
    dev as *mut snd_ac97
}

unsafe fn to_ac97_device(ptr: *mut c_void) -> *mut ac97_codec_device {
    ptr as *mut ac97_codec_device
}

unsafe extern "C" fn compat_ac97_release(dev: *mut device) {
    unsafe {
        kfree(to_ac97_t(dev) as *const c_void);
    }
}

unsafe extern "C" fn compat_ac97_reset(ac97: *mut snd_ac97) {
    unsafe {
        let adev = to_ac97_device((*ac97).private_data);
        let actrl = (*adev).ac97_ctrl;

        if let Some(reset) = (*(*actrl).ops).reset {
            reset(actrl);
        }
    }
}

unsafe extern "C" fn compat_ac97_warm_reset(ac97: *mut snd_ac97) {
    unsafe {
        let adev = to_ac97_device((*ac97).private_data);
        let actrl = (*adev).ac97_ctrl;

        if let Some(warm_reset) = (*(*actrl).ops).warm_reset {
            warm_reset(actrl);
        }
    }
}

unsafe extern "C" fn compat_ac97_write(ac97: *mut snd_ac97, reg: u16, val: u16) {
    unsafe {
        let adev = to_ac97_device((*ac97).private_data);
        let actrl = (*adev).ac97_ctrl;

        ((*(*actrl).ops).write.unwrap())(actrl, (*ac97).num, reg, val);
    }
}

unsafe extern "C" fn compat_ac97_read(ac97: *mut snd_ac97, reg: u16) -> u16 {
    unsafe {
        let adev = to_ac97_device((*ac97).private_data);
        let actrl = (*adev).ac97_ctrl;

        ((*(*actrl).ops).read.unwrap())(actrl, (*ac97).num, reg)
    }
}

static COMPAT_SND_AC97_BUS_OPS: snd_ac97_bus_ops = snd_ac97_bus_ops {
    reset: Some(compat_ac97_reset),
    warm_reset: Some(compat_ac97_warm_reset),
    write: Some(compat_ac97_write),
    read: Some(compat_ac97_read),
};

static mut COMPAT_SOC_AC97_BUS: snd_ac97_bus = snd_ac97_bus {
    ops: &COMPAT_SND_AC97_BUS_OPS,
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ac97_compat_alloc(adev: *mut ac97_codec_device) -> *mut snd_ac97 {
    unsafe {
        let ac97: *mut snd_ac97;
        let ret: c_int;

        ac97 = kzalloc(core::mem::size_of::<snd_ac97>(), 0) as *mut snd_ac97;
        if ac97.is_null() {
            return err_ptr(-ENOMEM);
        }

        (*ac97).private_data = adev as *mut c_void;
        (*ac97).bus = &raw mut COMPAT_SOC_AC97_BUS;

        (*ac97).dev.parent = &mut (*adev).dev;
        (*ac97).dev.release = Some(compat_ac97_release);
        dev_set_name(
            &mut (*ac97).dev,
            c"%s-compat".as_ptr(),
            dev_name(&(*adev).dev),
        );
        ret = device_register(&mut (*ac97).dev);
        if ret != 0 {
            put_device(&mut (*ac97).dev);
            return err_ptr(ret);
        }

        ac97
    }
}

// EXPORT_SYMBOL_GPL(snd_ac97_compat_alloc);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ac97_compat_release(ac97: *mut snd_ac97) {
    unsafe {
        device_unregister(&mut (*ac97).dev);
    }
}

// EXPORT_SYMBOL_GPL(snd_ac97_compat_release);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ac97_reset(
    ac97: *mut snd_ac97,
    try_warm: bool,
    id: c_uint,
    id_mask: c_uint,
) -> c_int {
    unsafe {
        let adev = to_ac97_device((*ac97).private_data);
        let actrl = (*adev).ac97_ctrl;
        let mut scanned: c_uint;

        let _ = id;

        if try_warm {
            compat_ac97_warm_reset(ac97);
            scanned = snd_ac97_bus_scan_one(actrl, (*adev).num);
            if ac97_ids_match(scanned, (*adev).vendor_id, id_mask) {
                return 1;
            }
        }

        compat_ac97_reset(ac97);
        compat_ac97_warm_reset(ac97);
        scanned = snd_ac97_bus_scan_one(actrl, (*adev).num);
        if ac97_ids_match(scanned, (*adev).vendor_id, id_mask) {
            return 0;
        }

        -ENODEV
    }
}

// EXPORT_SYMBOL_GPL(snd_ac97_reset);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
