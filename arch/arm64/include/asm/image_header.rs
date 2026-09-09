/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from the C header; the original include guards are omitted. */

pub const ARM64_IMAGE_MAGIC: &[u8; 4] = b"ARM\x64";

pub const ARM64_IMAGE_FLAG_BE_SHIFT: u32 = 0;
pub const ARM64_IMAGE_FLAG_PAGE_SIZE_SHIFT: u32 = ARM64_IMAGE_FLAG_BE_SHIFT + 1;
pub const ARM64_IMAGE_FLAG_PHYS_BASE_SHIFT: u32 = ARM64_IMAGE_FLAG_PAGE_SIZE_SHIFT + 2;
pub const ARM64_IMAGE_FLAG_BE_MASK: u64 = 0x1;
pub const ARM64_IMAGE_FLAG_PAGE_SIZE_MASK: u64 = 0x3;
pub const ARM64_IMAGE_FLAG_PHYS_BASE_MASK: u64 = 0x1;

pub const ARM64_IMAGE_FLAG_LE: u64 = 0;
pub const ARM64_IMAGE_FLAG_BE: u64 = 1;
pub const ARM64_IMAGE_FLAG_PAGE_SIZE_4K: u64 = 1;
pub const ARM64_IMAGE_FLAG_PAGE_SIZE_16K: u64 = 2;
pub const ARM64_IMAGE_FLAG_PAGE_SIZE_64K: u64 = 3;
pub const ARM64_IMAGE_FLAG_PHYS_BASE: u64 = 1;

/* Build-time assembler exclusion from the original header is represented by
 * providing the non-assembler declarations here. */

#[macro_export]
macro_rules! arm64_image_flag_field {
    ($flags:expr, BE) => {
        (($flags >> $crate::ARM64_IMAGE_FLAG_BE_SHIFT) & $crate::ARM64_IMAGE_FLAG_BE_MASK)
    };
    ($flags:expr, PAGE_SIZE) => {
        (($flags >> $crate::ARM64_IMAGE_FLAG_PAGE_SIZE_SHIFT)
            & $crate::ARM64_IMAGE_FLAG_PAGE_SIZE_MASK)
    };
    ($flags:expr, PHYS_BASE) => {
        (($flags >> $crate::ARM64_IMAGE_FLAG_PHYS_BASE_SHIFT)
            & $crate::ARM64_IMAGE_FLAG_PHYS_BASE_MASK)
    };
}

/*
 * struct arm64_image_header - arm64 kernel image header
 * See Documentation/arch/arm64/booting.rst for details
 *
 * @code0:        Executable code, or
 *   @mz_header   alternatively used for part of MZ header
 * @code1:        Executable code
 * @text_offset: Image load offset
 * @image_size:  Effective Image size
 * @flags:       kernel flags
 * @reserved:    reserved
 * @magic:       Magic number
 * @reserved5:   reserved, or
 *   @pe_header: alternatively used for PE COFF offset
 */
#[repr(C)]
pub struct arm64_image_header {
    pub code0: u32,
    pub code1: u32,
    pub text_offset: u64,
    pub image_size: u64,
    pub flags: u64,
    pub res2: u64,
    pub res3: u64,
    pub res4: u64,
    pub magic: u32,
    pub res5: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
