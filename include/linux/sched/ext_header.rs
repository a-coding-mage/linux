/* SPDX-License-Identifier: GPL-2.0 */
/*
 * BPF extensible scheduler class: Documentation/scheduler/sched-ext.rst
 *
 * Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2022 Tejun Heo <tj@kernel.org>
 * Copyright (c) 2022 David Vernet <dvernet@meta.com>
 */

#[cfg(CONFIG_SCHED_CLASS_EXT)]
pub mod sched_ext {
    #[repr(C)]
    pub enum ScxPublicConsts {
        SCX_OPS_NAME_LEN = 128,
        SCX_SLICE_DFL = 20 * 1_000_000,
        SCX_SLICE_BYPASS = 5 * 1_000_000,
        SCX_SLICE_INF = u64::MAX,
    }

    #[repr(u64)]
    pub enum ScxDsqIdFlags {
        SCX_DSQ_FLAG_BUILTIN = 1u64 << 63,
        SCX_DSQ_FLAG_LOCAL_ON = 1u64 << 62,
        SCX_DSQ_INVALID = (1u64 << 63) | 0,
        SCX_DSQ_GLOBAL = (1u64 << 63) | 1,
        SCX_DSQ_LOCAL = (1u64 << 63) | 2,
        SCX_DSQ_BYPASS = (1u64 << 63) | 3,
        SCX_DSQ_REJECT = (1u64 << 63) | 4,
        SCX_DSQ_RESCUE = (1u64 << 63) | 5,
        SCX_DSQ_LOCAL_ON = (1u64 << 63) | (1u64 << 62),
        SCX_DSQ_LOCAL_CPU_MASK = 0xffff_ffff,
    }

    #[repr(C)]
    pub struct ScxDeferredReenqUser { pub node: ListHead, pub flags: u64 }
    #[repr(C)]
    pub struct ScxDsqPcpu { pub dsq: *mut ScxDispatchQ, pub deferred_reenq_user: ScxDeferredReenqUser }

    #[repr(C)]
    pub struct ScxDispatchQ {
        pub lock: RawSpinlock,
        pub first_task: *mut TaskStruct,
        pub list: ListHead,
        pub priq: RbRoot,
        pub nr: u32,
        pub seq: u32,
        pub id: u64,
        pub hash_node: RhashHead,
        pub free_node: LlistNode,
        pub sched: *mut ScxSched,
        pub pcpu: *mut ScxDsqPcpu,
        pub rcu: RcuHead,
    }

    #[repr(C)]
    pub enum ScxEntFlags {
        SCX_TASK_QUEUED = 1 << 0,
        SCX_TASK_IN_CUSTODY = 1 << 1,
        SCX_TASK_RESET_RUNNABLE_AT = 1 << 2,
        SCX_TASK_DEQD_FOR_SLEEP = 1 << 3,
        SCX_TASK_SUB_INIT = 1 << 4,
        SCX_TASK_IMMED = 1 << 5,
        SCX_TASK_PROTECTED = 1 << 6,
        SCX_TASK_STATE_SHIFT = 8,
        SCX_TASK_STATE_BITS = 3,
        SCX_TASK_STATE_MASK = ((1 << 3) - 1) << 8,
        SCX_TASK_NONE = 0 << 8,
        SCX_TASK_INIT_BEGIN = 1 << 8,
        SCX_TASK_INIT = 2 << 8,
        SCX_TASK_READY = 3 << 8,
        SCX_TASK_ENABLED = 4 << 8,
        SCX_TASK_DEAD = 5 << 8,
        SCX_TASK_REENQ_REASON_SHIFT = 12,
        SCX_TASK_REENQ_REASON_BITS = 3,
        SCX_TASK_REENQ_REASON_MASK = ((1 << 3) - 1) << 12,
        SCX_TASK_REENQ_NONE = 0 << 12,
        SCX_TASK_REENQ_KFUNC = 1 << 12,
        SCX_TASK_REENQ_IMMED = 2 << 12,
        SCX_TASK_REENQ_PREEMPTED = 3 << 12,
        SCX_TASK_REENQ_CAP = 4 << 12,
        SCX_TASK_CURSOR = 1 << 31,
    }

