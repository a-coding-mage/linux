/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Utilities for mac80211 unit testing
 *
 * Copyright (C) 2024 Intel Corporation
 */

// Dependency corresponding to: #include "../ieee80211_i.h"

#[repr(C)]
pub struct t_sdata {
    pub sdata: *mut ieee80211_sub_if_data,
    pub wiphy: *mut wiphy,
    pub local: ieee80211_local,

    pub ctx: *mut core::ffi::c_void,

    pub band_2ghz: ieee80211_supported_band,
    pub band_5ghz: ieee80211_supported_band,
}

#[macro_export]
macro_rules! T_SDATA {
    ($test:expr) => {{
        let __t_sdata = unsafe {
            kunit_alloc_resource(
                $test,
                t_sdata_init,
                t_sdata_exit,
                GFP_KERNEL,
                core::ptr::null_mut(),
            )
        };
        KUNIT_ASSERT_NOT_NULL($test, __t_sdata);
        __t_sdata
    }};
}

unsafe extern "C" {
    pub fn t_sdata_init(
        resource: *mut kunit_resource,
        data: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    pub fn t_sdata_exit(resource: *mut kunit_resource);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
