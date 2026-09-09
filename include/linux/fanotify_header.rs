/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux sysctl and fanotify headers
// are intentionally referenced but not redefined here.

#[inline]
pub unsafe fn fan_group_flag<G>(group: *const G, flag: u32) -> u32 {
    // The C macro expands to ((group)->fanotify_data.flags & (flag)); the
    // layout of G is supplied by the including translation unit.
    *(group as *const u32).add(0) & flag
}

/*
 * Flags allowed to be passed from/to userspace.
 *
 * We intentionally do not add new bits to the old FAN_ALL_* constants, because
 * they are uapi exposed constants. If there are programs out there using
 * these constant, the programs may break if re-compiled with new uapi headers
 * and then run on an old kernel.
 */

/* Group classes where permission events are allowed */
pub const FANOTIFY_PERM_CLASSES: u32 = FAN_CLASS_CONTENT | FAN_CLASS_PRE_CONTENT;

pub const FANOTIFY_CLASS_BITS: u32 = FAN_CLASS_NOTIF | FANOTIFY_PERM_CLASSES;

pub const FANOTIFY_FID_BITS: u32 = FAN_REPORT_DFID_NAME_TARGET;

pub const FANOTIFY_INFO_MODES: u32 = FANOTIFY_FID_BITS | FAN_REPORT_PIDFD | FAN_REPORT_MNT;

/*
 * fanotify_init() flags that require CAP_SYS_ADMIN.
 * We do not allow unprivileged groups to request permission events.
 * We do not allow unprivileged groups to get other process pid in events.
 * We do not allow unprivileged groups to use unlimited resources.
 */
pub const FANOTIFY_ADMIN_INIT_FLAGS: u32 = FANOTIFY_PERM_CLASSES
    | FAN_REPORT_TID
    | FAN_REPORT_PIDFD
    | FAN_REPORT_FD_ERROR
    | FAN_UNLIMITED_QUEUE
    | FAN_UNLIMITED_MARKS;

/* fanotify_init() flags that are allowed for user without CAP_SYS_ADMIN. */
pub const FANOTIFY_USER_INIT_FLAGS: u32 = FAN_CLASS_NOTIF
    | FANOTIFY_FID_BITS
    | FAN_REPORT_MNT
    | FAN_CLOEXEC
    | FAN_NONBLOCK;

pub const FANOTIFY_INIT_FLAGS: u32 = FANOTIFY_ADMIN_INIT_FLAGS | FANOTIFY_USER_INIT_FLAGS;

/* Internal group flags */
pub const FANOTIFY_UNPRIV: u32 = 0x80000000;
pub const FANOTIFY_INTERNAL_GROUP_FLAGS: u32 = FANOTIFY_UNPRIV;

pub const FANOTIFY_MARK_TYPE_BITS: u32 = FAN_MARK_INODE
    | FAN_MARK_MOUNT
    | FAN_MARK_FILESYSTEM
    | FAN_MARK_MNTNS;

pub const FANOTIFY_MARK_CMD_BITS: u32 = FAN_MARK_ADD | FAN_MARK_REMOVE | FAN_MARK_FLUSH;

pub const FANOTIFY_MARK_IGNORE_BITS: u32 = FAN_MARK_IGNORED_MASK | FAN_MARK_IGNORE;

pub const FANOTIFY_MARK_FLAGS: u32 = FANOTIFY_MARK_TYPE_BITS
    | FANOTIFY_MARK_CMD_BITS
    | FANOTIFY_MARK_IGNORE_BITS
    | FAN_MARK_DONT_FOLLOW
    | FAN_MARK_ONLYDIR
    | FAN_MARK_IGNORED_SURV_MODIFY
    | FAN_MARK_EVICTABLE;

/* Events that can be reported with data type FSNOTIFY_EVENT_PATH. */
pub const FANOTIFY_PATH_EVENTS: u32 = FAN_ACCESS | FAN_MODIFY | FAN_CLOSE | FAN_OPEN | FAN_OPEN_EXEC;

/* Directory entry modification events. */
pub const FANOTIFY_DIRENT_EVENTS: u32 = FAN_MOVE | FAN_CREATE | FAN_DELETE | FAN_RENAME;

/* Content events can be used to inspect file content */
pub const FANOTIFY_CONTENT_PERM_EVENTS: u32 = FAN_OPEN_PERM | FAN_OPEN_EXEC_PERM | FAN_ACCESS_PERM;
/* Pre-content events can be used to fill file content */
pub const FANOTIFY_PRE_CONTENT_EVENTS: u32 = FAN_PRE_ACCESS;

/* Events that require a permission response from user */
pub const FANOTIFY_PERM_EVENTS: u32 = FANOTIFY_CONTENT_PERM_EVENTS | FANOTIFY_PRE_CONTENT_EVENTS;

/* Events that can be reported with event->fd */
pub const FANOTIFY_FD_EVENTS: u32 = FANOTIFY_PATH_EVENTS | FANOTIFY_PERM_EVENTS;

/* Events that can only be reported with data type FSNOTIFY_EVENT_INODE */
pub const FANOTIFY_INODE_EVENTS: u32 = FANOTIFY_DIRENT_EVENTS | FAN_ATTRIB | FAN_MOVE_SELF | FAN_DELETE_SELF;

/* Events that can only be reported with data type FSNOTIFY_EVENT_ERROR */
pub const FANOTIFY_ERROR_EVENTS: u32 = FAN_FS_ERROR;

pub const FANOTIFY_MOUNT_EVENTS: u32 = FAN_MNT_ATTACH | FAN_MNT_DETACH;

/* Events that user can request to be notified on */
pub const FANOTIFY_EVENTS: u32 = FANOTIFY_PATH_EVENTS
    | FANOTIFY_INODE_EVENTS
    | FANOTIFY_ERROR_EVENTS
    | FANOTIFY_MOUNT_EVENTS;

/* Extra flags that may be reported with event or control handling of events */
pub const FANOTIFY_EVENT_FLAGS: u32 = FAN_EVENT_ON_CHILD | FAN_ONDIR;

/* Events that may be reported to user */
pub const FANOTIFY_OUTGOING_EVENTS: u32 = FANOTIFY_EVENTS | FANOTIFY_PERM_EVENTS | FAN_Q_OVERFLOW | FAN_ONDIR;

/* Events and flags relevant only for directories */
pub const FANOTIFY_DIRONLY_EVENT_BITS: u32 = FANOTIFY_DIRENT_EVENTS | FAN_EVENT_ON_CHILD | FAN_ONDIR;

pub const ALL_FANOTIFY_EVENT_BITS: u32 = FANOTIFY_OUTGOING_EVENTS | FANOTIFY_EVENT_FLAGS;

/* These masks check for invalid bits in permission responses. */
pub const FANOTIFY_RESPONSE_ACCESS: u32 = FAN_ALLOW | FAN_DENY;
pub const FANOTIFY_RESPONSE_FLAGS: u32 = FAN_AUDIT | FAN_INFO;
pub const FANOTIFY_RESPONSE_VALID_MASK: u32 = FANOTIFY_RESPONSE_ACCESS
    | FANOTIFY_RESPONSE_FLAGS
    | (FAN_ERRNO_MASK << FAN_ERRNO_SHIFT);

/* Do not use these old uapi constants internally. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
