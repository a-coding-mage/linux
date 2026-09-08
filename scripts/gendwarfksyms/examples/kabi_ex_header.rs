/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of kabi_ex.h.  KABI_* annotations are represented by
 * comments or by the corresponding layout-preserving Rust fields. */

use core::ffi::c_void;

#[repr(C)]
pub struct s { pub a: i32 }

#[repr(i32)]
pub enum e { A = 0, B = 1, C = 2, D = 123456789 }

#[repr(C)]
pub struct ex0a {
    pub a: i32,
    /* KABI_RESERVE(0), KABI_RESERVE(1) */
    pub __kabi_reserved_0: usize,
    pub __kabi_reserved_1: usize,
}

#[repr(C)]
pub struct ex0b {
    pub a: i32,
    /* KABI_RESERVE(0), KABI_USE2(1, int b, int c) */
    pub __kabi_reserved_0: usize,
    pub __kabi_reserved_1: usize,
}

#[repr(C)]
pub struct ex0c {
    pub a: i32,
    /* KABI_USE(0, void *p), KABI_USE2(1, int b, int c) */
    pub p: *mut c_void,
    pub __kabi_reserved_1: usize,
}

#[repr(C)]
pub struct ex1a {
    pub a: u32,
    /* KABI_RESERVE_ARRAY(0, 64) */
    pub __kabi_reserved: [u8; 64],
}

#[repr(C)]
pub struct ex1b {
    pub a: u32,
    /* KABI_USE_ARRAY(0, 64, struct { void *p; KABI_RESERVE_ARRAY(1, 56); }) */
    pub __kabi_used: [u8; 64],
}

#[repr(C)]
pub struct ex1c {
    pub a: u32,
    /* KABI_USE_ARRAY(0, 64, void *p[8]) */
    pub __kabi_used: [u8; 64],
}

#[repr(C)]
pub struct ex2a {
    pub a: i32,
    pub b: usize,
    pub c: i32,
    pub d: usize,
}

#[repr(C)]
pub struct ex2b {
    pub a: i32,
    /* KABI_IGNORE(0, unsigned int n) */
    pub b: usize,
    pub c: i32,
    pub d: usize,
}

const _: () = assert!(core::mem::size_of::<ex2a>() == core::mem::size_of::<ex2b>());

#[repr(C)]
pub struct ex2c {
    pub a: i32,
    /* KABI_IGNORE(0, unsigned int n) */
    pub b: usize,
    pub c: i32,
    /* KABI_IGNORE(1, unsigned int m) */
    pub d: usize,
}

const _: () = assert!(core::mem::size_of::<ex2a>() == core::mem::size_of::<ex2c>());

#[repr(C)]
pub struct ex3a { pub a: usize, pub unused: usize }

#[repr(C)]
pub struct ex3b {
    pub a: usize,
    /* KABI_REPLACE(unsigned long, unused, unsigned long renamed) */
    pub unused: usize,
}

const _: () = assert!(core::mem::size_of::<ex3a>() == core::mem::size_of::<ex3b>());

#[repr(C)]
pub struct ex3c {
    pub a: usize,
    /* KABI_REPLACE(unsigned long, unused, long replaced) */
    pub unused: usize,
}

const _: () = assert!(core::mem::size_of::<ex3a>() == core::mem::size_of::<ex3c>());

#[repr(C)]
pub struct ex4a { pub a: usize /* KABI_IGNORE(0, unsigned long b) */ }

/* KABI_BYTE_SIZE(ex4a, 8) */
const _: () = assert!(core::mem::size_of::<ex4a>() == 8);

#[repr(C)]
pub struct ex5a { pub a: usize }

/* KABI_TYPE_STRING("s#ex5a", ... ) */
pub const KABI_TYPE_STRING_EX5A: &str =
    "structure_type ex5a { member pointer_type { s#ex4a } byte_size(8) p data_member_location(0) } byte_size(8)";

#[repr(C)]
pub struct ex5b { pub a: usize }

pub const KABI_TYPE_STRING_EX5B: &str =
    "structure_type ex5b { member pointer_type { s#ex5c } byte_size(8) p data_member_location(0) } byte_size(8)";
pub const KABI_TYPE_STRING_EX5C: &str =
    "structure_type ex5c { member base_type int byte_size(4) encoding(5) n data_member_location(0) } byte_size(8)";

/* KABI_TYPE_STRING("ex6a", "variable s#ex5c") */
pub const KABI_TYPE_STRING_EX6A: &str = "variable s#ex5c";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
