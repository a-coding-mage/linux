/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by the corresponding IPC header: ipc64_perm.

/*
 * The semid64_ds structure for x86 architecture.
 * Note extra padding because this structure is passed back and forth
 * between kernel and user space.
 *
 * Pad space is left for:
 * - 2 miscellaneous 32-bit values
 *
 * x86_64 and x32 incorrectly added padding here, so the structures
 * are still incompatible with the padding on x86.
 */
#[repr(C)]
pub struct semid64_ds {
    pub sem_perm: ipc64_perm, /* permissions .. see ipc.h */

    #[cfg(target_arch = "x86")]
    pub sem_otime: ::core::ffi::c_ulong, /* last semop time */
    #[cfg(target_arch = "x86")]
    pub sem_otime_high: ::core::ffi::c_ulong,
    #[cfg(target_arch = "x86")]
    pub sem_ctime: ::core::ffi::c_ulong, /* last change time */
    #[cfg(target_arch = "x86")]
    pub sem_ctime_high: ::core::ffi::c_ulong,

    #[cfg(not(target_arch = "x86"))]
    pub sem_otime: __kernel_long_t, /* last semop time */
    #[cfg(not(target_arch = "x86"))]
    pub __unused1: __kernel_ulong_t,
    #[cfg(not(target_arch = "x86"))]
    pub sem_ctime: __kernel_long_t, /* last change time */
    #[cfg(not(target_arch = "x86"))]
    pub __unused2: __kernel_ulong_t,

    pub sem_nsems: __kernel_ulong_t, /* no. of semaphores in array */
    pub __unused3: __kernel_ulong_t,
    pub __unused4: __kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
