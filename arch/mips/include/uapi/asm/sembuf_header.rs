/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependency supplied by <asm/ipcbuf.h>. */

/*
 * The semid64_ds structure for the MIPS architecture.
 * Note extra padding because this structure is passed back and forth
 * between kernel and user space.
 *
 * Pad space is left for 2 miscellaneous 64-bit values on mips64,
 * but used for the upper 32 bit of the time values on mips32.
 */

#[cfg(target_arch = "mips64")]
#[repr(C)]
pub struct semid64_ds {
    pub sem_perm: ipc64_perm, /* permissions .. see ipc.h */
    pub sem_otime: i64,       /* last semop time */
    pub sem_ctime: i64,       /* last change time */
    pub sem_nsems: u64,       /* no. of semaphores in array */
    pub __unused1: u64,
    pub __unused2: u64,
}

#[cfg(not(target_arch = "mips64"))]
#[repr(C)]
pub struct semid64_ds {
    pub sem_perm: ipc64_perm, /* permissions .. see ipc.h */
    pub sem_otime: u32,       /* last semop time */
    pub sem_ctime: u32,       /* last change time */
    pub sem_nsems: u32,       /* no. of semaphores in array */
    pub sem_otime_high: u32,
    pub sem_ctime_high: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
