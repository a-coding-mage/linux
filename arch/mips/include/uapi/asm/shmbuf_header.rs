/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the corresponding IPC and POSIX type headers are
// intentionally referenced but not defined here.

/*
 * The shmid64_ds structure for the MIPS architecture.
 * Note extra padding because this structure is passed back and forth
 * between kernel and user space.
 *
 * As MIPS was lacking proper padding after shm_?time, we use 48 bits
 * of the padding at the end to store a few additional bits of the time.
 * libc implementations need to take care to convert this into a proper
 * data structure when moving to 64-bit time_t.
 */

#[cfg(target_arch = "mips64")]
#[repr(C)]
pub struct shmid64_ds {
    pub shm_perm: ipc64_perm,       /* operation perms */
    pub shm_segsz: __kernel_size_t, /* size of segment (bytes) */
    pub shm_atime: ::kernel::ffi::c_long, /* last attach time */
    pub shm_dtime: ::kernel::ffi::c_long, /* last detach time */
    pub shm_ctime: ::kernel::ffi::c_long, /* last change time */
    pub shm_cpid: __kernel_pid_t,    /* pid of creator */
    pub shm_lpid: __kernel_pid_t,    /* pid of last operator */
    pub shm_nattch: ::kernel::ffi::c_ulong, /* no. of current attaches */
    pub __unused1: ::kernel::ffi::c_ulong,
    pub __unused2: ::kernel::ffi::c_ulong,
}

#[cfg(not(target_arch = "mips64"))]
#[repr(C)]
pub struct shmid64_ds {
    pub shm_perm: ipc64_perm,       /* operation perms */
    pub shm_segsz: __kernel_size_t, /* size of segment (bytes) */
    pub shm_atime: ::kernel::ffi::c_ulong, /* last attach time */
    pub shm_dtime: ::kernel::ffi::c_ulong, /* last detach time */
    pub shm_ctime: ::kernel::ffi::c_ulong, /* last change time */
    pub shm_cpid: __kernel_pid_t,    /* pid of creator */
    pub shm_lpid: __kernel_pid_t,    /* pid of last operator */
    pub shm_nattch: ::kernel::ffi::c_ulong, /* no. of current attaches */
    pub shm_atime_high: ::kernel::ffi::c_ushort,
    pub shm_dtime_high: ::kernel::ffi::c_ushort,
    pub shm_ctime_high: ::kernel::ffi::c_ushort,
    pub __unused1: ::kernel::ffi::c_ushort,
}

#[repr(C)]
pub struct shminfo64 {
    pub shmmax: ::kernel::ffi::c_ulong,
    pub shmmin: ::kernel::ffi::c_ulong,
    pub shmmni: ::kernel::ffi::c_ulong,
    pub shmseg: ::kernel::ffi::c_ulong,
    pub shmall: ::kernel::ffi::c_ulong,
    pub __unused1: ::kernel::ffi::c_ulong,
    pub __unused2: ::kernel::ffi::c_ulong,
    pub __unused3: ::kernel::ffi::c_ulong,
    pub __unused4: ::kernel::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
