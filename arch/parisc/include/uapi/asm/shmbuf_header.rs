/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * The shmid64_ds structure for parisc architecture.
 * Note extra padding because this structure is passed back and forth
 * between kernel and user space.
 *
 * Pad space is left for:
 * - 2 miscellaneous 32-bit values
 */

#[repr(C)]
pub struct shmid64_ds {
    pub shm_perm: ipc64_perm, /* operation perms */
    #[cfg(target_pointer_width = "64")]
    pub shm_atime: ::core::ffi::c_long, /* last attach time */
    #[cfg(target_pointer_width = "64")]
    pub shm_dtime: ::core::ffi::c_long, /* last detach time */
    #[cfg(target_pointer_width = "64")]
    pub shm_ctime: ::core::ffi::c_long, /* last change time */
    #[cfg(not(target_pointer_width = "64"))]
    pub shm_atime_high: ::core::ffi::c_ulong,
    #[cfg(not(target_pointer_width = "64"))]
    pub shm_atime: ::core::ffi::c_ulong, /* last attach time */
    #[cfg(not(target_pointer_width = "64"))]
    pub shm_dtime_high: ::core::ffi::c_ulong,
    #[cfg(not(target_pointer_width = "64"))]
    pub shm_dtime: ::core::ffi::c_ulong, /* last detach time */
    #[cfg(not(target_pointer_width = "64"))]
    pub shm_ctime_high: ::core::ffi::c_ulong,
    #[cfg(not(target_pointer_width = "64"))]
    pub shm_ctime: ::core::ffi::c_ulong, /* last change time */
    #[cfg(not(target_pointer_width = "64"))]
    pub __pad4: ::core::ffi::c_uint,
    pub shm_segsz: __kernel_size_t, /* size of segment (bytes) */
    pub shm_cpid: __kernel_pid_t, /* pid of creator */
    pub shm_lpid: __kernel_pid_t, /* pid of last operator */
    pub shm_nattch: ::core::ffi::c_ulong, /* no. of current attaches */
    pub __unused1: ::core::ffi::c_ulong,
    pub __unused2: ::core::ffi::c_ulong,
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
