/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <asm/byteorder.h> supplies the target byte-order condition.

pub type word_type = isize;

#[cfg(target_endian = "big")]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DWstruct {
    pub high: i32,
    pub low: i32,
}

#[cfg(target_endian = "little")]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DWstruct {
    pub low: i32,
    pub high: i32,
}

#[repr(C)]
pub union DWunion {
    pub s: DWstruct,
    pub ll: i64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
