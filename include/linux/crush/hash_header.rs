/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies:
// - __KERNEL__: <linux/types.h>
// - otherwise: "crush_compat.h"

pub const CRUSH_HASH_RJENKINS1: i32 = 0;

pub const CRUSH_HASH_DEFAULT: i32 = CRUSH_HASH_RJENKINS1;

unsafe extern "C" {
    pub fn crush_hash_name(type_: i32) -> *const core::ffi::c_char;

    pub fn crush_hash32(type_: i32, a: u32) -> u32;
    pub fn crush_hash32_2(type_: i32, a: u32, b: u32) -> u32;
    pub fn crush_hash32_3(type_: i32, a: u32, b: u32, c: u32) -> u32;
    pub fn crush_hash32_4(type_: i32, a: u32, b: u32, c: u32, d: u32) -> u32;
    pub fn crush_hash32_5(type_: i32, a: u32, b: u32, c: u32, d: u32, e: u32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
