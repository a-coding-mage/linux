/* SPDX-License-Identifier: GPL-2.0 */
/*
 * A demo sched_ext core-scheduler which always makes every sibling CPU pair
 * execute from the same CPU cgroup.
 *
 * This scheduler is a minimal implementation and would need some form of
 * priority handling both inside each cgroup and across the cgroups to be
 * practically useful.
 *
 * Each CPU in the system is paired with exactly one other CPU, according to a
 * "stride" value that can be specified when the BPF scheduler program is first
 * loaded. Throughout the runtime of the scheduler, these CPU pairs guarantee
 * that they will only ever schedule tasks that belong to the same CPU cgroup.
 *
 * Scheduler Initialization
 * ------------------------
 *
 * The scheduler BPF program is first initialized from user space, before it is
 * enabled. During this initialization process, each CPU on the system is
 * assigned several values that are constant throughout its runtime:
 *
 * 1. *Pair CPU*: The CPU that it synchronizes with when making scheduling
 *		  decisions. Paired CPUs always schedule tasks from the same
 *		  CPU cgroup, and synchronize with each other to guarantee
 *		  that this constraint is not violated.
 * 2. *Pair ID*:  Each CPU pair is assigned a Pair ID, which is used to access
 *		  a struct pair_ctx object that is shared between the pair.
 * 3. *In-pair-index*: An index, 0 or 1, that is assigned to each core in the
 *		       pair. Each struct pair_ctx has an active_mask field,
 *		       which is a bitmap used to indicate whether each core
 *		       in the pair currently has an actively running task.
 *		       This index specifies which entry in the bitmap corresponds
 *		       to each CPU in the pair.
 *
 * During this initialization, the CPUs are paired according to a "stride" that
 * may be specified when invoking the user space program that initializes and
 * loads the scheduler. By default, the stride is 1/2 the total number of CPUs.
 *
 * Tasks and cgroups
 * -----------------
 *
 * Every cgroup in the system is registered with the scheduler using the
 * pair_cgroup_init() callback, and every task in the system is associated with
 * exactly one cgroup. At a high level, the idea with the pair scheduler is to
 * always schedule tasks from the same cgroup within a given CPU pair. When a
 * task is enqueued (i.e. passed to the pair_enqueue() callback function), its
 * cgroup ID is read from its task struct, and then a corresponding queue map
 * is used to FIFO-enqueue the task for that cgroup.
 *
 * If you look through the implementation of the scheduler, you'll notice that
 * there is quite a bit of complexity involved with looking up the per-cgroup
 * FIFO queue that we enqueue tasks in. For example, there is a cgrp_q_idx_hash
 * BPF hash map that is used to map a cgroup ID to a globally unique ID that's
 * allocated in the BPF program. This is done because we use separate maps to
 * store the FIFO queue of tasks, and the length of that map, per cgroup. This
 * complexity is only present because of current deficiencies in BPF that will
 * soon be addressed. The main point to keep in mind is that newly enqueued
 * tasks are added to their cgroup's FIFO queue.
 *
 * Dispatching tasks
 * -----------------
 *
 * This section will describe how enqueued tasks are dispatched and scheduled.
 * Tasks are dispatched in pair_dispatch(), and at a high level the workflow is
 * as follows:
 *
 * 1. Fetch the struct pair_ctx for the current CPU. As mentioned above, this is
 *    the structure that's used to synchronize amongst the two pair CPUs in their
 *    scheduling decisions. After any of the following events have occurred:
 *
 * - The cgroup's slice run has expired, or
 * - The cgroup becomes empty, or
 * - Either CPU in the pair is preempted by a higher priority scheduling class
 *
 * The cgroup transitions to the draining state and stops executing new tasks
 * from the cgroup.
 *
 * 2. If the pair is still executing a task, mark the pair_ctx as draining, and
 *    wait for the pair CPU to be preempted.
 *
 * 3. Otherwise, if the pair CPU is not running a task, we can move onto
 *    scheduling new tasks. Pop the next cgroup id from the top_q queue.
 *
 * 4. Pop a task from that cgroup's FIFO task queue, and begin executing it.
 *
 * Note again that this scheduling behavior is simple, but the implementation
 * is complex mostly because this it hits several BPF shortcomings and has to
 * work around in often awkward ways. Most of the shortcomings are expected to
 * be resolved in the near future which should allow greatly simplifying this
 * scheduler.
 *
 * Dealing with preemption
 * -----------------------
 *
 * SCX is the lowest priority sched_class, and could be preempted by them at
 * any time. To address this, the scheduler watches every sched_switch from
 * a tracepoint and edge-detects when a CPU leaves and returns to SCX
 * control.
 *
 * When a higher-priority class takes a CPU away from a running SCX task -
 * a sched_switch from an SCX task to a higher-priority task - we mark the
 * pair_ctx as having been preempted and then invoke:
 *
 * scx_bpf_kick_cpu(pair_cpu, SCX_KICK_PREEMPT | SCX_KICK_WAIT);
 *
 * This preempts the pair CPU, and waits until it has re-entered the scheduler
 * before returning. This is necessary to ensure that the higher priority
 * sched_class that preempted our scheduler does not schedule a task
 * concurrently with our pair CPU.
 *
 * When the CPU returns to SCX or idle, we unmark the preemption in the
 * pair_ctx and send another resched IPI to the pair CPU to re-enable pair
 * scheduling.
 *
 * A switch from idle straight to a higher-priority task is not a release:
 * the CPU was not running an SCX task, so there is nothing to drain and no
 * reason to make the pair wait. Kicking SCX_KICK_WAIT on every such wakeup
 * would stall the pair CPU behind rt bursts it was never coupled to.
 *
 * Note that sched_setscheduler() on a running task changes its class in
 * place without a context switch, so such transitions are only observed at
 * the task's next switch. Until then the stale active_mask bit makes the
 * pair wait in try_dispatch(), which is bounded by that next switch.
 *
 * Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2022 Tejun Heo <tj@kernel.org>
 * Copyright (c) 2022 David Vernet <dvernet@meta.com>
 */

