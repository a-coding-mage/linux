/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * The ipc64_perm structure for PA-RISC is almost identical to
 * kern_ipc_perm as we have always had 32-bit UIDs and GIDs in the kernel.
 * 'seq' has been changed from long to int so that it's the same size
 * on 64-bit kernels as on 32-bit ones.
 *
 * The following kernel type aliases are supplied by the corresponding
 * POSIX type definitions.
 */

#[repr(C)]
pub struct ipc64_perm {
    pub key: __kernel_key_t,
    pub uid: __kernel_uid_t,
    pub gid: __kernel_gid_t,
    pub cuid: __kernel_uid_t,
    pub cgid: __kernel_gid_t,
    #[cfg(not(target_pointer_width = "64"))]
    pub __pad1: u16,
    pub mode: __kernel_mode_t,
    pub __pad2: u16,
    pub seq: u16,
    pub __pad3: u32,
    pub __unused1: u64,
    pub __unused2: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
