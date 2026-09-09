/*
 * Copyright (c) 2013-2015, Mellanox Technologies. All rights reserved.
 *
 * This software is available under a choice of one of two licenses: GPLv2
 * or the OpenIB.org BSD license. It is provided "AS IS", without warranty.
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct mlx5_core_cq {
    pub cqn: u32,
    pub cqe_sz: i32,
    pub set_ci_db: *mut __be32,
    pub arm_db: *mut __be32,
    pub refcount: refcount_t,
    pub free: completion,
    pub vector: u32,
    pub irqn: u32,
    pub comp: Option<unsafe extern "C" fn(*mut mlx5_core_cq, *mut mlx5_eqe)>,
    pub event: Option<unsafe extern "C" fn(*mut mlx5_core_cq, mlx5_event)>,
    pub cons_index: u32,
    pub arm_sn: u32,
    pub dbg: *mut mlx5_rsc_debug,
    pub pid: i32,
    pub tasklet_ctx: mlx5_cq_tasklet_ctx,
    pub reset_notify_added: i32,
    pub reset_notify: list_head,
    pub eq: *mut mlx5_eq_comp,
    pub uid: u16,
}

#[repr(C)]
pub struct mlx5_cq_tasklet_ctx {
    pub list: list_head,
    pub comp: Option<unsafe extern "C" fn(*mut mlx5_core_cq, *mut mlx5_eqe)>,
    pub priv_: *mut core::ffi::c_void,
}

pub const MLX5_CQE_SYNDROME_LOCAL_LENGTH_ERR: i32 = 0x01;
pub const MLX5_CQE_SYNDROME_LOCAL_QP_OP_ERR: i32 = 0x02;
pub const MLX5_CQE_SYNDROME_LOCAL_PROT_ERR: i32 = 0x04;
pub const MLX5_CQE_SYNDROME_WR_FLUSH_ERR: i32 = 0x05;
pub const MLX5_CQE_SYNDROME_MW_BIND_ERR: i32 = 0x06;
pub const MLX5_CQE_SYNDROME_BAD_RESP_ERR: i32 = 0x10;
pub const MLX5_CQE_SYNDROME_LOCAL_ACCESS_ERR: i32 = 0x11;
pub const MLX5_CQE_SYNDROME_REMOTE_INVAL_REQ_ERR: i32 = 0x12;
pub const MLX5_CQE_SYNDROME_REMOTE_ACCESS_ERR: i32 = 0x13;
pub const MLX5_CQE_SYNDROME_REMOTE_OP_ERR: i32 = 0x14;
pub const MLX5_CQE_SYNDROME_TRANSPORT_RETRY_EXC_ERR: i32 = 0x15;
pub const MLX5_CQE_SYNDROME_RNR_RETRY_EXC_ERR: i32 = 0x16;
pub const MLX5_CQE_SYNDROME_REMOTE_ABORTED_ERR: i32 = 0x22;

pub const MLX5_CQE_OWNER_MASK: i32 = 1;
pub const MLX5_CQE_REQ: i32 = 0;
pub const MLX5_CQE_RESP_WR_IMM: i32 = 1;
pub const MLX5_CQE_RESP_SEND: i32 = 2;
pub const MLX5_CQE_RESP_SEND_IMM: i32 = 3;
pub const MLX5_CQE_RESP_SEND_INV: i32 = 4;
pub const MLX5_CQE_RESIZE_CQ: i32 = 5;
pub const MLX5_CQE_SIG_ERR: i32 = 12;
pub const MLX5_CQE_REQ_ERR: i32 = 13;
pub const MLX5_CQE_RESP_ERR: i32 = 14;
pub const MLX5_CQE_INVALID: i32 = 15;

pub const MLX5_CQ_MODIFY_PERIOD: u32 = 1 << 0;
pub const MLX5_CQ_MODIFY_COUNT: u32 = 1 << 1;
pub const MLX5_CQ_MODIFY_OVERRUN: u32 = 1 << 2;
pub const MLX5_CQ_MODIFY_PERIOD_MODE: u32 = 1 << 4;

pub const MLX5_CQ_OPMOD_RESIZE: i32 = 1;
pub const MLX5_MODIFY_CQ_MASK_LOG_SIZE: i32 = 1 << 0;
pub const MLX5_MODIFY_CQ_MASK_PG_OFFSET: i32 = 1 << 1;
pub const MLX5_MODIFY_CQ_MASK_PG_SIZE: i32 = 1 << 2;

#[repr(C)]
pub struct mlx5_cq_modify_params {
    pub type_: i32,
    pub params: mlx5_cq_modify_params_union,
}

#[repr(C)]
pub union mlx5_cq_modify_params_union {
    pub resize: mlx5_cq_resize_params,
    pub moder: mlx5_cq_empty_params,
    pub mapping: mlx5_cq_empty_params,
}

#[repr(C)]
pub struct mlx5_cq_resize_params {
    pub page_offset: u32,
    pub log_cq_size: u8,
}

#[repr(C)]
pub struct mlx5_cq_empty_params {}

pub const CQE_STRIDE_64: i32 = 0;
pub const CQE_STRIDE_128: i32 = 1;
pub const CQE_STRIDE_128_PAD: i32 = 2;

// These depend on the bitfield definitions of cqc in the driver headers.
pub const MLX5_MAX_CQ_PERIOD: u32 = (1u32 << __mlx5_bit_sz_cqc_cq_period()) - 1;
pub const MLX5_MAX_CQ_COUNT: u32 = (1u32 << __mlx5_bit_sz_cqc_cq_max_count()) - 1;

#[inline]
pub unsafe fn cqe_sz_to_mlx_sz(size: u8, padding_128_en: i32) -> i32 {
    if padding_128_en != 0 {
        CQE_STRIDE_128_PAD
    } else if size == 64 {
        CQE_STRIDE_64
    } else {
        CQE_STRIDE_128
    }
}

#[inline]
pub unsafe fn mlx5_cq_set_ci(cq: *mut mlx5_core_cq) {
    *(*cq).set_ci_db = cpu_to_be32((*cq).cons_index & 0xffffff);
}

pub const MLX5_CQ_DB_REQ_NOT_SOL: u32 = 1 << 24;
pub const MLX5_CQ_DB_REQ_NOT: u32 = 0 << 24;

#[inline]
pub unsafe fn mlx5_cq_arm(cq: *mut mlx5_core_cq, cmd: u32, uar_page: *mut core::ffi::c_void, cons_index: u32) {
    let mut doorbell: [__be32; 2] = [0; 2];
    let sn = (*cq).arm_sn & 3;
    let ci = cons_index & 0xffffff;
    *(*cq).arm_db = cpu_to_be32((sn << 28) | cmd | ci);
    wmb();
    doorbell[0] = cpu_to_be32((sn << 28) | cmd | ci);
    doorbell[1] = cpu_to_be32((*cq).cqn);
    mlx5_write64(doorbell.as_mut_ptr(), (uar_page as *mut u8).add(MLX5_CQ_DOORBELL as usize) as *mut core::ffi::c_void);
}

#[inline]
pub unsafe fn mlx5_cq_hold(cq: *mut mlx5_core_cq) { refcount_inc(&mut (*cq).refcount); }

#[inline]
pub unsafe fn mlx5_cq_put(cq: *mut mlx5_core_cq) {
    if refcount_dec_and_test(&mut (*cq).refcount) { complete(&mut (*cq).free); }
}

extern "C" {
    pub fn mlx5_add_cq_to_tasklet(cq: *mut mlx5_core_cq, eqe: *mut mlx5_eqe);
    pub fn mlx5_create_cq(dev: *mut mlx5_core_dev, cq: *mut mlx5_core_cq, input: *mut u32, inlen: i32, output: *mut u32, outlen: i32) -> i32;
    pub fn mlx5_core_create_cq(dev: *mut mlx5_core_dev, cq: *mut mlx5_core_cq, input: *mut u32, inlen: i32, output: *mut u32, outlen: i32) -> i32;
    pub fn mlx5_core_destroy_cq(dev: *mut mlx5_core_dev, cq: *mut mlx5_core_cq) -> i32;
    pub fn mlx5_core_query_cq(dev: *mut mlx5_core_dev, cq: *mut mlx5_core_cq, output: *mut u32) -> i32;
    pub fn mlx5_core_modify_cq(dev: *mut mlx5_core_dev, cq: *mut mlx5_core_cq, input: *mut u32, inlen: i32) -> i32;
    pub fn mlx5_core_modify_cq_moderation(dev: *mut mlx5_core_dev, cq: *mut mlx5_core_cq, cq_period: u16, cq_max_count: u16) -> i32;
    pub fn mlx5_debug_cq_add(dev: *mut mlx5_core_dev, cq: *mut mlx5_core_cq) -> i32;
    pub fn mlx5_debug_cq_remove(dev: *mut mlx5_core_dev, cq: *mut mlx5_core_cq);
}

#[inline]
pub unsafe fn mlx5_dump_err_cqe(dev: *mut mlx5_core_dev, err_cqe: *mut mlx5_err_cqe) {
    print_hex_dump(KERN_WARNING, "", DUMP_PREFIX_OFFSET, 16, 1, err_cqe as *const core::ffi::c_void, core::mem::size_of::<mlx5_err_cqe>(), false);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
