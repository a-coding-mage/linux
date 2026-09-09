/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// The non-x86_64 or non-ILP32 configuration uses the generic msgbuf
// definition from asm-generic/msgbuf.h.
//
// The x86_64 ILP32 configuration has the layout below.

/*
 * The msqid64_ds structure for x86 architecture with x32 ABI.
 *
 * On x86-32 and x86-64 we can just use the generic definition, but
 * x32 uses the same binary layout as x86_64, which is different
 * from other 32-bit architectures.
 */
#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
#[repr(C)]
pub struct msqid64_ds {
    pub msg_perm: ipc64_perm,
    pub msg_stime: __kernel_long_t, // last msgsnd time
    pub msg_rtime: __kernel_long_t, // last msgrcv time
    pub msg_ctime: __kernel_long_t, // last change time
    pub msg_cbytes: __kernel_ulong_t, // current number of bytes on queue
    pub msg_qnum: __kernel_ulong_t, // number of messages in queue
    pub msg_qbytes: __kernel_ulong_t, // max number of bytes on queue
    pub msg_lspid: __kernel_pid_t, // pid of last msgsnd
    pub msg_lrpid: __kernel_pid_t, // last receive pid
    pub __unused4: __kernel_ulong_t,
    pub __unused5: __kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
