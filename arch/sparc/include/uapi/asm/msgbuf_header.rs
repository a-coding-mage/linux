/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by the corresponding IPC header: `ipc64_perm`.

/*
 * The msqid64_ds structure for sparc64 architecture.
 * Note extra padding because this structure is passed back and forth
 * between kernel and user space.
 *
 * Pad space is left for:
 * - 2 miscellaneous 32-bit values
 */
#[repr(C)]
pub struct msqid64_ds {
    pub msg_perm: ipc64_perm,
    #[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
    pub msg_stime: ::core::ffi::c_long, // last msgsnd time
    #[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
    pub msg_rtime: ::core::ffi::c_long, // last msgrcv time
    #[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
    pub msg_ctime: ::core::ffi::c_long, // last change time
    #[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
    pub msg_stime_high: ::core::ffi::c_ulong,
    #[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
    pub msg_stime: ::core::ffi::c_ulong, // last msgsnd time
    #[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
    pub msg_rtime_high: ::core::ffi::c_ulong,
    #[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
    pub msg_rtime: ::core::ffi::c_ulong, // last msgrcv time
    #[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
    pub msg_ctime_high: ::core::ffi::c_ulong,
    #[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
    pub msg_ctime: ::core::ffi::c_ulong, // last change time
    pub msg_cbytes: ::core::ffi::c_ulong, // current number of bytes on queue
    pub msg_qnum: ::core::ffi::c_ulong, // number of messages in queue
    pub msg_qbytes: ::core::ffi::c_ulong, // max number of bytes on queue
    pub msg_lspid: __kernel_pid_t, // pid of last msgsnd
    pub msg_lrpid: __kernel_pid_t, // last receive pid
    pub __unused1: ::core::ffi::c_ulong,
    pub __unused2: ::core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
