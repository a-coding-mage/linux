/* SPDX-License-Identifier: GPL-2.0 */
/*
 * A simple scheduler.
 *
 * By default, it operates as a simple global weighted vtime scheduler and can
 * be switched to FIFO scheduling. It also demonstrates the following niceties.
 *
 * - Statistics tracking how many tasks are queued to local and global dsq's.
 * - Termination notification for userspace.
 *
 * While very simple, this scheduler should work reasonably well on CPUs with a
 * uniform L3 cache topology. While preemption is not implemented, the fact that
 * the scheduling queue is shared across all CPUs means that whatever is at the
 * front of the queue is likely to be executed fairly quickly given enough
 * number of CPUs. The FIFO scheduling mode may be beneficial to some workloads
 * but comes with the usual problems with FIFO scheduling where saturating
 * threads can easily drown out interactive ones.
 *
 * Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2022 Tejun Heo <tj@kernel.org>
 * Copyright (c) 2022 David Vernet <dvernet@meta.com>
 */

// Rust translation of dependency intent from: #include <scx/common.bpf.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

type bool_ = bool;
type s32 = i32;
type u32 = u32;
type u64 = u64;

#[used]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

extern "C" {
    static fifo_sched: bool_;
    static SCX_DSQ_LOCAL: u64;
    static SCX_SLICE_DFL: u64;

    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void)
        -> *mut core::ffi::c_void;
    fn scx_bpf_select_cpu_dfl(
        p: *mut task_struct,
        prev_cpu: s32,
        wake_flags: u64,
        is_idle: *mut bool_,
    ) -> s32;
    fn scx_bpf_dsq_insert(p: *mut task_struct, dsq_id: u64, slice: u64, enq_flags: u64);
    fn scx_bpf_dsq_insert_vtime(
        p: *mut task_struct,
        dsq_id: u64,
        slice: u64,
        vtime: u64,
        enq_flags: u64,
    );
    fn scx_bpf_dsq_move_to_local(dsq_id: u64, enq_flags: u64);
    fn time_before(a: u64, b: u64) -> bool_;
    fn scale_by_task_weight_inverse(p: *mut task_struct, delta: u64) -> u64;
    fn scx_bpf_task_set_dsq_vtime(p: *mut task_struct, vtime: u64);
    fn scx_bpf_create_dsq(dsq_id: u64, node: s32) -> s32;
    fn scx_bpf_error(fmt: *const u8, ...);
    fn UEI_RECORD(uei: *mut uei, ei: *mut scx_exit_info);
}

static mut vtime_now: u64 = 0;

// UEI_DEFINE(uei);
#[repr(C)]
pub struct uei {
    _private: [u8; 0],
}

static mut uei: uei = uei { _private: [] };

/*
 * Built-in DSQs such as SCX_DSQ_GLOBAL cannot be used as priority queues
 * (meaning, cannot be dispatched to with scx_bpf_dsq_insert_vtime()). We
 * therefore create a separate DSQ with ID 0 that we dispatch to and consume
 * from. If scx_simple only supported global FIFO scheduling, then we could just
 * use SCX_DSQ_GLOBAL.
 */
const SHARED_DSQ: u64 = 0;

// Original BPF map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
//     __uint(key_size, sizeof(u32));
//     __uint(value_size, sizeof(u64));
//     __uint(max_entries, 2);          /* [local, global] */
// } stats SEC(".maps");
#[repr(C)]
pub struct stats_map {
    _private: [u8; 0],
}

#[used]
#[link_section = ".maps"]
static mut stats: stats_map = stats_map { _private: [] };

#[repr(C)]
pub struct task_struct {
    pub scx: sched_ext_entity,
}

#[repr(C)]
pub struct sched_ext_entity {
    pub dsq_vtime: u64,
    pub slice: u64,
}

#[repr(C)]
pub struct scx_exit_info {
    _private: [u8; 0],
}