    #[repr(C)] pub enum ScxEntDsqFlags { SCX_TASK_DSQ_ON_PRIQ = 1 << 0 }
    #[repr(C)] pub enum ScxDsqLnodeFlags {
        SCX_DSQ_LNODE_ITER_CURSOR = 1 << 0,
        __SCX_DSQ_LNODE_PRIV_SHIFT = 16,
    }

    #[repr(C)]
    pub struct ScxDsqListNode { pub node: ListHead, pub flags: u32, pub priv_: u32 }

    /* INIT_DSQ_LIST_CURSOR(__cursor, __dsq, __flags) */
    #[inline]
    pub unsafe fn init_dsq_list_cursor(cursor: &mut ScxDsqListNode, dsq: *const ScxDispatchQ, flags: u32) {
        cursor.node = ListHead::init();
        cursor.flags = (1 << 0) | flags;
        cursor.priv_ = core::ptr::read_volatile(&(*dsq).seq);
    }

    #[repr(C)]
    pub struct SchedExtEntity {
        #[cfg(CONFIG_CGROUPS)] pub sched: *mut ScxSched,
        pub dsq: *mut ScxDispatchQ,
        pub ops_state: AtomicLong,
        pub ddsp_dsq_id: u64,
        pub ddsp_enq_flags: u64,
        pub ddsp_slice: u64,
        pub ddsp_vtime: u64,
        pub dsq_list: ScxDsqListNode,
        pub dsq_priq: RbNode,
        pub dsq_seq: u32,
        pub dsq_flags: u32,
        pub flags: u32,
        pub weight: u32,
        pub reenq_cnt: u32,
        pub sticky_cpu: i32,
        pub holding_cpu: i32,
        pub selected_cpu: i32,
        pub runnable_cpu: i32,
        pub kf_tasks: [*mut TaskStruct; 2],
        pub runnable_node: ListHead,
        pub runnable_at: usize,
        #[cfg(CONFIG_EXT_SUB_SCHED)] pub rescue_at: usize,
        pub tid: u64,
        pub tid_hash_node: RhashHead,
        pub slice: u64,
        pub dsq_vtime: u64,
        pub slice_oob: Atomic64,
        pub reenq_reason_caps: u64,
        pub reenq_reason_cid: i32,
        pub disallow: bool,
        #[cfg(CONFIG_EXT_GROUP_SCHED)] pub cgrp_moving_from: *mut Cgroup,
        pub tasks_node: ListHead,
    }

    extern "C" {
        pub fn sched_ext_dead(p: *mut TaskStruct);
        pub fn print_scx_info(log_lvl: *const core::ffi::c_char, p: *mut TaskStruct);
        pub fn scx_softlockup(dur_s: u32);
        pub fn scx_hardlockup(cpu: i32) -> bool;
        pub fn scx_rcu_cpu_stall(stalled_mask: *const Cpumask) -> bool;
    }
}

#[cfg(not(CONFIG_SCHED_CLASS_EXT))]
pub unsafe fn sched_ext_dead(_p: *mut TaskStruct) {}
#[cfg(not(CONFIG_SCHED_CLASS_EXT))]
pub unsafe fn print_scx_info(_log_lvl: *const core::ffi::c_char, _p: *mut TaskStruct) {}
#[cfg(not(CONFIG_SCHED_CLASS_EXT))]
pub unsafe fn scx_softlockup(_dur_s: u32) {}
#[cfg(not(CONFIG_SCHED_CLASS_EXT))]
pub unsafe fn scx_hardlockup(_cpu: i32) -> bool { false }
#[cfg(not(CONFIG_SCHED_CLASS_EXT))]
pub unsafe fn scx_rcu_cpu_stall(_stalled_mask: *const Cpumask) -> bool { false }

#[repr(C)]
pub struct ScxTaskGroup {
    #[cfg(CONFIG_EXT_GROUP_SCHED)] pub sched: *mut ScxSched,
    #[cfg(CONFIG_EXT_GROUP_SCHED)] pub flags: u32,
    #[cfg(CONFIG_EXT_GROUP_SCHED)] pub weight: u32,
    #[cfg(CONFIG_EXT_GROUP_SCHED)] pub bw_period_us: u64,
    #[cfg(CONFIG_EXT_GROUP_SCHED)] pub bw_quota_us: u64,
    #[cfg(CONFIG_EXT_GROUP_SCHED)] pub bw_burst_us: u64,
    #[cfg(CONFIG_EXT_GROUP_SCHED)] pub idle: bool,
}

/* External kernel types supplied by other headers. */
#[allow(non_camel_case_types)] pub enum ListHead {}
#[allow(non_camel_case_types)] pub enum RawSpinlock {}
#[allow(non_camel_case_types)] pub enum RbRoot {}
#[allow(non_camel_case_types)] pub enum RbNode {}
#[allow(non_camel_case_types)] pub enum RhashHead {}
#[allow(non_camel_case_types)] pub enum LlistNode {}
#[allow(non_camel_case_types)] pub enum RcuHead {}
#[allow(non_camel_case_types)] pub enum TaskStruct {}
#[allow(non_camel_case_types)] pub enum ScxSched {}
#[allow(non_camel_case_types)] pub enum AtomicLong {}
#[allow(non_camel_case_types)] pub enum Atomic64 {}
#[allow(non_camel_case_types)] pub enum Cgroup {}
#[allow(non_camel_case_types)] pub enum Cpumask {}

impl ListHead { pub const fn init() -> Self { unsafe { core::mem::zeroed() } } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
