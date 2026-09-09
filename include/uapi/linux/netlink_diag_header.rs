/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: __u8, __u16, and __u32 are supplied by the translated Linux
// types definitions.

#[repr(C)]
pub struct netlink_diag_req {
    pub sdiag_family: __u8,
    pub sdiag_protocol: __u8,
    pub pad: __u16,
    pub ndiag_ino: __u32,
    pub ndiag_show: __u32,
    pub ndiag_cookie: [__u32; 2],
}

#[repr(C)]
pub struct netlink_diag_msg {
    pub ndiag_family: __u8,
    pub ndiag_type: __u8,
    pub ndiag_protocol: __u8,
    pub ndiag_state: __u8,
    pub ndiag_portid: __u32,
    pub ndiag_dst_portid: __u32,
    pub ndiag_dst_group: __u32,
    pub ndiag_ino: __u32,
    pub ndiag_cookie: [__u32; 2],
}

#[repr(C)]
pub struct netlink_diag_ring {
    pub ndr_block_size: __u32,
    pub ndr_block_nr: __u32,
    pub ndr_frame_size: __u32,
    pub ndr_frame_nr: __u32,
}

/* NETLINK_DIAG_NONE, standard nl API requires this attribute! */
pub const NETLINK_DIAG_NONE: i32 = 0;
pub const NETLINK_DIAG_MEMINFO: i32 = 1;
pub const NETLINK_DIAG_GROUPS: i32 = 2;
pub const NETLINK_DIAG_RX_RING: i32 = 3;
pub const NETLINK_DIAG_TX_RING: i32 = 4;
pub const NETLINK_DIAG_FLAGS: i32 = 5;
pub const __NETLINK_DIAG_MAX: i32 = 6;

pub const NETLINK_DIAG_MAX: i32 = __NETLINK_DIAG_MAX - 1;

pub const NDIAG_PROTO_ALL: __u8 = !0;

pub const NDIAG_SHOW_MEMINFO: __u32 = 0x00000001; /* show memory info of a socket */
pub const NDIAG_SHOW_GROUPS: __u32 = 0x00000002; /* show groups of a netlink socket */
// Deprecated since 4.6; present in non-kernel builds in the original header.
pub const NDIAG_SHOW_RING_CFG: __u32 = 0x00000004; /* show ring configuration */
pub const NDIAG_SHOW_FLAGS: __u32 = 0x00000008; /* show flags of a netlink socket */

/* flags */
pub const NDIAG_FLAG_CB_RUNNING: __u32 = 0x00000001;
pub const NDIAG_FLAG_PKTINFO: __u32 = 0x00000002;
pub const NDIAG_FLAG_BROADCAST_ERROR: __u32 = 0x00000004;
pub const NDIAG_FLAG_NO_ENOBUFS: __u32 = 0x00000008;
pub const NDIAG_FLAG_LISTEN_ALL_NSID: __u32 = 0x00000010;
pub const NDIAG_FLAG_CAP_ACK: __u32 = 0x00000020;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
