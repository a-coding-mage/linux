/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// Translated from asm-generic/posix_types.h.
// The C header guard and include are intentionally omitted; bit-width and
// externally supplied type overrides are build-time concerns.

/*
 * This file is generally used by user-level software, so you need to
 * be a little careful about namespace pollution etc.
 *
 * First the types that are often defined in different ways across
 * architectures, so that you can override them.
 */

pub type __kernel_long_t = isize;
pub type __kernel_ulong_t = usize;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_mode_t = u32;
pub type __kernel_pid_t = i32;
pub type __kernel_ipc_pid_t = i32;
pub type __kernel_uid_t = u32;
pub type __kernel_gid_t = u32;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t = i32;
pub type __kernel_uid32_t = u32;
pub type __kernel_gid32_t = u32;
pub type __kernel_old_uid_t = __kernel_uid_t;
pub type __kernel_old_gid_t = __kernel_gid_t;
pub type __kernel_old_dev_t = u32;

/*
 * Most 32 bit architectures use "unsigned int" size_t,
 * and all 64 bit architectures use "unsigned long" size_t.
 *
 * Rust's usize/isize/ptrdiff-equivalent types preserve the native pointer
 * width represented by the C conditional on __BITS_PER_LONG.
 */
pub type __kernel_size_t = usize;
pub type __kernel_ssize_t = isize;
pub type __kernel_ptrdiff_t = isize;

#[repr(C)]
pub struct __kernel_fsid_t {
    pub val: [i32; 2],
}

/*
 * anything below here should be completely generic
 */
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = i64;
pub type __kernel_uoff_t = u64;
pub type __kernel_old_time_t = __kernel_long_t;
// In the C header, __kernel_time_t is omitted when __KERNEL__ is defined.
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_time64_t = i64;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = i32;
pub type __kernel_clockid_t = i32;
pub type __kernel_caddr_t = *mut u8;
pub type __kernel_uid16_t = u16;
pub type __kernel_gid16_t = u16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
