/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */

/*
 * The ipc64_perm structure for the powerpc is identical to
 * kern_ipc_perm as we have always had 32-bit UIDs and GIDs in the
 * kernel.  Note extra padding because this structure is passed back
 * and forth between kernel and user space.  Pad space is left for:
 *	- 1 32-bit value to fill up for 8-byte alignment
 *	- 2 miscellaneous 64-bit values
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 */

// Dependency supplied by the surrounding kernel type definitions:
// __kernel_key_t, __kernel_uid_t, __kernel_gid_t, and __kernel_mode_t.

#[repr(C)]
pub struct ipc64_perm {
    pub key: __kernel_key_t,
    pub uid: __kernel_uid_t,
    pub gid: __kernel_gid_t,
    pub cuid: __kernel_uid_t,
    pub cgid: __kernel_gid_t,
    pub mode: __kernel_mode_t,
    pub seq: u32,
    pub __pad1: u32,
    pub __unused1: u64,
    pub __unused2: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
