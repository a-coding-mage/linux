/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependencies from the original C header:
 * #include <linux/compiler.h>
 * #include <linux/types.h>
 */

/* Valid values for seccomp.mode and prctl(PR_SET_SECCOMP, <mode>) */
pub const SECCOMP_MODE_DISABLED: u32 = 0; /* seccomp is not in use. */
pub const SECCOMP_MODE_STRICT: u32 = 1; /* uses hard-coded filter. */
pub const SECCOMP_MODE_FILTER: u32 = 2; /* uses user-supplied filter. */

/* Valid operations for seccomp syscall. */
pub const SECCOMP_SET_MODE_STRICT: u32 = 0;
pub const SECCOMP_SET_MODE_FILTER: u32 = 1;
pub const SECCOMP_GET_ACTION_AVAIL: u32 = 2;
pub const SECCOMP_GET_NOTIF_SIZES: u32 = 3;

/* Valid flags for SECCOMP_SET_MODE_FILTER */
pub const SECCOMP_FILTER_FLAG_TSYNC: usize = 1usize << 0;
pub const SECCOMP_FILTER_FLAG_LOG: usize = 1usize << 1;
pub const SECCOMP_FILTER_FLAG_SPEC_ALLOW: usize = 1usize << 2;
pub const SECCOMP_FILTER_FLAG_NEW_LISTENER: usize = 1usize << 3;
pub const SECCOMP_FILTER_FLAG_TSYNC_ESRCH: usize = 1usize << 4;
/* Received notifications wait in killable state (only respond to fatal signals) */
pub const SECCOMP_FILTER_FLAG_WAIT_KILLABLE_RECV: usize = 1usize << 5;

/*
 * All BPF programs must return a 32-bit value.
 * The bottom 16-bits are for optional return data.
 * The upper 16-bits are ordered from least permissive values to most,
 * as a signed value (so 0x8000000 is negative).
 *
 * The ordering ensures that a min_t() over composed return values always
 * selects the least permissive choice.
 */
pub const SECCOMP_RET_KILL_PROCESS: u32 = 0x80000000u32; /* kill the process */
pub const SECCOMP_RET_KILL_THREAD: u32 = 0x00000000u32; /* kill the thread */
pub const SECCOMP_RET_KILL: u32 = SECCOMP_RET_KILL_THREAD;
pub const SECCOMP_RET_TRAP: u32 = 0x00030000u32; /* disallow and force a SIGSYS */
pub const SECCOMP_RET_ERRNO: u32 = 0x00050000u32; /* returns an errno */
pub const SECCOMP_RET_USER_NOTIF: u32 = 0x7fc00000u32; /* notifies userspace */
pub const SECCOMP_RET_TRACE: u32 = 0x7ff00000u32; /* pass to a tracer or disallow */
pub const SECCOMP_RET_LOG: u32 = 0x7ffc0000u32; /* allow after logging */
pub const SECCOMP_RET_ALLOW: u32 = 0x7fff0000u32; /* allow */

/* Masks for the return value sections. */
pub const SECCOMP_RET_ACTION_FULL: u32 = 0xffff0000u32;
pub const SECCOMP_RET_ACTION: u32 = 0x7fff0000u32;
pub const SECCOMP_RET_DATA: u32 = 0x0000ffffu32;

/**
 * struct seccomp_data - the format the BPF program executes over.
 * @nr: the system call number
 * @arch: indicates system call convention as an AUDIT_ARCH_* value
 *        as defined in <linux/audit.h>.
 * @instruction_pointer: at the time of the system call.
 * @args: up to 6 system call arguments always stored as 64-bit values
 *        regardless of the architecture.
 */
#[repr(C)]
pub struct seccomp_data {
    pub nr: i32,
    pub arch: __u32,
    pub instruction_pointer: __u64,
    pub args: [__u64; 6],
}

#[repr(C)]
pub struct seccomp_notif_sizes {
    pub seccomp_notif: __u16,
    pub seccomp_notif_resp: __u16,
    pub seccomp_data: __u16,
}

#[repr(C)]
pub struct seccomp_notif {
    pub id: __u64,
    pub pid: __u32,
    pub flags: __u32,
    pub data: seccomp_data,
}

