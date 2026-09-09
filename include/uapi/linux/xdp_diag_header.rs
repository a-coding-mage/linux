/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * xdp_diag: interface for query/monitor XDP sockets
 * Copyright(c) 2019 Intel Corporation.
 */

// Dependency intent: the C header includes <linux/types.h>.

#[repr(C)]
pub struct xdp_diag_req {
    pub sdiag_family: u8,
    pub sdiag_protocol: u8,
    pub pad: u16,
    pub xdiag_ino: u32,
    pub xdiag_show: u32,
    pub xdiag_cookie: [u32; 2],
}

#[repr(C)]
pub struct xdp_diag_msg {
    pub xdiag_family: u8,
    pub xdiag_type: u8,
    pub pad: u16,
    pub xdiag_ino: u32,
    pub xdiag_cookie: [u32; 2],
}

pub const XDP_SHOW_INFO: i32 = 1 << 0; // Basic information
pub const XDP_SHOW_RING_CFG: i32 = 1 << 1;
pub const XDP_SHOW_UMEM: i32 = 1 << 2;
pub const XDP_SHOW_MEMINFO: i32 = 1 << 3;
pub const XDP_SHOW_STATS: i32 = 1 << 4;

pub const XDP_DIAG_NONE: i32 = 0;
pub const XDP_DIAG_INFO: i32 = 1;
pub const XDP_DIAG_UID: i32 = 2;
pub const XDP_DIAG_RX_RING: i32 = 3;
pub const XDP_DIAG_TX_RING: i32 = 4;
pub const XDP_DIAG_UMEM: i32 = 5;
pub const XDP_DIAG_UMEM_FILL_RING: i32 = 6;
pub const XDP_DIAG_UMEM_COMPLETION_RING: i32 = 7;
pub const XDP_DIAG_MEMINFO: i32 = 8;
pub const XDP_DIAG_STATS: i32 = 9;
pub const __XDP_DIAG_MAX: i32 = 10;

pub const XDP_DIAG_MAX: i32 = __XDP_DIAG_MAX - 1;

#[repr(C)]
pub struct xdp_diag_info {
    pub ifindex: u32,
    pub queue_id: u32,
}

#[repr(C)]
pub struct xdp_diag_ring {
    pub entries: u32, // num descs
}

pub const XDP_DU_F_ZEROCOPY: i32 = 1 << 0;

#[repr(C)]
pub struct xdp_diag_umem {
    pub size: u64,
    pub id: u32,
    pub num_pages: u32,
    pub chunk_size: u32,
    pub headroom: u32,
    pub ifindex: u32,
    pub queue_id: u32,
    pub flags: u32,
    pub refs: u32,
}

#[repr(C)]
pub struct xdp_diag_stats {
    pub n_rx_dropped: u64,
    pub n_rx_invalid: u64,
    pub n_rx_full: u64,
    pub n_fill_ring_empty: u64,
    pub n_tx_invalid: u64,
    pub n_tx_ring_empty: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
