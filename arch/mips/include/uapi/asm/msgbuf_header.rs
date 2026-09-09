/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: `ipc64_perm` and `__kernel_pid_t` are supplied by asm/ipcbuf.h.

/*
 * The msqid64_ds structure for the MIPS architecture.
 * Note extra padding because this structure is passed back and forth
 * between kernel and user space.
 *
 * Pad space is left for:
 * - 2 miscellaneous unsigned long values
 */

#[cfg(target_arch = "mips64")]
#[repr(C)]
pub struct msqid64_ds {
    pub msg_perm: ipc64_perm,
    pub msg_stime: ::core::ffi::c_long, /* last msgsnd time */
    pub msg_rtime: ::core::ffi::c_long, /* last msgrcv time */
    pub msg_ctime: ::core::ffi::c_long, /* last change time */
    pub msg_cbytes: ::core::ffi::c_ulong, /* current number of bytes on queue */
    pub msg_qnum: ::core::ffi::c_ulong, /* number of messages in queue */
    pub msg_qbytes: ::core::ffi::c_ulong, /* max number of bytes on queue */
    pub msg_lspid: __kernel_pid_t, /* pid of last msgsnd */
    pub msg_lrpid: __kernel_pid_t, /* last receive pid */
    pub __unused4: ::core::ffi::c_ulong,
    pub __unused5: ::core::ffi::c_ulong,
}

#[cfg(all(target_arch = "mips", target_endian = "big"))]
#[repr(C)]
pub struct msqid64_ds {
    pub msg_perm: ipc64_perm,
    pub msg_stime_high: ::core::ffi::c_ulong,
    pub msg_stime: ::core::ffi::c_ulong, /* last msgsnd time */
    pub msg_rtime_high: ::core::ffi::c_ulong,
    pub msg_rtime: ::core::ffi::c_ulong, /* last msgrcv time */
    pub msg_ctime_high: ::core::ffi::c_ulong,
    pub msg_ctime: ::core::ffi::c_ulong, /* last change time */
    pub msg_cbytes: ::core::ffi::c_ulong, /* current number of bytes on queue */
    pub msg_qnum: ::core::ffi::c_ulong, /* number of messages in queue */
    pub msg_qbytes: ::core::ffi::c_ulong, /* max number of bytes on queue */
    pub msg_lspid: __kernel_pid_t, /* pid of last msgsnd */
    pub msg_lrpid: __kernel_pid_t, /* last receive pid */
    pub __unused4: ::core::ffi::c_ulong,
    pub __unused5: ::core::ffi::c_ulong,
}

#[cfg(all(target_arch = "mips", target_endian = "little"))]
#[repr(C)]
pub struct msqid64_ds {
    pub msg_perm: ipc64_perm,
    pub msg_stime: ::core::ffi::c_ulong, /* last msgsnd time */
    pub msg_stime_high: ::core::ffi::c_ulong,
    pub msg_rtime: ::core::ffi::c_ulong, /* last msgrcv time */
    pub msg_rtime_high: ::core::ffi::c_ulong,
    pub msg_ctime: ::core::ffi::c_ulong, /* last change time */
    pub msg_ctime_high: ::core::ffi::c_ulong,
    pub msg_cbytes: ::core::ffi::c_ulong, /* current number of bytes on queue */
    pub msg_qnum: ::core::ffi::c_ulong, /* number of messages in queue */
    pub msg_qbytes: ::core::ffi::c_ulong, /* max number of bytes on queue */
    pub msg_lspid: __kernel_pid_t, /* pid of last msgsnd */
    pub msg_lrpid: __kernel_pid_t, /* last receive pid */
    pub __unused4: ::core::ffi::c_ulong,
    pub __unused5: ::core::ffi::c_ulong,
}

// C source warning: no endianness set.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
