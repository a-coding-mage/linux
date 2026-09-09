/* SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) */
/*
 * Copyright (c) 2022, Microsoft Corporation. All rights reserved.
 */

/*
 * Dependencies supplied by the original Linux headers:
 * <linux/types.h>, <rdma/ib_user_ioctl_verbs.h>
 */

/*
 * Increment this value if any changes that break userspace ABI
 * compatibility are made.
 */

pub const MANA_IB_UVERBS_ABI_VERSION: i32 = 1;

#[repr(i32)]
pub enum mana_ib_create_cq_flags {
    /* Reserved for backward compatibility. Legacy
     * kernel versions use it to create CQs in RNIC
     */
    MANA_IB_CREATE_RNIC_CQ = 1 << 0,
}

#[repr(C)]
pub struct mana_ib_create_cq {
    pub buf_addr: u64,
    pub comp_mask: u16,
    pub reserved0: u16,
    pub reserved1: u32,
}

#[repr(C)]
pub struct mana_ib_create_cq_resp {
    pub cqid: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct mana_ib_create_qp {
    pub sq_buf_addr: u64,
    pub sq_buf_size: u32,
    pub port: u32,
}

#[repr(C)]
pub struct mana_ib_create_qp_resp {
    pub sqid: u32,
    pub cqid: u32,
    pub tx_vp_offset: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct mana_ib_create_rc_qp {
    pub queue_buf: [u64; 4],
    pub queue_size: [u32; 4],
}

#[repr(C)]
pub struct mana_ib_create_rc_qp_resp {
    pub queue_id: [u32; 4],
}

#[repr(C)]
pub struct mana_ib_create_uc_qp {
    pub queue_buf: [u64; 3],
    pub queue_size: [u32; 3],
    pub comp_mask: u32,
}

#[repr(C)]
pub struct mana_ib_create_uc_qp_resp {
    pub queue_id: [u32; 3],
    pub reserved: u32,
}

#[repr(C)]
pub struct mana_ib_create_wq {
    pub wq_buf_addr: u64,
    pub wq_buf_size: u32,
    pub reserved: u32,
}

/* RX Hash function flags */
#[repr(i32)]
pub enum mana_ib_rx_hash_function_flags {
    MANA_IB_RX_HASH_FUNC_TOEPLITZ = 1 << 0,
}

#[repr(C)]
pub struct mana_ib_create_qp_rss {
    pub rx_hash_fields_mask: u64,
    pub rx_hash_function: u8,
    pub reserved: [u8; 7],
    pub rx_hash_key_len: u32,
    pub rx_hash_key: [u8; 40],
    pub port: u32,
}

#[repr(C)]
pub struct rss_resp_entry {
    pub cqid: u32,
    pub wqid: u32,
}

#[repr(C)]
pub struct mana_ib_create_qp_rss_resp {
    pub num_entries: u64,
    pub entries: [rss_resp_entry; 64],
}

#[repr(i32)]
pub enum mana_ib_ucontext_support {
    MANA_IB_UCNTX_ALLOC_PDN_SUPPORT = 1 << 0,
}

#[repr(C)]
pub struct mana_ib_alloc_ucontext_resp {
    pub comp_mask: u64,
}

#[repr(i32)]
pub enum mana_ib_create_pd_flags {
    MANA_IB_PD_SHORT_PDN = 1 << 0,
}

#[repr(C)]
pub struct mana_ib_alloc_pd {
    pub comp_mask: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct mana_ib_alloc_pd_resp {
    pub pdn: u32,
    pub reserved: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
