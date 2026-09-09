/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* coredump_{req,ack} flags */
pub const COREDUMP_KERNEL: u64 = 1u64 << 0;
pub const COREDUMP_USERSPACE: u64 = 1u64 << 1;
pub const COREDUMP_REJECT: u64 = 1u64 << 2;
pub const COREDUMP_WAIT: u64 = 1u64 << 3;

/* struct coredump_req - message kernel sends to userspace */
#[repr(C)]
pub struct coredump_req {
    pub size: __u32,
    pub size_ack: __u32,
    pub mask: __u64,
}

pub const COREDUMP_REQ_SIZE_VER0: u32 = 16u32; /* size of first published struct */

/* struct coredump_ack - message userspace sends to kernel */
#[repr(C)]
pub struct coredump_ack {
    pub size: __u32,
    pub spare: __u32,
    pub mask: __u64,
}

pub const COREDUMP_ACK_SIZE_VER0: u32 = 16u32; /* size of first published struct */

/* enum coredump_mark - Markers for the coredump socket */
#[repr(u32)]
pub enum coredump_mark {
    COREDUMP_MARK_REQACK = 0u32,
    COREDUMP_MARK_MINSIZE = 1u32,
    COREDUMP_MARK_MAXSIZE = 2u32,
    COREDUMP_MARK_UNSUPPORTED = 3u32,
    COREDUMP_MARK_CONFLICTING = 4u32,
    __COREDUMP_MARK_MAX = 1u32 << 31,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
