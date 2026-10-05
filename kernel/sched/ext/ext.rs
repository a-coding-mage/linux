// SPDX-License-Identifier: GPL-2.0
// Source repair of the existing ext.rs against the pinned native baseline
// 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. Not a completed translation.
// Native leaves are an explicit, unqualified C runtime boundary.
#![no_std]

compile_error!("SOURCE ONLY HOLD: sched_ext core owners and qualification remain incomplete");

use core::{mem::MaybeUninit, ptr};
use kernel::bindings::sched_ext_core_native::*;
use kernel::ffi::{c_char, c_int, c_long, c_uint, c_ulong};

// Existing identities now project F01's exact native enum authority.
pub const SCX_SLICE_OOB_DUR_BITS: u32 =
    kernel::bindings::sched_ext_core_native::SCX_SLICE_OOB_DUR_BITS as u32;
pub const SCX_SLICE_OOB_ID_BITS: u32 =
    kernel::bindings::sched_ext_core_native::SCX_SLICE_OOB_ID_BITS as u32;
pub const SCX_SLICE_OOB_DUR_MASK: u64 =
    kernel::bindings::sched_ext_core_native::SCX_SLICE_OOB_DUR_MASK as u64;
pub const SCX_SLICE_OOB_ID_SHIFT: u32 =
    kernel::bindings::sched_ext_core_native::SCX_SLICE_OOB_ID_SHIFT as u32;
pub const SCX_SLICE_OOB_ID_MASK: u64 =
    kernel::bindings::sched_ext_core_native::SCX_SLICE_OOB_ID_MASK as u64;
pub const SCX_SLICE_OOB_PENDING: u64 =
    kernel::bindings::sched_ext_core_native::SCX_SLICE_OOB_PENDING as u64;

mod sched_ext_core_slice_cursor;
// Select the Rust-owned body over internal.h's native declaration glob.
pub(crate) use sched_ext_core_slice_cursor::scx_task_slice_ended;

mod sched_ext_core_task_lifetime;
// Explicit owned identities take precedence over matching native declarations.
pub(crate) use sched_ext_core_task_lifetime::{
    __scx_disable_and_exit_task, __scx_enable_task, __scx_init_task,
    __scx_task_iter_maybe_relock, __scx_task_iter_rq_unlock, init_scx_entity,
    lupos_scx_core_task_dead_unpublish_body,
    lupos_scx_core_task_post_fork_publish_body,
    lupos_scx_core_task_prio_changed_body, lupos_scx_core_task_reweight_body,
    lupos_scx_core_task_switched_from_body, lupos_scx_core_task_switched_to_body,
    lupos_scx_core_task_switching_to_body, sched_ext_dead, scx_alloc_tid,
    scx_allow_ttwu_queue, scx_cancel_fork, scx_check_setscheduler,
    scx_disable_and_exit_task, scx_disable_task, scx_enable_task, scx_fork,
    scx_get_task_state, scx_post_fork, scx_pre_fork, scx_set_task_state,
    scx_sub_init_cancel_task, scx_task_iter_next, scx_task_iter_next_locked,
    scx_task_iter_relock, scx_task_iter_start, scx_task_iter_stop,
    scx_task_iter_unlock, scx_tid_hash_insert, task_dead_and_done, task_should_scx,
};
#[cfg(CONFIG_EXT_GROUP_SCHED)]
pub(crate) use sched_ext_core_task_lifetime::tg_cgrp;
pub(crate) use sched_ext_core_slice_cursor::*;

mod sched_ext_core_cgroup;
pub(crate) use sched_ext_core_cgroup::{
    root_cgroup, scx_cgroup_exit, scx_cgroup_init, scx_cgroup_lock, scx_cgroup_unlock,
};
#[cfg(CONFIG_EXT_GROUP_SCHED)]
pub(crate) use sched_ext_core_cgroup::{
    lupos_scx_core_cgroup_tg_cgrp, scx_cgroup_can_attach, scx_cgroup_cancel_attach,
    scx_cgroup_move_task, scx_cgroup_task_sched, scx_group_set_bandwidth,
    scx_group_set_idle, scx_group_set_weight, scx_tg_init, scx_tg_knob_sched,
    scx_tg_offline, scx_tg_online, scx_tg_sched,
};

