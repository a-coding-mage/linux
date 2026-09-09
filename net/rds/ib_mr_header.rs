/*
 * Copyright (c) 2016 Oracle.  All rights reserved.
 *
 * This software is available to you under a choice of one of two
 * licenses.  You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the
 * OpenIB.org BSD license below:
 *
 *     Redistribution and use in source and binary forms, with or
 *     without modification, are permitted provided that the following
 *     conditions are met:
 *
 *      - Redistributions of source code must retain the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer.
 *
 *      - Redistributions in binary form must reproduce the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer in the documentation and/or other materials
 *        provided with the distribution.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

// Dependencies supplied by the surrounding kernel/RDS translation.

pub const RDS_MR_1M_POOL_SIZE: usize = 8192 / 2;
pub const RDS_MR_1M_MSG_SIZE: usize = 256;
pub const RDS_MR_8K_MSG_SIZE: usize = 2;
pub const RDS_MR_8K_SCALE: usize = 256 / (RDS_MR_8K_MSG_SIZE + 1);
pub const RDS_MR_8K_POOL_SIZE: usize = RDS_MR_8K_SCALE * (8192 / 2);

#[repr(C)]
#[derive Copy, Clone, PartialEq, Eq)]
pub enum rds_ib_fr_state {
    FRMR_IS_FREE,  /* mr invalidated & ready for use */
    FRMR_IS_INUSE, /* mr is in use or used & can be invalidated */
    FRMR_IS_STALE, /* Stale MR and needs to be dropped  */
}

#[repr(C)]
pub struct rds_ib_frmr {
    pub mr: *mut ib_mr,
    pub fr_state: rds_ib_fr_state,
    pub fr_inv: bool,
    pub fr_inv_done: wait_queue_head_t,
    pub fr_reg: bool,
    pub fr_reg_done: wait_queue_head_t,
    pub fr_wr: ib_send_wr,
    pub dma_npages: u32,
    pub sg_byte_len: u32,
}

/* This is stored as mr->r_trans_private. */
#[repr(C)]
pub struct rds_ib_mr {
    pub work: delayed_work,
    pub device: *mut rds_ib_device,
    pub pool: *mut rds_ib_mr_pool,
    pub ic: *mut rds_ib_connection,
    pub llnode: llist_node,

    /* unmap_list is for freeing */
    pub unmap_list: list_head,
    pub remap_count: u32,
    pub sg: *mut scatterlist,
    pub sg_len: u32,
    pub sg_dma_len: i32,
    pub odp: u8,
    pub u: rds_ib_mr_u,
}

#[repr(C)]
pub union rds_ib_mr_u {
    pub frmr: ::core::mem::ManuallyDrop<rds_ib_frmr>,
    pub mr: *mut ib_mr,
}

/* Our own little MR pool */
#[repr(C)]
pub struct rds_ib_mr_pool {
    pub pool_type: u32,
    pub flush_lock: mutex,       /* serialize fmr invalidate */
    pub flush_worker: delayed_work, /* flush worker */
    pub item_count: atomic_t,    /* total # of MRs */
    pub dirty_count: atomic_t,   /* # dirty of MRs */
    pub drop_list: llist_head,   /* MRs not reached max_maps */
    pub free_list: llist_head,   /* unused MRs */
    pub clean_list: llist_head,  /* unused & unmapped MRs */
    pub flush_wait: wait_queue_head_t,
    pub clean_lock: spinlock_t,  /* "clean_list" concurrency */
    pub free_pinned: atomic_t,   /* memory pinned by free MRs */
    pub max_items: ::core::ffi::c_ulong,
    pub max_items_soft: ::core::ffi::c_ulong,
    pub max_free_pinned: ::core::ffi::c_ulong,
    pub max_pages: u32,
}

extern "C" {
    pub static mut rds_ib_mr_wq: *mut workqueue_struct;

    pub fn rds_ib_create_mr_pool(rds_dev: *mut rds_ib_device, npages: i32) -> *mut rds_ib_mr_pool;
    pub fn rds_ib_get_mr_info(rds_ibdev: *mut rds_ib_device, iinfo: *mut rds_info_rdma_connection);
    pub fn rds6_ib_get_mr_info(rds_ibdev: *mut rds_ib_device, iinfo6: *mut rds6_info_rdma_connection);
    pub fn rds_ib_destroy_mr_pool(pool: *mut rds_ib_mr_pool);
    pub fn rds_ib_get_mr(sg: *mut scatterlist, nents: ::core::ffi::c_ulong, rs: *mut rds_sock,
                         key_ret: *mut u32, conn: *mut rds_connection, start: u64, length: u64,
                         need_odp: i32) -> *mut ::core::ffi::c_void;
    pub fn rds_ib_sync_mr(trans_private: *mut ::core::ffi::c_void, dir: i32);
    pub fn rds_ib_free_mr(trans_private: *mut ::core::ffi::c_void, invalidate: i32);
    pub fn rds_ib_flush_mrs();
    pub fn rds_ib_mr_init() -> i32;
    pub fn rds_ib_mr_exit();
    pub fn rds_ib_get_lkey(trans_private: *mut ::core::ffi::c_void) -> u32;
    pub fn __rds_ib_teardown_mr(mr: *mut rds_ib_mr);
    pub fn rds_ib_teardown_mr(mr: *mut rds_ib_mr);
    pub fn rds_ib_reuse_mr(pool: *mut rds_ib_mr_pool) -> *mut rds_ib_mr;
    pub fn rds_ib_flush_mr_pool(pool: *mut rds_ib_mr_pool, arg: i32, mr: *mut *mut rds_ib_mr) -> i32;
    pub fn rds_ib_try_reuse_ibmr(pool: *mut rds_ib_mr_pool) -> *mut rds_ib_mr;
    pub fn rds_ib_reg_frmr(rds_ibdev: *mut rds_ib_device, ic: *mut rds_ib_connection,
                           sg: *mut scatterlist, nents: ::core::ffi::c_ulong, key: *mut u32) -> *mut rds_ib_mr;
    pub fn rds_ib_unreg_frmr(list: *mut list_head, nfreed: *mut u32,
                             unpinned: *mut ::core::ffi::c_ulong, goal: u32);
    pub fn rds_ib_free_frmr_list(mr: *mut rds_ib_mr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
