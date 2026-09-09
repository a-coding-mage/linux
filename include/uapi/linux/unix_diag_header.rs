/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: __u8, __u16, and __u32 are supplied by the translated Linux types.

#[repr(C)]
pub struct unix_diag_req {
    pub sdiag_family: __u8,
    pub sdiag_protocol: __u8,
    pub pad: __u16,
    pub udiag_states: __u32,
    pub udiag_ino: __u32,
    pub udiag_show: __u32,
    pub udiag_cookie: [__u32; 2],
}

pub const UDIAG_SHOW_NAME: __u32 = 0x00000001; // show name (not path)
pub const UDIAG_SHOW_VFS: __u32 = 0x00000002; // show VFS inode info
pub const UDIAG_SHOW_PEER: __u32 = 0x00000004; // show peer socket info
pub const UDIAG_SHOW_ICONS: __u32 = 0x00000008; // show pending connections
pub const UDIAG_SHOW_RQLEN: __u32 = 0x00000010; // show skb receive queue len
pub const UDIAG_SHOW_MEMINFO: __u32 = 0x00000020; // show memory info of a socket
pub const UDIAG_SHOW_UID: __u32 = 0x00000040; // show socket's UID

#[repr(C)]
pub struct unix_diag_msg {
    pub udiag_family: __u8,
    pub udiag_type: __u8,
    pub udiag_state: __u8,
    pub pad: __u8,
    pub udiag_ino: __u32,
    pub udiag_cookie: [__u32; 2],
}

#[repr(i32)]
pub enum unix_diag_attr {
    // UNIX_DIAG_NONE, standard nl API requires this attribute!
    UNIX_DIAG_NAME,
    UNIX_DIAG_VFS,
    UNIX_DIAG_PEER,
    UNIX_DIAG_ICONS,
    UNIX_DIAG_RQLEN,
    UNIX_DIAG_MEMINFO,
    UNIX_DIAG_SHUTDOWN,
    UNIX_DIAG_UID,
    __UNIX_DIAG_MAX,
}

pub const UNIX_DIAG_MAX: i32 = __UNIX_DIAG_MAX as i32 - 1;

#[repr(C)]
pub struct unix_diag_vfs {
    pub udiag_vfs_ino: __u32,
    pub udiag_vfs_dev: __u32,
}

#[repr(C)]
pub struct unix_diag_rqlen {
    pub udiag_rqueue: __u32,
    pub udiag_wqueue: __u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
