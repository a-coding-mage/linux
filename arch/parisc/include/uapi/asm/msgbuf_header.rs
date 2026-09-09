/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// The declarations from <asm/bitsperlong.h> and <asm/ipcbuf.h> are supplied
// by the surrounding translation unit.

/*
 * The msqid64_ds structure for parisc architecture, copied from sparc.
 * Note extra padding because this structure is passed back and forth
 * between kernel and user space.
 *
 * Pad space is left for:
 * - 2 miscellaneous 32-bit values
 */
#[repr(C)]
pub struct msqid64_ds {
    pub msg_perm: ipc64_perm,

    #[cfg(target_pointer_width = "64")]
    pub msg_stime: isize, /* last msgsnd time */
    #[cfg(target_pointer_width = "64")]
    pub msg_rtime: isize, /* last msgrcv time */
    #[cfg(target_pointer_width = "64")]
    pub msg_ctime: isize, /* last change time */

    #[cfg(target_pointer_width = "32")]
    pub msg_stime_high: usize,
    #[cfg(target_pointer_width = "32")]
    pub msg_stime: usize, /* last msgsnd time */
    #[cfg(target_pointer_width = "32")]
    pub msg_rtime_high: usize,
    #[cfg(target_pointer_width = "32")]
    pub msg_rtime: usize, /* last msgrcv time */
    #[cfg(target_pointer_width = "32")]
    pub msg_ctime_high: usize,
    #[cfg(target_pointer_width = "32")]
    pub msg_ctime: usize, /* last change time */

    pub msg_cbytes: usize, /* current number of bytes on queue */
    pub msg_qnum: usize,   /* number of messages in queue */
    pub msg_qbytes: usize, /* max number of bytes on queue */
    pub msg_lspid: __kernel_pid_t, /* pid of last msgsnd */
    pub msg_lrpid: __kernel_pid_t, /* last receive pid */
    pub __unused1: usize,
    pub __unused2: usize,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