mod sched_ext_core_object_lifetime;
pub(crate) use sched_ext_core_object_lifetime::{
    alloc_exit_info, alloc_pnode, destroy_dsq, exit_dsq, free_exit_info, free_pnode,
    lupos_scx_core_object_enable_seq_body, lupos_scx_core_object_events_body,
    lupos_scx_core_object_free_dsq_irq_body, lupos_scx_core_object_free_dsq_rcu_body,
    lupos_scx_core_object_hotplug_seq_body, lupos_scx_core_object_link_locked_body,
    lupos_scx_core_object_nr_rejected_body, lupos_scx_core_object_ops_body,
    lupos_scx_core_object_release_body, lupos_scx_core_object_sched_free_body,
    lupos_scx_core_object_state_body, lupos_scx_core_object_switch_all_body,
    lupos_scx_core_object_uevent_body, lupos_scx_core_object_unlink_locked_body,
    scx_alloc_and_add_sched, scx_exit_reason, scx_init_dsq, scx_link_sched,
    scx_sched_sysfs_add, scx_set_cmask_scratch_alloc, scx_set_cmask_scratch_free,
    scx_unlink_sched,
};
#[cfg(CONFIG_EXT_SUB_SCHED)]
pub(crate) use sched_ext_core_object_lifetime::{
    lupos_scx_core_object_caps_body, lupos_scx_core_object_caps_one_body,
};

pub fn u32_before(a: u32, b: u32) -> bool {
    (a.wrapping_sub(b) as i32) < 0
}

/// Check signed range before converting to the native CPU index (ext.c:970-973).
///
/// # Safety
/// Native CPU topology state is initialized and available in the caller's
/// scheduler context. No validity precondition is imposed on cpu: its lower
/// and upper bounds are checked before consulting the possible-CPU bitmap.
unsafe fn __cpu_valid(cpu: i32) -> bool {
    // SAFETY: nr_cpu_ids is native configured state. cpu_possible is only
    // called for a nonnegative in-range CPU, so no invalid bit is accessed.
    unsafe {
        lupos_scx_core_likely_cpu_valid(
            cpu >= 0
                && (cpu as u32) < lupos_scx_core_nr_cpu_ids()
                && lupos_scx_core_cpu_possible(cpu),
        )
    }
}

/// Check a BPF CPU argument and request scheduler error exit on failure.
///
/// # Safety
/// sch must be live under the caller's original scheduler lifetime protection.
/// where_ is NULL or a readable NUL-terminated string for the whole synchronous
/// native diagnostic call; that call must be legal in the caller's context.
#[no_mangle]
pub unsafe extern "C" fn scx_cpu_valid(
    sch: *mut scx_sched,
    cpu: i32,
    where_: *const c_char,
) -> bool {
    // SAFETY: The caller pins sch and the optional diagnostic string. The
    // native leaf expands the real scx_error macro and preserves NULL handling.
    unsafe {
        if __cpu_valid(cpu) {
            true
        } else {
            lupos_scx_core_error_invalid_cpu(sch, cpu, where_);
            false
        }
    }
}

/// Set an unprotected task's slice, superseding its pending out-of-band request.
///
/// # Safety
/// p is live and the caller holds p's current rq lock, stabilizing task flags,
/// rq association and slice ownership. slice is a native u64 duration or
/// SCX_SLICE_INF. The native lock assertion and atomic primitives remain live.
#[no_mangle]
pub unsafe extern "C" fn scx_set_task_slice(p: *mut task_struct, slice: u64) -> bool {
    // SAFETY: The caller pins p under its rq lock. Protected-task rejection
    // returns without changing either slice or OOB; successful slice assignment
    // precedes the native read-then-set clearing of a pending request.
    unsafe {
        if !set_task_slice_keep_oob(p, slice) {
            return false;
        }
        clear_task_slice_oob(p);
        true
    }
}

/// Apply the original bypass-slice parameter range (100us through 100ms).
///
/// # Safety
/// Only the native parameter callback may supply its live parameter descriptor
/// and readable NUL-terminated input; the descriptor's unsigned-int storage is
/// writable under the native parameter subsystem's serialization.
#[export_name = "lupos_scx_core_set_slice_us"]
pub unsafe extern "C" fn set_slice_us(val: *const c_char, kp: *const kernel_param) -> c_int {
    // SAFETY: Native parser keeps configured parameter behavior and owns the
    // descriptor's layout; arguments retain the callback's lifetime.
    unsafe {
        lupos_scx_core_param_set_uint_minmax(
            val, kp, 100, 100 * LUPOS_SCX_CORE_USEC_PER_MSEC as c_uint,
        )
    }
}

