/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by the Linux types bindings: __u8, __u16, and __u32.

#[repr(C)]
pub struct packet_diag_req {
    pub sdiag_family: __u8,
    pub sdiag_protocol: __u8,
    pub pad: __u16,
    pub pdiag_ino: __u32,
    pub pdiag_show: __u32,
    pub pdiag_cookie: [__u32; 2],
}

pub const PACKET_SHOW_INFO: __u32 = 0x00000001; // Basic packet_sk information
pub const PACKET_SHOW_MCLIST: __u32 = 0x00000002; // A set of packet_diag_mclist-s
pub const PACKET_SHOW_RING_CFG: __u32 = 0x00000004; // Rings configuration parameters
pub const PACKET_SHOW_FANOUT: __u32 = 0x00000008;
pub const PACKET_SHOW_MEMINFO: __u32 = 0x00000010;
pub const PACKET_SHOW_FILTER: __u32 = 0x00000020;

#[repr(C)]
pub struct packet_diag_msg {
    pub pdiag_family: __u8,
    pub pdiag_type: __u8,
    pub pdiag_num: __u16,
    pub pdiag_ino: __u32,
    pub pdiag_cookie: [__u32; 2],
}

// PACKET_DIAG_NONE, standard nl API requires this attribute!
pub const PACKET_DIAG_INFO: i32 = 1;
pub const PACKET_DIAG_MCLIST: i32 = 2;
pub const PACKET_DIAG_RX_RING: i32 = 3;
pub const PACKET_DIAG_TX_RING: i32 = 4;
pub const PACKET_DIAG_FANOUT: i32 = 5;
pub const PACKET_DIAG_UID: i32 = 6;
pub const PACKET_DIAG_MEMINFO: i32 = 7;
pub const PACKET_DIAG_FILTER: i32 = 8;
pub const __PACKET_DIAG_MAX: i32 = 9;

pub const PACKET_DIAG_MAX: i32 = __PACKET_DIAG_MAX - 1;

#[repr(C)]
pub struct packet_diag_info {
    pub pdi_index: __u32,
    pub pdi_version: __u32,
    pub pdi_reserve: __u32,
    pub pdi_copy_thresh: __u32,
    pub pdi_tstamp: __u32,
    pub pdi_flags: __u32,
}

pub const PDI_RUNNING: __u32 = 0x1;
pub const PDI_AUXDATA: __u32 = 0x2;
pub const PDI_ORIGDEV: __u32 = 0x4;
pub const PDI_VNETHDR: __u32 = 0x8;
pub const PDI_LOSS: __u32 = 0x10;

#[repr(C)]
pub struct packet_diag_mclist {
    pub pdmc_index: __u32,
    pub pdmc_count: __u32,
    pub pdmc_type: __u16,
    pub pdmc_alen: __u16,
    pub pdmc_addr: [__u8; 32], // MAX_ADDR_LEN
}

#[repr(C)]
pub struct packet_diag_ring {
    pub pdr_block_size: __u32,
    pub pdr_block_nr: __u32,
    pub pdr_frame_size: __u32,
    pub pdr_frame_nr: __u32,
    pub pdr_retire_tmo: __u32,
    pub pdr_sizeof_priv: __u32,
    pub pdr_features: __u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
