/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * The shmid64_ds structure for sparc architecture.
 * Note extra padding because this structure is passed back and forth
 * between kernel and user space.
 *
 * Pad space is left for:
 * - 2 miscellaneous 32-bit values
 *
 * The C header includes asm/ipcbuf.h and asm/posix_types.h; the referenced
 * types are supplied by those dependencies.
 */
#[repr(C)]
pub struct shmid64_ds {
    pub shm_perm: ipc64_perm, /* operation perms */
    #[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
    pub shm_atime: libc::c_long, /* last attach time */
    #[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
    pub shm_dtime: libc::c_long, /* last detach time */
    #[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
    pub shm_ctime: libc::c_long, /* last change time */
    #[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
    pub shm_atime_high: libc::c_ulong,
    #[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
    pub shm_atime: libc::c_ulong, /* last attach time */
    #[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
    pub shm_dtime_high: libc::c_ulong,
    #[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
    pub shm_dtime: libc::c_ulong, /* last detach time */
    #[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
    pub shm_ctime_high: libc::c_ulong,
    #[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
    pub shm_ctime: libc::c_ulong, /* last change time */
    pub shm_segsz: __kernel_size_t, /* size of segment (bytes) */
    pub shm_cpid: __kernel_pid_t, /* pid of creator */
    pub shm_lpid: __kernel_pid_t, /* pid of last operator */
    pub shm_nattch: libc::c_ulong, /* no. of current attaches */
    pub __unused1: libc::c_ulong,
    pub __unused2: libc::c_ulong,
}

#[repr(C)]
pub struct shminfo64 {
    pub shmmax: libc::c_ulong,
    pub shmmin: libc::c_ulong,
    pub shmmni: libc::c_ulong,
    pub shmseg: libc::c_ulong,
    pub shmall: libc::c_ulong,
    pub __unused1: libc::c_ulong,
    pub __unused2: libc::c_ulong,
    pub __unused3: libc::c_ulong,
    pub __unused4: libc::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
