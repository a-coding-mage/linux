// Translated from sched_ext/scx_flatcg.h.
// Original header guard: __SCX_EXAMPLE_FLATCG_H

pub const FCG_HWEIGHT_ONE: u64 = 1u64 << 16;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum fcg_stat_idx {
    FCG_STAT_ACT = 0,
    FCG_STAT_DEACT,
    FCG_STAT_LOCAL,
    FCG_STAT_GLOBAL,

    FCG_STAT_HWT_UPDATES,
    FCG_STAT_HWT_CACHE,
    FCG_STAT_HWT_SKIP,
    FCG_STAT_HWT_RACE,

    FCG_STAT_ENQ_SKIP,
    FCG_STAT_ENQ_RACE,

    FCG_STAT_CNS_KEEP,
    FCG_STAT_CNS_EXPIRE,
    FCG_STAT_CNS_EMPTY,
    FCG_STAT_CNS_GONE,

    FCG_STAT_PNC_NO_CGRP,
    FCG_STAT_PNC_NEXT,
    FCG_STAT_PNC_EMPTY,
    FCG_STAT_PNC_GONE,
    FCG_STAT_PNC_RACE,
    FCG_STAT_PNC_FAIL,

    FCG_STAT_BAD_REMOVAL,

    FCG_NR_STATS,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fcg_cgrp_ctx {
    pub nr_active: u32,
    pub nr_runnable: u32,
    pub queued: u32,
    pub weight: u32,
    pub hweight: u32,
    pub child_weight_sum: u64,
    pub hweight_gen: u64,
    pub cvtime_delta: i64,
    pub tvtime_now: u64,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