/// Apply the original bypass load-balance range (disabled through 10s).
///
/// # Safety
/// Same parameter descriptor, input and subsystem-serialization requirements
/// as set_slice_us; kp must refer to the unsigned-int interval parameter.
#[export_name = "lupos_scx_core_set_bypass_lb_intv_us"]
pub unsafe extern "C" fn set_bypass_lb_intv_us(
    val: *const c_char, kp: *const kernel_param,
) -> c_int {
    // SAFETY: Native parser receives the original inclusive limits and live kp.
    unsafe {
        lupos_scx_core_param_set_uint_minmax(
            val, kp, 0, 10 * LUPOS_SCX_CORE_USEC_PER_SEC as c_uint,
        )
    }
}

/// Read the negotiated TID lookup static key.
///
/// # Safety
/// Caller must obey the surrounding scheduler lifetime protocol when using the
/// result to access the TID table. Reading a key does not pin that table.
pub(crate) unsafe fn scx_tid_to_task_enabled() -> bool {
    // SAFETY: This leaf reads the single native key using static_branch_likely.
    unsafe { lupos_scx_core_tid_to_task_enabled() }
}

/// Test whether a DSQ's lock domain is the containing rq.
///
/// # Safety
/// dsq points to a live initialized DSQ; the caller pins its lifetime and id.
pub(crate) unsafe fn dsq_is_rq_owned(dsq: *mut scx_dispatch_q) -> bool {
    // SAFETY: The borrowed DSQ is native-layout storage with a stable id.
    unsafe {
        let id = (*dsq).id;
        id == LUPOS_SCX_CORE_DSQ_LOCAL as u64
            || id == LUPOS_SCX_CORE_DSQ_REJECT as u64
            || id == LUPOS_SCX_CORE_DSQ_RESCUE as u64
    }
}

/// Convert a wrap-aware jiffies delta with the original signed-long result.
///
/// # Safety
/// at/now obey the native time_after range convention. The caller must keep
/// the original representable signed-long millisecond-delta domain; no new
/// arithmetic admission or change to native conversion policy is implied.
pub(crate) unsafe fn jiffies_delta_msecs(at: c_ulong, now: c_ulong) -> c_long {
    // SAFETY: Native time_after and conversion preserve configured HZ/word
    // width. Unsigned jiffies subtraction wraps just as in the native source.
    unsafe {
        if lupos_scx_core_time_after(at, now) {
            lupos_scx_core_jiffies_to_msecs(at.wrapping_sub(now)) as c_long
        } else {
            (lupos_scx_core_jiffies_to_msecs(now.wrapping_sub(at)) as c_long).wrapping_neg()
        }
    }
}

/// Test the original stable hierarchy's ancestor identity.
///
/// # Safety
/// Both schedulers and sch's complete flexible ancestor array are initialized
/// and pinned by the caller's original enable/RCU protection. Levels are the
/// nonnegative hierarchy indices established when the schedulers were linked.
#[no_mangle]
pub unsafe extern "C" fn scx_is_descendant(
    sch: *mut scx_sched, ancestor: *mut scx_sched,
) -> bool {
    // SAFETY: Compare levels before indexing sch's native flexible array.
    unsafe {
        if (*sch).level < (*ancestor).level {
            return false;
        }
        lupos_scx_core_ancestor_at(sch, (*ancestor).level) == ancestor
    }
}

/// Locate a scheduler's NUMA-node global DSQ for a valid CPU.
///
/// # Safety
/// sch's pnode array, its selected entry and the CPU topology are initialized
/// and pinned by the caller's original scheduler/hotplug lifetime protection.
pub(crate) unsafe fn find_global_dsq(sch: *mut scx_sched, cpu: i32) -> *mut scx_dispatch_q {
    // SAFETY: A valid possible CPU supplies an allocated native pnode index.
    unsafe {
        let node = lupos_scx_core_cpu_to_node(cpu);
        let pnode = *(*sch).pnode.add(node as usize);
        ptr::addr_of_mut!((*pnode).global_dsq)
    }
}

/// Look up a user DSQ under the caller's hash/RCU lifetime protection.
///
/// # Safety
/// sch's DSQ hash is initialized and pinned. Caller provides the native hash
/// reader protection and must continue it while using the returned DSQ.
pub(crate) unsafe fn find_user_dsq(sch: *mut scx_sched, dsq_id: u64) -> *mut scx_dispatch_q {
    // SAFETY: The native lookup borrows this stack key synchronously and uses
    // the one original dsq_hash_params instance with native offsets.
    unsafe { lupos_scx_core_dsq_lookup(ptr::addr_of_mut!((*sch).dsq_hash), &dsq_id) }
}

