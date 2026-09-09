/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the corresponding translated headers:
// asm/fcntl.h, linux/openat2.h, linux/types.h

pub const F_SETLEASE: i32 = F_LINUX_SPECIFIC_BASE + 0;
pub const F_GETLEASE: i32 = F_LINUX_SPECIFIC_BASE + 1;

/*
 * Request nofications on a directory.
 * See below for events that may be notified.
 */
pub const F_NOTIFY: i32 = F_LINUX_SPECIFIC_BASE + 2;

pub const F_DUPFD_QUERY: i32 = F_LINUX_SPECIFIC_BASE + 3;

/* Was the file just created? */
pub const F_CREATED_QUERY: i32 = F_LINUX_SPECIFIC_BASE + 4;

/*
 * Cancel a blocking posix lock; internal use only until we expose an
 * asynchronous lock api to userspace:
 */
pub const F_CANCELLK: i32 = F_LINUX_SPECIFIC_BASE + 5;

/* Create a file descriptor with FD_CLOEXEC set. */
pub const F_DUPFD_CLOEXEC: i32 = F_LINUX_SPECIFIC_BASE + 6;

/* Set and get of pipe page size array */
pub const F_SETPIPE_SZ: i32 = F_LINUX_SPECIFIC_BASE + 7;
pub const F_GETPIPE_SZ: i32 = F_LINUX_SPECIFIC_BASE + 8;

/* Set/Get seals */
pub const F_ADD_SEALS: i32 = F_LINUX_SPECIFIC_BASE + 9;
pub const F_GET_SEALS: i32 = F_LINUX_SPECIFIC_BASE + 10;

/* Types of seals */
pub const F_SEAL_SEAL: u32 = 0x0001; /* prevent further seals from being set */
pub const F_SEAL_SHRINK: u32 = 0x0002; /* prevent file from shrinking */
pub const F_SEAL_GROW: u32 = 0x0004; /* prevent file from growing */
pub const F_SEAL_WRITE: u32 = 0x0008; /* prevent writes */
pub const F_SEAL_FUTURE_WRITE: u32 = 0x0010; /* prevent future writes while mapped */
pub const F_SEAL_EXEC: u32 = 0x0020; /* prevent chmod modifying exec bits */
/* (1U << 31) is reserved for signed error codes */

/*
 * Set/Get write life time hints. {GET,SET}_RW_HINT operate on the
 * underlying inode, while {GET,SET}_FILE_RW_HINT operate only on the
 * specific file.
 */
pub const F_GET_RW_HINT: i32 = F_LINUX_SPECIFIC_BASE + 11;
pub const F_SET_RW_HINT: i32 = F_LINUX_SPECIFIC_BASE + 12;
pub const F_GET_FILE_RW_HINT: i32 = F_LINUX_SPECIFIC_BASE + 13;
pub const F_SET_FILE_RW_HINT: i32 = F_LINUX_SPECIFIC_BASE + 14;

/*
 * Valid hint values for F_{GET,SET}_RW_HINT. 0 is "not set", or can be
 * used to clear any hints previously set.
 */
pub const RWH_WRITE_LIFE_NOT_SET: i32 = 0;
pub const RWH_WRITE_LIFE_NONE: i32 = 1;
pub const RWH_WRITE_LIFE_SHORT: i32 = 2;
pub const RWH_WRITE_LIFE_MEDIUM: i32 = 3;
pub const RWH_WRITE_LIFE_LONG: i32 = 4;
pub const RWH_WRITE_LIFE_EXTREME: i32 = 5;

/*
 * The originally introduced spelling is remained from the first
 * versions of the patch set that introduced the feature, see commit
 * v4.13-rc1~212^2~51.
 */
pub const RWF_WRITE_LIFE_NOT_SET: i32 = RWH_WRITE_LIFE_NOT_SET;

/* Set/Get delegations */
pub const F_GETDELEG: i32 = F_LINUX_SPECIFIC_BASE + 15;
pub const F_SETDELEG: i32 = F_LINUX_SPECIFIC_BASE + 16;

/* Argument structure for F_GETDELEG and F_SETDELEG */
#[repr(C)]
pub struct delegation {
    pub d_flags: __u32, /* Must be 0 */
    pub d_type: __u16,  /* F_RDLCK, F_WRLCK, F_UNLCK */
    pub __pad: __u16,   /* Must be 0 */
}

