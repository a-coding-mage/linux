/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Depends on Linux UAPI integer types from <linux/types.h>. */

/* the following events that user-space can register for */
pub const FAN_ACCESS: u32 = 0x00000001; /* File was accessed */
pub const FAN_MODIFY: u32 = 0x00000002; /* File was modified */
pub const FAN_ATTRIB: u32 = 0x00000004; /* Metadata changed */
pub const FAN_CLOSE_WRITE: u32 = 0x00000008; /* Writable file closed */
pub const FAN_CLOSE_NOWRITE: u32 = 0x00000010; /* Unwritable file closed */
pub const FAN_OPEN: u32 = 0x00000020; /* File was opened */
pub const FAN_MOVED_FROM: u32 = 0x00000040; /* File was moved from X */
pub const FAN_MOVED_TO: u32 = 0x00000080; /* File was moved to Y */
pub const FAN_CREATE: u32 = 0x00000100; /* Subfile was created */
pub const FAN_DELETE: u32 = 0x00000200; /* Subfile was deleted */
pub const FAN_DELETE_SELF: u32 = 0x00000400; /* Self was deleted */
pub const FAN_MOVE_SELF: u32 = 0x00000800; /* Self was moved */
pub const FAN_OPEN_EXEC: u32 = 0x00001000; /* File was opened for exec */

pub const FAN_Q_OVERFLOW: u32 = 0x00004000; /* Event queued overflowed */
pub const FAN_FS_ERROR: u32 = 0x00008000; /* Filesystem error */

pub const FAN_OPEN_PERM: u32 = 0x00010000; /* File open in perm check */
pub const FAN_ACCESS_PERM: u32 = 0x00020000; /* File accessed in perm check */
pub const FAN_OPEN_EXEC_PERM: u32 = 0x00040000; /* File open/exec in perm check */
/* #define FAN_DIR_MODIFY 0x00080000 */ /* Deprecated (reserved) */

pub const FAN_PRE_ACCESS: u32 = 0x00100000; /* Pre-content access hook */
pub const FAN_MNT_ATTACH: u32 = 0x01000000; /* Mount was attached */
pub const FAN_MNT_DETACH: u32 = 0x02000000; /* Mount was detached */

pub const FAN_EVENT_ON_CHILD: u32 = 0x08000000; /* Interested in child events */

pub const FAN_RENAME: u32 = 0x10000000; /* File was renamed */

pub const FAN_ONDIR: u32 = 0x40000000; /* Event occurred against dir */

/* helper events */
pub const FAN_CLOSE: u32 = FAN_CLOSE_WRITE | FAN_CLOSE_NOWRITE; /* close */
pub const FAN_MOVE: u32 = FAN_MOVED_FROM | FAN_MOVED_TO; /* moves */

/* flags used for fanotify_init() */
pub const FAN_CLOEXEC: u32 = 0x00000001;
pub const FAN_NONBLOCK: u32 = 0x00000002;

/* These are NOT bitwise flags.  Both bits are used together.  */
pub const FAN_CLASS_NOTIF: u32 = 0x00000000;
pub const FAN_CLASS_CONTENT: u32 = 0x00000004;
pub const FAN_CLASS_PRE_CONTENT: u32 = 0x00000008;

/* Deprecated - do not use this in programs and do not add new flags here! */
pub const FAN_ALL_CLASS_BITS: u32 = FAN_CLASS_NOTIF | FAN_CLASS_CONTENT | FAN_CLASS_PRE_CONTENT;

pub const FAN_UNLIMITED_QUEUE: u32 = 0x00000010;
pub const FAN_UNLIMITED_MARKS: u32 = 0x00000020;
pub const FAN_ENABLE_AUDIT: u32 = 0x00000040;

/* Flags to determine fanotify event format */
pub const FAN_REPORT_PIDFD: u32 = 0x00000080; /* Report pidfd for event->pid */
pub const FAN_REPORT_TID: u32 = 0x00000100; /* event->pid is thread id */
pub const FAN_REPORT_FID: u32 = 0x00000200; /* Report unique file id */
pub const FAN_REPORT_DIR_FID: u32 = 0x00000400; /* Report unique directory id */
pub const FAN_REPORT_NAME: u32 = 0x00000800; /* Report events with name */
pub const FAN_REPORT_TARGET_FID: u32 = 0x00001000; /* Report dirent target id  */
pub const FAN_REPORT_FD_ERROR: u32 = 0x00002000; /* event->fd can report error */
pub const FAN_REPORT_MNT: u32 = 0x00004000; /* Report mount events */

/* Convenience macro - FAN_REPORT_NAME requires FAN_REPORT_DIR_FID */
pub const FAN_REPORT_DFID_NAME: u32 = FAN_REPORT_DIR_FID | FAN_REPORT_NAME;
/* Convenience macro - FAN_REPORT_TARGET_FID requires all other FID flags */
pub const FAN_REPORT_DFID_NAME_TARGET: u32 =
    FAN_REPORT_DFID_NAME | FAN_REPORT_FID | FAN_REPORT_TARGET_FID;

