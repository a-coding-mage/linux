/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the corresponding architecture headers:
// asm/bitsperlong.h and asm/ipcbuf.h

/*
 * The semid64_ds structure for parisc architecture.
 * Note extra padding because this structure is passed back and forth
 * between kernel and user space.
 *
 * Pad space is left for:
 * - 2 miscellaneous 32-bit values
 */
#[repr(C)]
pub struct semid64_ds {
    pub sem_perm: ipc64_perm, /* permissions .. see ipc.h */

    #[cfg(target_pointer_width = "64")]
    pub sem_otime: isize, /* last semop time */
    #[cfg(target_pointer_width = "64")]
    pub sem_ctime: isize, /* last change time */

    #[cfg(target_pointer_width = "32")]
    pub sem_otime_high: usize,
    #[cfg(target_pointer_width = "32")]
    pub sem_otime: usize, /* last semop time */
    #[cfg(target_pointer_width = "32")]
    pub sem_ctime_high: usize,
    #[cfg(target_pointer_width = "32")]
    pub sem_ctime: usize, /* last change time */

    pub sem_nsems: usize, /* no. of semaphores in array */
    pub __unused1: usize,
    pub __unused2: usize,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
