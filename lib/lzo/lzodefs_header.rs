/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  lzodefs.h -- architecture, OS and compiler specific defines
 *
 *  Copyright (C) 1996-2012 Markus F.X.J. Oberhumer <markus@oberhumer.com>
 *
 *  The full LZO package can be found at:
 *  http://www.oberhumer.com/opensource/lzo/
 *
 *  Changed for Linux kernel use by:
 *  Nitin Gupta <nitingupta910@gmail.com>
 *  Richard Purdie <rpurdie@openedhand.com>
 */

/* Version
 *  0: original lzo version
 *  1: lzo with support for RLE
 */
pub const LZO_VERSION: u32 = 1;

/* These macros depend on the externally supplied get_unaligned/put_unaligned. */
#[macro_export]
macro_rules! COPY4 {
    ($dst:expr, $src:expr) => {{
        unsafe {
            put_unaligned(get_unaligned(($src as *const u32)), ($dst as *mut u32));
        }
    }};
}

/* CONFIG_X86_64 and CONFIG_ARM64 select the native 64-bit copy in the C header. */
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[macro_export]
macro_rules! COPY8 {
    ($dst:expr, $src:expr) => {{
        unsafe {
            put_unaligned(get_unaligned(($src as *const u64)), ($dst as *mut u64));
        }
    }};
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
#[macro_export]
macro_rules! COPY8 {
    ($dst:expr, $src:expr) => {{
        COPY4!($dst, $src);
        COPY4!(($dst as *mut u8).add(4), ($src as *const u8).add(4));
    }};
}

/* The original header rejects simultaneous __BIG_ENDIAN and __LITTLE_ENDIAN. */
/* CONFIG_X86_64/CONFIG_ARM64: LZO_USE_CTZ64=1, LZO_USE_CTZ32=1,
 * LZO_FAST_64BIT_MEMORY_ACCESS. */
/* CONFIG_X86/CONFIG_PPC, or ARM with __LINUX_ARM_ARCH__ >= 5: LZO_USE_CTZ32=1. */

pub const M1_MAX_OFFSET: u32 = 0x0400;
pub const M2_MAX_OFFSET: u32 = 0x0800;
pub const M3_MAX_OFFSET: u32 = 0x4000;
pub const M4_MAX_OFFSET_V0: u32 = 0xbfff;
pub const M4_MAX_OFFSET_V1: u32 = 0xbffe;

pub const M1_MIN_LEN: u32 = 2;
pub const M1_MAX_LEN: u32 = 2;
pub const M2_MIN_LEN: u32 = 3;
pub const M2_MAX_LEN: u32 = 8;
pub const M3_MIN_LEN: u32 = 3;
pub const M3_MAX_LEN: u32 = 33;
pub const M4_MIN_LEN: u32 = 3;
pub const M4_MAX_LEN: u32 = 9;

pub const M1_MARKER: u32 = 0;
pub const M2_MARKER: u32 = 64;
pub const M3_MARKER: u32 = 32;
pub const M4_MARKER: u32 = 16;

pub const MIN_ZERO_RUN_LENGTH: u32 = 4;
pub const MAX_ZERO_RUN_LENGTH: u32 = 2047 + MIN_ZERO_RUN_LENGTH;

pub type lzo_dict_t = u16;
pub const D_BITS: u32 = 13;
pub const D_SIZE: u32 = 1u32 << D_BITS;
pub const D_MASK: u32 = D_SIZE - 1;
pub const D_HIGH: u32 = (D_MASK >> 1) + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