/* Deprecated - do not use this in programs and do not add new flags here! */
pub const FAN_ALL_INIT_FLAGS: u32 =
    FAN_CLOEXEC | FAN_NONBLOCK | FAN_ALL_CLASS_BITS | FAN_UNLIMITED_QUEUE | FAN_UNLIMITED_MARKS;

/* flags used for fanotify_modify_mark() */
pub const FAN_MARK_ADD: u32 = 0x00000001;
pub const FAN_MARK_REMOVE: u32 = 0x00000002;
pub const FAN_MARK_DONT_FOLLOW: u32 = 0x00000004;
pub const FAN_MARK_ONLYDIR: u32 = 0x00000008;
/* FAN_MARK_MOUNT is        0x00000010 */
pub const FAN_MARK_IGNORED_MASK: u32 = 0x00000020;
pub const FAN_MARK_IGNORED_SURV_MODIFY: u32 = 0x00000040;
pub const FAN_MARK_FLUSH: u32 = 0x00000080;
/* FAN_MARK_FILESYSTEM is   0x00000100 */
pub const FAN_MARK_EVICTABLE: u32 = 0x00000200;
/* This bit is mutually exclusive with FAN_MARK_IGNORED_MASK bit */
pub const FAN_MARK_IGNORE: u32 = 0x00000400;

/* These are NOT bitwise flags.  Both bits can be used togther.  */
pub const FAN_MARK_INODE: u32 = 0x00000000;
pub const FAN_MARK_MOUNT: u32 = 0x00000010;
pub const FAN_MARK_FILESYSTEM: u32 = 0x00000100;
pub const FAN_MARK_MNTNS: u32 = 0x00000110;

/*
 * Convenience macro - FAN_MARK_IGNORE requires FAN_MARK_IGNORED_SURV_MODIFY
 * for non-inode mark types.
 */
pub const FAN_MARK_IGNORE_SURV: u32 = FAN_MARK_IGNORE | FAN_MARK_IGNORED_SURV_MODIFY;

/* Deprecated - do not use this in programs and do not add new flags here! */
pub const FAN_ALL_MARK_FLAGS: u32 = FAN_MARK_ADD
    | FAN_MARK_REMOVE
    | FAN_MARK_DONT_FOLLOW
    | FAN_MARK_ONLYDIR
    | FAN_MARK_MOUNT
    | FAN_MARK_IGNORED_MASK
    | FAN_MARK_IGNORED_SURV_MODIFY
    | FAN_MARK_FLUSH;

/* Deprecated - do not use this in programs and do not add new flags here! */
pub const FAN_ALL_EVENTS: u32 = FAN_ACCESS | FAN_MODIFY | FAN_CLOSE | FAN_OPEN;

/*
 * All events which require a permission response from userspace
 */
/* Deprecated - do not use this in programs and do not add new flags here! */
pub const FAN_ALL_PERM_EVENTS: u32 = FAN_OPEN_PERM | FAN_ACCESS_PERM;

/* Deprecated - do not use this in programs and do not add new flags here! */
pub const FAN_ALL_OUTGOING_EVENTS: u32 = FAN_ALL_EVENTS | FAN_ALL_PERM_EVENTS | FAN_Q_OVERFLOW;

pub const FANOTIFY_METADATA_VERSION: u32 = 3;

#[repr(C)]
pub struct fanotify_event_metadata {
    pub event_len: __u32,
    pub vers: __u8,
    pub reserved: __u8,
    pub metadata_len: __u16,
    pub mask: __aligned_u64,
    pub fd: __s32,
    pub pid: __s32,
}

pub const FAN_EVENT_INFO_TYPE_FID: u32 = 1;
pub const FAN_EVENT_INFO_TYPE_DFID_NAME: u32 = 2;
pub const FAN_EVENT_INFO_TYPE_DFID: u32 = 3;
pub const FAN_EVENT_INFO_TYPE_PIDFD: u32 = 4;
pub const FAN_EVENT_INFO_TYPE_ERROR: u32 = 5;
pub const FAN_EVENT_INFO_TYPE_RANGE: u32 = 6;
pub const FAN_EVENT_INFO_TYPE_MNT: u32 = 7;

/* Special info types for FAN_RENAME */
pub const FAN_EVENT_INFO_TYPE_OLD_DFID_NAME: u32 = 10;
/* Reserved for FAN_EVENT_INFO_TYPE_OLD_DFID 11 */
pub const FAN_EVENT_INFO_TYPE_NEW_DFID_NAME: u32 = 12;
/* Reserved for FAN_EVENT_INFO_TYPE_NEW_DFID 13 */

