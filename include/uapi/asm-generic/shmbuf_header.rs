/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from asm-generic/shmbuf.h.
// Dependencies supplied by the surrounding ABI bindings:
// `ipc64_perm`, `__kernel_size_t`, and `__kernel_pid_t`.

/*
 * The shmid64_ds structure for x86 architecture.
 * Note extra padding because this structure is passed back and forth
 * between kernel and user space.
 *
 * shmid64_ds was originally meant to be architecture specific, but
 * everyone just ended up making identical copies without specific
 * optimizations, so we may just as well all use the same one.
 *
 * 64 bit architectures use a 64-bit long time field here, while
 * 32 bit architectures have a pair of unsigned long values.
 * On big-endian systems, the lower half is in the wrong place.
 *
 * Pad space is left for:
 * - 2 miscellaneous 32-bit values
 */
#[repr(C)]
pub struct shmid64_ds {
    pub shm_perm: ipc64_perm, /* operation perms */
    pub shm_segsz: __kernel_size_t, /* size of segment (bytes) */

    #[cfg(target_pointer_width = "64")]
    pub shm_atime: i64, /* last attach time */
    #[cfg(target_pointer_width = "64")]
    pub shm_dtime: i64, /* last detach time */
    #[cfg(target_pointer_width = "64")]
    pub shm_ctime: i64, /* last change time */

    #[cfg(not(target_pointer_width = "64"))]
    pub shm_atime: usize, /* last attach time */
    #[cfg(not(target_pointer_width = "64"))]
    pub shm_atime_high: usize,
    #[cfg(not(target_pointer_width = "64"))]
    pub shm_dtime: usize, /* last detach time */
    #[cfg(not(target_pointer_width = "64"))]
    pub shm_dtime_high: usize,
    #[cfg(not(target_pointer_width = "64"))]
    pub shm_ctime: usize, /* last change time */
    #[cfg(not(target_pointer_width = "64"))]
    pub shm_ctime_high: usize,

    pub shm_cpid: __kernel_pid_t, /* pid of creator */
    pub shm_lpid: __kernel_pid_t, /* pid of last operator */
    pub shm_nattch: usize, /* no. of current attaches */
    pub __unused4: usize,
    pub __unused5: usize,
}

#[repr(C)]
pub struct shminfo64 {
    pub shmmax: usize,
    pub shmmin: usize,
    pub shmmni: usize,
    pub shmseg: usize,
    pub shmall: usize,
    pub __unused1: usize,
    pub __unused2: usize,
    pub __unused3: usize,
    pub __unused4: usize,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
