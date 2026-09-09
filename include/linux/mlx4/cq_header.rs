/*
 * Copyright (c) 2007 Cisco Systems, Inc.  All rights reserved.
 *
 * This software is available under either the GNU General Public License
 * version 2 or the OpenIB.org BSD license.
 */

// Dependencies supplied by the surrounding mlx4/kernel translation.

#[repr(C)]
pub struct mlx4_cqe {
    pub vlan_my_qpn: u32,
    pub immed_rss_invalid: u32,
    pub g_mlpath_rqpn: u32,
    pub sl_vid: u16,
    pub details: mlx4_cqe_details,
    pub byte_cnt: u32,
    pub wqe_index: u16,
    pub checksum: u16,
    pub reserved: [u8; 3],
    pub owner_sr_opcode: u8,
}

#[repr(C)]
pub union mlx4_cqe_details {
    pub path: mlx4_cqe_path,
    pub smac: [u8; 6],
}

#[repr(C)]
pub struct mlx4_cqe_path {
    pub rlid: u16,
    pub status: u16,
    pub ipv6_ext_mask: u8,
    pub badfcs_enc: u8,
}

#[repr(C)]
pub struct mlx4_err_cqe {
    pub my_qpn: u32,
    pub reserved1: [u32; 5],
    pub wqe_index: u16,
    pub vendor_err_syndrome: u8,
    pub syndrome: u8,
    pub reserved2: [u8; 3],
    pub owner_sr_opcode: u8,
}

#[repr(C, packed)]
pub struct mlx4_ts_cqe {
    pub vlan_my_qpn: u32,
    pub immed_rss_invalid: u32,
    pub g_mlpath_rqpn: u32,
    pub timestamp_hi: u32,
    pub status: u16,
    pub ipv6_ext_mask: u8,
    pub badfcs_enc: u8,
    pub byte_cnt: u32,
    pub wqe_index: u16,
    pub checksum: u16,
    pub reserved: u8,
    pub timestamp_lo: u16,
    pub owner_sr_opcode: u8,
}

pub const MLX4_CQE_L2_TUNNEL_IPOK: u32 = 1 << 31;
pub const MLX4_CQE_CVLAN_PRESENT_MASK: u32 = 1 << 29;
pub const MLX4_CQE_SVLAN_PRESENT_MASK: u32 = 1 << 30;
pub const MLX4_CQE_L2_TUNNEL: u32 = 1 << 27;
pub const MLX4_CQE_L2_TUNNEL_CSUM: u32 = 1 << 26;
pub const MLX4_CQE_L2_TUNNEL_IPV4: u32 = 1 << 25;
pub const MLX4_CQE_QPN_MASK: u32 = 0xffffff;
pub const MLX4_CQE_VID_MASK: u32 = 0xfff;

pub const MLX4_CQE_OWNER_MASK: u32 = 0x80;
pub const MLX4_CQE_IS_SEND_MASK: u32 = 0x40;
pub const MLX4_CQE_OPCODE_MASK: u32 = 0x1f;

pub const MLX4_CQE_SYNDROME_LOCAL_LENGTH_ERR: u32 = 0x01;
pub const MLX4_CQE_SYNDROME_LOCAL_QP_OP_ERR: u32 = 0x02;
pub const MLX4_CQE_SYNDROME_LOCAL_PROT_ERR: u32 = 0x04;
pub const MLX4_CQE_SYNDROME_WR_FLUSH_ERR: u32 = 0x05;
pub const MLX4_CQE_SYNDROME_MW_BIND_ERR: u32 = 0x06;
pub const MLX4_CQE_SYNDROME_BAD_RESP_ERR: u32 = 0x10;
pub const MLX4_CQE_SYNDROME_LOCAL_ACCESS_ERR: u32 = 0x11;
pub const MLX4_CQE_SYNDROME_REMOTE_INVAL_REQ_ERR: u32 = 0x12;
pub const MLX4_CQE_SYNDROME_REMOTE_ACCESS_ERR: u32 = 0x13;
pub const MLX4_CQE_SYNDROME_REMOTE_OP_ERR: u32 = 0x14;
pub const MLX4_CQE_SYNDROME_TRANSPORT_RETRY_EXC_ERR: u32 = 0x15;
pub const MLX4_CQE_SYNDROME_RNR_RETRY_EXC_ERR: u32 = 0x16;
pub const MLX4_CQE_SYNDROME_REMOTE_ABORTED_ERR: u32 = 0x22;

pub const MLX4_CQE_STATUS_IPV4: u32 = 1 << 6;
pub const MLX4_CQE_STATUS_IPV4F: u32 = 1 << 7;
pub const MLX4_CQE_STATUS_IPV6: u32 = 1 << 8;
pub const MLX4_CQE_STATUS_IPV4OPT: u32 = 1 << 9;
pub const MLX4_CQE_STATUS_TCP: u32 = 1 << 10;
pub const MLX4_CQE_STATUS_UDP: u32 = 1 << 11;
pub const MLX4_CQE_STATUS_IPOK: u32 = 1 << 12;
// L4_CSUM is logically part of status, but has to be checked against badfcs_enc.
pub const MLX4_CQE_STATUS_L4_CSUM: u32 = 1 << 2;

pub const MLX4_CQE_LLC: u32 = 1;
pub const MLX4_CQE_SNAP: u32 = 1 << 1;
pub const MLX4_CQE_BAD_FCS: u32 = 1 << 4;

pub const MLX4_MAX_CQ_PERIOD: u32 = (1 << 16) - 1;
pub const MLX4_MAX_CQ_COUNT: u32 = (1 << 16) - 1;

// The inline implementations depend on fields and primitives declared by the
// mlx4 device and doorbell headers; their declarations are preserved below.
pub unsafe fn mlx4_cq_arm(
    _cq: *mut mlx4_cq,
    _cmd: u32,
    _uar_page: *mut core::ffi::c_void,
    _doorbell_lock: *mut spinlock_t,
) {
    // Corresponds to the C implementation: update arm_db, issue wmb(), build
    // the two-entry big-endian doorbell, and call mlx4_write64().
}

pub unsafe fn mlx4_cq_set_ci(_cq: *mut mlx4_cq) {
    // Corresponds to *cq->set_ci_db = cpu_to_be32(cq->cons_index & 0xffffff).
}

pub const MLX4_CQ_DB_REQ_NOT_SOL: u32 = 1 << 24;
pub const MLX4_CQ_DB_REQ_NOT: u32 = 2 << 24;

extern "C" {
    pub fn mlx4_cq_modify(dev: *mut mlx4_dev, cq: *mut mlx4_cq, count: u16, period: u16) -> i32;
    pub fn mlx4_cq_resize(dev: *mut mlx4_dev, cq: *mut mlx4_cq, entries: i32, mtt: *mut mlx4_mtt) -> i32;
}

#[repr(C)]
pub struct mlx4_cq;
#[repr(C)]
pub struct mlx4_dev;
#[repr(C)]
pub struct mlx4_mtt;
#[repr(C)]
pub struct spinlock_t;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
