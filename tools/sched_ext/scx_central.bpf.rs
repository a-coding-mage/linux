// SPDX-License-Identifier: GPL-2.0
/*
 * A central FIFO sched_ext scheduler which demonstrates the following:
 *
 * a. Making all scheduling decisions from one CPU:
 *
 *    The central CPU is the only one making scheduling decisions. All other
 *    CPUs kick the central CPU when they run out of tasks to run.
 *
 *    There is one global BPF queue and the central CPU schedules all CPUs by
 *    dispatching from the global queue to each CPU's local dsq from dispatch().
 *    This isn't the most straightforward. e.g. It'd be easier to bounce
 *    through per-CPU BPF queues. The current design is chosen to maximally
 *    utilize and verify various SCX mechanisms such as LOCAL_ON dispatching.
 *
 * b. Tickless operation
 *
 *    All tasks are dispatched with the infinite slice which allows stopping the
 *    ticks on CONFIG_NO_HZ_FULL kernels running with the proper nohz_full
 *    parameter. The tickless operation can be observed through
 *    /proc/interrupts.
 *
 *    Periodic switching is enforced by a periodic timer checking all CPUs and
 *    preempting them as necessary. Unfortunately, BPF timer currently doesn't
 *    have a way to pin to a specific CPU, so the periodic timer isn't pinned to
 *    the central CPU.
 *
 * c. Preemption
 *
 *    Kthreads are unconditionally queued to the head of a matching local dsq
 *    and dispatched with SCX_DSQ_PREEMPT. This ensures that a kthread is always
 *    prioritized over user threads, which is required for ensuring forward
 *    progress as e.g. the periodic timer may run on a ksoftirqd and if the
 *    ksoftirqd gets starved by a user thread, there may not be anything else to
 *    vacate that user thread.
 *
 *    SCX_KICK_PREEMPT is used to trigger scheduling and CPUs to move to the
 *    next tasks.
 *
 * This scheduler is designed to maximize usage of various SCX mechanisms. A
 * more practical implementation would likely put the scheduling loop outside
 * the central CPU's dispatch() path and add some form of priority mechanism.
 *
 * Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2022 Tejun Heo <tj@kernel.org>
 * Copyright (c) 2022 David Vernet <dvernet@meta.com>
 */

// Depends on definitions normally provided by <scx/common.bpf.h>.

pub type s32 = i32;
pub type u32 = u32;
pub type u64 = u64;

pub const FALLBACK_DSQ_ID: u64 = 0;
pub const MS_TO_NS: u64 = 1000u64 * 1000;
pub const TIMER_INTERVAL_NS: u64 = 1 * MS_TO_NS;

extern "C" {
    static mut central_cpu: s32;
    static mut nr_cpu_ids: u32;
    static mut slice_ns: u64;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut timer_pinned: bool = true;
#[no_mangle]
pub static mut timer_started: bool = false;
#[no_mangle]
pub static mut nr_total: u64 = 0;
#[no_mangle]
pub static mut nr_locals: u64 = 0;
#[no_mangle]
pub static mut nr_queued: u64 = 0;
#[no_mangle]
pub static mut nr_lost_pids: u64 = 0;
#[no_mangle]
pub static mut nr_timers: u64 = 0;
#[no_mangle]
pub static mut nr_dispatches: u64 = 0;
#[no_mangle]
pub static mut nr_mismatches: u64 = 0;
#[no_mangle]
pub static mut nr_retries: u64 = 0;
#[no_mangle]
pub static mut nr_overflows: u64 = 0;

// UEI_DEFINE(uei);
extern "C" {
    static mut uei: u8;
}

#[repr(C)]
pub struct bpf_timer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub pid: s32,
    pub flags: u32,
    pub nr_cpus_allowed: u32,
    pub cpus_ptr: *const cpumask,
}

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scx_exit_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct central_timer {
    pub timer: bpf_timer,
}

