/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by the corresponding IPC header.

/*
 * The semid64_ds structure for most architectures (though it came from x86_32
 * originally). Note extra padding because this structure is passed back and
 * forth between kernel and user space.
 *
 * semid64_ds was originally meant to be architecture specific, but
 * everyone just ended up making identical copies without specific
 * optimizations, so we may just as well all use the same one.
 *
 * 64 bit architectures use a 64-bit long time field here, while
 * 32 bit architectures have a pair of unsigned long values.
 *
 * On big-endian systems, the padding is in the wrong place for
 * historic reasons, so user space has to reconstruct a time_t
 * value using
 *
 * user_semid_ds.sem_otime = kernel_semid64_ds.sem_otime +
 *         ((long long)kernel_semid64_ds.sem_otime_high << 32)
 *
 * Pad space is left for 2 miscellaneous 32-bit values
 */
#[repr(C)]
pub struct semid64_ds {
    pub sem_perm: ipc64_perm, /* permissions .. see ipc.h */
    #[cfg(target_pointer_width = "64")]
    pub sem_otime: core::ffi::c_long, /* last semop time */
    #[cfg(target_pointer_width = "64")]
    pub sem_ctime: core::ffi::c_long, /* last change time */
    #[cfg(target_pointer_width = "32")]
    pub sem_otime: core::ffi::c_ulong, /* last semop time */
    #[cfg(target_pointer_width = "32")]
    pub sem_otime_high: core::ffi::c_ulong,
    #[cfg(target_pointer_width = "32")]
    pub sem_ctime: core::ffi::c_ulong, /* last change time */
    #[cfg(target_pointer_width = "32")]
    pub sem_ctime_high: core::ffi::c_ulong,
    pub sem_nsems: core::ffi::c_ulong, /* no. of semaphores in array */
    pub __unused3: core::ffi::c_ulong,
    pub __unused4: core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