/*
 * Valid flags for struct seccomp_notif_resp
 *
 * Note, the SECCOMP_USER_NOTIF_FLAG_CONTINUE flag must be used with caution!
 * If set by the process supervising the syscalls of another process the
 * syscall will continue. This is problematic because of an inherent TOCTOU.
 * An attacker can exploit the time while the supervised process is waiting on
 * a response from the supervising process to rewrite syscall arguments which
 * are passed as pointers of the intercepted syscall.
 * It should be absolutely clear that this means that the seccomp notifier
 * _cannot_ be used to implement a security policy! It should only ever be used
 * in scenarios where a more privileged process supervises the syscalls of a
 * lesser privileged process to get around kernel-enforced security
 * restrictions when the privileged process deems this safe. In other words,
 * in order to continue a syscall the supervising process should be sure that
 * another security mechanism or the kernel itself will sufficiently block
 * syscalls if arguments are rewritten to something unsafe.
 *
 * Similar precautions should be applied when stacking SECCOMP_RET_USER_NOTIF
 * or SECCOMP_RET_TRACE. For SECCOMP_RET_USER_NOTIF filters acting on the
 * same syscall, the most recently added filter takes precedence. This means
 * that the new SECCOMP_RET_USER_NOTIF filter can override any
 * SECCOMP_IOCTL_NOTIF_SEND from earlier filters, essentially allowing all
 * such filtered syscalls to be executed by sending the response
 * SECCOMP_USER_NOTIF_FLAG_CONTINUE. Note that SECCOMP_RET_TRACE can equally
 * be overriden by SECCOMP_USER_NOTIF_FLAG_CONTINUE.
 */
pub const SECCOMP_USER_NOTIF_FLAG_CONTINUE: usize = 1usize << 0;

#[repr(C)]
pub struct seccomp_notif_resp {
    pub id: __u64,
    pub val: __s64,
    pub error: __s32,
    pub flags: __u32,
}

pub const SECCOMP_USER_NOTIF_FD_SYNC_WAKE_UP: usize = 1usize << 0;

/* valid flags for seccomp_notif_addfd */
pub const SECCOMP_ADDFD_FLAG_SETFD: usize = 1usize << 0; /* Specify remote fd */
pub const SECCOMP_ADDFD_FLAG_SEND: usize = 1usize << 1; /* Addfd and return it, atomically */

/**
 * struct seccomp_notif_addfd
 * @id: The ID of the seccomp notification
 * @flags: SECCOMP_ADDFD_FLAG_*
 * @srcfd: The local fd number
 * @newfd: Optional remote FD number if SETFD option is set, otherwise 0.
 * @newfd_flags: The O_* flags the remote FD should have applied
 */
#[repr(C)]
pub struct seccomp_notif_addfd {
    pub id: __u64,
    pub flags: __u32,
    pub srcfd: __u32,
    pub newfd: __u32,
    pub newfd_flags: __u32,
}

pub const SECCOMP_IOC_MAGIC: u8 = b'!';

macro_rules! SECCOMP_IO {
    ($nr:expr) => {
        _IO!(SECCOMP_IOC_MAGIC, $nr)
    };
}

macro_rules! SECCOMP_IOR {
    ($nr:expr, $type:ty) => {
        _IOR!(SECCOMP_IOC_MAGIC, $nr, $type)
    };
}

macro_rules! SECCOMP_IOW {
    ($nr:expr, $type:ty) => {
        _IOW!(SECCOMP_IOC_MAGIC, $nr, $type)
    };
}

macro_rules! SECCOMP_IOWR {
    ($nr:expr, $type:ty) => {
        _IOWR!(SECCOMP_IOC_MAGIC, $nr, $type)
    };
}

/* Flags for seccomp notification fd ioctl. */
pub const SECCOMP_IOCTL_NOTIF_RECV: usize = SECCOMP_IOWR!(0, seccomp_notif);
pub const SECCOMP_IOCTL_NOTIF_SEND: usize = SECCOMP_IOWR!(1, seccomp_notif_resp);
pub const SECCOMP_IOCTL_NOTIF_ID_VALID: usize = SECCOMP_IOW!(2, __u64);
/* On success, the return value is the remote process's added fd number */
pub const SECCOMP_IOCTL_NOTIF_ADDFD: usize = SECCOMP_IOW!(3, seccomp_notif_addfd);

pub const SECCOMP_IOCTL_NOTIF_SET_FLAGS: usize = SECCOMP_IOW!(4, __u64);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
