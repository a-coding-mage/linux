// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of sched/ext/sub.c.  Kernel types, constants, helpers and
// macros referenced here are supplied by the surrounding sched_ext sources.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)] pub struct scx_sched { _private: [u8; 0] }
#[repr(C)] pub struct scx_pshard { _private: [u8; 0] }
#[repr(C)] pub struct scx_cmask { _private: [u8; 0] }
#[repr(C)] pub struct scx_cmask_ref { _private: [u8; 0] }
#[repr(C)] pub struct scx_sched_pcpu { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct rq { _private: [u8; 0] }
#[repr(C)] pub struct cgroup { _private: [u8; 0] }
#[repr(C)] pub struct seq_buf { _private: [u8; 0] }
#[repr(C)] pub struct bpf_prog_aux { _private: [u8; 0] }
#[repr(C)] pub struct cgroup_task_migrate_ctx { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }
#[repr(C)] pub struct kthread_work { _private: [u8; 0] }

// Rescue parameters are latched at root enable, before rescue execution.
static mut scx_rescue_bw_1024: i32 = 0;
static mut scx_rescue_quantum_ns: i64 = 0;
static mut scx_rescue_sat_delta_ns: i64 = 0;
static mut scx_rescue_decay_halflife: usize = 0;
static mut scx_rescue_overload_after: usize = 0;

#[cfg(feature = "CONFIG_EXT_SUB_SCHED")]
extern "C" {
    pub fn scx_skip_subtree_pre(pos: *mut scx_sched, root: *mut scx_sched) -> *mut scx_sched;
    pub fn scx_next_descendant_pre(pos: *mut scx_sched, root: *mut scx_sched) -> *mut scx_sched;
    pub fn scx_set_task_sched(p: *mut task_struct, sch: *mut scx_sched);
    pub fn sch_cgroup(sch: *mut scx_sched) -> *mut cgroup;
    pub fn set_cgroup_sched(cgrp: *mut cgroup, sch: *mut scx_sched);
    pub fn scx_free_pshards(sch: *mut scx_sched);
    pub fn scx_alloc_pshards(sch: *mut scx_sched) -> i32;
    pub fn scx_init_root_caps(sch: *mut scx_sched);
    pub fn scx_rescue_charge(rq: *mut rq, delta_exec: i64);
    pub fn scx_rescue_end(rq: *mut rq);
    pub fn scx_rescue_keep(rq: *mut rq, p: *mut task_struct) -> bool;
    pub fn scx_rescue_flush(rq: *mut rq);
    pub fn scx_rescue_dump(s: *mut seq_buf, rq: *mut rq);
    pub fn scx_rescue_set_knobs(sch: *mut scx_sched);
    pub fn scx_rescue_init(rq: *mut rq);
    pub fn scx_resolve_local_dsq(sch: *mut scx_sched, rq: *mut rq, p: *mut task_struct, enq_flags: *mut u64) -> *mut c_void;
    pub fn scx_task_reenq_on_cap_revoke(rq: *mut rq, p: *mut task_struct) -> bool;
    pub fn scx_reenq_reject(rq: *mut rq);
    pub fn scx_process_sync_ecaps(rq: *mut rq, prev: *mut task_struct);
    pub fn scx_unbypass_replay_ecaps(rq: *mut rq, sch: *mut scx_sched);
    pub fn scx_online_ecaps(rq: *mut rq);
    pub fn scx_offline_ecaps(rq: *mut rq);
    pub fn scx_discard_ecaps_to_sync(cpu: i32, pcpu: *mut scx_sched_pcpu);
    pub fn scx_discard_stale_ecaps_syncs();
    pub fn drain_descendants(sch: *mut scx_sched);
    pub fn scx_sub_disable(sch: *mut scx_sched);
    pub fn scx_sub_enable_workfn(work: *mut kthread_work);
}

// cgroup notifier callbacks and BPF kfuncs retain the C ABI and pointer
// semantics. The implementations are provided by the kernel integration unit
// when CONFIG_EXT_SUB_SCHED is enabled.
extern "C" {
    pub fn scx_bpf_sub_dispatch(cgroup_id: u64, aux: *const bpf_prog_aux) -> bool;
    pub fn scx_bpf_sub_grant(cgroup_id: u64, caps: u64, cmask: *const scx_cmask,
                             denied: *mut scx_cmask, aux: *const bpf_prog_aux) -> i32;
    pub fn scx_bpf_sub_revoke(cgroup_id: u64, caps: u64, cmask: *const scx_cmask,
                              aux: *const bpf_prog_aux);
    pub fn scx_bpf_sub_caps(cgroup_id: u64, caps: u64, out: *mut scx_cmask,
                            aux: *const bpf_prog_aux) -> i32;
    pub fn scx_bpf_sub_kill_bstr(cgroup_id: u64, fmt: *mut c_char,
                                 data: *mut u64, data__sz: u32,
                                 aux: *const bpf_prog_aux) -> i32;
    pub fn scx_pstack_recursion_on_dispatch(prog: *mut c_void);
    pub fn scx_pstack_recursion_on_caps_updated(prog: *mut c_void);
}

#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
pub unsafe fn scx_bpf_sub_grant(_: u64, _: u64, _: *const scx_cmask,
                                _: *mut scx_cmask, _: *const bpf_prog_aux) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
pub unsafe fn scx_bpf_sub_revoke(_: u64, _: u64, _: *const scx_cmask, _: *const bpf_prog_aux) {}
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
pub unsafe fn scx_bpf_sub_caps(_: u64, _: u64, _: *mut scx_cmask, _: *const bpf_prog_aux) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
pub unsafe fn scx_bpf_sub_kill_bstr(_: u64, _: *mut c_char, _: *mut u64, _: u32, _: *const bpf_prog_aux) -> i32 { -95 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
