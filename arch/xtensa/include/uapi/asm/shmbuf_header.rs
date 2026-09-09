/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * include/asm-xtensa/shmbuf.h
 *
 * The shmid64_ds structure for Xtensa architecture.
 * Note extra padding because this structure is passed back and forth
 * between kernel and user space, but the padding is on the wrong
 * side for big-endian xtensa, for historic reasons.
 *
 * Pad space is left for:
 * - 2 miscellaneous 32-bit values
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

// Dependency declarations from <asm/ipcbuf.h> and <asm/posix_types.h> are
// supplied externally.

#[repr(C)]
pub struct shmid64_ds {
    pub shm_perm: ipc64_perm,       /* operation perms */
    pub shm_segsz: __kernel_size_t, /* size of segment (bytes) */
    pub shm_atime: ::core::ffi::c_ulong, /* last attach time */
    pub shm_atime_high: ::core::ffi::c_ulong,
    pub shm_dtime: ::core::ffi::c_ulong, /* last detach time */
    pub shm_dtime_high: ::core::ffi::c_ulong,
    pub shm_ctime: ::core::ffi::c_ulong, /* last change time */
    pub shm_ctime_high: ::core::ffi::c_ulong,
    pub shm_cpid: __kernel_pid_t, /* pid of creator */
    pub shm_lpid: __kernel_pid_t, /* pid of last operator */
    pub shm_nattch: ::core::ffi::c_ulong, /* no. of current attaches */
    pub __unused4: ::core::ffi::c_ulong,
    pub __unused5: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct shminfo64 {
    pub shmmax: ::core::ffi::c_ulong,
    pub shmmin: ::core::ffi::c_ulong,
    pub shmmni: ::core::ffi::c_ulong,
    pub shmseg: ::core::ffi::c_ulong,
    pub shmall: ::core::ffi::c_ulong,
    pub __unused1: ::core::ffi::c_ulong,
    pub __unused2: ::core::ffi::c_ulong,
    pub __unused3: ::core::ffi::c_ulong,
    pub __unused4: ::core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
