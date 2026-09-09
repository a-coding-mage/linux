/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Intel MIC Platform Software Stack (MPSS); Intel SCIF driver. */
/* Translated from scif_ioctl.h. The _IO* macros are supplied by Linux ioctl dependencies. */

#[repr(C)]
pub struct scif_port_id {
    pub node: u16,
    pub port: u16,
}

#[repr(C)]
pub struct scifioctl_connect {
    pub self_: scif_port_id,
    pub peer: scif_port_id,
}

#[repr(C)]
pub struct scifioctl_accept {
    pub flags: i32,
    pub peer: scif_port_id,
    pub endpt: u64,
}

#[repr(C)]
pub struct scifioctl_msg {
    pub msg: u64,
    pub len: i32,
    pub flags: i32,
    pub out_len: i32,
}

#[repr(C)]
pub struct scifioctl_reg {
    pub addr: u64,
    pub len: u64,
    pub offset: i64,
    pub prot: i32,
    pub flags: i32,
    pub out_offset: i64,
}

#[repr(C)]
pub struct scifioctl_unreg {
    pub offset: i64,
    pub len: u64,
}

#[repr(C)]
pub struct scifioctl_copy {
    pub loffset: i64,
    pub len: u64,
    pub roffset: i64,
    pub addr: u64,
    pub flags: i32,
}

#[repr(C)]
pub struct scifioctl_fence_mark {
    pub flags: i32,
    pub mark: u64,
}

#[repr(C)]
pub struct scifioctl_fence_signal {
    pub loff: i64,
    pub lval: u64,
    pub roff: i64,
    pub rval: u64,
    pub flags: i32,
}

#[repr(C)]
pub struct scifioctl_node_ids {
    pub nodes: u64,
    pub self_: u64,
    pub len: i32,
}

pub const SCIF_BIND: _ = _IOWR(b's' as _, 1, core::mem::size_of::<u64>());
pub const SCIF_LISTEN: _ = _IOW(b's' as _, 2, core::mem::size_of::<i32>());
pub const SCIF_CONNECT: _ = _IOWR(b's' as _, 3, core::mem::size_of::<scifioctl_connect>());
pub const SCIF_ACCEPTREQ: _ = _IOWR(b's' as _, 4, core::mem::size_of::<scifioctl_accept>());
pub const SCIF_ACCEPTREG: _ = _IOWR(b's' as _, 5, core::mem::size_of::<u64>());
pub const SCIF_SEND: _ = _IOWR(b's' as _, 6, core::mem::size_of::<scifioctl_msg>());
pub const SCIF_RECV: _ = _IOWR(b's' as _, 7, core::mem::size_of::<scifioctl_msg>());
pub const SCIF_REG: _ = _IOWR(b's' as _, 8, core::mem::size_of::<scifioctl_reg>());
pub const SCIF_UNREG: _ = _IOWR(b's' as _, 9, core::mem::size_of::<scifioctl_unreg>());
pub const SCIF_READFROM: _ = _IOWR(b's' as _, 10, core::mem::size_of::<scifioctl_copy>());
pub const SCIF_WRITETO: _ = _IOWR(b's' as _, 11, core::mem::size_of::<scifioctl_copy>());
pub const SCIF_VREADFROM: _ = _IOWR(b's' as _, 12, core::mem::size_of::<scifioctl_copy>());
pub const SCIF_VWRITETO: _ = _IOWR(b's' as _, 13, core::mem::size_of::<scifioctl_copy>());
pub const SCIF_GET_NODEIDS: _ = _IOWR(b's' as _, 14, core::mem::size_of::<scifioctl_node_ids>());
pub const SCIF_FENCE_MARK: _ = _IOWR(b's' as _, 15, core::mem::size_of::<scifioctl_fence_mark>());
pub const SCIF_FENCE_WAIT: _ = _IOWR(b's' as _, 16, core::mem::size_of::<i32>());
pub const SCIF_FENCE_SIGNAL: _ = _IOWR(b's' as _, 17, core::mem::size_of::<scifioctl_fence_signal>());

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
