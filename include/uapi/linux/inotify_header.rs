/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Inode based directory notification for Linux
 *
 * Copyright (C) 2005 John McCutchan
 */

// For O_CLOEXEC and O_NONBLOCK: supplied by the Linux fcntl dependency.
// Linux fixed-width types are represented here by their Rust equivalents.

/*
 * struct inotify_event - structure read from the inotify device for each event
 *
 * When you are watching a directory, you will receive the filename for events
 * such as IN_CREATE, IN_DELETE, IN_OPEN, IN_CLOSE, ..., relative to the wd.
 */
#[repr(C)]
pub struct inotify_event {
    pub wd: i32,       /* watch descriptor */
    pub mask: u32,     /* watch mask */
    pub cookie: u32,   /* cookie to synchronize two events */
    pub len: u32,      /* length (including nulls) of name */
    pub name: [core::ffi::c_char; 0], /* stub for possible name */
}

/* the following are legal, implemented events that user-space can watch for */
pub const IN_ACCESS: u32 = 0x00000001;       /* File was accessed */
pub const IN_MODIFY: u32 = 0x00000002;       /* File was modified */
pub const IN_ATTRIB: u32 = 0x00000004;       /* Metadata changed */
pub const IN_CLOSE_WRITE: u32 = 0x00000008;  /* Writable file was closed */
pub const IN_CLOSE_NOWRITE: u32 = 0x00000010; /* Unwritable file closed */
pub const IN_OPEN: u32 = 0x00000020;         /* File was opened */
pub const IN_MOVED_FROM: u32 = 0x00000040;   /* File was moved from X */
pub const IN_MOVED_TO: u32 = 0x00000080;     /* File was moved to Y */
pub const IN_CREATE: u32 = 0x00000100;       /* Subfile was created */
pub const IN_DELETE: u32 = 0x00000200;       /* Subfile was deleted */
pub const IN_DELETE_SELF: u32 = 0x00000400;  /* Self was deleted */
pub const IN_MOVE_SELF: u32 = 0x00000800;    /* Self was moved */

/* the following are legal events.  they are sent as needed to any watch */
pub const IN_UNMOUNT: u32 = 0x00002000;      /* Backing fs was unmounted */
pub const IN_Q_OVERFLOW: u32 = 0x00004000;   /* Event queued overflowed */
pub const IN_IGNORED: u32 = 0x00008000;      /* File was ignored */

/* helper events */
pub const IN_CLOSE: u32 = IN_CLOSE_WRITE | IN_CLOSE_NOWRITE; /* close */
pub const IN_MOVE: u32 = IN_MOVED_FROM | IN_MOVED_TO;        /* moves */

/* special flags */
pub const IN_ONLYDIR: u32 = 0x01000000;      /* only watch the path if it is a directory */
pub const IN_DONT_FOLLOW: u32 = 0x02000000;  /* don't follow a sym link */
pub const IN_EXCL_UNLINK: u32 = 0x04000000;  /* exclude events on unlinked objects */
pub const IN_MASK_CREATE: u32 = 0x10000000;  /* only create watches */
pub const IN_MASK_ADD: u32 = 0x20000000;     /* add to the mask of an already existing watch */
pub const IN_ISDIR: u32 = 0x40000000;        /* event occurred against dir */
pub const IN_ONESHOT: u32 = 0x80000000;      /* only send event once */

/*
 * All of the events - we build the list by hand so that we can add flags in
 * the future and not break backward compatibility.  Apps will get only the
 * events that they originally wanted.  Be sure to add new events here!
 */
pub const IN_ALL_EVENTS: u32 = IN_ACCESS | IN_MODIFY | IN_ATTRIB | IN_CLOSE_WRITE |
    IN_CLOSE_NOWRITE | IN_OPEN | IN_MOVED_FROM | IN_MOVED_TO | IN_DELETE |
    IN_CREATE | IN_DELETE_SELF | IN_MOVE_SELF;

/* Flags for sys_inotify_init1.  */
// These names are supplied by the Linux fcntl dependency.
pub const IN_CLOEXEC: u32 = O_CLOEXEC;
pub const IN_NONBLOCK: u32 = O_NONBLOCK;

/*
 * ioctl numbers: inotify uses 'I' prefix for all ioctls,
 * except historical FIONREAD, which is based on 'T'.
 *
 * INOTIFY_IOC_SETNEXTWD: set desired number of next created
 * watch descriptor.
 */
pub const INOTIFY_IOC_SETNEXTWD: u32 = _IOW(b'I' as u32, 0, i32);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
