/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// C header guard: _UAPI_LINUX_TYPES_H
// C dependency: <asm/types.h>
// C dependency: <linux/posix_types.h>

// __ASSEMBLY__ excludes the declarations below in assembly builds.

// __SIZEOF_INT128__
#[cfg(target_pointer_width = "64")]
pub type __s128 = i128;
#[cfg(target_pointer_width = "64")]
pub type __u128 = u128;

// sparse's __CHECKER__ bitwise annotation has no direct Rust equivalent.
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;

pub type __sum16 = __u16;
pub type __wsum = __u32;

// C: #define __aligned_u64 __u64 __attribute__((aligned(8)))
#[repr(align(8))]
pub struct __aligned_u64(pub __u64);

// C: #define __aligned_s64 __s64 __attribute__((aligned(8)))
#[repr(align(8))]
pub struct __aligned_s64(pub __s64);

// C: #define __aligned_be64 __be64 __attribute__((aligned(8)))
#[repr(align(8))]
pub struct __aligned_be64(pub __be64);

// C: #define __aligned_le64 __le64 __attribute__((aligned(8)))
#[repr(align(8))]
pub struct __aligned_le64(pub __le64);

pub type __poll_t = __u32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
