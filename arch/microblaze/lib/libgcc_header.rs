/* SPDX-License-Identifier: GPL-2.0 */

// Original dependency: <asm/byteorder.h>.

pub type word_type = i32;

#[cfg(target_endian = "big")]
#[repr(C)]
pub struct DWstruct {
    pub high: i32,
    pub low: i32,
}

#[cfg(target_endian = "little")]
#[repr(C)]
pub struct DWstruct {
    pub low: i32,
    pub high: i32,
}

#[repr(C)]
pub union DWunion {
    pub s: DWstruct,
    pub ll: i64,
}

unsafe extern "C" {
    pub fn __ashldi3(u: i64, b: word_type) -> i64;
    pub fn __ashrdi3(u: i64, b: word_type) -> i64;
    pub fn __cmpdi2(a: i64, b: i64) -> word_type;
    pub fn __lshrdi3(u: i64, b: word_type) -> i64;
    pub fn __muldi3(u: i64, v: i64) -> i64;
    pub fn __ucmpdi2(a: u64, b: u64) -> word_type;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
