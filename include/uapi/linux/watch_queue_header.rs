/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent from the original header: linux/types.h, linux/fcntl.h,
// and linux/ioctl.h provide __u32, __u64, O_EXCL, and _IO.

pub const O_NOTIFICATION_PIPE: _ = O_EXCL; /* Parameter to pipe2() selecting notification pipe */

pub const IOC_WATCH_QUEUE_SET_SIZE: _ = _IO('W' as _, 0x60); /* Set the size in pages */
pub const IOC_WATCH_QUEUE_SET_FILTER: _ = _IO('W' as _, 0x61); /* Set the filter */

#[repr(u32)]
pub enum watch_notification_type {
    WATCH_TYPE_META = 0, /* Special record */
    WATCH_TYPE_KEY_NOTIFY = 1, /* Key change event notification */
    WATCH_TYPE__NR = 2,
}

#[repr(u32)]
pub enum watch_meta_notification_subtype {
    WATCH_META_REMOVAL_NOTIFICATION = 0, /* Watched object was removed */
    WATCH_META_LOSS_NOTIFICATION = 1, /* Data loss occurred */
}

/*
 * Notification record header.  This is aligned to 64-bits so that subclasses
 * can contain __u64 fields.
 */
#[repr(C)]
pub struct watch_notification {
    /* C bitfields: type:24 and subtype:8, sharing one __u32 storage unit. */
    pub type_subtype: __u32,
    pub info: __u32,
}

pub const WATCH_INFO_LENGTH: __u32 = 0x0000007f; /* Length of record */
pub const WATCH_INFO_LENGTH__SHIFT: __u32 = 0;
pub const WATCH_INFO_ID: __u32 = 0x0000ff00; /* ID of watchpoint */
pub const WATCH_INFO_ID__SHIFT: __u32 = 8;
pub const WATCH_INFO_TYPE_INFO: __u32 = 0xffff0000; /* Type-specific info */
pub const WATCH_INFO_TYPE_INFO__SHIFT: __u32 = 16;
pub const WATCH_INFO_FLAG_0: __u32 = 0x00010000; /* Type-specific info, flag bit 0 */
pub const WATCH_INFO_FLAG_1: __u32 = 0x00020000; /* ... */
pub const WATCH_INFO_FLAG_2: __u32 = 0x00040000;
pub const WATCH_INFO_FLAG_3: __u32 = 0x00080000;
pub const WATCH_INFO_FLAG_4: __u32 = 0x00100000;
pub const WATCH_INFO_FLAG_5: __u32 = 0x00200000;
pub const WATCH_INFO_FLAG_6: __u32 = 0x00400000;
pub const WATCH_INFO_FLAG_7: __u32 = 0x00800000;

/* Notification filtering rules (IOC_WATCH_QUEUE_SET_FILTER). */
#[repr(C)]
pub struct watch_notification_type_filter {
    pub type_: __u32, /* Type to apply filter to */
    pub info_filter: __u32, /* Filter on watch_notification::info */
    pub info_mask: __u32, /* Mask of relevant bits in info_filter */
    pub subtype_filter: [__u32; 8], /* Bitmask of subtypes to filter on */
}

#[repr(C)]
pub struct watch_notification_filter {
    pub nr_filters: __u32, /* Number of filters */
    pub __reserved: __u32, /* Must be 0 */
    pub filters: [watch_notification_type_filter; 0],
}

/*
 * Extended watch removal notification.  This is used optionally if the type
 * wants to indicate an identifier for the object being watched, if there is
 * such.  This can be distinguished by the length.
 *
 * type -> WATCH_TYPE_META
 * subtype -> WATCH_META_REMOVAL_NOTIFICATION
 */
#[repr(C)]
pub struct watch_notification_removal {
    pub watch: watch_notification,
    pub id: __u64, /* Type-dependent identifier */
}

/* Type of key/keyring change notification. */
#[repr(u32)]
pub enum key_notification_subtype {
    NOTIFY_KEY_INSTANTIATED = 0, /* Key was instantiated (aux is error code) */
    NOTIFY_KEY_UPDATED = 1, /* Key was updated */
    NOTIFY_KEY_LINKED = 2, /* Key (aux) was added to watched keyring */
    NOTIFY_KEY_UNLINKED = 3, /* Key (aux) was removed from watched keyring */
    NOTIFY_KEY_CLEARED = 4, /* Keyring was cleared */
    NOTIFY_KEY_REVOKED = 5, /* Key was revoked */
    NOTIFY_KEY_INVALIDATED = 6, /* Key was invalidated */
    NOTIFY_KEY_SETATTR = 7, /* Key's attributes got changed */
}

/*
 * Key/keyring notification record.
 * - watch.type = WATCH_TYPE_KEY_NOTIFY
 * - watch.subtype = enum key_notification_type
 */
#[repr(C)]
pub struct key_notification {
    pub watch: watch_notification,
    pub key_id: __u32, /* The key/keyring affected */
    pub aux: __u32, /* Per-type auxiliary data */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
