/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// The generic ipc64_perm structure.
// Note extra padding because this structure is passed back and forth
// between kernel and user space.
//
// ipc64_perm was originally meant to be architecture specific, but
// everyone just ended up making identical copies without specific
// optimizations, so we may just as well all use the same one.
//
// Pad space is left for:
// - 32-bit mode_t on architectures that only had 16 bit
// - 32-bit seq
// - 2 miscellaneous 32-bit values

#[repr(C)]
pub struct ipc64_perm {
    pub key: __kernel_key_t,
    pub uid: __kernel_uid32_t,
    pub gid: __kernel_gid32_t,
    pub cuid: __kernel_uid32_t,
    pub cgid: __kernel_gid32_t,
    pub mode: __kernel_mode_t,
    // pad if mode_t is u16:
    pub __pad1: [u8; 4 - core::mem::size_of::<__kernel_mode_t>()],
    pub seq: u16,
    pub __pad2: u16,
    pub __unused1: __kernel_ulong_t,
    pub __unused2: __kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
