// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2010 ARM Ltd.
 * Written by Catalin Marinas <catalin.marinas@arm.com>
 */

use core::ffi::c_int;

#[repr(C)]
pub struct OuterCache {
    pub disable: Option<unsafe extern "C" fn()>,
}

extern "C" {
    pub static mut outer_cache: OuterCache;
    fn irqs_disabled() -> bool;
    fn num_online_cpus() -> c_int;
    fn warn_on(condition: bool);
}

pub unsafe fn outer_disable() {
    // Corresponds to the Linux WARN_ON macro.
    warn_on(!irqs_disabled());
    warn_on(num_online_cpus() > 1);

    if let Some(disable) = (*core::ptr::addr_of!(outer_cache)).disable {
        disable();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