// BPF map definitions translated from the anonymous SEC(".maps") structs.
#[repr(C)]
pub struct central_q_map {
    _private: [u8; 0],
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut central_q: central_q_map = central_q_map { _private: [] };

// can't use percpu map due to bad lookups
extern "C" {
    static mut cpu_gimme_task: bool;
    static mut cpu_started_at: u64;
}

#[repr(C)]
pub struct central_timer_map {
    _private: [u8; 0],
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut central_timer: central_timer_map = central_timer_map { _private: [] };

extern "C" {
    static PF_KTHREAD: u32;
    static SCX_DSQ_LOCAL: u64;
    static SCX_DSQ_LOCAL_ON: u64;
    static SCX_SLICE_INF: u64;
    static SCX_ENQ_PREEMPT: u64;
    static SCX_KICK_PREEMPT: u64;
    static SCX_KICK_IDLE: u64;
    static BPF_MAX_LOOPS: s32;
    static BPF_F_TIMER_CPU_PIN: u64;
    static EINVAL: s32;
    static ESRCH: s32;
    static CLOCK_MONOTONIC: s32;
    static SCX_OPS_ENQ_LAST: u64;

    fn bpf_map_push_elem(map: *mut central_q_map, value: *const s32, flags: u64) -> s32;
    fn bpf_map_pop_elem(map: *mut central_q_map, value: *mut s32) -> s32;
    fn bpf_map_lookup_elem(map: *mut central_timer_map, key: *const u32) -> *mut bpf_timer;
    fn bpf_task_from_pid(pid: s32) -> *mut task_struct;
    fn bpf_task_release(p: *mut task_struct);
    fn bpf_cpumask_test_cpu(cpu: s32, cpumask: *const cpumask) -> bool;
    fn bpf_get_smp_processor_id() -> s32;
    fn bpf_timer_start(timer: *mut bpf_timer, nsecs: u64, flags: u64) -> s32;
    fn bpf_timer_init(timer: *mut bpf_timer, map: *mut central_timer_map, clockid: s32) -> s32;
    fn bpf_timer_set_callback(
        timer: *mut bpf_timer,
        callback: unsafe extern "C" fn(*mut core::ffi::c_void, *mut s32, *mut bpf_timer) -> s32,
    ) -> s32;
    fn scx_bpf_dsq_insert(p: *mut task_struct, dsq_id: u64, slice: u64, enq_flags: u64);
    fn scx_bpf_task_running(p: *mut task_struct) -> bool;
    fn scx_bpf_kick_cpu(cpu: s32, flags: u64);
    fn scx_bpf_task_cpu(p: *mut task_struct) -> s32;
    fn scx_bpf_dispatch_nr_slots() -> u32;
    fn scx_bpf_dsq_move_to_local(dsq_id: u64, flags: u64) -> bool;
    fn scx_bpf_now() -> u64;
    fn scx_bpf_dsq_nr_queued(dsq_id: u64) -> u64;
    fn scx_bpf_create_dsq(dsq_id: u64, node: s32) -> s32;
    fn is_migration_disabled(p: *mut task_struct) -> bool;
    fn time_before(a: u64, b: u64) -> bool;
    fn scx_bpf_error(fmt: *const u8, ...);
    fn UEI_RECORD(uei: *mut u8, ei: *mut scx_exit_info);
}

unsafe fn array_elem_ptr<T>(base: *mut T, idx: s32, nr: u32) -> *mut T {
    if idx < 0 || idx as u32 >= nr {
        core::ptr::null_mut()
    } else {
        base.add(idx as usize)
    }
}

unsafe fn sync_fetch_and_add(ptr: *mut u64, val: u64) -> u64 {
    let old = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, old.wrapping_add(val));
    old
}

unsafe fn sync_fetch_and_sub(ptr: *mut u64, val: u64) -> u64 {
    let old = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, old.wrapping_sub(val));
    old
}

#[no_mangle]
pub unsafe extern "C" fn central_select_cpu(
    _p: *mut task_struct,
    _prev_cpu: s32,
    _wake_flags: u64,
) -> s32 {
    /*
     * Steer wakeups to the central CPU as much as possible to avoid
     * disturbing other CPUs. It's safe to blindly return the central cpu as
     * select_cpu() is a hint and if @p can't be on it, the kernel will
     * automatically pick a fallback CPU.
     */
    central_cpu
}

