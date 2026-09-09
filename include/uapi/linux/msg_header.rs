/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependency declarations from <linux/ipc.h> and <asm/msgbuf.h> are supplied
 * by the surrounding translation unit. */

/* ipcs ctl commands */
pub const MSG_STAT: i32 = 11;
pub const MSG_INFO: i32 = 12;
pub const MSG_STAT_ANY: i32 = 13;

/* msgrcv options */
pub const MSG_NOERROR: i32 = 0o10000; /* no error if message is too big */
pub const MSG_EXCEPT: i32 = 0o20000; /* recv any msg except of specified type. */
pub const MSG_COPY: i32 = 0o40000; /* copy (not remove) all queue messages */

/* Obsolete, used only for backwards compatibility and libc5 compiles */
#[repr(C)]
pub struct msqid_ds {
    pub msg_perm: ipc_perm,
    pub msg_first: *mut msg, /* first message on queue,unused  */
    pub msg_last: *mut msg, /* last message in queue,unused */
    pub msg_stime: __kernel_old_time_t, /* last msgsnd time */
    pub msg_rtime: __kernel_old_time_t, /* last msgrcv time */
    pub msg_ctime: __kernel_old_time_t, /* last change time */
    pub msg_lcbytes: ::core::ffi::c_ulong, /* Reuse junk fields for 32 bit */
    pub msg_lqbytes: ::core::ffi::c_ulong, /* ditto */
    pub msg_cbytes: u16, /* current number of bytes on queue */
    pub msg_qnum: u16, /* number of messages in queue */
    pub msg_qbytes: u16, /* max number of bytes on queue */
    pub msg_lspid: __kernel_ipc_pid_t, /* pid of last msgsnd */
    pub msg_lrpid: __kernel_ipc_pid_t, /* last receive pid */
}

/* Include the definition of msqid64_ds */

/* message buffer for msgsnd and msgrcv calls */
#[repr(C)]
pub struct msgbuf {
    pub mtype: __kernel_long_t, /* type of message */
    pub mtext: [::core::ffi::c_char; 1], /* message text */
}

/* buffer for msgctl calls IPC_INFO, MSG_INFO */
#[repr(C)]
pub struct msginfo {
    pub msgpool: i32,
    pub msgmap: i32,
    pub msgmax: i32,
    pub msgmnb: i32,
    pub msgmni: i32,
    pub msgssz: i32,
    pub msgtql: i32,
    pub msgseg: u16,
}

/*
 * MSGMNI, MSGMAX and MSGMNB are default values which can be
 * modified by sysctl.
 *
 * MSGMNI is the upper limit for the number of messages queues per
 * namespace.
 * It has been chosen to be as large possible without facilitating
 * scenarios where userspace causes overflows when adjusting the limits via
 * operations of the form retrieve current limit; add X; update limit".
 *
 * MSGMNB is the default size of a new message queue. Non-root tasks can
 * decrease the size with msgctl(IPC_SET), root tasks
 * (actually: CAP_SYS_RESOURCE) can both increase and decrease the queue
 * size. The optimal value is application dependent.
 * 16384 is used because it was always used (since 0.99.10)
 *
 * MAXMAX is the maximum size of an individual message, it's a global
 * (per-namespace) limit that applies for all message queues.
 * It's set to 1/2 of MSGMNB, to ensure that at least two messages fit into
 * the queue. This is also an arbitrary choice (since 2.6.0).
 */

pub const MSGMNI: i32 = 32000; /* <= IPCMNI */ /* max # of msg queue identifiers */
pub const MSGMAX: i32 = 8192; /* <= INT_MAX */ /* max size of message (bytes) */
pub const MSGMNB: i32 = 16384; /* <= INT_MAX */ /* default max size of a message queue */

/* unused */
pub const MSGPOOL: i32 = MSGMNI * MSGMNB / 1024; /* size in kbytes of message pool */
pub const MSGTQL: i32 = MSGMNB; /* number of system message headers */
pub const MSGMAP: i32 = MSGMNB; /* number of entries in message map */
pub const MSGSSZ: i32 = 16; /* message segment size */
pub const __MSGSEG: i32 = (MSGPOOL * 1024) / MSGSSZ; /* max no. of segments */
pub const MSGSEG: i32 = if __MSGSEG <= 0xffff { __MSGSEG } else { 0xffff };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
