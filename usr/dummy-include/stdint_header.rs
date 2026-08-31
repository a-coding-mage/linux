/* SPDX-License-Identifier: GPL-2.0-only */

/* Depends on linux/types.h definitions: __u64, __u32, __u16, __u8,
 * __s64, __s32, __s16, and __s8.
 */

pub type uint64_t = __u64;
pub type uint32_t = __u32;
pub type uint16_t = __u16;
pub type uint8_t = __u8;

pub type int64_t = __s64;
pub type int32_t = __s32;
pub type int16_t = __s16;
pub type int8_t = __s8;
