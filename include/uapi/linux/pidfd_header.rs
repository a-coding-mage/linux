/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Flags for pidfd_open(). */
pub const PIDFD_NONBLOCK: _ = O_NONBLOCK;
pub const PIDFD_THREAD: _ = O_EXCL;

/* These flags are kernel-internal and are available only when building the kernel. */
// #ifdef __KERNEL__
// pub const PIDFD_STALE: _ = CLONE_PIDFD;
// pub const PIDFD_AUTOKILL: _ = O_TRUNC;
// #endif

/* Flags for pidfd_send_signal(). */
pub const PIDFD_SIGNAL_THREAD: usize = 1usize << 0;
pub const PIDFD_SIGNAL_THREAD_GROUP: usize = 1usize << 1;
pub const PIDFD_SIGNAL_PROCESS_GROUP: usize = 1usize << 2;

/* Flags for pidfd_info. */
pub const PIDFD_INFO_PID: usize = 1usize << 0; /* Always returned, even if not requested */
pub const PIDFD_INFO_CREDS: usize = 1usize << 1; /* Always returned, even if not requested */
pub const PIDFD_INFO_CGROUPID: usize = 1usize << 2; /* Always returned if available, even if not requested */
pub const PIDFD_INFO_EXIT: usize = 1usize << 3; /* Only returned if requested. */
pub const PIDFD_INFO_COREDUMP: usize = 1usize << 4; /* Only returned if requested. */
pub const PIDFD_INFO_SUPPORTED_MASK: usize = 1usize << 5; /* Want/got supported mask flags */
pub const PIDFD_INFO_COREDUMP_SIGNAL: usize = 1usize << 6; /* Always returned if PIDFD_INFO_COREDUMP is requested. */
pub const PIDFD_INFO_COREDUMP_CODE: usize = 1usize << 7; /* Always returned if PIDFD_INFO_COREDUMP is requested. */

pub const PIDFD_INFO_SIZE_VER0: usize = 64; /* sizeof first published struct */
pub const PIDFD_INFO_SIZE_VER1: usize = 72; /* sizeof second published struct */
pub const PIDFD_INFO_SIZE_VER2: usize = 80; /* sizeof third published struct */
pub const PIDFD_INFO_SIZE_VER3: usize = 88; /* sizeof fourth published struct */

/*
 * Values for @coredump_mask in pidfd_info.
 * Only valid if PIDFD_INFO_COREDUMP is set in @mask.
 *
 * Note, the @PIDFD_COREDUMP_ROOT flag indicates that the generated
 * coredump should be treated as sensitive and access should only be
 * granted to privileged users.
 */
pub const PIDFD_COREDUMPED: u32 = 1u32 << 0; /* Did crash and... */
pub const PIDFD_COREDUMP_SKIP: u32 = 1u32 << 1; /* coredumping generation was skipped. */
pub const PIDFD_COREDUMP_USER: u32 = 1u32 << 2; /* coredump was done as the user. */
pub const PIDFD_COREDUMP_ROOT: u32 = 1u32 << 3; /* coredump was done as root. */

/* ...and for userland we make life simpler - PIDFD_SELF refers to the current
 * thread, PIDFD_SELF_PROCESS refers to the process thread group leader.
 *
 * For nearly all practical uses, a user will want to use PIDFD_SELF.
 */
pub const PIDFD_SELF: _ = PIDFD_SELF_THREAD;
pub const PIDFD_SELF_PROCESS: _ = PIDFD_SELF_THREAD_GROUP;

#[repr(C)]
pub struct pidfd_info {
    /* This mask is similar to the request_mask in statx(2). */
    pub mask: __u64,
    /* The information in these fields might be stale when received. */
    pub cgroupid: __u64,
    pub pid: __u32,
    pub tgid: __u32,
    pub ppid: __u32,
    pub ruid: __u32,
    pub rgid: __u32,
    pub euid: __u32,
    pub egid: __u32,
    pub suid: __u32,
    pub sgid: __u32,
    pub fsuid: __u32,
    pub fsgid: __u32,
    pub exit_code: __s32,
    pub coredump_mask: __u32,
    pub coredump_signal: __u32,
    pub coredump_code: __u32,
    pub coredump_pad: __u32, /* align supported_mask to 8 bytes */
    pub supported_mask: __u64, /* Mask flags that this kernel supports */
}

pub const PIDFS_IOCTL_MAGIC: _ = 0xFF;

pub const PIDFD_GET_CGROUP_NAMESPACE: _ = _IO(PIDFS_IOCTL_MAGIC, 1);
pub const PIDFD_GET_IPC_NAMESPACE: _ = _IO(PIDFS_IOCTL_MAGIC, 2);
pub const PIDFD_GET_MNT_NAMESPACE: _ = _IO(PIDFS_IOCTL_MAGIC, 3);
pub const PIDFD_GET_NET_NAMESPACE: _ = _IO(PIDFS_IOCTL_MAGIC, 4);
pub const PIDFD_GET_PID_NAMESPACE: _ = _IO(PIDFS_IOCTL_MAGIC, 5);
pub const PIDFD_GET_PID_FOR_CHILDREN_NAMESPACE: _ = _IO(PIDFS_IOCTL_MAGIC, 6);
pub const PIDFD_GET_TIME_NAMESPACE: _ = _IO(PIDFS_IOCTL_MAGIC, 7);
pub const PIDFD_GET_TIME_FOR_CHILDREN_NAMESPACE: _ = _IO(PIDFS_IOCTL_MAGIC, 8);
pub const PIDFD_GET_USER_NAMESPACE: _ = _IO(PIDFS_IOCTL_MAGIC, 9);
pub const PIDFD_GET_UTS_NAMESPACE: _ = _IO(PIDFS_IOCTL_MAGIC, 10);
pub const PIDFD_GET_INFO: _ = _IOWR(PIDFS_IOCTL_MAGIC, 11, pidfd_info);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