unsafe fn stat_inc(idx: u32) {
    let cnt_p = bpf_map_lookup_elem(
        &mut stats as *mut _ as *mut core::ffi::c_void,
        &idx as *const _ as *const core::ffi::c_void,
    ) as *mut u64;
    if !cnt_p.is_null() {
        *cnt_p = (*cnt_p).wrapping_add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn simple_select_cpu(
    p: *mut task_struct,
    prev_cpu: s32,
    wake_flags: u64,
) -> s32 {
    let mut is_idle: bool_ = false;
    let cpu: s32;

    cpu = scx_bpf_select_cpu_dfl(p, prev_cpu, wake_flags, &mut is_idle);
    if is_idle {
        stat_inc(0); /* count local queueing */
        scx_bpf_dsq_insert(p, SCX_DSQ_LOCAL, SCX_SLICE_DFL, 0);
    }

    cpu
}

#[no_mangle]
pub unsafe extern "C" fn simple_enqueue(p: *mut task_struct, enq_flags: u64) {
    stat_inc(1); /* count global queueing */

    if fifo_sched {
        scx_bpf_dsq_insert(p, SHARED_DSQ, SCX_SLICE_DFL, enq_flags);
    } else {
        let mut vtime: u64 = (*p).scx.dsq_vtime;

        /*
         * Limit the amount of budget that an idling task can accumulate
         * to one slice.
         */
        if time_before(vtime, vtime_now.wrapping_sub(SCX_SLICE_DFL)) {
            vtime = vtime_now.wrapping_sub(SCX_SLICE_DFL);
        }

        scx_bpf_dsq_insert_vtime(p, SHARED_DSQ, SCX_SLICE_DFL, vtime, enq_flags);
    }
}

#[no_mangle]
pub unsafe extern "C" fn simple_dispatch(_cpu: s32, _prev: *mut task_struct) {
    scx_bpf_dsq_move_to_local(SHARED_DSQ, 0);
}

#[no_mangle]
pub unsafe extern "C" fn simple_running(p: *mut task_struct) {
    if fifo_sched {
        return;
    }

    /*
     * Global vtime always progresses forward as tasks start executing. The
     * test and update can be performed concurrently from multiple CPUs and
     * thus racy. Any error should be contained and temporary. Let's just
     * live with it.
     */
    if time_before(vtime_now, (*p).scx.dsq_vtime) {
        vtime_now = (*p).scx.dsq_vtime;
    }
}

#[no_mangle]
pub unsafe extern "C" fn simple_stopping(p: *mut task_struct, runnable: bool_) {
    let _ = runnable;

    if fifo_sched {
        return;
    }

    /*
     * Scale the execution time by the inverse of the weight and charge.
     *
     * Note that the default yield implementation yields by setting
     * @p->scx.slice to zero and the following would treat the yielding task
     * as if it has consumed all its slice. If this penalizes yielding tasks
     * too much, determine the execution time by taking explicit timestamps
     * instead of depending on @p->scx.slice.
     */
    let delta: u64 = scale_by_task_weight_inverse(p, SCX_SLICE_DFL.wrapping_sub((*p).scx.slice));

    scx_bpf_task_set_dsq_vtime(p, (*p).scx.dsq_vtime.wrapping_add(delta));
}

#[no_mangle]
pub unsafe extern "C" fn simple_enable(p: *mut task_struct) {
    scx_bpf_task_set_dsq_vtime(p, vtime_now);
}

#[no_mangle]
pub unsafe extern "C" fn simple_init() -> s32 {
    let ret: s32;

    ret = scx_bpf_create_dsq(SHARED_DSQ, -1);
    if ret != 0 {
        scx_bpf_error(
            b"failed to create DSQ %d (%d)\0".as_ptr(),
            SHARED_DSQ as core::ffi::c_int,
            ret,
        );
        return ret;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn simple_exit(ei: *mut scx_exit_info) {
    UEI_RECORD(&mut uei, ei);
}

// SCX_OPS_DEFINE(simple_ops,
//            .select_cpu      = (void *)simple_select_cpu,
//            .enqueue         = (void *)simple_enqueue,
//            .dispatch        = (void *)simple_dispatch,
//            .running         = (void *)simple_running,
//            .stopping        = (void *)simple_stopping,
//            .enable          = (void *)simple_enable,
//            .init            = (void *)simple_init,
//            .exit            = (void *)simple_exit,
//            .name            = "simple");
#[repr(C)]
pub struct scx_ops {
    pub select_cpu: Option<unsafe extern "C" fn(*mut task_struct, s32, u64) -> s32>,
    pub enqueue: Option<unsafe extern "C" fn(*mut task_struct, u64)>,
    pub dispatch: Option<unsafe extern "C" fn(s32, *mut task_struct)>,
    pub running: Option<unsafe extern "C" fn(*mut task_struct)>,
    pub stopping: Option<unsafe extern "C" fn(*mut task_struct, bool_)>,
    pub enable: Option<unsafe extern "C" fn(*mut task_struct)>,
    pub init: Option<unsafe extern "C" fn() -> s32>,
    pub exit: Option<unsafe extern "C" fn(*mut scx_exit_info)>,
    pub name: *const u8,
}

#[no_mangle]
pub static simple_ops: scx_ops = scx_ops {
    select_cpu: Some(simple_select_cpu),
    enqueue: Some(simple_enqueue),
    dispatch: Some(simple_dispatch),
    running: Some(simple_running),
    stopping: Some(simple_stopping),
    enable: Some(simple_enable),
    init: Some(simple_init),
    exit: Some(simple_exit),
    name: b"simple\0".as_ptr(),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
