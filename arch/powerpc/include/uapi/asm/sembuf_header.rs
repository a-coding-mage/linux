/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */

// Dependency supplied by asm/ipcbuf.h:
// struct ipc64_perm

/*
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 */

/*
 * The semid64_ds structure for PPC architecture.
 * Note extra padding because this structure is passed back and forth
 * between kernel and user space.
 *
 * Pad space is left for:
 * - 2 miscellaneous 32/64-bit values
 */

#[repr(C)]
pub struct semid64_ds {
    pub sem_perm: ipc64_perm, /* permissions .. see ipc.h */
    #[cfg(not(target_arch = "powerpc64"))]
    pub sem_otime_high: core::ffi::c_ulong,
    #[cfg(not(target_arch = "powerpc64"))]
    pub sem_otime: core::ffi::c_ulong, /* last semop time */
    #[cfg(not(target_arch = "powerpc64"))]
    pub sem_ctime_high: core::ffi::c_ulong,
    #[cfg(not(target_arch = "powerpc64"))]
    pub sem_ctime: core::ffi::c_ulong, /* last change time */
    #[cfg(target_arch = "powerpc64")]
    pub sem_otime: core::ffi::c_long, /* last semop time */
    #[cfg(target_arch = "powerpc64")]
    pub sem_ctime: core::ffi::c_long, /* last change time */
    pub sem_nsems: core::ffi::c_ulong, /* no. of semaphores in array */
    pub __unused3: core::ffi::c_ulong,
    pub __unused4: core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
