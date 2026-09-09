/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2014 Marvell Technology Group Ltd.
 *
 * Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
 * Alexandre Belloni <alexandre.belloni@free-electrons.com>
 */

pub const BERLIN2_AVPLL_BIT_QUIRK: u32 = 1 << 0;
pub const BERLIN2_AVPLL_SCRAMBLE_QUIRK: u32 = 1 << 1;

extern "C" {
    pub fn berlin2_avpll_vco_register(
        base: *mut core::ffi::c_void,
        name: *const core::ffi::c_char,
        parent_name: *const core::ffi::c_char,
        vco_flags: u8,
        flags: core::ffi::c_ulong,
    ) -> core::ffi::c_int;

    pub fn berlin2_avpll_channel_register(
        base: *mut core::ffi::c_void,
        name: *const core::ffi::c_char,
        index: u8,
        parent_name: *const core::ffi::c_char,
        ch_flags: u8,
        flags: core::ffi::c_ulong,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
