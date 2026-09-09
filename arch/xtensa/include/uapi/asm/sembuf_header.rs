/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * include/asm-xtensa/sembuf.h
 *
 * The semid64_ds structure for Xtensa architecture.
 *
 * Note extra padding because this structure is passed back and forth
 * between kernel and user space.
 *
 * Pad space is left for:
 * - 2 miscellaneous 32-bit values
 */

use core::ffi::c_ulong;

#[repr(C)]
pub struct semid64_ds {
    /* permissions .. see ipc.h */
    pub sem_perm: ipc64_perm,

    /* __XTENSA_EL__ corresponds to the little-endian Xtensa layout. */
    #[cfg(target_endian = "little")]
    pub sem_otime: c_ulong,      /* last semop time */
    #[cfg(target_endian = "little")]
    pub sem_otime_high: c_ulong,
    #[cfg(target_endian = "little")]
    pub sem_ctime: c_ulong,      /* last change time */
    #[cfg(target_endian = "little")]
    pub sem_ctime_high: c_ulong,

    #[cfg(not(target_endian = "little"))]
    pub sem_otime_high: c_ulong,
    #[cfg(not(target_endian = "little"))]
    pub sem_otime: c_ulong,      /* last semop time */
    #[cfg(not(target_endian = "little"))]
    pub sem_ctime_high: c_ulong,
    #[cfg(not(target_endian = "little"))]
    pub sem_ctime: c_ulong,      /* last change time */

    pub sem_nsems: c_ulong,      /* no. of semaphores in array */
    pub __unused3: c_ulong,
    pub __unused4: c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
