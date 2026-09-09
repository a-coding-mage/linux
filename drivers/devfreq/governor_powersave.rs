// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/drivers/devfreq/governor_powersave.c
 *
 *  Copyright (C) 2011 Samsung Electronics
 *\tMyungJoo Ham <myungjoo.ham@samsung.com>
 */

// Dependencies supplied by the Linux devfreq, module, and governor headers.

use core::ffi::{c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct devfreq {
    pub lock: mutex,
    _private: [u8; 0],
}

#[repr(C)]
pub struct devfreq_governor {
    pub name: *const core::ffi::c_char,
    pub get_target_freq:
        Option<unsafe extern "C" fn(df: *mut devfreq, freq: *mut c_ulong) -> c_int>,
    pub event_handler:
        Option<unsafe extern "C" fn(devfreq: *mut devfreq, event: c_uint, data: *mut c_void) -> c_int>,
}

extern "C" {
    static DEVFREQ_MIN_FREQ: c_ulong;
    static DEVFREQ_GOV_START: c_uint;
    static DEVFREQ_GOV_POWERSAVE: *const core::ffi::c_char;

    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn update_devfreq(devfreq: *mut devfreq) -> c_int;
    fn devfreq_add_governor(governor: *mut devfreq_governor) -> c_int;
    fn devfreq_remove_governor(governor: *mut devfreq_governor) -> c_int;
    fn pr_err(format: *const core::ffi::c_char, ...);
}

unsafe extern "C" fn devfreq_powersave_func(
    df: *mut devfreq,
    freq: *mut c_ulong,
) -> c_int {
    let _ = df;
    /*
     * target callback should be able to get ceiling value as
     * said in devfreq.h
     */
    unsafe {
        *freq = DEVFREQ_MIN_FREQ;
    }
    0
}

unsafe extern "C" fn devfreq_powersave_handler(
    devfreq: *mut devfreq,
    event: c_uint,
    data: *mut c_void,
) -> c_int {
    let _ = data;
    let mut ret: c_int = 0;

    if event == DEVFREQ_GOV_START {
        unsafe {
            mutex_lock(&mut (*devfreq).lock);
            ret = update_devfreq(devfreq);
            mutex_unlock(&mut (*devfreq).lock);
        }
    }

    ret
}

static mut devfreq_powersave: devfreq_governor = devfreq_governor {
    name: unsafe { DEVFREQ_GOV_POWERSAVE },
    get_target_freq: Some(devfreq_powersave_func),
    event_handler: Some(devfreq_powersave_handler),
};

unsafe extern "C" fn devfreq_powersave_init() -> c_int {
    unsafe { devfreq_add_governor(&mut devfreq_powersave) }
}

// Equivalent of subsys_initcall(devfreq_powersave_init).

unsafe extern "C" fn devfreq_powersave_exit() {
    let ret: c_int;

    unsafe {
        ret = devfreq_remove_governor(&mut devfreq_powersave);
        if ret != 0 {
            // Equivalent of pr_err("%s: failed remove governor %d\n", __func__, ret).
            pr_err(b"%s: failed remove governor %d\0".as_ptr() as *const _, ret);
        }
    }

    return;
}

// Equivalent of module_exit(devfreq_powersave_exit).
// MODULE_DESCRIPTION("DEVFREQ Powersave governor");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
