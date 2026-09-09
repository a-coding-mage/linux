/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2016 Maxime Ripard. All rights reserved.
 */

// Dependency provided by the Linux clock-provider and CCU common interfaces.

#[repr(C)]
pub struct ccu_frac_internal {
    pub enable: u32,
    pub select: u32,

    pub rates: [::core::ffi::c_ulong; 2],
}

#[macro_export]
macro_rules! _SUNXI_CCU_FRAC {
    ($enable:expr, $select:expr, $rate1:expr, $rate2:expr) => {
        ccu_frac_internal {
            enable: $enable,
            select: $select,
            rates: [$rate1, $rate2],
        }
    };
}

extern "C" {
    pub fn ccu_frac_helper_is_enabled(
        common: *mut ccu_common,
        cf: *mut ccu_frac_internal,
    ) -> bool;
    pub fn ccu_frac_helper_enable(
        common: *mut ccu_common,
        cf: *mut ccu_frac_internal,
    );
    pub fn ccu_frac_helper_disable(
        common: *mut ccu_common,
        cf: *mut ccu_frac_internal,
    );

    pub fn ccu_frac_helper_has_rate(
        common: *mut ccu_common,
        cf: *mut ccu_frac_internal,
        rate: ::core::ffi::c_ulong,
    ) -> bool;

    pub fn ccu_frac_helper_read_rate(
        common: *mut ccu_common,
        cf: *mut ccu_frac_internal,
    ) -> ::core::ffi::c_ulong;

    pub fn ccu_frac_helper_set_rate(
        common: *mut ccu_common,
        cf: *mut ccu_frac_internal,
        rate: ::core::ffi::c_ulong,
        lock: u32,
    ) -> i32;
}

// Supplied by ccu_common.h.
pub struct ccu_common;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
