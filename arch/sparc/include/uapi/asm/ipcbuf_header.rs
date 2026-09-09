/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: <linux/posix_types.h>

/*
 * The ipc64_perm structure for sparc/sparc64 architecture.
 * Note extra padding because this structure is passed back and forth
 * between kernel and user space.
 *
 * Pad space is left for:
 * - 32-bit seq
 * - on sparc for 32 bit mode (it is 32 bit on sparc64)
 * - 2 miscellaneous 64-bit values
 */
#[repr(C)]
pub struct ipc64_perm {
    pub key: __kernel_key_t,
    pub uid: __kernel_uid32_t,
    pub gid: __kernel_gid32_t,
    pub cuid: __kernel_uid32_t,
    pub cgid: __kernel_gid32_t,
    // C build-time condition: this field is present when __arch64__ is not defined.
    #[cfg(not(arch64))]
    pub __pad0: ::core::primitive::u16,
    pub mode: __kernel_mode_t,
    pub __pad1: ::core::primitive::u16,
    pub seq: ::core::primitive::u16,
    pub __unused1: ::core::primitive::u64,
    pub __unused2: ::core::primitive::u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
