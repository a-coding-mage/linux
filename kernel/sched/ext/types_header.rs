/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Early sched_ext type definitions.
 *
 * Copyright (c) 2026 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2026 Tejun Heo <tj@kernel.org>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external: linux types, jiffies, overflow, time64, and sched topology.

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum scx_consts {
    SCX_DSP_DFL_MAX_BATCH = 32,
    SCX_DSP_MAX_LOOPS = 32,
    SCX_WATCHDOG_MAX_TIMEOUT = 30 * HZ,

    /* rescue knob defaults and limits, see scx_rescue_timerfn() */
    SCX_RESCUE_DFL_BW_PPT = 20, /* parts per thousand, 2% */
    SCX_RESCUE_MAX_BW_PPT = 250, /* 25% */
    SCX_RESCUE_DISABLE = U32_MAX, /* disables rescue */
    SCX_RESCUE_DFL_QUANTUM_US = 5000,
    SCX_RESCUE_MIN_QUANTUM_US = 1000,
    SCX_RESCUE_MAX_QUANTUM_US = 100000,
    SCX_RESCUE_MIN_SLICE_US = 1000, /* floor of the divided slice */
    SCX_RESCUE_OVERLOAD_MULT = 16, /* overload threshold in funding periods */
    SCX_RESCUE_MIN_OVERLOAD_MS = 1000,
    SCX_RESCUE_MAX_OVERLOAD_MS = 15000,

    /* per-CPU chunk size for p->scx.tid allocation, see scx_alloc_tid() */
    SCX_TID_CHUNK = 1024,

    SCX_EXIT_BT_LEN = 64,
    SCX_EXIT_MSG_LEN = 1024,
    SCX_EXIT_DUMP_DFL_LEN = 32768,

    SCX_CPUPERF_ONE = SCHED_CAPACITY_SCALE,

    /*
     * Iterating all tasks may take a while. Periodically drop
     * scx_tasks_lock to avoid causing e.g. CSD and RCU stalls.
     */
    SCX_TASK_ITER_BATCH = 32,

    SCX_BYPASS_HOST_NTH = 2,

    SCX_BYPASS_LB_DFL_INTV_US = 500 * USEC_PER_MSEC,
    SCX_BYPASS_LB_DONOR_PCT = 125,
    SCX_BYPASS_LB_MIN_DELTA_DIV = 4,
    SCX_BYPASS_LB_BATCH = 256,

    SCX_REENQ_MAX_REPEAT = 256,

    SCX_SUB_MAX_DEPTH = 4,
}

/* Per-cid topology information. */
#[repr(C)]
pub struct scx_cid_topo {
    pub core_cid: i32,
    pub core_idx: i32,
    pub llc_cid: i32,
    pub llc_idx: i32,
    pub node_cid: i32,
    pub node_idx: i32,
    pub shard_cid: i32,
    pub shard_idx: i32,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum scx_cid_consts {
    SCX_CID_SHARD_SIZE_DFL = 24,
    SCX_CID_SHARD_MAX_CPUS = 512,
}

/* Per-shard metadata for O(1) shard->cid-range lookup. */
#[repr(C)]
pub struct scx_cid_shard {
    pub base_cid: i32,
    pub nr_cids: i32,
}

/*
 * cmask: variable-length, base-windowed bitmap over cid space.
 * The flexible array is aligned to the global 64-cid grid; head and tail
 * padding remain zero under the cmask helpers.
 */
#[repr(C)]
pub struct scx_cmask {
    pub base: u32,
    pub nr_cids: u32,
    pub alloc_words: u32,
    pub bits: [u64; 0],
}

/* Number of u64 words of bits[] storage covering nr_cids regardless of base. */
#[inline]
pub const fn scx_cmask_nr_words(nr_cids: u32) -> u32 {
    (((nr_cids as u64) + 63) / 64 + 1) as u32
}

/* Rust equivalent of the on-stack flexible-array cmask definitions. */
#[macro_export]
macro_rules! __SCX_CMASK_DEFINE {
    ($name:ident, $base:expr, $nr_cids:expr, $alloc_cids:expr) => {
        let mut $name = {
            let words = $crate::scx_cmask_nr_words($alloc_cids);
            let mut storage = vec![0u64; words as usize];
            storage.shrink_to_fit();
            ($base, $nr_cids, words, storage)
        };
    };
}

#[macro_export]
macro_rules! SCX_CMASK_DEFINE {
    ($name:ident, $base:expr, $nr_cids:expr) => {
        $crate::__SCX_CMASK_DEFINE!($name, $base, $nr_cids, $nr_cids);
    };
}

#[macro_export]
macro_rules! SCX_CMASK_DEFINE_SHARD {
    ($name:ident, $base:expr, $nr_cids:expr) => {
        $crate::__SCX_CMASK_DEFINE!($name, $base, $nr_cids, SCX_CID_SHARD_MAX_CPUS);
    };
}

#[repr(C)]
pub struct scx_cmask_ref {
    pub sch: *mut scx_sched,
    pub src: *mut scx_cmask,
    pub base: u32,
    pub nr_cids: u32,
    pub shard_first: i32,
    pub shard_end: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