#[no_mangle]
pub unsafe extern "C" fn central_enqueue(p: *mut task_struct, enq_flags: u64) {
    let pid: s32 = (*p).pid;

    sync_fetch_and_add(&mut nr_total, 1);

    /*
     * Push per-cpu kthreads at the head of local dsq's and preempt the
     * corresponding CPU. This ensures that e.g. ksoftirqd isn't blocked
     * behind other threads which is necessary for forward progress
     * guarantee as we depend on the BPF timer which may run from ksoftirqd.
     */
    if ((*p).flags & PF_KTHREAD) != 0 && (*p).nr_cpus_allowed == 1 {
        sync_fetch_and_add(&mut nr_locals, 1);
        scx_bpf_dsq_insert(
            p,
            SCX_DSQ_LOCAL,
            SCX_SLICE_INF,
            enq_flags | SCX_ENQ_PREEMPT,
        );
        return;
    }

    if bpf_map_push_elem(&mut central_q, &pid, 0) != 0 {
        sync_fetch_and_add(&mut nr_overflows, 1);
        scx_bpf_dsq_insert(p, FALLBACK_DSQ_ID, SCX_SLICE_INF, enq_flags);
        return;
    }

    sync_fetch_and_add(&mut nr_queued, 1);

    if !scx_bpf_task_running(p) {
        scx_bpf_kick_cpu(central_cpu, SCX_KICK_PREEMPT);
    }
}

unsafe fn dispatch_to_cpu(cpu: s32) -> bool {
    let mut p: *mut task_struct;
    let mut pid: s32 = 0;
    let mut __i: s32 = 0;

    while __i < BPF_MAX_LOOPS {
        if bpf_map_pop_elem(&mut central_q, &mut pid) != 0 {
            break;
        }

        sync_fetch_and_sub(&mut nr_queued, 1);

        p = bpf_task_from_pid(pid);
        if p.is_null() {
            sync_fetch_and_add(&mut nr_lost_pids, 1);
            __i += 1;
            continue;
        }

        /*
         * If we can't run the task at the top for whatever reason,
         * bounce it to the fallback dsq. Also check
         * is_migration_disabled() explicitly as p->cpus_ptr may not
         * reflect the migration-disabled state yet if
         * migrate_disable_switch() hasn't run.
         */
        if !bpf_cpumask_test_cpu(cpu, (*p).cpus_ptr)
            || (is_migration_disabled(p) && scx_bpf_task_cpu(p) != cpu)
        {
            sync_fetch_and_add(&mut nr_mismatches, 1);
            scx_bpf_dsq_insert(p, FALLBACK_DSQ_ID, SCX_SLICE_INF, 0);
            bpf_task_release(p);
            /*
             * We might run out of dispatch buffer slots if we continue dispatching
             * to the fallback DSQ, without dispatching to the local DSQ of the
             * target CPU. In such a case, break the loop now as will fail the
             * next dispatch operation.
             */
            if scx_bpf_dispatch_nr_slots() == 0 {
                break;
            }
            __i += 1;
            continue;
        }

        /* dispatch to local and mark that @cpu doesn't need more */
        scx_bpf_dsq_insert(p, SCX_DSQ_LOCAL_ON | cpu as u64, SCX_SLICE_INF, 0);

        if cpu != central_cpu {
            scx_bpf_kick_cpu(cpu, SCX_KICK_IDLE);
        }

        bpf_task_release(p);
        return true;
    }

    false
}

