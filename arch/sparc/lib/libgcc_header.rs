/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: equivalent of <asm/byteorder.h>.

pub type word_type = i32;

#[repr(C)]
pub struct DWstruct {
    pub high: i32,
    pub low: i32,
}

#[repr(C)]
pub union DWunion {
    pub s: DWstruct,
    pub ll: i64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
