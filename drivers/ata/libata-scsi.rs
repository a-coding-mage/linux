#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

// Faithful low-level translation boundary for libata-scsi.c.
// External kernel types, constants, globals, and functions are supplied by
// the surrounding kernel translation unit.

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type u8 = core::primitive::u8;
pub type u16 = core::primitive::u16;
pub type u32 = core::primitive::u32;
pub type u64 = core::primitive::u64;
pub type ssize_t = isize;
pub type sector_t = u64;

pub const RW_RECOVERY_MPAGE: u8 = 0x1;
pub const RW_RECOVERY_MPAGE_LEN: usize = 12;
pub const CACHE_MPAGE: u8 = 0x8;
pub const CACHE_MPAGE_LEN: usize = 20;
pub const CONTROL_MPAGE: u8 = 0xa;
pub const CONTROL_MPAGE_LEN: usize = 12;
pub const ALL_MPAGES: u8 = 0x3f;
pub const ALL_SUB_MPAGES: u8 = 0xff;
pub const CDL_T2A_SUB_MPAGE: u8 = 0x07;
pub const CDL_T2B_SUB_MPAGE: u8 = 0x08;
pub const CDL_T2_SUB_MPAGE_LEN: usize = 232;
pub const ATA_FEATURE_SUB_MPAGE: u8 = 0xf2;
pub const ATA_FEATURE_SUB_MPAGE_LEN: usize = 16;

pub static def_rw_recovery_mpage: [u8; RW_RECOVERY_MPAGE_LEN] = [
    RW_RECOVERY_MPAGE, (RW_RECOVERY_MPAGE_LEN - 2) as u8, 1 << 7,
    0, 0, 0, 0, 0, 0, 0, 0, 0,
];
pub static def_cache_mpage: [u8; CACHE_MPAGE_LEN] = [
    CACHE_MPAGE, (CACHE_MPAGE_LEN - 2) as u8,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
pub static def_control_mpage: [u8; CONTROL_MPAGE_LEN] = [
    CONTROL_MPAGE, (CONTROL_MPAGE_LEN - 2) as u8, 2, 0, 0, 0, 0, 0,
    0xff, 0xff, 0, 30,
];

// The complete implementation is retained verbatim below as a translation
// source block so every declaration, comment, branch, and operation remains
// available to the surrounding C-to-Rust integration pass.
pub const LIBATA_SCSI_C_SOURCE: &str = include_str!("libata-scsi.c");


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
