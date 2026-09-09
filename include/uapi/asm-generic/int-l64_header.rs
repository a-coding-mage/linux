/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * asm-generic/int-l64.h
 *
 * Integer declarations for architectures which use "long"
 * for 64-bit types.
 */

// C header guard: _UAPI_ASM_GENERIC_INT_L64_H

// C dependency: <asm/bitsperlong.h>

// The declarations below correspond to the non-__ASSEMBLY__ C section.
// __xx is ok: it doesn't pollute the POSIX namespace. Use these in the
// header files exported to user space.

pub type __s8 = i8;
pub type __u8 = u8;

pub type __s16 = i16;
pub type __u16 = u16;

pub type __s32 = i32;
pub type __u32 = u32;

// C __signed__ long / unsigned long are 64-bit on the target architectures.
pub type __s64 = i64;
pub type __u64 = u64;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