/* Types of directory notifications that may be requested. */
pub const DN_ACCESS: u32 = 0x00000001; /* File accessed */
pub const DN_MODIFY: u32 = 0x00000002; /* File modified */
pub const DN_CREATE: u32 = 0x00000004; /* File created */
pub const DN_DELETE: u32 = 0x00000008; /* File removed */
pub const DN_RENAME: u32 = 0x00000010; /* File renamed */
pub const DN_ATTRIB: u32 = 0x00000020; /* File changed attibutes */
pub const DN_MULTISHOT: u32 = 0x80000000; /* Don't remove notifier */

/* Reserved kernel ranges [-100], [-10000, -40000]. */
pub const AT_FDCWD: i32 = -100; /* Special value for dirfd used to
                                   indicate openat should use the
                                   current working directory. */

/*
 * The concept of process and threads in userland and the kernel is a confusing
 * one - within the kernel every thread is a 'task' with its own individual PID,
 * however from userland's point of view threads are grouped by a single PID,
 * which is that of the 'thread group leader', typically the first thread
 * spawned.
 *
 * To cut the Gideon knot, for internal kernel usage, we refer to
 * PIDFD_SELF_THREAD to refer to the current thread (or task from a kernel
 * perspective), and PIDFD_SELF_THREAD_GROUP to refer to the current thread
 * group leader...
 */
pub const PIDFD_SELF_THREAD: i32 = -10000; /* Current thread. */
pub const PIDFD_SELF_THREAD_GROUP: i32 = -10001; /* Current thread group leader. */

pub const FD_PIDFS_ROOT: i32 = -10002; /* Root of the pidfs filesystem */
pub const FD_NSFS_ROOT: i32 = -10003; /* Root of the nsfs filesystem */
pub const FD_FAILFS_ROOT: i32 = -10004; /* Root of the failfs filesystem */
pub const FD_INVALID: i32 = -10009; /* Invalid file descriptor: -10000 - EBADF = -10009 */

/* Generic flags for the *at(2) family of syscalls. */

/* Reserved for per-syscall flags 0xff. */
pub const AT_SYMLINK_NOFOLLOW: i32 = 0x100; /* Do not follow symbolic links. */
/* Reserved for per-syscall flags 0x200 */
pub const AT_SYMLINK_FOLLOW: i32 = 0x400; /* Follow symbolic links. */
pub const AT_NO_AUTOMOUNT: i32 = 0x800; /* Suppress terminal automount traversal. */
pub const AT_EMPTY_PATH: i32 = 0x1000; /* Allow empty relative pathname to operate on dirfd directly. */

/*
 * These flags are currently statx(2)-specific, but they could be made generic
 * in the future and so they should not be used for other per-syscall flags.
 */
pub const AT_STATX_SYNC_TYPE: i32 = 0x6000; /* Type of synchronisation required from statx() */
pub const AT_STATX_SYNC_AS_STAT: i32 = 0x0000; /* - Do whatever stat() does */
pub const AT_STATX_FORCE_SYNC: i32 = 0x2000; /* - Force the attributes to be sync'd with the server */
pub const AT_STATX_DONT_SYNC: i32 = 0x4000; /* - Don't sync attributes with the server */

pub const AT_RECURSIVE: i32 = 0x8000; /* Apply to the entire subtree */

/* Per-syscall flags for the *at(2) family of syscalls. */
/* See the source header for the full rationale and implementer note. */

/* Flags for renameat2(2) (must match legacy RENAME_* flags). */
pub const AT_RENAME_NOREPLACE: i32 = 0x0001;
pub const AT_RENAME_EXCHANGE: i32 = 0x0002;
pub const AT_RENAME_WHITEOUT: i32 = 0x0004;

/* Flag for faccessat(2). */
pub const AT_EACCESS: i32 = 0x200; /* Test access permitted for effective IDs, not real IDs. */
/* Flag for unlinkat(2). */
pub const AT_REMOVEDIR: i32 = 0x200; /* Remove directory instead of unlinking file. */
/* Flags for name_to_handle_at(2). */
pub const AT_HANDLE_FID: i32 = 0x200; /* File handle is needed to compare object identity and may not be usable with open_by_handle_at(2). */
pub const AT_HANDLE_MNT_ID_UNIQUE: i32 = 0x001; /* Return the u64 unique mount ID. */
pub const AT_HANDLE_CONNECTABLE: i32 = 0x002; /* Request a connectable file handle */

/* Flags for execveat2(2). */
pub const AT_EXECVE_CHECK: i32 = 0x10000; /* Only perform a check if execution would be allowed. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
