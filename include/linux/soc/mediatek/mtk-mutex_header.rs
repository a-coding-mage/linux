/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2015 MediaTek Inc.
 */

// Forward declarations from the C header.
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtk_mutex {
    _private: [u8; 0],
}

// Declared by another header/dependency.
#[repr(C)]
pub enum mtk_ddp_comp_id {}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mtk_mutex_mod_index {
    // MDP table index
    MUTEX_MOD_IDX_MDP_RDMA0,
    MUTEX_MOD_IDX_MDP_RSZ0,
    MUTEX_MOD_IDX_MDP_RSZ1,
    MUTEX_MOD_IDX_MDP_TDSHP0,
    MUTEX_MOD_IDX_MDP_WROT0,
    MUTEX_MOD_IDX_MDP_WDMA,
    MUTEX_MOD_IDX_MDP_AAL0,
    MUTEX_MOD_IDX_MDP_CCORR0,
    MUTEX_MOD_IDX_MDP_HDR0,
    MUTEX_MOD_IDX_MDP_COLOR0,
    MUTEX_MOD_IDX_MDP_RDMA1,
    MUTEX_MOD_IDX_MDP_RDMA2,
    MUTEX_MOD_IDX_MDP_RDMA3,
    MUTEX_MOD_IDX_MDP_STITCH0,
    MUTEX_MOD_IDX_MDP_FG0,
    MUTEX_MOD_IDX_MDP_FG1,
    MUTEX_MOD_IDX_MDP_FG2,
    MUTEX_MOD_IDX_MDP_FG3,
    MUTEX_MOD_IDX_MDP_HDR1,
    MUTEX_MOD_IDX_MDP_HDR2,
    MUTEX_MOD_IDX_MDP_HDR3,
    MUTEX_MOD_IDX_MDP_AAL1,
    MUTEX_MOD_IDX_MDP_AAL2,
    MUTEX_MOD_IDX_MDP_AAL3,
    MUTEX_MOD_IDX_MDP_RSZ2,
    MUTEX_MOD_IDX_MDP_RSZ3,
    MUTEX_MOD_IDX_MDP_MERGE2,
    MUTEX_MOD_IDX_MDP_MERGE3,
    MUTEX_MOD_IDX_MDP_TDSHP1,
    MUTEX_MOD_IDX_MDP_TDSHP2,
    MUTEX_MOD_IDX_MDP_TDSHP3,
    MUTEX_MOD_IDX_MDP_COLOR1,
    MUTEX_MOD_IDX_MDP_COLOR2,
    MUTEX_MOD_IDX_MDP_COLOR3,
    MUTEX_MOD_IDX_MDP_OVL0,
    MUTEX_MOD_IDX_MDP_OVL1,
    MUTEX_MOD_IDX_MDP_PAD0,
    MUTEX_MOD_IDX_MDP_PAD1,
    MUTEX_MOD_IDX_MDP_PAD2,
    MUTEX_MOD_IDX_MDP_PAD3,
    MUTEX_MOD_IDX_MDP_TCC0,
    MUTEX_MOD_IDX_MDP_TCC1,
    MUTEX_MOD_IDX_MDP_WROT1,
    MUTEX_MOD_IDX_MDP_WROT2,
    MUTEX_MOD_IDX_MDP_WROT3,

    MUTEX_MOD_IDX_MAX, // ALWAYS keep at the end
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mtk_mutex_sof_index {
    MUTEX_SOF_IDX_SINGLE_MODE,

    MUTEX_SOF_IDX_MAX, // ALWAYS keep at the end
}

extern "C" {
    pub fn mtk_mutex_get(dev: *mut device) -> *mut mtk_mutex;
    pub fn mtk_mutex_prepare(mutex: *mut mtk_mutex) -> i32;
    pub fn mtk_mutex_add_comp(mutex: *mut mtk_mutex, id: mtk_ddp_comp_id);
    pub fn mtk_mutex_enable(mutex: *mut mtk_mutex);
    pub fn mtk_mutex_enable_by_cmdq(mutex: *mut mtk_mutex, pkt: *mut core::ffi::c_void) -> i32;
    pub fn mtk_mutex_disable(mutex: *mut mtk_mutex);
    pub fn mtk_mutex_remove_comp(mutex: *mut mtk_mutex, id: mtk_ddp_comp_id);
    pub fn mtk_mutex_unprepare(mutex: *mut mtk_mutex);
    pub fn mtk_mutex_put(mutex: *mut mtk_mutex);
    pub fn mtk_mutex_acquire(mutex: *mut mtk_mutex);
    pub fn mtk_mutex_release(mutex: *mut mtk_mutex);
    pub fn mtk_mutex_write_mod(
        mutex: *mut mtk_mutex,
        idx: mtk_mutex_mod_index,
        clear: bool,
    ) -> i32;
    pub fn mtk_mutex_write_sof(
        mutex: *mut mtk_mutex,
        idx: mtk_mutex_sof_index,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
