// SPDX-License-Identifier: GPL-2.0
/*
 * A CPU0 scheduler.
 *
 * This scheduler queues all tasks to a shared DSQ and only dispatches them on
 * CPU0 in FIFO order. This is useful for testing bypass behavior when many
 * tasks are concentrated on a single CPU. If the load balancer doesn't work,
 * bypass mode can trigger task hangs or RCU stalls as the queue is long and
 * there's only one CPU working on it.
 *
 * - Statistics tracking how many tasks are queued to local and CPU0 DSQs.
 * - Termination notification for userspace.
 *
 * Copyright (c) 2025 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2025 Tejun Heo <tj@kernel.org>
 */

// C dependency intent: #include <scx/common.bpf.h>

pub type u32 = ::core::ffi::c_uint;
pub type u64 = ::core::ffi::c_ulonglong;
pub type s32 = ::core::ffi::c_int;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scx_exit_info {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut stats: StatsMap;

    fn bpf_map_lookup_elem(map: *mut StatsMap, key: *const u32) -> *mut u64;
    fn scx_bpf_task_cpu(p: *mut task_struct) -> s32;
    fn scx_bpf_dsq_insert(p: *mut task_struct, dsq_id: u64, slice: u64, enq_flags: u64);
    fn scx_bpf_dsq_move_to_local(dsq_id: u64, flags: u64);
    fn scx_bpf_create_dsq(dsq_id: u64, node: s32) -> s32;
    fn scx_bpf_error(fmt: *const ::core::ffi::c_char, ...);
    fn UEI_RECORD(uei: *mut Uei, ei: *mut scx_exit_info);
}

// char _license[] SEC("license") = "GPL";
#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// UEI_DEFINE(uei);
#[repr(C)]
pub struct Uei {
    _private: [u8; 0],
}

#[unsafe(no_mangle)]
pub static mut uei: Uei = Uei { _private: [] };

/*
 * We create a custom DSQ with ID 0 that we dispatch to and consume from on
 * CPU0.
 */
pub const DSQ_CPU0: u64 = 0;

pub const BPF_MAP_TYPE_PERCPU_ARRAY: u32 = 0;
pub const SCX_DSQ_LOCAL: u64 = 0;
pub const SCX_SLICE_DFL: u64 = 0;

#[repr(C)]
pub struct StatsMap {
    _private: [u8; 0],
}

/*
 * Original BPF map declaration:
 *
 * struct {
 *      __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
 *      __uint(key_size, sizeof(u32));
 *      __uint(value_size, sizeof(u64));
 *      __uint(max_entries, 2);                 // [local, cpu0]
 * } stats SEC(".maps");
 */

unsafe fn stat_inc(idx: u32) {
    let cnt_p: *mut u64 = unsafe { bpf_map_lookup_elem(&raw mut stats, &idx) };
    if !cnt_p.is_null() {
        unsafe {
            *cnt_p = (*cnt_p).wrapping_add(1);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu0_select_cpu(
    _p: *mut task_struct,
    _prev_cpu: s32,
    _wake_flags: u64,
) -> s32 {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu0_enqueue(p: *mut task_struct, enq_flags: u64) {
    /*
     * select_cpu() always picks CPU0. If @p is not on CPU0, it can't run on
     * CPU 0. Queue on whichever CPU it's currently only.
     */
    if unsafe { scx_bpf_task_cpu(p) } != 0 {
        unsafe {
            stat_inc(0); // count local queueing
            scx_bpf_dsq_insert(p, SCX_DSQ_LOCAL, SCX_SLICE_DFL, 0);
        }
        return;
    }

    unsafe {
        stat_inc(1); // count cpu0 queueing
        scx_bpf_dsq_insert(p, DSQ_CPU0, SCX_SLICE_DFL, enq_flags);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu0_dispatch(cpu: s32, _prev: *mut task_struct) {
    if cpu == 0 {
        unsafe {
            scx_bpf_dsq_move_to_local(DSQ_CPU0, 0);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu0_init() -> s32 {
    let ret: s32;

    ret = unsafe { scx_bpf_create_dsq(DSQ_CPU0, -1) };
    if ret != 0 {
        unsafe {
            scx_bpf_error(
                c"failed to create DSQ %d (%d)".as_ptr(),
                DSQ_CPU0 as ::core::ffi::c_int,
                ret,
            );
        }
        return ret;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu0_exit(ei: *mut scx_exit_info) {
    unsafe {
        UEI_RECORD(&raw mut uei, ei);
    }
}

/*
 * SCX_OPS_DEFINE(cpu0_ops,
 *        .select_cpu              = (void *)cpu0_select_cpu,
 *        .enqueue                 = (void *)cpu0_enqueue,
 *        .dispatch                = (void *)cpu0_dispatch,
 *        .init                    = (void *)cpu0_init,
 *        .exit                    = (void *)cpu0_exit,
 *        .name                    = "cpu0");
 */
#[repr(C)]
pub struct ScxOps {
    pub select_cpu: *mut ::core::ffi::c_void,
    pub enqueue: *mut ::core::ffi::c_void,
    pub dispatch: *mut ::core::ffi::c_void,
    pub init: *mut ::core::ffi::c_void,
    pub exit: *mut ::core::ffi::c_void,
    pub name: *const ::core::ffi::c_char,
}

#[unsafe(no_mangle)]
pub static mut cpu0_ops: ScxOps = ScxOps {
    select_cpu: cpu0_select_cpu as *mut ::core::ffi::c_void,
    enqueue: cpu0_enqueue as *mut ::core::ffi::c_void,
    dispatch: cpu0_dispatch as *mut ::core::ffi::c_void,
    init: cpu0_init as *mut ::core::ffi::c_void,
    exit: cpu0_exit as *mut ::core::ffi::c_void,
    name: c"cpu0".as_ptr(),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
