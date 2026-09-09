/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Rust translation of sparc/include/uapi/asm/posix_types.h.
 *
 * The original include of asm-generic/posix_types.h supplies additional
 * declarations and is intentionally left as an external dependency.
 */

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
pub type __kernel_old_uid_t = u16;
#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
pub type __kernel_old_gid_t = u16;

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
pub type __kernel_suseconds_t = i32;

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
pub type __kernel_long_t = i64;
#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
pub type __kernel_ulong_t = u64;

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
#[repr(C)]
pub struct __kernel_old_timeval {
    pub tv_sec: __kernel_long_t,
    pub tv_usec: __kernel_suseconds_t,
}

#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
pub type __kernel_size_t = u32;
#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
pub type __kernel_ssize_t = i32;
#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
pub type __kernel_ptrdiff_t = i32;

#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
pub type __kernel_ipc_pid_t = u16;

#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
pub type __kernel_uid_t = u16;
#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
pub type __kernel_gid_t = u16;

#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
pub type __kernel_mode_t = u16;

#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
pub type __kernel_daddr_t = i32;

#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
pub type __kernel_old_dev_t = u16;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
