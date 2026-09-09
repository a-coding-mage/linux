// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

use core::ffi::c_char;

unsafe extern "C" {
    fn smp_send_stop();
    fn __vmstop();
}

#[no_mangle]
pub unsafe extern "C" fn machine_power_off() {
    smp_send_stop();
    __vmstop();
}

#[no_mangle]
pub unsafe extern "C" fn machine_halt() {}

#[no_mangle]
pub unsafe extern "C" fn machine_restart(_cmd: *mut c_char) {}

pub static mut pm_power_off: Option<unsafe extern "C" fn()> = None;

// EXPORT_SYMBOL(pm_power_off);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