/* Variable length info record following event metadata */
#[repr(C)]
pub struct fanotify_event_info_header {
    pub info_type: __u8,
    pub pad: __u8,
    pub len: __u16,
}

/*
 * Unique file identifier info record.
 * This structure is used for records of types FAN_EVENT_INFO_TYPE_FID,
 * FAN_EVENT_INFO_TYPE_DFID and FAN_EVENT_INFO_TYPE_DFID_NAME.
 * For FAN_EVENT_INFO_TYPE_DFID_NAME there is additionally a null terminated
 * name immediately after the file handle.
 */
#[repr(C)]
pub struct fanotify_event_info_fid {
    pub hdr: fanotify_event_info_header,
    pub fsid: __kernel_fsid_t,
    /*
     * Following is an opaque struct file_handle that can be passed as
     * an argument to open_by_handle_at(2).
     */
    pub handle: [::core::ffi::c_uchar; 0],
}

/*
 * This structure is used for info records of type FAN_EVENT_INFO_TYPE_PIDFD.
 * It holds a pidfd for the pid that was responsible for generating an event.
 */
#[repr(C)]
pub struct fanotify_event_info_pidfd {
    pub hdr: fanotify_event_info_header,
    pub pidfd: __s32,
}

#[repr(C)]
pub struct fanotify_event_info_error {
    pub hdr: fanotify_event_info_header,
    pub error: __s32,
    pub error_count: __u32,
}

#[repr(C)]
pub struct fanotify_event_info_range {
    pub hdr: fanotify_event_info_header,
    pub pad: __u32,
    pub offset: __u64,
    pub count: __u64,
}

#[repr(C)]
pub struct fanotify_event_info_mnt {
    pub hdr: fanotify_event_info_header,
    pub mnt_id: __u64,
}

/*
 * User space may need to record additional information about its decision.
 * The extra information type records what kind of information is included.
 * The default is none. We also define an extra information buffer whose
 * size is determined by the extra information type.
 *
 * If the information type is Audit Rule, then the information following
 * is the rule number that triggered the user space decision that
 * requires auditing.
 */

pub const FAN_RESPONSE_INFO_NONE: u32 = 0;
pub const FAN_RESPONSE_INFO_AUDIT_RULE: u32 = 1;

#[repr(C)]
pub struct fanotify_response {
    pub fd: __s32,
    pub response: __u32,
}

#[repr(C)]
pub struct fanotify_response_info_header {
    pub r#type: __u8,
    pub pad: __u8,
    pub len: __u16,
}

#[repr(C)]
pub struct fanotify_response_info_audit_rule {
    pub hdr: fanotify_response_info_header,
    pub rule_number: __u32,
    pub subj_trust: __u32,
    pub obj_trust: __u32,
}

/* Legit userspace responses to a _PERM event */
pub const FAN_ALLOW: u32 = 0x01;
pub const FAN_DENY: u32 = 0x02;
/* errno other than EPERM can specified in upper byte of deny response */
pub const FAN_ERRNO_BITS: u32 = 8;
pub const FAN_ERRNO_SHIFT: u32 = 32 - FAN_ERRNO_BITS;
pub const FAN_ERRNO_MASK: u32 = (1 << FAN_ERRNO_BITS) - 1;

pub const fn FAN_DENY_ERRNO(err: __u32) -> __u32 {
    FAN_DENY | ((err & FAN_ERRNO_MASK) << FAN_ERRNO_SHIFT)
}

pub const FAN_AUDIT: u32 = 0x10; /* Bitmask to create audit record for result */
pub const FAN_INFO: u32 = 0x20; /* Bitmask to indicate additional information */

/* No fd set in event */
pub const FAN_NOFD: i32 = -1;
pub const FAN_NOPIDFD: i32 = FAN_NOFD;
pub const FAN_EPIDFD: i32 = -2;

/* Helper functions to deal with fanotify_event_metadata buffers */
pub const FAN_EVENT_METADATA_LEN: usize = ::core::mem::size_of::<fanotify_event_metadata>();

pub unsafe fn FAN_EVENT_NEXT(
    meta: *mut fanotify_event_metadata,
    len: *mut usize,
) -> *mut fanotify_event_metadata {
    unsafe {
        *len = (*len).wrapping_sub((*meta).event_len as usize);
        (meta as *mut ::core::ffi::c_char).add((*meta).event_len as usize)
            as *mut fanotify_event_metadata
    }
}

pub unsafe fn FAN_EVENT_OK(meta: *const fanotify_event_metadata, len: usize) -> bool {
    unsafe {
        (len as isize) >= (FAN_EVENT_METADATA_LEN as isize)
            && ((*meta).event_len as isize) >= (FAN_EVENT_METADATA_LEN as isize)
            && ((*meta).event_len as isize) <= (len as isize)
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
