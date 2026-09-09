/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2016 HGST, a Western Digital Company.
 */

// Dependency declarations supplied by <rdma/ib_verbs.h> and related headers
// are intentionally left external to this translation unit.

#[repr(C)]
pub enum ib_mr {}

#[repr(C)]
pub enum ib_qp {}

#[repr(C)]
pub enum list_head {}

#[repr(C)]
pub enum ib_mr_type {}

extern "C" {
    pub fn ib_mr_pool_get(qp: *mut ib_qp, list: *mut list_head) -> *mut ib_mr;

    pub fn ib_mr_pool_put(
        qp: *mut ib_qp,
        list: *mut list_head,
        mr: *mut ib_mr,
    );

    pub fn ib_mr_pool_init(
        qp: *mut ib_qp,
        list: *mut list_head,
        nr: ::core::ffi::c_int,
        type_: ib_mr_type,
        max_num_sg: u32,
        max_num_meta_sg: u32,
    ) -> ::core::ffi::c_int;

    pub fn ib_mr_pool_destroy(qp: *mut ib_qp, list: *mut list_head);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
