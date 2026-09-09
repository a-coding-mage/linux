/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB) */
/*
 * Copyright (c) 2007 Cisco Systems, Inc. All rights reserved.
 * Copyright (c) 2007, 2008 Mellanox Technologies. All rights reserved.
 *
 * This software is available to you under a choice of one of two
 * licenses. You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the
 * OpenIB.org BSD license.
 */

/* Dependency equivalent of <linux/types.h>. */

/* Increment this value if any changes that break userspace ABI compatibility are made. */
pub const MLX4_IB_UVERBS_NO_DEV_CAPS_ABI_VERSION: u32 = 3;
pub const MLX4_IB_UVERBS_ABI_VERSION: u32 = 4;

/*
 * These structures are kept free of pointer types so that their layout is
 * identical on 32-bit and 64-bit architectures.
 */

#[repr(C)]
pub struct mlx4_ib_alloc_ucontext_resp_v3 {
    pub qp_tab_size: u32,
    pub bf_reg_size: u16,
    pub bf_regs_per_page: u16,
}

pub const MLX4_USER_DEV_CAP_LARGE_CQE: u32 = 1u32 << 0;

#[repr(C)]
pub struct mlx4_ib_alloc_ucontext_resp {
    pub dev_caps: u32,
    pub qp_tab_size: u32,
    pub bf_reg_size: u16,
    pub bf_regs_per_page: u16,
    pub cqe_size: u32,
}

#[repr(C)]
pub struct mlx4_ib_alloc_pd_resp {
    pub pdn: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct mlx4_ib_create_cq {
    pub buf_addr: u64,
    pub db_addr: u64,
}

#[repr(C)]
pub struct mlx4_ib_create_cq_resp {
    pub cqn: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct mlx4_ib_resize_cq {
    pub buf_addr: u64,
}

#[repr(C)]
pub struct mlx4_ib_create_srq {
    pub buf_addr: u64,
    pub db_addr: u64,
}

#[repr(C)]
pub struct mlx4_ib_create_srq_resp {
    pub srqn: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct mlx4_ib_create_qp_rss {
    pub rx_hash_fields_mask: u64, /* Use enum mlx4_ib_rx_hash_fields */
    pub rx_hash_function: u8, /* Use enum mlx4_ib_rx_hash_function_flags */
    pub reserved: [u8; 7],
    pub rx_hash_key: [u8; 40],
    pub comp_mask: u32,
    pub reserved1: u32,
}

#[repr(C)]
pub struct mlx4_ib_create_qp {
    pub buf_addr: u64,
    pub db_addr: u64,
    pub log_sq_bb_count: u8,
    pub log_sq_stride: u8,
    pub sq_no_prefetch: u8,
    pub reserved: u8,
    pub inl_recv_sz: u32,
}

#[repr(C)]
pub struct mlx4_ib_create_wq {
    pub buf_addr: u64,
    pub db_addr: u64,
    pub log_range_size: u8,
    pub reserved: [u8; 3],
    pub comp_mask: u32,
}

#[repr(C)]
pub struct mlx4_ib_modify_wq {
    pub comp_mask: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct mlx4_ib_create_rwq_ind_tbl_resp {
    pub response_length: u32,
    pub reserved: u32,
}

/* RX Hash function flags */
pub const MLX4_IB_RX_HASH_FUNC_TOEPLITZ: u32 = 1u32 << 0;

/*
 * RX Hash flags. Each flag represents a packet field that participates in
 * RX Hash calculation when set.
 */
pub const MLX4_IB_RX_HASH_SRC_IPV4: u32 = 1u32 << 0;
pub const MLX4_IB_RX_HASH_DST_IPV4: u32 = 1u32 << 1;
pub const MLX4_IB_RX_HASH_SRC_IPV6: u32 = 1u32 << 2;
pub const MLX4_IB_RX_HASH_DST_IPV6: u32 = 1u32 << 3;
pub const MLX4_IB_RX_HASH_SRC_PORT_TCP: u32 = 1u32 << 4;
pub const MLX4_IB_RX_HASH_DST_PORT_TCP: u32 = 1u32 << 5;
pub const MLX4_IB_RX_HASH_SRC_PORT_UDP: u32 = 1u32 << 6;
pub const MLX4_IB_RX_HASH_DST_PORT_UDP: u32 = 1u32 << 7;
pub const MLX4_IB_RX_HASH_INNER: u64 = 1u64 << 31;

#[repr(C)]
pub struct mlx4_ib_rss_caps {
    pub rx_hash_fields_mask: u64, /* enum mlx4_ib_rx_hash_fields */
    pub rx_hash_function: u8, /* enum mlx4_ib_rx_hash_function_flags */
    pub reserved: [u8; 7],
}

pub const MLX4_IB_QUERY_DEV_RESP_MASK_CORE_CLOCK_OFFSET: u32 = 1u32 << 0;

#[repr(C)]
pub struct mlx4_ib_tso_caps {
    pub max_tso: u32, /* Maximum tso payload size in bytes */
    /* Corresponding bit will be set if qp type from enum ib_qp_type is supported. */
    pub supported_qpts: u32,
}

#[repr(C)]
pub struct mlx4_uverbs_ex_query_device_resp {
    pub comp_mask: u32,
    pub response_length: u32,
    pub hca_core_clock_offset: u64,
    pub max_inl_recv_sz: u32,
    pub reserved: u32,
    pub rss_caps: mlx4_ib_rss_caps,
    pub tso_caps: mlx4_ib_tso_caps,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
