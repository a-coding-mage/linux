/* SPDX-License-Identifier: GPL-2.0 */

pub const RISCV_IMAGE_MAGIC: &[u8; 8] = b"RISCV\0\0\0";
pub const RISCV_IMAGE_MAGIC2: &[u8; 4] = b"RSC\x05";

pub const RISCV_IMAGE_FLAG_BE_SHIFT: u32 = 0;
pub const RISCV_IMAGE_FLAG_BE_MASK: u32 = 0x1;

pub const RISCV_IMAGE_FLAG_LE: u32 = 0;
pub const RISCV_IMAGE_FLAG_BE: u32 = 1;

// CONFIG_CPU_BIG_ENDIAN: conversion of header fields to LE not yet implemented.
pub const __HEAD_FLAG_BE: u32 = RISCV_IMAGE_FLAG_LE;

pub const fn __HEAD_FLAG(field: u32, field_shift: u32) -> u32 {
    field << field_shift
}

pub const __HEAD_FLAGS: u32 = __HEAD_FLAG(__HEAD_FLAG_BE, RISCV_IMAGE_FLAG_BE_SHIFT);

pub const RISCV_HEADER_VERSION_MAJOR: u32 = 0;
pub const RISCV_HEADER_VERSION_MINOR: u32 = 2;
pub const RISCV_HEADER_VERSION: u32 =
    RISCV_HEADER_VERSION_MAJOR << 16 | RISCV_HEADER_VERSION_MINOR;

pub const fn riscv_image_flag_field(flags: u64, field_shift: u32, field_mask: u64) -> u64 {
    (flags >> field_shift) & field_mask
}

/**
 * struct riscv_image_header - riscv kernel image header
 * @code0:          Executable code
 * @code1:          Executable code
 * @text_offset:    Image load offset (little endian)
 * @image_size:     Effective Image size (little endian)
 * @flags:          kernel flags (little endian)
 * @version:        version
 * @res1:            reserved
 * @res2:            reserved
 * @magic:          Magic number (RISC-V specific; deprecated)
 * @magic2:         Magic number 2 (to match the ARM64 'magic' field pos)
 * @res3:            reserved (will be used for PE COFF offset)
 *
 * The intention is for this header format to be shared between multiple
 * architectures to avoid a proliferation of image header formats.
 */
#[repr(C)]
pub struct riscv_image_header {
    pub code0: u32,
    pub code1: u32,
    pub text_offset: u64,
    pub image_size: u64,
    pub flags: u64,
    pub version: u32,
    pub res1: u32,
    pub res2: u64,
    pub magic: u64,
    pub magic2: u32,
    pub res3: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