// Dependencies originally supplied by:
// #include <scx/common.bpf.h>
// #include "scx_pair.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_void};
use core::ptr;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

type bool_ = bool;

const MAX_RT_PRIO: i32 = 100;

extern "C" {
    static MAX_CGRPS: i32;
    static MAX_QUEUED: i32;
    static BPF_MAX_LOOPS: i32;
    static BPF_ANY: u64;
    static SCX_DSQ_GLOBAL: u64;
    static SCX_SLICE_DFL: u64;
    static SCX_KICK_PREEMPT: u32;
    static SCX_KICK_WAIT: u32;
    static EINVAL: i32;
    static ENOENT: i32;
    static EAGAIN: i32;
    static EBUSY: i32;
}

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kernfs_node {
    pub id: u64,
}

#[repr(C)]
pub struct cgroup {
    pub kn: *mut kernfs_node,
}

#[repr(C)]
pub struct task_struct {
    pub pid: i32,
    pub prio: i32,
}

#[repr(C)]
pub struct scx_exit_info {
    _private: [u8; 0],
}

extern "C" {
    fn scx_bpf_task_cgroup(p: *mut task_struct) -> *mut cgroup;
    fn bpf_cgroup_release(cgrp: *mut cgroup);
    fn bpf_map_lookup_elem(map: *const c_void, key: *const c_void) -> *mut c_void;
    fn bpf_map_push_elem(map: *mut c_void, value: *const c_void, flags: u64) -> i32;
    fn bpf_map_pop_elem(map: *mut c_void, value: *mut c_void) -> i32;
    fn bpf_map_update_elem(
        map: *mut c_void,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> i32;
    fn bpf_map_delete_elem(map: *mut c_void, key: *const c_void) -> i32;
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn scx_bpf_error(fmt: *const c_char, ...);
    fn scx_bpf_now() -> u64;
    fn scx_bpf_dsq_insert(p: *mut task_struct, dsq_id: u64, slice: u64, enq_flags: u64);
    fn bpf_task_from_pid(pid: i32) -> *mut task_struct;
    fn bpf_task_release(p: *mut task_struct);
    fn scx_bpf_kick_cpu(cpu: i32, flags: u32);
    fn bpf_get_smp_processor_id() -> i32;
    fn UEI_RECORD(uei: *mut c_void, ei: *mut scx_exit_info);
}

unsafe fn ARRAY_ELEM_PTR<T>(base: *const T, idx: i32, max: u32) -> *const T {
    if idx < 0 || idx as u32 >= max {
        ptr::null()
    } else {
        base.add(idx as usize)
    }
}

unsafe fn MEMBER_VPTR<T>(base: *mut T, idx: i32) -> *mut T {
    if idx < 0 || idx >= MAX_CGRPS {
        ptr::null_mut()
    } else {
        base.add(idx as usize)
    }
}

fn time_before(a: u64, b: u64) -> bool {
    (a.wrapping_sub(b) as i64) < 0
}

unsafe fn sync_fetch_and_add_u64(p: *mut u64, v: u64) -> u64 {
    (*(p as *mut AtomicU64)).fetch_add(v, Ordering::SeqCst)
}

unsafe fn sync_fetch_and_add_u32(p: *mut u32, v: u32) -> u32 {
    (*(p as *mut AtomicU32)).fetch_add(v, Ordering::SeqCst)
}

unsafe fn sync_val_compare_and_swap_u64(p: *mut u64, old: u64, new: u64) -> u64 {
    match (*(p as *mut AtomicU64)).compare_exchange(old, new, Ordering::SeqCst, Ordering::SeqCst) {
        Ok(v) | Err(v) => v,
    }
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

/* !0 for veristat, set during init */
#[no_mangle]
pub static nr_cpu_ids: u32 = 1;

/* a pair of CPUs stay on a cgroup for this duration */
#[no_mangle]
pub static pair_batch_dur_ns: u32 = 0;

/* cpu ID -> pair cpu ID */
#[no_mangle]
pub static pair_cpu: [i32; 0] = [];

/* cpu ID -> pair_id */
#[no_mangle]
pub static pair_id: [u32; 0] = [];

/* CPU ID -> CPU # in the pair (0 or 1) */
#[no_mangle]
pub static in_pair_idx: [u32; 0] = [];

#[repr(C)]
pub struct pair_ctx {
    pub lock: bpf_spin_lock,

    /* the cgroup the pair is currently executing */
    pub cgid: u64,

    /* the pair started executing the current cgroup at */
    pub started_at: u64,

    /* whether the current cgroup is draining */
    pub draining: bool_,

    /* the CPUs that are currently active on the cgroup */
    pub active_mask: u32,

    /*
     * the CPUs that are currently preempted and running tasks in a
     * different scheduler.
     */
    pub preempted_mask: u32,
}

// BPF map definitions translated from SEC(".maps") anonymous structs.
#[no_mangle]
#[link_section = ".maps"]
pub static mut pair_ctx: *mut c_void = ptr::null_mut();

/* queue of cgrp_q's possibly with tasks on them */
#[no_mangle]
#[link_section = ".maps"]
pub static mut top_q: *mut c_void = ptr::null_mut();

/* per-cgroup q which FIFOs the tasks from the cgroup */
#[repr(C)]
pub struct cgrp_q {
    _private: [u8; 0],
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut cgrp_q_arr: *mut c_void = ptr::null_mut();

#[no_mangle]
pub static mut cgrp_q_len: [u64; MAX_CGRPS as usize] = [0; MAX_CGRPS as usize];

/*
 * This and cgrp_q_idx_hash combine into a poor man's IDR. This likely would be
 * useful to have as a map type.
 */
#[no_mangle]
pub static mut cgrp_q_idx_cursor: u32 = 0;
#[no_mangle]
pub static mut cgrp_q_idx_busy: [u64; MAX_CGRPS as usize] = [0; MAX_CGRPS as usize];

/*
 * All added up, the following is what we do:
 *
 * 1. When a cgroup is enabled, RR cgroup_q_idx_busy array doing cmpxchg looking
 *    for a free ID. If not found, fail cgroup creation with -EBUSY.
 *
 * 2. Hash the cgroup ID to the allocated cgrp_q_idx in the following
 *    cgrp_q_idx_hash.
 *
 * 3. Whenever a cgrp_q needs to be accessed, first look up the cgrp_q_idx from
 *    cgrp_q_idx_hash and then access the corresponding entry in cgrp_q_arr.
 *
 * This is sadly complicated for something pretty simple. Hopefully, we should
 * be able to simplify in the future.
 */
#[no_mangle]
#[link_section = ".maps"]
pub static mut cgrp_q_idx_hash: *mut c_void = ptr::null_mut();

/* statistics */
#[no_mangle]
pub static mut nr_total: u64 = 0;
#[no_mangle]
pub static mut nr_dispatched: u64 = 0;
#[no_mangle]
pub static mut nr_missing: u64 = 0;
#[no_mangle]
pub static mut nr_kicks: u64 = 0;
#[no_mangle]
pub static mut nr_preemptions: u64 = 0;
#[no_mangle]
pub static mut nr_exps: u64 = 0;
#[no_mangle]
pub static mut nr_exp_waits: u64 = 0;
#[no_mangle]
pub static mut nr_exp_empty: u64 = 0;
#[no_mangle]
pub static mut nr_cgrp_next: u64 = 0;
#[no_mangle]
pub static mut nr_cgrp_coll: u64 = 0;
#[no_mangle]
pub static mut nr_cgrp_empty: u64 = 0;

// UEI_DEFINE(uei);
#[no_mangle]
pub static mut uei: *mut c_void = ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn pair_enqueue(p: *mut task_struct, _enq_flags: u64) {
    let mut cgrp: *mut cgroup;
    let mut cgq: *mut cgrp_q;
    let pid: i32 = (*p).pid;
    let cgid: u64;
    let mut q_idx: *mut u32;
    let mut cgq_len: *mut u64;

    sync_fetch_and_add_u64(&mut nr_total, 1);

    cgrp = scx_bpf_task_cgroup(p);
    cgid = (*(*cgrp).kn).id;
    bpf_cgroup_release(cgrp);

    /* find the cgroup's q and push @p into it */
    q_idx = bpf_map_lookup_elem(cgrp_q_idx_hash, &cgid as *const _ as *const c_void) as *mut u32;
    if q_idx.is_null() {
        scx_bpf_error(b"failed to lookup q_idx for cgroup[%llu]\0".as_ptr() as *const c_char, cgid);
        return;
    }

    cgq = bpf_map_lookup_elem(cgrp_q_arr, q_idx as *const c_void) as *mut cgrp_q;
    if cgq.is_null() {
        scx_bpf_error(
            b"failed to lookup q_arr for cgroup[%llu] q_idx[%u]\0".as_ptr() as *const c_char,
            cgid,
            *q_idx,
        );
        return;
    }

    if bpf_map_push_elem(cgq as *mut c_void, &pid as *const _ as *const c_void, 0) != 0 {
        scx_bpf_error(b"cgroup[%llu] queue overflow\0".as_ptr() as *const c_char, cgid);
        return;
    }

    /* bump q len, if going 0 -> 1, queue cgroup into the top_q */
    cgq_len = MEMBER_VPTR(cgrp_q_len.as_mut_ptr(), *q_idx as i32);
    if cgq_len.is_null() {
        scx_bpf_error(b"MEMBER_VTPR malfunction\0".as_ptr() as *const c_char);
        return;
    }

    if sync_fetch_and_add_u64(cgq_len, 1) == 0
        && bpf_map_push_elem(top_q, &cgid as *const _ as *const c_void, 0) != 0
    {
        scx_bpf_error(b"top_q overflow\0".as_ptr() as *const c_char);
    }
}

unsafe fn lookup_pairc_and_mask(cpu: i32, pairc: *mut *mut pair_ctx, mask: *mut u32) -> i32 {
    let mut vptr: *mut u32;

    vptr = ARRAY_ELEM_PTR(pair_id.as_ptr(), cpu, nr_cpu_ids) as *mut u32;
    if vptr.is_null() {
        return -EINVAL;
    }

    *pairc = bpf_map_lookup_elem(pair_ctx, vptr as *const c_void) as *mut pair_ctx;
    if (*pairc).is_null() {
        return -EINVAL;
    }

    vptr = ARRAY_ELEM_PTR(in_pair_idx.as_ptr(), cpu, nr_cpu_ids) as *mut u32;
    if vptr.is_null() {
        return -EINVAL;
    }

    *mask = 1u32 << *vptr;

    0
}

/*
 * A task is above SCX whenever its effective priority is in the rt/dl
 * range. Test p->prio rather than p->policy: rt_mutex_setprio() boosts
 * a PI beneficiary into the rt/dl classes with its policy left
 * untouched, so a policy test would misclassify boosted tasks in both
 * directions. p->prio follows the boost and the deboost.
 *
 * This still cannot tell fair and SCX tasks apart. It is complete only
 * because scx_pair runs in switch-all mode, where no fair class task
 * exists; in partial mode fair is also above SCX and can take the CPU.
 */
unsafe fn pair_task_is_highpri(p: *mut task_struct) -> bool {
    (*p).prio < MAX_RT_PRIO
}

unsafe fn pair_cpu_acquire_locked(pairc: *mut pair_ctx, in_pair_mask: u32, kick_flags: *mut u32) {
    (*pairc).preempted_mask &= !in_pair_mask;
    /* Kick the pair CPU, unless it was also preempted. */
    *kick_flags = if (*pairc).preempted_mask == 0 { SCX_KICK_PREEMPT } else { 0 };
}

unsafe fn pair_cpu_release_locked(pairc: *mut pair_ctx, in_pair_mask: u32, kick_flags: *mut u32) {
    (*pairc).preempted_mask |= in_pair_mask;
    (*pairc).active_mask &= !in_pair_mask;
    /* Kick the pair CPU if it's still running. */
    *kick_flags = if (*pairc).active_mask != 0 {
        SCX_KICK_PREEMPT | SCX_KICK_WAIT
    } else {
        0
    };
    (*pairc).draining = true;
}

#[inline(never)]
unsafe fn try_dispatch(cpu: i32) -> i32 {
    let mut pairc: *mut pair_ctx = ptr::null_mut();
    let mut cgq_map: *mut bpf_map;
    let mut p: *mut task_struct;
    let now: u64 = scx_bpf_now();
    let mut kick_pair: bool = false;
    let expired: bool;
    let pair_preempted: bool;
    let mut vptr: *mut u32;
    let mut in_pair_mask: u32 = 0;
    let mut pid: i32 = 0;
    let q_idx: i32;
    let cgid: u64;
    let ret: i32;

    ret = lookup_pairc_and_mask(cpu, &mut pairc, &mut in_pair_mask);
    if ret != 0 {
        scx_bpf_error(
            b"failed to lookup pairc and in_pair_mask for cpu[%d]\0".as_ptr() as *const c_char,
            cpu,
        );
        return -ENOENT;
    }

    bpf_spin_lock(&mut (*pairc).lock);
    (*pairc).active_mask &= !in_pair_mask;

    expired = time_before((*pairc).started_at.wrapping_add(pair_batch_dur_ns as u64), now);
    if expired || (*pairc).draining {
        let mut new_cgid: u64 = 0;

        sync_fetch_and_add_u64(&mut nr_exps, 1);

        /*
         * We're done with the current cgid. An obvious optimization
         * would be not draining if the next cgroup is the current one.
         * For now, be dumb and always expire.
         */
        (*pairc).draining = true;

        pair_preempted = (*pairc).preempted_mask != 0;
        if (*pairc).active_mask != 0 || pair_preempted {
            /*
             * The other CPU is still active, or is no longer under
             * our control due to e.g. being preempted by a higher
             * priority sched_class. We want to wait until this
             * cgroup expires, or until control of our pair CPU has
             * been returned to us.
             *
             * If the pair controls its CPU, and the time already
             * expired, kick.  When the other CPU arrives at
             * dispatch and clears its active mask, it'll push the
             * pair to the next cgroup and kick this CPU.
             */
            sync_fetch_and_add_u64(&mut nr_exp_waits, 1);
            bpf_spin_unlock(&mut (*pairc).lock);
            if expired && !pair_preempted {
                kick_pair = true;
            }
            goto_out_maybe_kick(cpu, kick_pair);
            return 0;
        }

        bpf_spin_unlock(&mut (*pairc).lock);

        /*
         * Pick the next cgroup. It'd be easier / cleaner to not drop
         * pairc->lock and use stronger synchronization here especially
         * given that we'll be switching cgroups significantly less
         * frequently than tasks. Unfortunately, bpf_spin_lock can't
         * really protect anything non-trivial. Let's do opportunistic
         * operations instead.
         */
        for _ in 0..BPF_MAX_LOOPS {
            let mut q_idx: *mut u32;
            let mut cgq_len: *mut u64;

            if bpf_map_pop_elem(top_q, &mut new_cgid as *mut _ as *mut c_void) != 0 {
                /* no active cgroup, go idle */
                sync_fetch_and_add_u64(&mut nr_exp_empty, 1);
                return 0;
            }

            q_idx =
                bpf_map_lookup_elem(cgrp_q_idx_hash, &new_cgid as *const _ as *const c_void)
                    as *mut u32;
            if q_idx.is_null() {
                continue;
            }

            /*
             * This is the only place where empty cgroups are taken
             * off the top_q.
             */
            cgq_len = MEMBER_VPTR(cgrp_q_len.as_mut_ptr(), *q_idx as i32);
            if cgq_len.is_null() || *cgq_len == 0 {
                continue;
            }

            /*
             * If it has any tasks, requeue as we may race and not
             * execute it.
             */
            bpf_map_push_elem(top_q, &new_cgid as *const _ as *const c_void, 0);
            break;
        }

        bpf_spin_lock(&mut (*pairc).lock);

        /*
         * The other CPU may already have started on a new cgroup while
         * we dropped the lock. Make sure that we're still draining and
         * start on the new cgroup.
         */
        if (*pairc).draining && (*pairc).active_mask == 0 {
            sync_fetch_and_add_u64(&mut nr_cgrp_next, 1);
            (*pairc).cgid = new_cgid;
            (*pairc).started_at = now;
            (*pairc).draining = false;
            kick_pair = true;
        } else {
            sync_fetch_and_add_u64(&mut nr_cgrp_coll, 1);
        }
    }

    cgid = (*pairc).cgid;
    (*pairc).active_mask |= in_pair_mask;
    bpf_spin_unlock(&mut (*pairc).lock);

    /* again, it'd be better to do all these with the lock held, oh well */
    vptr = bpf_map_lookup_elem(cgrp_q_idx_hash, &cgid as *const _ as *const c_void) as *mut u32;
    if vptr.is_null() {
        scx_bpf_error(b"failed to lookup q_idx for cgroup[%llu]\0".as_ptr() as *const c_char, cgid);
        return -ENOENT;
    }
    q_idx = *vptr as i32;

    /* claim one task from cgrp_q w/ q_idx */
    for _ in 0..BPF_MAX_LOOPS {
        let mut cgq_len: *mut u64;
        let len: u64;

        cgq_len = MEMBER_VPTR(cgrp_q_len.as_mut_ptr(), q_idx);
        len = if !cgq_len.is_null() {
            ptr::read_volatile(cgq_len)
        } else {
            0
        };
        if cgq_len.is_null() || len == 0 {
            /* the cgroup must be empty, expire and repeat */
            sync_fetch_and_add_u64(&mut nr_cgrp_empty, 1);
            bpf_spin_lock(&mut (*pairc).lock);
            (*pairc).draining = true;
            (*pairc).active_mask &= !in_pair_mask;
            bpf_spin_unlock(&mut (*pairc).lock);
            return -EAGAIN;
        }

        if sync_val_compare_and_swap_u64(cgq_len, len, len - 1) != len {
            continue;
        }

        break;
    }

    cgq_map = bpf_map_lookup_elem(cgrp_q_arr, &q_idx as *const _ as *const c_void) as *mut bpf_map;
    if cgq_map.is_null() {
        scx_bpf_error(
            b"failed to lookup cgq_map for cgroup[%llu] q_idx[%d]\0".as_ptr() as *const c_char,
            cgid,
            q_idx,
        );
        return -ENOENT;
    }

    if bpf_map_pop_elem(cgq_map as *mut c_void, &mut pid as *mut _ as *mut c_void) != 0 {
        scx_bpf_error(
            b"cgq_map is empty for cgroup[%llu] q_idx[%d]\0".as_ptr() as *const c_char,
            cgid,
            q_idx,
        );
        return -ENOENT;
    }

    p = bpf_task_from_pid(pid);
    if !p.is_null() {
        sync_fetch_and_add_u64(&mut nr_dispatched, 1);
        scx_bpf_dsq_insert(p, SCX_DSQ_GLOBAL, SCX_SLICE_DFL, 0);
        bpf_task_release(p);
    } else {
        /* we don't handle dequeues, retry on lost tasks */
        sync_fetch_and_add_u64(&mut nr_missing, 1);
        return -EAGAIN;
    }

    goto_out_maybe_kick(cpu, kick_pair);
    0
}

unsafe fn goto_out_maybe_kick(cpu: i32, kick_pair: bool) {
    if kick_pair {
        let pair: *mut i32 = ARRAY_ELEM_PTR(pair_cpu.as_ptr(), cpu, nr_cpu_ids) as *mut i32;
        if !pair.is_null() {
            sync_fetch_and_add_u64(&mut nr_kicks, 1);
            scx_bpf_kick_cpu(*pair, SCX_KICK_PREEMPT);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn pair_dispatch(cpu: i32, _prev: *mut task_struct) {
    for _ in 0..BPF_MAX_LOOPS {
        if try_dispatch(cpu) != -EAGAIN {
            break;
        }
    }
}

#[no_mangle]
#[link_section = "tp_btf/sched_switch"]
pub unsafe extern "C" fn pair_sched_switch(
    _preempt: bool,
    prev: *mut task_struct,
    next: *mut task_struct,
    _prev_state: u32,
) -> i32 {
    let ret: i32;
    let cpu: i32 = bpf_get_smp_processor_id();
    let mut in_pair_mask: u32 = 0;
    let mut pairc: *mut pair_ctx = ptr::null_mut();
    let mut kick_flags: u32 = 0;
    let preempted: bool;
    let release: bool;
    let acquire: bool;

    ret = lookup_pairc_and_mask(cpu, &mut pairc, &mut in_pair_mask);
    if ret != 0 {
        return 0;
    }

    /*
     * This runs on every context switch in the system. A CPU's own
     * preempted_mask bit is only ever written by this tracepoint
     * running on that CPU, so the unlocked read is exact and the
     * pair-shared lock is only taken on actual transitions.
     */
    preempted = ((*pairc).preempted_mask & in_pair_mask) != 0;
    if (*next).pid != 0 && pair_task_is_highpri(next) {
        /* an SCX task lost the CPU to a higher-priority class */
        release = !preempted && (*prev).pid != 0 && !pair_task_is_highpri(prev);
        acquire = false;
    } else {
        /* the CPU is back under SCX control (or idle) */
        release = false;
        acquire = preempted;
    }
    if !release && !acquire {
        return 0;
    }

    bpf_spin_lock(&mut (*pairc).lock);
    if release {
        pair_cpu_release_locked(pairc, in_pair_mask, &mut kick_flags);
        sync_fetch_and_add_u64(&mut nr_preemptions, 1);
    } else {
        pair_cpu_acquire_locked(pairc, in_pair_mask, &mut kick_flags);
    }
    bpf_spin_unlock(&mut (*pairc).lock);

    if kick_flags != 0 {
        let pair: *mut i32 = ARRAY_ELEM_PTR(pair_cpu.as_ptr(), cpu, nr_cpu_ids) as *mut i32;

        if !pair.is_null() {
            sync_fetch_and_add_u64(&mut nr_kicks, 1);
            scx_bpf_kick_cpu(*pair, kick_flags);
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn pair_cgroup_init(cgrp: *mut cgroup) -> i32 {
    let cgid: u64 = (*(*cgrp).kn).id;
    let mut i: i32;
    let mut q_idx: i32 = 0;

    i = 0;
    while i < MAX_CGRPS {
        q_idx = (sync_fetch_and_add_u32(&mut cgrp_q_idx_cursor, 1) as i32) % MAX_CGRPS;
        if sync_val_compare_and_swap_u64(&mut cgrp_q_idx_busy[q_idx as usize], 0, 1) == 0 {
            break;
        }
        i += 1;
    }
    if i == MAX_CGRPS {
        return -EBUSY;
    }

    if bpf_map_update_elem(
        cgrp_q_idx_hash,
        &cgid as *const _ as *const c_void,
        &q_idx as *const _ as *const c_void,
        BPF_ANY,
    ) != 0
    {
        let busy: *mut u64 = MEMBER_VPTR(cgrp_q_idx_busy.as_mut_ptr(), q_idx);
        if !busy.is_null() {
            *busy = 0;
        }
        return -EBUSY;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn pair_cgroup_exit(cgrp: *mut cgroup) {
    let cgid: u64 = (*(*cgrp).kn).id;
    let mut q_idx: *mut i32;

    q_idx = bpf_map_lookup_elem(cgrp_q_idx_hash, &cgid as *const _ as *const c_void) as *mut i32;
    if !q_idx.is_null() {
        let busy: *mut u64 = MEMBER_VPTR(cgrp_q_idx_busy.as_mut_ptr(), *q_idx);
        if !busy.is_null() {
            *busy = 0;
        }
        bpf_map_delete_elem(cgrp_q_idx_hash, &cgid as *const _ as *const c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn pair_exit(ei: *mut scx_exit_info) {
    UEI_RECORD(uei, ei);
}

// SCX_OPS_DEFINE(pair_ops,
//        .enqueue         = (void *)pair_enqueue,
//        .dispatch        = (void *)pair_dispatch,
//        .cgroup_init     = (void *)pair_cgroup_init,
//        .cgroup_exit     = (void *)pair_cgroup_exit,
//        .exit            = (void *)pair_exit,
//        .name            = "pair");
#[repr(C)]
pub struct scx_ops {
    pub enqueue: Option<unsafe extern "C" fn(*mut task_struct, u64)>,
    pub dispatch: Option<unsafe extern "C" fn(i32, *mut task_struct)>,
    pub cgroup_init: Option<unsafe extern "C" fn(*mut cgroup) -> i32>,
    pub cgroup_exit: Option<unsafe extern "C" fn(*mut cgroup)>,
    pub exit: Option<unsafe extern "C" fn(*mut scx_exit_info)>,
    pub name: *const c_char,
}

#[no_mangle]
pub static pair_ops: scx_ops = scx_ops {
    enqueue: Some(pair_enqueue),
    dispatch: Some(pair_dispatch),
    cgroup_init: Some(pair_cgroup_init),
    cgroup_exit: Some(pair_cgroup_exit),
    exit: Some(pair_exit),
    name: b"pair\0".as_ptr() as *const c_char,
};