/// Choose a task's underlying policy class, preserving stop-task identity.
///
/// # Safety
/// p is live and its class, policy and priority are stable under the caller's
/// original scheduler locks. Returned native class storage has static lifetime.
pub(crate) unsafe fn scx_setscheduler_class(p: *mut task_struct) -> *const sched_class {
    // SAFETY: The task's native fields and class symbols remain valid here.
    unsafe {
        let stop = lupos_scx_core_stop_class();
        if (*p).sched_class == stop {
            stop
        } else {
            lupos_scx_core_setscheduler_class((*p).policy as c_int, (*p).prio)
        }
    }
}

/// Find the nearest non-bypassing ancestor's bypass DSQ, or the root's.
///
/// # Safety
/// The hierarchy and per-CPU scheduler storage remain live under the original
/// enqueue path's protection; cpu is valid and the appropriate rq is locked.
pub(crate) unsafe fn bypass_enq_target_dsq(
    sch: *mut scx_sched, cpu: i32,
) -> *mut scx_dispatch_q {
    // SAFETY: Hierarchy links are stable; the native per-CPU leaf only indexes
    // the caller's valid CPU. The disabled configuration does not walk parents.
    unsafe {
        #[cfg(CONFIG_EXT_SUB_SCHED)]
        let sch = {
            let mut at = sch;
            while !lupos_scx_core_parent(at).is_null()
                && lupos_scx_core_bypassing(at, cpu)
            {
                at = lupos_scx_core_parent(at);
            }
            at
        };
        lupos_scx_core_bypass_dsq(sch, cpu)
    }
}

/// Test immediate SCX execution availability while preserving dispatch promises.
///
/// # Safety
/// rq is live and its rq lock is held. Its next_class/current/SCX state remain
/// stable under that lock; enq_flags has the original enqueue flag semantics.
pub(crate) unsafe fn rq_is_open(rq: *mut rq, enq_flags: u64) -> bool {
    // SAFETY: The caller holds the rq lock; keep its native lockdep assertion
    // before any field read. Current is dereferenced only on the PREEMPT path.
    unsafe {
        lupos_scx_core_assert_rq_open(rq);
        let ext = lupos_scx_core_ext_class();
        if lupos_scx_core_class_above((*rq).next_class, ext) {
            return false;
        }
        if lupos_scx_core_class_above(ext, (*rq).next_class) {
            return true;
        }
        if (*rq).scx.flags & LUPOS_SCX_CORE_RQ_IN_DISPATCH as u32 != 0 {
            return true;
        }
        if enq_flags & LUPOS_SCX_CORE_ENQ_PREEMPT as u64 != 0 {
            // Native field access handles the configured curr/donor union.
            // Preserve the original plain rq->curr read, not READ_ONCE.
            let curr = lupos_scx_core_rq_curr(rq);
            return (*curr).sched_class != ext
                || lupos_scx_core_likely_not_protected(
                    (*curr).scx.flags & LUPOS_SCX_CORE_TASK_PROTECTED as u32 == 0,
                );
        }
        false
    }
}

/// Account an rq unlock that may invalidate a core-wide scheduling pick.
///
/// # Safety
/// rq is live and locked, with current-CPU scheduling context pinned exactly
/// as required by the native pick/dispatch callsites.
pub(crate) unsafe fn scx_rq_lock_drop(rq: *mut rq) {
    // SAFETY: The native assertion is always retained. The configured field is
    // accessed only when CONFIG_SCHED_CORE supplies it and core scheduling is on.
    unsafe {
        lupos_scx_core_assert_rq_lock_drop(rq);
        #[cfg(CONFIG_SCHED_CORE)]
        if lupos_scx_core_sched_core_enabled(rq) {
            (*rq).scx.lock_drop_seq = (*rq).scx.lock_drop_seq.wrapping_add(1);
        }
    }
}

