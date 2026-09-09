/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Linker script macros to generate Image header fields.
 *
 * Copyright (C) 2014 ARM Ltd.
 *
 * This file is a Rust translation of the linker-script header.  The original
 * C preprocessor include and LINKER_SCRIPT guard are intentionally retained
 * here as comments because their meaning is supplied by the linker-script
 * build environment.
 */

/* Original dependency: <asm/image.h>. */

/*
 * There aren't any ELF relocations we can use to endian-swap values known only
 * at link time (e.g. the subtraction of two symbol addresses), so we must get
 * the linker to endian-swap certain values before emitting them.
 *
 * Note that, in order for this to work when building the ELF64 PIE executable
 * (for KASLR), these values should not be referenced via R_AARCH64_ABS64
 * relocations, since these are fixed up at runtime rather than at build time
 * when PIE is in effect. So we need to split them up in 32-bit high and low
 * words.
 */

#[cfg(CONFIG_CPU_BIG_ENDIAN)]
#[inline]
pub const fn data_le32(data: u64) -> u64 {
    (((data & 0x0000_00ff) << 24)
        | ((data & 0x0000_ff00) << 8)
        | ((data & 0x00ff_0000) >> 8)
        | ((data & 0xff00_0000) >> 24))
}

#[cfg(not(CONFIG_CPU_BIG_ENDIAN))]
#[inline]
pub const fn data_le32(data: u64) -> u64 {
    data & 0xffff_ffff
}

/*
 * DEFINE_IMAGE_LE64(sym, data): emit the low and high 32-bit linker-script
 * fields for a 64-bit value.  The low/high names are passed explicitly since
 * Rust has no stable identifier-concatenation equivalent to sym##_lo32.
 */
macro_rules! define_image_le64 {
    ($sym_lo32:ident, $sym_hi32:ident, $data:expr) => {
        $sym_lo32 = data_le32(($data) & 0xffff_ffff);
        $sym_hi32 = data_le32(($data) >> 32);
    };
}

/* __HEAD_FLAG(field) = (__HEAD_FLAG_##field << ARM64_IMAGE_FLAG_##field##_SHIFT). */
macro_rules! __head_flag {
    ($field_flag:expr, $field_shift:expr) => {
        ($field_flag << $field_shift)
    };
}

#[cfg(CONFIG_CPU_BIG_ENDIAN)]
pub const __HEAD_FLAG_BE: u64 = ARM64_IMAGE_FLAG_BE;

#[cfg(not(CONFIG_CPU_BIG_ENDIAN))]
pub const __HEAD_FLAG_BE: u64 = ARM64_IMAGE_FLAG_LE;

pub const __HEAD_FLAG_PAGE_SIZE: u64 = (PAGE_SHIFT - 10) / 2;
pub const __HEAD_FLAG_PHYS_BASE: u64 = 1;

pub const __HEAD_FLAGS: u64 =
    __head_flag!(__HEAD_FLAG_BE, ARM64_IMAGE_FLAG_BE_SHIFT)
        | __head_flag!(__HEAD_FLAG_PAGE_SIZE, ARM64_IMAGE_FLAG_PAGE_SIZE_SHIFT)
        | __head_flag!(__HEAD_FLAG_PHYS_BASE, ARM64_IMAGE_FLAG_PHYS_BASE_SHIFT);

/*
 * These will output as part of the Image header, which should be little-endian
 * regardless of the endianness of the kernel. While constant values could be
 * endian swapped in head.S, all are done here for consistency.
 */
macro_rules! head_symbols {
    ($kernel_size_lo32:ident, $kernel_size_hi32:ident,
     $kernel_flags_lo32:ident, $kernel_flags_hi32:ident,
     $end:expr, $text:expr) => {
        define_image_le64!($kernel_size_lo32, $kernel_size_hi32, $end - $text);
        define_image_le64!($kernel_flags_lo32, $kernel_flags_hi32, __HEAD_FLAGS);
    };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
