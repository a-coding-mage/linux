/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by the corresponding Linux POSIX types translation:
// __kernel_key_t, __kernel_uid32_t, __kernel_gid32_t, and __kernel_mode_t.

/*
 * The user_ipc_perm structure for S/390 architecture.
 * Note extra padding because this structure is passed back and forth
 * between kernel and user space.
 *
 * Pad space is left for:
 * - 32-bit mode_t and seq
 * - 2 miscellaneous 32-bit values
 */
#[repr(C)]
pub struct ipc64_perm {
    pub key: __kernel_key_t,
    pub uid: __kernel_uid32_t,
    pub gid: __kernel_gid32_t,
    pub cuid: __kernel_uid32_t,
    pub cgid: __kernel_gid32_t,
    pub mode: __kernel_mode_t,
    pub __pad1: u16,
    pub seq: u16,
    pub __unused1: ::core::ffi::c_ulong,
    pub __unused2: ::core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
