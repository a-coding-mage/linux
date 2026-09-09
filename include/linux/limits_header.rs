/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux headers:
// <uapi/linux/limits.h>, <linux/types.h>, and <vdso/limits.h>.

pub const SIZE_MAX: size_t = !(0 as size_t);
pub const SSIZE_MAX: ssize_t = (SIZE_MAX >> 1) as ssize_t;
pub const PHYS_ADDR_MAX: phys_addr_t = !(0 as phys_addr_t);

pub const RESOURCE_SIZE_MAX: resource_size_t = !(0 as resource_size_t);

pub const U8_MAX: u8 = !(0 as u8);
pub const S8_MAX: s8 = (U8_MAX >> 1) as s8;
pub const S8_MIN: s8 = (-S8_MAX - 1) as s8;
pub const U16_MAX: u16 = !(0 as u16);
pub const S16_MAX: s16 = (U16_MAX >> 1) as s16;
pub const S16_MIN: s16 = (-S16_MAX - 1) as s16;
pub const U32_MAX: u32 = !(0 as u32);
pub const U32_MIN: u32 = 0;
pub const S32_MAX: s32 = (U32_MAX >> 1) as s32;
pub const S32_MIN: s32 = (-S32_MAX - 1) as s32;
pub const U64_MAX: u64 = !(0 as u64);
pub const S64_MAX: s64 = (U64_MAX >> 1) as s64;
pub const S64_MIN: s64 = (-S64_MAX - 1) as s64;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