unsafe fn start_central_timer() {
    let mut timer: *mut bpf_timer;
    let key: u32 = 0;
    let mut ret: s32;

    if timer_started {
        return;
    }

    timer = bpf_map_lookup_elem(&mut central_timer, &key);
    if timer.is_null() {
        scx_bpf_error(c"failed to lookup central timer".as_ptr() as *const u8);
        return;
    }

    ret = bpf_timer_start(timer, TIMER_INTERVAL_NS, BPF_F_TIMER_CPU_PIN);
    /*
     * BPF_F_TIMER_CPU_PIN is pretty new (>=6.7). If we're running in a
     * kernel which doesn't have it, bpf_timer_start() will return -EINVAL.
     * Retry without the PIN. This would be the perfect use case for
     * bpf_core_enum_value_exists() but the enum type doesn't have a name
     * and can't be used with bpf_core_enum_value_exists(). Oh well...
     */
    if ret == -EINVAL {
        timer_pinned = false;
        ret = bpf_timer_start(timer, TIMER_INTERVAL_NS, 0);
    }

    if ret != 0 {
        scx_bpf_error(c"bpf_timer_start failed (%d)".as_ptr() as *const u8, ret);
        return;
    }

    timer_started = true;
}

#[no_mangle]
pub unsafe extern "C" fn central_dispatch(cpu: s32, _prev: *mut task_struct) {
    if cpu == central_cpu {
        start_central_timer();

        /* dispatch for all other CPUs first */
        sync_fetch_and_add(&mut nr_dispatches, 1);

        let mut cpu_iter: s32 = 0;
        while cpu_iter < nr_cpu_ids as s32 {
            let gimme: *mut bool;

            if scx_bpf_dispatch_nr_slots() == 0 {
                break;
            }

            /* central's gimme is never set */
            gimme = array_elem_ptr(&mut cpu_gimme_task, cpu_iter, nr_cpu_ids);
            if gimme.is_null() || !*gimme {
                cpu_iter += 1;
                continue;
            }

            if dispatch_to_cpu(cpu_iter) {
                *gimme = false;
            }

            cpu_iter += 1;
        }

        /*
         * Retry if we ran out of dispatch buffer slots as we might have
         * skipped some CPUs and also need to dispatch for self. The ext
         * core automatically retries if the local dsq is empty but we
         * can't rely on that as we're dispatching for other CPUs too.
         * Kick self explicitly to retry.
         */
        if scx_bpf_dispatch_nr_slots() == 0 {
            sync_fetch_and_add(&mut nr_retries, 1);
            scx_bpf_kick_cpu(central_cpu, SCX_KICK_PREEMPT);
            return;
        }

        /* look for a task to run on the central CPU */
        if scx_bpf_dsq_move_to_local(FALLBACK_DSQ_ID, 0) {
            return;
        }
        dispatch_to_cpu(central_cpu);
    } else {
        let gimme: *mut bool;

        if scx_bpf_dsq_move_to_local(FALLBACK_DSQ_ID, 0) {
            return;
        }

        gimme = array_elem_ptr(&mut cpu_gimme_task, cpu, nr_cpu_ids);
        if !gimme.is_null() {
            *gimme = true;
        }

        /*
         * Force dispatch on the scheduling CPU so that it finds a task
         * to run for us.
         */
        scx_bpf_kick_cpu(central_cpu, SCX_KICK_PREEMPT);
    }
}

#[no_mangle]
pub unsafe extern "C" fn central_running(p: *mut task_struct) {
    let cpu: s32 = scx_bpf_task_cpu(p);
    let started_at: *mut u64 = array_elem_ptr(&mut cpu_started_at, cpu, nr_cpu_ids);
    if !started_at.is_null() {
        let now = scx_bpf_now();
        *started_at = if now != 0 { now } else { 1 }; /* 0 indicates idle */
    }
}

#[no_mangle]
pub unsafe extern "C" fn central_stopping(p: *mut task_struct, _runnable: bool) {
    let cpu: s32 = scx_bpf_task_cpu(p);
    let started_at: *mut u64 = array_elem_ptr(&mut cpu_started_at, cpu, nr_cpu_ids);
    if !started_at.is_null() {
        *started_at = 0;
    }
}

