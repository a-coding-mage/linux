/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent from <asm/bitsperlong.h> and <asm/ipcbuf.h> is preserved
// through the target-width condition and the externally supplied types.

/*
 * generic msqid64_ds structure.
 *
 * Note extra padding because this structure is passed back and forth
 * between kernel and user space.
 *
 * msqid64_ds was originally meant to be architecture specific, but
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
pub struct msqid64_ds {
    pub msg_perm: ipc64_perm,
    #[cfg(target_pointer_width = "64")]
    pub msg_stime: kernel::ffi::c_long, /* last msgsnd time */
    #[cfg(target_pointer_width = "64")]
    pub msg_rtime: kernel::ffi::c_long, /* last msgrcv time */
    #[cfg(target_pointer_width = "64")]
    pub msg_ctime: kernel::ffi::c_long, /* last change time */
    #[cfg(target_pointer_width = "32")]
    pub msg_stime: kernel::ffi::c_ulong, /* last msgsnd time */
    #[cfg(target_pointer_width = "32")]
    pub msg_stime_high: kernel::ffi::c_ulong,
    #[cfg(target_pointer_width = "32")]
    pub msg_rtime: kernel::ffi::c_ulong, /* last msgrcv time */
    #[cfg(target_pointer_width = "32")]
    pub msg_rtime_high: kernel::ffi::c_ulong,
    #[cfg(target_pointer_width = "32")]
    pub msg_ctime: kernel::ffi::c_ulong, /* last change time */
    #[cfg(target_pointer_width = "32")]
    pub msg_ctime_high: kernel::ffi::c_ulong,
    pub msg_cbytes: kernel::ffi::c_ulong, /* current number of bytes on queue */
    pub msg_qnum: kernel::ffi::c_ulong, /* number of messages in queue */
    pub msg_qbytes: kernel::ffi::c_ulong, /* max number of bytes on queue */
    pub msg_lspid: __kernel_pid_t, /* pid of last msgsnd */
    pub msg_lrpid: __kernel_pid_t, /* last receive pid */
    pub __unused4: kernel::ffi::c_ulong,
    pub __unused5: kernel::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
