/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: BCM63XX_CS_H

unsafe extern "C" {
    pub fn bcm63xx_set_cs_base(cs: core::ffi::c_uint, base: u32, size: core::ffi::c_uint)
        -> core::ffi::c_int;
    pub fn bcm63xx_set_cs_timing(
        cs: core::ffi::c_uint,
        wait: core::ffi::c_uint,
        setup: core::ffi::c_uint,
        hold: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    pub fn bcm63xx_set_cs_param(cs: core::ffi::c_uint, flags: u32) -> core::ffi::c_int;
    pub fn bcm63xx_set_cs_status(cs: core::ffi::c_uint, enable: core::ffi::c_int)
        -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
