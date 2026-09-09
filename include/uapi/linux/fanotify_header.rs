/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependency on linux/types.h is represented by the corresponding external Rust types. */

pub const FAN_ACCESS: u32 = 0x00000001;
pub const FAN_MODIFY: u32 = 0x00000002;
pub const FAN_ATTRIB: u32 = 0x00000004;
pub const FAN_CLOSE_WRITE: u32 = 0x00000008;
pub const FAN_CLOSE_NOWRITE: u32 = 0x00000010;
pub const FAN_OPEN: u32 = 0x00000020;
pub const FAN_MOVED_FROM: u32 = 0x00000040;
pub const FAN_MOVED_TO: u32 = 0x00000080;
pub const FAN_CREATE: u32 = 0x00000100;
pub const FAN_DELETE: u32 = 0x00000200;
pub const FAN_DELETE_SELF: u32 = 0x00000400;
pub const FAN_MOVE_SELF: u32 = 0x00000800;
pub const FAN_OPEN_EXEC: u32 = 0x00001000;
pub const FAN_Q_OVERFLOW: u32 = 0x00004000;
pub const FAN_FS_ERROR: u32 = 0x00008000;
pub const FAN_OPEN_PERM: u32 = 0x00010000;
pub const FAN_ACCESS_PERM: u32 = 0x00020000;
pub const FAN_OPEN_EXEC_PERM: u32 = 0x00040000;
pub const FAN_PRE_ACCESS: u32 = 0x00100000;
pub const FAN_MNT_ATTACH: u32 = 0x01000000;
pub const FAN_MNT_DETACH: u32 = 0x02000000;
pub const FAN_EVENT_ON_CHILD: u32 = 0x08000000;
pub const FAN_RENAME: u32 = 0x10000000;
pub const FAN_ONDIR: u32 = 0x40000000;
pub const FAN_CLOSE: u32 = FAN_CLOSE_WRITE | FAN_CLOSE_NOWRITE;
pub const FAN_MOVE: u32 = FAN_MOVED_FROM | FAN_MOVED_TO;

pub const FAN_CLOEXEC: u32 = 0x00000001;
pub const FAN_NONBLOCK: u32 = 0x00000002;
pub const FAN_CLASS_NOTIF: u32 = 0x00000000;
pub const FAN_CLASS_CONTENT: u32 = 0x00000004;
pub const FAN_CLASS_PRE_CONTENT: u32 = 0x00000008;
pub const FAN_ALL_CLASS_BITS: u32 = FAN_CLASS_NOTIF | FAN_CLASS_CONTENT | FAN_CLASS_PRE_CONTENT;
pub const FAN_UNLIMITED_QUEUE: u32 = 0x00000010;
pub const FAN_UNLIMITED_MARKS: u32 = 0x00000020;
pub const FAN_ENABLE_AUDIT: u32 = 0x00000040;
pub const FAN_REPORT_PIDFD: u32 = 0x00000080;
pub const FAN_REPORT_TID: u32 = 0x00000100;
pub const FAN_REPORT_FID: u32 = 0x00000200;
pub const FAN_REPORT_DIR_FID: u32 = 0x00000400;
pub const FAN_REPORT_NAME: u32 = 0x00000800;
pub const FAN_REPORT_TARGET_FID: u32 = 0x00001000;
pub const FAN_REPORT_FD_ERROR: u32 = 0x00002000;
pub const FAN_REPORT_MNT: u32 = 0x00004000;
pub const FAN_REPORT_DFID_NAME: u32 = FAN_REPORT_DIR_FID | FAN_REPORT_NAME;
pub const FAN_REPORT_DFID_NAME_TARGET: u32 = FAN_REPORT_DFID_NAME | FAN_REPORT_FID | FAN_REPORT_TARGET_FID;
pub const FAN_ALL_INIT_FLAGS: u32 = FAN_CLOEXEC | FAN_NONBLOCK | FAN_ALL_CLASS_BITS | FAN_UNLIMITED_QUEUE | FAN_UNLIMITED_MARKS;

pub const FAN_MARK_ADD: u32 = 0x00000001;
pub const FAN_MARK_REMOVE: u32 = 0x00000002;
pub const FAN_MARK_DONT_FOLLOW: u32 = 0x00000004;
pub const FAN_MARK_ONLYDIR: u32 = 0x00000008;
pub const FAN_MARK_IGNORED_MASK: u32 = 0x00000020;
pub const FAN_MARK_IGNORED_SURV_MODIFY: u32 = 0x00000040;
pub const FAN_MARK_FLUSH: u32 = 0x00000080;
pub const FAN_MARK_EVICTABLE: u32 = 0x00000200;
pub const FAN_MARK_IGNORE: u32 = 0x00000400;
pub const FAN_MARK_INODE: u32 = 0x00000000;
pub const FAN_MARK_MOUNT: u32 = 0x00000010;
pub const FAN_MARK_FILESYSTEM: u32 = 0x00000100;
pub const FAN_MARK_MNTNS: u32 = 0x00000110;
pub const FAN_MARK_IGNORE_SURV: u32 = FAN_MARK_IGNORE | FAN_MARK_IGNORED_SURV_MODIFY;
pub const FAN_ALL_MARK_FLAGS: u32 = FAN_MARK_ADD | FAN_MARK_REMOVE | FAN_MARK_DONT_FOLLOW | FAN_MARK_ONLYDIR | FAN_MARK_MOUNT | FAN_MARK_IGNORED_MASK | FAN_MARK_IGNORED_SURV_MODIFY | FAN_MARK_FLUSH;
pub const FAN_ALL_EVENTS: u32 = FAN_ACCESS | FAN_MODIFY | FAN_CLOSE | FAN_OPEN;
pub const FAN_ALL_PERM_EVENTS: u32 = FAN_OPEN_PERM | FAN_ACCESS_PERM;
pub const FAN_ALL_OUTGOING_EVENTS: u32 = FAN_ALL_EVENTS | FAN_ALL_PERM_EVENTS | FAN_Q_OVERFLOW;
pub const FANOTIFY_METADATA_VERSION: u8 = 3;

