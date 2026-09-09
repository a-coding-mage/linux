/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/*
 * Copyright (c) 2017-2018 Mellanox Technologies. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation.

/* Mark entry as containing driver specific details, it is used to provide QP subtype for now */
pub const RESTRACK_DD: u32 = XA_MARK_1;

#[repr(C)]
pub struct ib_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

/**
 * enum rdma_restrack_type - HW objects to track
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_restrack_type {
    /** @RDMA_RESTRACK_PD: Protection domain (PD) */
    RDMA_RESTRACK_PD,
    /** @RDMA_RESTRACK_CQ: Completion queue (CQ) */
    RDMA_RESTRACK_CQ,
    /** @RDMA_RESTRACK_QP: Queue pair (QP) */
    RDMA_RESTRACK_QP,
    /** @RDMA_RESTRACK_CM_ID: Connection Manager ID (CM_ID) */
    RDMA_RESTRACK_CM_ID,
    /** @RDMA_RESTRACK_MR: Memory Region (MR) */
    RDMA_RESTRACK_MR,
    /** @RDMA_RESTRACK_CTX: Verbs contexts (CTX) */
    RDMA_RESTRACK_CTX,
    /** @RDMA_RESTRACK_COUNTER: Statistic Counter */
    RDMA_RESTRACK_COUNTER,
    /** @RDMA_RESTRACK_SRQ: Shared receive queue (SRQ) */
    RDMA_RESTRACK_SRQ,
    /** @RDMA_RESTRACK_DMAH: DMA handle */
    RDMA_RESTRACK_DMAH,
    /** @RDMA_RESTRACK_COMP_CNTR: Completion Counter */
    RDMA_RESTRACK_COMP_CNTR,
    /** @RDMA_RESTRACK_MAX: Last entry, used for array dclarations */
    RDMA_RESTRACK_MAX,
}

/**
 * struct rdma_restrack_entry - metadata per-entry
 */
#[repr(C)]
pub struct rdma_restrack_entry {
    /** @valid: validity indicator */
    pub valid: bool,
    /** @no_track: don't add this entry to restrack DB */
    pub no_track: u8,
    /** @kref: Protect destroy of the resource */
    pub kref: kref,
    /** @comp: Signal that all consumers of resource are completed their work */
    pub comp: completion,
    /** @task: owner of resource tracking entity */
    pub task: *mut task_struct,
    /** @kern_name: name of owner for the kernel created entities. */
    pub kern_name: *const core::ffi::c_char,
    /** @type: various objects in restrack database */
    pub type_: rdma_restrack_type,
    /** @user: user resource */
    pub user: bool,
    /** @id: ID to expose to users */
    pub id: u32,
}

extern "C" {
    pub fn rdma_restrack_count(
        dev: *mut ib_device,
        type_: rdma_restrack_type,
        show_details: bool,
    ) -> u32;
}

/** rdma_is_kernel_res() - check the owner of resource */
#[inline]
pub unsafe fn rdma_is_kernel_res(res: *const rdma_restrack_entry) -> bool {
    !(*res).user
}

extern "C" {
    pub fn rdma_restrack_get(res: *mut rdma_restrack_entry) -> i32;
    pub fn rdma_restrack_put(res: *mut rdma_restrack_entry) -> i32;
    pub fn rdma_nl_put_driver_u32(msg: *mut sk_buff, name: *const core::ffi::c_char, value: u32) -> i32;
    pub fn rdma_nl_put_driver_u32_hex(msg: *mut sk_buff, name: *const core::ffi::c_char, value: u32) -> i32;
    pub fn rdma_nl_put_driver_u64(msg: *mut sk_buff, name: *const core::ffi::c_char, value: u64) -> i32;
    pub fn rdma_nl_put_driver_u64_hex(msg: *mut sk_buff, name: *const core::ffi::c_char, value: u64) -> i32;
    pub fn rdma_nl_put_driver_string(msg: *mut sk_buff, name: *const core::ffi::c_char, str_: *const core::ffi::c_char) -> i32;
    pub fn rdma_nl_stat_hwcounter_entry(msg: *mut sk_buff, name: *const core::ffi::c_char, value: u64) -> i32;
    pub fn rdma_restrack_get_byid(dev: *mut ib_device, type_: rdma_restrack_type, id: u32) -> *mut rdma_restrack_entry;
}

/** rdma_restrack_no_track() - don't add resource to the DB */
#[inline]
pub unsafe fn rdma_restrack_no_track(res: *mut rdma_restrack_entry) {
    (*res).no_track = 1;
}

#[inline]
pub unsafe fn rdma_restrack_is_tracked(res: *mut rdma_restrack_entry) -> bool {
    (*res).no_track == 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
