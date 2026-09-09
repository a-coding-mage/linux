/* SPDX-License-Identifier: GPL-2.0 */

// The C typedef uses GCC's __word__ mode; retain its integer intent here.
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

#[cfg(target_endian = "big")]
#[repr(C)]
pub struct TWstruct {
    pub high: i64,
    pub low: i64,
}

#[cfg(target_endian = "little")]
#[repr(C)]
pub struct TWstruct {
    pub low: i64,
    pub high: i64,
}

#[repr(C)]
pub union DWunion {
    pub s: DWstruct,
    pub ll: i64,
}

// The C definition is conditional on CONFIG_64BIT && CONFIG_CPU_MIPSR6.
// Rust has no file-local equivalent for those kernel configuration symbols.
#[repr(C)]
pub union TWunion {
    pub s: TWstruct,
    pub ti: i128,
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