unsafe extern "C" fn central_timerfn(
    _map: *mut core::ffi::c_void,
    _key: *mut s32,
    timer: *mut bpf_timer,
) -> s32 {
    let now: u64 = scx_bpf_now();
    let mut nr_to_kick: u64 = nr_queued;
    let mut curr_cpu: s32;

    curr_cpu = bpf_get_smp_processor_id();
    if timer_pinned && curr_cpu != central_cpu {
        scx_bpf_error(
            c"Central timer ran on CPU %d, not central CPU %d".as_ptr() as *const u8,
            curr_cpu,
            central_cpu,
        );
        return 0;
    }

    let mut i: s32 = 0;
    while i < nr_cpu_ids as s32 {
        let cpu: s32 = ((nr_timers + i as u64) % nr_cpu_ids as u64) as s32;
        let started_at: *mut u64;

        if cpu == central_cpu {
            i += 1;
            continue;
        }

        /* kick iff the current one exhausted its slice */
        started_at = array_elem_ptr(&mut cpu_started_at, cpu, nr_cpu_ids);
        if !started_at.is_null()
            && *started_at != 0
            && time_before(now, (*started_at).wrapping_add(slice_ns))
        {
            i += 1;
            continue;
        }

        /* and there's something pending */
        if scx_bpf_dsq_nr_queued(FALLBACK_DSQ_ID) != 0
            || scx_bpf_dsq_nr_queued(SCX_DSQ_LOCAL_ON | cpu as u64) != 0
        {
        } else if nr_to_kick != 0 {
            nr_to_kick = nr_to_kick.wrapping_sub(1);
        } else {
            i += 1;
            continue;
        }

        scx_bpf_kick_cpu(cpu, SCX_KICK_PREEMPT);
        i += 1;
    }

    bpf_timer_start(timer, TIMER_INTERVAL_NS, BPF_F_TIMER_CPU_PIN);
    sync_fetch_and_add(&mut nr_timers, 1);
    0
}

#[no_mangle]
pub unsafe extern "C" fn central_init() -> s32 {
    let key: u32 = 0;
    let timer: *mut bpf_timer;
    let mut ret: s32;

    ret = scx_bpf_create_dsq(FALLBACK_DSQ_ID, -1);
    if ret != 0 {
        scx_bpf_error(c"scx_bpf_create_dsq failed (%d)".as_ptr() as *const u8, ret);
        return ret;
    }

    timer = bpf_map_lookup_elem(&mut central_timer, &key);
    if timer.is_null() {
        return -ESRCH;
    }

    bpf_timer_init(timer, &mut central_timer, CLOCK_MONOTONIC);
    bpf_timer_set_callback(timer, central_timerfn);

    scx_bpf_kick_cpu(central_cpu, 0);

    0
}

#[no_mangle]
pub unsafe extern "C" fn central_exit(ei: *mut scx_exit_info) {
    UEI_RECORD(&mut uei, ei);
}

#[repr(C)]
pub struct scx_ops {
    pub flags: u64,
    pub select_cpu: *mut core::ffi::c_void,
    pub enqueue: *mut core::ffi::c_void,
    pub dispatch: *mut core::ffi::c_void,
    pub running: *mut core::ffi::c_void,
    pub stopping: *mut core::ffi::c_void,
    pub init: *mut core::ffi::c_void,
    pub exit: *mut core::ffi::c_void,
    pub name: *const u8,
}

#[no_mangle]
pub static mut central_ops: scx_ops = scx_ops {
    /*
     * We are offloading all scheduling decisions to the central CPU
     * and thus being the last task on a given CPU doesn't mean
     * anything special. Enqueue the last tasks like any other tasks.
     */
    flags: unsafe { SCX_OPS_ENQ_LAST },
    select_cpu: central_select_cpu as *mut core::ffi::c_void,
    enqueue: central_enqueue as *mut core::ffi::c_void,
    dispatch: central_dispatch as *mut core::ffi::c_void,
    running: central_running as *mut core::ffi::c_void,
    stopping: central_stopping as *mut core::ffi::c_void,
    init: central_init as *mut core::ffi::c_void,
    exit: central_exit as *mut core::ffi::c_void,
    name: c"central".as_ptr() as *const u8,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
