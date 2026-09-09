/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * For non-x86_64 or non-ILP32 builds, the C header includes the generic
 * shmbuf definition. That external dependency is intentionally not
 * reimplemented here.
 */

/*
 * The shmid64_ds structure for x86 architecture with x32 ABI.
 *
 * On x86-32 and x86-64 we can just use the generic definition, but
 * x32 uses the same binary layout as x86_64, which is different
 * from other 32-bit architectures.
 */

#[repr(C)]
pub struct shmid64_ds {
    pub shm_perm: ipc64_perm,      /* operation perms */
    pub shm_segsz: __kernel_size_t, /* size of segment (bytes) */
    pub shm_atime: __kernel_long_t, /* last attach time */
    pub shm_dtime: __kernel_long_t, /* last detach time */
    pub shm_ctime: __kernel_long_t, /* last change time */
    pub shm_cpid: __kernel_pid_t,  /* pid of creator */
    pub shm_lpid: __kernel_pid_t,  /* pid of last operator */
    pub shm_nattch: __kernel_ulong_t, /* no. of current attaches */
    pub __unused4: __kernel_ulong_t,
    pub __unused5: __kernel_ulong_t,
}

#[repr(C)]
pub struct shminfo64 {
    pub shmmax: __kernel_ulong_t,
    pub shmmin: __kernel_ulong_t,
    pub shmmni: __kernel_ulong_t,
    pub shmseg: __kernel_ulong_t,
    pub shmall: __kernel_ulong_t,
    pub __unused1: __kernel_ulong_t,
    pub __unused2: __kernel_ulong_t,
    pub __unused3: __kernel_ulong_t,
    pub __unused4: __kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
