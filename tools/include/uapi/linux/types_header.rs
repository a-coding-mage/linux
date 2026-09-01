/* SPDX-License-Identifier: GPL-2.0 */

/* Rust translation of include/uapi/linux/types.h.
 *
 * C dependency:
 *   #include <asm-generic/int-ll64.h>
 *
 * The integer base types (__u16, __u32, __u64) are supplied by that dependency.
 *
 * The original header excludes these declarations for __ASSEMBLER__.
 */

/* copied from linux:include/uapi/linux/types.h */
/* C __bitwise is a sparse type annotation macro and has no runtime layout. */
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;

pub type __sum16 = __u16;
pub type __wsum = __u32;

/* C macros:
 *   #define __aligned_u64  __u64  __attribute__((aligned(8)))
 *   #define __aligned_be64 __be64 __attribute__((aligned(8)))
 *   #define __aligned_le64 __le64 __attribute__((aligned(8)))
 */
#[repr(transparent)]
#[repr(align(8))]
pub struct __aligned_u64(pub __u64);

#[repr(transparent)]
#[repr(align(8))]
pub struct __aligned_be64(pub __be64);

#[repr(transparent)]
#[repr(align(8))]
pub struct __aligned_le64(pub __le64);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