#[repr(C)]
pub struct fanotify_event_metadata { pub event_len: u32, pub vers: u8, pub reserved: u8, pub metadata_len: u16, pub mask: u64, pub fd: i32, pub pid: i32 }
pub const FAN_EVENT_INFO_TYPE_FID: u8 = 1;
pub const FAN_EVENT_INFO_TYPE_DFID_NAME: u8 = 2;
pub const FAN_EVENT_INFO_TYPE_DFID: u8 = 3;
pub const FAN_EVENT_INFO_TYPE_PIDFD: u8 = 4;
pub const FAN_EVENT_INFO_TYPE_ERROR: u8 = 5;
pub const FAN_EVENT_INFO_TYPE_RANGE: u8 = 6;
pub const FAN_EVENT_INFO_TYPE_MNT: u8 = 7;
pub const FAN_EVENT_INFO_TYPE_OLD_DFID_NAME: u8 = 10;
pub const FAN_EVENT_INFO_TYPE_NEW_DFID_NAME: u8 = 12;

#[repr(C)]
pub struct fanotify_event_info_header { pub info_type: u8, pub pad: u8, pub len: u16 }
#[repr(C)]
pub struct fanotify_event_info_fid { pub hdr: fanotify_event_info_header, pub fsid: __kernel_fsid_t, pub handle: [u8; 0] }
#[repr(C)]
pub struct fanotify_event_info_pidfd { pub hdr: fanotify_event_info_header, pub pidfd: i32 }
#[repr(C)]
pub struct fanotify_event_info_error { pub hdr: fanotify_event_info_header, pub error: i32, pub error_count: u32 }
#[repr(C)]
pub struct fanotify_event_info_range { pub hdr: fanotify_event_info_header, pub pad: u32, pub offset: u64, pub count: u64 }
#[repr(C)]
pub struct fanotify_event_info_mnt { pub hdr: fanotify_event_info_header, pub mnt_id: u64 }

pub const FAN_RESPONSE_INFO_NONE: u8 = 0;
pub const FAN_RESPONSE_INFO_AUDIT_RULE: u8 = 1;
#[repr(C)]
pub struct fanotify_response { pub fd: i32, pub response: u32 }
#[repr(C)]
pub struct fanotify_response_info_header { pub type_: u8, pub pad: u8, pub len: u16 }
#[repr(C)]
pub struct fanotify_response_info_audit_rule { pub hdr: fanotify_response_info_header, pub rule_number: u32, pub subj_trust: u32, pub obj_trust: u32 }

pub const FAN_ALLOW: u32 = 0x01;
pub const FAN_DENY: u32 = 0x02;
pub const FAN_ERRNO_BITS: u32 = 8;
pub const FAN_ERRNO_SHIFT: u32 = 32 - FAN_ERRNO_BITS;
pub const FAN_ERRNO_MASK: u32 = (1 << FAN_ERRNO_BITS) - 1;
#[inline] pub const fn FAN_DENY_ERRNO(err: u32) -> u32 { FAN_DENY | ((err & FAN_ERRNO_MASK) << FAN_ERRNO_SHIFT) }
pub const FAN_AUDIT: u32 = 0x10;
pub const FAN_INFO: u32 = 0x20;
pub const FAN_NOFD: i32 = -1;
pub const FAN_NOPIDFD: i32 = FAN_NOFD;
pub const FAN_EPIDFD: i32 = -2;

pub const FAN_EVENT_METADATA_LEN: usize = core::mem::size_of::<fanotify_event_metadata>();
#[inline] pub unsafe fn FAN_EVENT_NEXT(meta: *mut fanotify_event_metadata, len: &mut usize) -> *mut fanotify_event_metadata {
    *len -= (*meta).event_len as usize;
    (meta as *mut u8).add((*meta).event_len as usize) as *mut fanotify_event_metadata
}
#[inline] pub unsafe fn FAN_EVENT_OK(meta: *const fanotify_event_metadata, len: usize) -> bool {
    (len as isize) >= FAN_EVENT_METADATA_LEN as isize && (*meta).event_len as isize >= FAN_EVENT_METADATA_LEN as isize && (*meta).event_len as usize <= len
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