/// Transfer rq locking and any SCX callback lock tracking from one rq to another.
///
/// # Safety
/// Caller owns from's rq lock and is allowed to drop it and acquire to under
/// the original scheduler lock-order protocol. Both rqs remain live, local
/// per-CPU state is stable and IRQ/preemption context matches the native path.
/// No field borrowed under the old lock may remain assumed stable afterward.
pub(crate) unsafe fn switch_rq_lock(from: *mut rq, to: *mut rq) {
    // SAFETY: Keep tracking cleared before unlocking, count the drop while
    // from remains locked, and restore tracking only after locking to.
    unsafe {
        let tracked = lupos_scx_core_locked_rq() == from;
        if tracked {
            lupos_scx_core_update_locked_rq(ptr::null_mut());
        }
        scx_rq_lock_drop(from);
        lupos_scx_core_raw_spin_rq_unlock(from);
        lupos_scx_core_raw_spin_rq_lock(to);
        if tracked {
            lupos_scx_core_update_locked_rq(to);
        }
    }
}

/// Invoke the proper typed affinity callback, constructing a CID arena cmask.
///
/// # Safety
/// sch/task/cpumask and the applicable ops callback are live and authorized.
/// Caller holds the relevant task rq lock, pins the CPU, CID tables and per-CPU
/// scratch allocation, and supplies rq or NULL according to the original
/// callback-tracking contract. Scratch is initialized for kernel geometry and
/// exclusively writable by this CPU; callbacks must not nest task guards.
pub(crate) unsafe fn scx_call_op_set_cpumask(
    sch: *mut scx_sched, rq: *mut rq, task: *mut task_struct, cpumask: *const cpumask,
) {
    // SAFETY: CID owner initializes the native reference completely before it
    // is read. The native SCX_CALL_* leaves preserve typed union dispatch,
    // nested rq save/restore, task guards, warnings and arena-address handling.
    unsafe {
        if lupos_scx_core_is_cid_type() {
            let kern_va = lupos_scx_core_this_cmask_scratch(sch);
            let mut cmask_ref = MaybeUninit::<scx_cmask_ref>::uninit();
            scx_cmask_ref_init_kern(
                sch, kern_va, 0, lupos_scx_core_num_possible_cpus(), cmask_ref.as_mut_ptr(),
            );
            scx_cmask_ref_from_cpumask(cmask_ref.as_ptr(), cpumask);
            lupos_scx_core_call_set_cmask(sch, rq, task, kern_va);
        } else {
            lupos_scx_core_call_set_cpumask(sch, rq, task, cpumask);
        }
    }
}

/// Read the original native enable-state atomic.
///
/// # Safety
/// Caller must not infer stronger memory ordering or scheduler lifetime
/// protection than the original atomic_read provides.
pub(crate) unsafe fn scx_enable_state() -> scx_enable_state {
    // SAFETY: The single F00 native atomic retains its original initialization.
    unsafe { lupos_scx_core_enable_state_read() as scx_enable_state }
}

/// Atomically replace enable state and return the previous state.
///
/// # Safety
/// to is a valid native enable state; caller owns the original enable/disable
/// transition protocol, including its external locks and lifecycle ordering.
pub(crate) unsafe fn scx_set_enable_state(to: scx_enable_state) -> scx_enable_state {
    // SAFETY: Native atomic_xchg preserves the original read-modify-write order.
    unsafe { lupos_scx_core_enable_state_xchg(to as c_int) as scx_enable_state }
}

/// Atomically change a matching enable state, preserving try-cmpxchg semantics.
///
/// # Safety
/// Both arguments are valid native states and the caller owns the original
/// enable/disable protocol. This helper alone does not authorize a transition.
pub(crate) unsafe fn scx_tryset_enable_state(
    to: scx_enable_state, from: scx_enable_state,
) -> bool {
    // SAFETY: The writable expected-value scratch lives through exactly one
    // native try-cmpxchg call, matching the original local int from_v.
    unsafe {
        let mut from_v = from as c_int;
        lupos_scx_core_enable_state_try_cmpxchg(&mut from_v, to as c_int)
    }
}

/// Wait for the full ops-state word to change, retaining load-acquire semantics.
///
/// # Safety
/// p remains live and its opss state is SCX_QUEUEING or SCX_DISPATCHING. The
/// caller's context permits this busy-wait and must not prevent the owning
/// enqueue/dispatch path from making progress. opss includes its original qseq.
pub(crate) unsafe fn wait_ops_state(p: *mut task_struct, opss: c_ulong) {
    // SAFETY: Relax is executed before each acquire load, including the first;
    // comparing as native unsigned long matches C's usual signed conversion.
    unsafe {
        loop {
            lupos_scx_core_cpu_relax();
            if lupos_scx_core_ops_state_read_acquire(p) as c_ulong != opss {
                break;
            }
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
