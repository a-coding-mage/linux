// SPDX-License-Identifier: GPL-2.0
/*
 * BPF extensible scheduler class: built-in idle CPU tracking policy.
 * Continuation of idle.rs at baseline 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
 * Existing claim/pick/topology/selection/update/kfunc entry-point spine retained;
 * missing behavior repaired against that baseline's idle.c and native headers.
 * Native leaves are an explicit, unqualified C runtime boundary, not Rust
 * algorithm coverage. No old idle.c/ext.c implementation fallback is admitted.
 */
#![no_std]

//! # Common FFI safety requirements
//!
//! Every public unsafe entry also requires the native configured ABI and its
//! documented kernel context. A pointer described as live must be non-null,
//! correctly aligned and valid for the native object's stated accesses and
//! lifetime; only explicitly optional masks may be null. Input values must be
//! initialized, and writable outputs must permit the stated writes without
//! concurrent conflicting access. Native cpumask extents, not a fabricated
//! Rust layout, determine the required mask storage. No kernel object ownership
//! is transferred. Returned masks remain native-owned and must not be mutated
//! or freed by callers.
//!
//! BPF continuations must enter through their corresponding native typed thunk
//! after the actual verifier, kfunc flags and context filter have accepted the
//! call, or satisfy those same pointer, lifetime and context guarantees. In
//! particular, selection-context sampling must retain the native BPF CPU-pinning
//! guarantee; an inner RCU guard alone is not a migration-disable operation.

compile_error!("SOURCE ONLY HOLD: sched_ext idle is not admitted");

use core::{marker::PhantomData, ptr};
use kernel::bindings::sched_ext_idle_native::*;
use kernel::ffi::{c_int, c_uint, c_ulong};

const EBUSY: c_int = LUPOS_SCX_IDLE_EBUSY as c_int;
const EINVAL: c_int = LUPOS_SCX_IDLE_EINVAL as c_int;
const ENOENT: c_int = LUPOS_SCX_IDLE_ENOENT as c_int;
const ENODEV: c_int = LUPOS_SCX_IDLE_ENODEV as c_int;
const EOPNOTSUPP: c_int = LUPOS_SCX_IDLE_EOPNOTSUPP as c_int;
const NUMA_NO_NODE: c_int = LUPOS_SCX_IDLE_NUMA_NO_NODE as c_int;
const SCX_PICK_IDLE_CORE: u64 = LUPOS_SCX_IDLE_PICK_CORE as u64;
const SCX_PICK_IDLE_IN_NODE: u64 = LUPOS_SCX_IDLE_PICK_IN_NODE as u64;
const SCX_WAKE_SYNC: u64 = LUPOS_SCX_IDLE_WAKE_SYNC as u64;
const SCX_OPS_BUILTIN_IDLE_PER_NODE: u64 = LUPOS_SCX_IDLE_OPS_PER_NODE as u64;
const SCX_OPS_KEEP_BUILTIN_IDLE: u64 = LUPOS_SCX_IDLE_OPS_KEEP as u64;

// Guards are synchronous, stack-bound and non-Send. Drop order releases RCU
// before preemption, and the selection path releases both before task pi_lock.
struct RcuGuard(PhantomData<*mut ()>);
impl RcuGuard {
    unsafe fn lock() -> Self {
        // SAFETY: Caller's scheduler/kfunc context allows this native RCU lock.
        unsafe { lupos_scx_idle_rcu_lock() };
        Self(PhantomData)
    }
}
impl Drop for RcuGuard {
    fn drop(&mut self) {
        // SAFETY: Exactly one synchronous acquisition, on the same task.
        unsafe { lupos_scx_idle_rcu_unlock() };
    }
}
struct PreemptGuard(PhantomData<*mut ()>);
impl PreemptGuard {
    unsafe fn lock() -> Self {
        // SAFETY: No sleeping operation occurs while this guard is live.
        unsafe { lupos_scx_idle_preempt_disable() };
        Self(PhantomData)
    }
}
impl Drop for PreemptGuard {
    fn drop(&mut self) {
        // SAFETY: Balances this guard's single disable on the same task.
        unsafe { lupos_scx_idle_preempt_enable() };
    }
}
struct PiGuard {
    task: *mut task_struct,
    flags: c_ulong,
    _not_send: PhantomData<*mut ()>,
}
impl PiGuard {
    unsafe fn lock(task: *mut task_struct) -> Self {
        // SAFETY: Called only in a truly unlocked context; task remains live.
        let flags = unsafe { lupos_scx_idle_lock_pi(task) };
        Self { task, flags, _not_send: PhantomData }
    }
}
impl Drop for PiGuard {
    fn drop(&mut self) {
        // SAFETY: Owns this task's pi_lock and its matching saved IRQ flags.
        unsafe { lupos_scx_idle_unlock_pi(self.task, self.flags) };
    }
}

unsafe fn scx_cpu_node_if_enabled(cpu: c_int) -> c_int {
    // SAFETY: cpu is valid; configured static key and topology stay native.
    unsafe {
        if lupos_scx_idle_per_node_maybe() { lupos_scx_idle_cpu_node(cpu) }
        else { NUMA_NO_NODE }
    }
}

unsafe fn scx_idle_test_and_clear_cpu(cpu: c_int) -> bool {
    // SAFETY: CPU was validated by the caller or selected from a native mask.
    // Idle masks are permanent; their relaxed SMT hint and atomic CPU claim
    // use native configured cpumask primitives, as in idle.c.
    unsafe {
        let node = scx_cpu_node_if_enabled(cpu);
        if lupos_scx_idle_smt_active() {
            let smt = lupos_scx_idle_siblings(cpu);
            let idle_smts = lupos_scx_idle_smt_mask(node);
            if lupos_scx_idle_mask_intersects(smt, idle_smts) {
                lupos_scx_idle_mask_andnot(idle_smts, idle_smts, smt);
            } else if lupos_scx_idle_mask_test(cpu, idle_smts) {
                // An offline CPU need not appear in its own sibling mask.
                lupos_scx_idle_mask_clear_cpu(cpu, idle_smts);
            }
        }
        lupos_scx_idle_mask_claim(cpu, lupos_scx_idle_cpu_mask(node))
    }
}

unsafe fn pick_idle_cpu_in_node(allowed: *const cpumask, node: c_int, flags: u64) -> c_int {
    // SAFETY: allowed is a live mask; node is global or an allocated possible
    // node. Retrying loses no ownership and clears stale offline SMT hints.
    unsafe {
        loop {
            if lupos_scx_idle_smt_active() {
                let cpu = lupos_scx_idle_mask_any_and(lupos_scx_idle_smt_mask(node), allowed);
                if cpu < lupos_scx_idle_nr_cpus() {
                    if scx_idle_test_and_clear_cpu(cpu as c_int) { return cpu as c_int; }
                    continue;
                }
                if flags & SCX_PICK_IDLE_CORE != 0 { return -EBUSY; }
            }
            let cpu = lupos_scx_idle_mask_any_and(lupos_scx_idle_cpu_mask(node), allowed);
            if cpu >= lupos_scx_idle_nr_cpus() { return -EBUSY; }
            if scx_idle_test_and_clear_cpu(cpu as c_int) { return cpu as c_int; }
        }
    }
}

#[cfg(CONFIG_NUMA)]
unsafe fn pick_idle_cpu_from_online_nodes(allowed: *const cpumask, node: c_int, flags: u64) -> c_int {
    // SAFETY: The caller holds RCU for nearest-node topology; the guard pins
    // per-CPU scratch. Every iteration retains the ORIGINAL starting node,
    // matching for_each_node_numadist rather than a greedy nearest-next walk.
    unsafe {
        let _preempt = PreemptGuard::lock();
        let unvisited = lupos_scx_idle_unvisited();
        lupos_scx_idle_nodes_online(unvisited);
        lupos_scx_idle_node_clear(node, unvisited);
        loop {
            let next = lupos_scx_idle_nearest_node(node, unvisited);
            if next >= LUPOS_SCX_IDLE_MAX_NUMNODES as c_int { return -EBUSY; }
            let cpu = pick_idle_cpu_in_node(allowed, next, flags);
            if cpu >= 0 { return cpu; }
            lupos_scx_idle_node_clear(next, unvisited);
        }
    }
}
#[cfg(not(CONFIG_NUMA))]
unsafe fn pick_idle_cpu_from_online_nodes(_: *const cpumask, _: c_int, _: u64) -> c_int {
    // Exact non-NUMA oracle branch; no other nodes exist to search.
    -EBUSY
}

unsafe fn scx_pick_idle_cpu(allowed: *const cpumask, node: c_int, flags: u64) -> c_int {
    // SAFETY: Caller pins allowed, node masks and (for NUMA) RCU topology.
    unsafe {
        let cpu = pick_idle_cpu_in_node(allowed, node, flags);
        if cpu >= 0 { return cpu; }
        if node == NUMA_NO_NODE || flags & SCX_PICK_IDLE_IN_NODE != 0 { return -EBUSY; }
        pick_idle_cpu_from_online_nodes(allowed, node, flags)
    }
}

/// Read an LLC domain's native weight, including lazy native debug callbacks.
///
/// # Safety
/// cpu is valid and the caller holds RCU plus topology hotplug protection.
#[export_name = "lupos_scx_idle_llc_weight_rust"]
pub unsafe extern "C" fn llc_weight(cpu: c_int) -> c_uint {
    // SAFETY: Caller holds RCU; native dereference supplies actual domain layout.
    unsafe {
        let sd = lupos_scx_idle_llc_domain(cpu);
        if sd.is_null() { 0 } else { (*sd).span_weight }
    }
}
/// Borrow the LLC span without evaluating disabled native debug arguments.
///
/// # Safety
/// cpu is valid and RCU remains held throughout every use of the result.
#[export_name = "lupos_scx_idle_llc_span_rust"]
pub unsafe extern "C" fn llc_span(cpu: c_int) -> *const cpumask {
    // SAFETY: RCU pins domain and its native flexible-array span.
    unsafe {
        let sd = lupos_scx_idle_llc_domain(cpu);
        if sd.is_null() { ptr::null() } else { lupos_scx_idle_domain_span(sd) }
    }
}
unsafe fn numa_weight(cpu: c_int) -> c_uint {
    // SAFETY: RCU pins domain and groups; absent topology returns zero.
    unsafe {
        let sd = lupos_scx_idle_numa_domain(cpu);
        if sd.is_null() || (*sd).groups.is_null() { 0 } else { (*(*sd).groups).group_weight }
    }
}
/// Borrow the NUMA group span, also used by lazy native debug callbacks.
///
/// # Safety
/// cpu is valid and RCU remains held throughout every use of the result.
#[export_name = "lupos_scx_idle_numa_span_rust"]
pub unsafe extern "C" fn numa_span(cpu: c_int) -> *const cpumask {
    // SAFETY: RCU pins domain/groups and native group flexible-array span.
    unsafe {
        let sd = lupos_scx_idle_numa_domain(cpu);
        if sd.is_null() || (*sd).groups.is_null() { ptr::null() }
        else { lupos_scx_idle_group_span((*sd).groups) }
    }
}
unsafe fn llc_numa_mismatch() -> bool {
    // SAFETY: RCU and caller's hotplug protection pin all online domain reads.
    unsafe {
        let mut cpu = lupos_scx_idle_online_scan(0);
        while cpu < lupos_scx_idle_cpu_iter_limit() {
            if llc_weight(cpu as c_int) != numa_weight(cpu as c_int) { return true; }
            cpu = lupos_scx_idle_online_scan(cpu + 1);
        }
        false
    }
}

/// Refresh default selection's LLC and NUMA optimizations.
///
/// # Safety
/// ops is live under scheduler-enable or CPU-hotplug serialization; online
/// CPUs are stable and nonempty. The native cpuslocked static-key requirement
/// is met by the CPU-hotplug read lock or the supported hotplug-writer context.
#[no_mangle]
pub unsafe extern "C" fn scx_idle_update_selcpu_topology(ops: *mut sched_ext_ops) {
    // SAFETY: The caller pins ops and online CPUs; local RCU pins topology.
    unsafe {
        let mut enable_llc = false;
        let mut enable_numa = false;
        let cpu = lupos_scx_idle_mask_first(lupos_scx_idle_online_mask()) as c_int;
        {
            let _rcu = RcuGuard::lock();
            let nr = llc_weight(cpu);
            if nr > 0 {
                enable_llc = nr < lupos_scx_idle_nr_online();
                lupos_scx_idle_debug_llc(cpu);
            }
            if (*ops).flags & SCX_OPS_BUILTIN_IDLE_PER_NODE == 0 {
                let nr = numa_weight(cpu);
                if nr > 0 {
                    enable_numa = nr < lupos_scx_idle_nr_online() && llc_numa_mismatch();
                    lupos_scx_idle_debug_numa(cpu, nr);
                }
            }
        }
        lupos_scx_idle_debug_keys(enable_llc, enable_numa);
        lupos_scx_idle_set_llc(enable_llc);
        lupos_scx_idle_set_numa(enable_numa);
    }
}

unsafe fn task_affinity_all(p: *const task_struct) -> bool {
    // SAFETY: Caller holds p's pi_lock or rq lock; field has its native C type.
    unsafe { (*p).nr_cpus_allowed as c_uint >= lupos_scx_idle_nr_possible() }
}

/// Select and claim a CPU using the builtin policy.
///
/// # Safety
/// p is live with pi_lock or its rq lock held; prev_cpu is valid and any
/// non-NULL allowed mask remains readable for the call. Idle storage is
/// initialized and tracking enabled. Invocation permits preempt/RCU guards.
#[no_mangle]
pub unsafe extern "C" fn scx_select_cpu_dfl(
    p: *mut task_struct, prev_cpu: c_int, wake_flags: u64,
    cpus_allowed: *const cpumask, flags: u64,
) -> c_int {
    // SAFETY: Caller supplies stable affinity; guards pin scratch CPU and
    // RCU topology. All branches release RCU before reenabling preemption.
    unsafe {
        let mut llc_cpus = ptr::null();
        let mut numa_cpus = ptr::null();
        let mut allowed = if cpus_allowed.is_null() { (*p).cpus_ptr } else { cpus_allowed };
        let node = scx_cpu_node_if_enabled(prev_cpu);
        let _preempt = PreemptGuard::lock();
        if allowed != (*p).cpus_ptr {
            let local = lupos_scx_idle_local_mask();
            if task_affinity_all(p) {
                allowed = cpus_allowed;
            } else if lupos_scx_idle_mask_and(local, cpus_allowed, (*p).cpus_ptr) {
                allowed = local;
            } else {
                return -EBUSY;
            }
        }
        let is_prev_allowed = lupos_scx_idle_mask_test(prev_cpu, allowed);
        let _rcu = RcuGuard::lock();
        if lupos_scx_idle_numa_maybe() {
            let local = lupos_scx_idle_local_numa_mask();
            let cpus = numa_span(prev_cpu);
            if allowed == (*p).cpus_ptr && task_affinity_all(p) {
                numa_cpus = cpus;
            } else if !cpus.is_null() && lupos_scx_idle_mask_and(local, allowed, cpus) {
                numa_cpus = local;
            }
        }
        if lupos_scx_idle_llc_maybe() {
            let local = lupos_scx_idle_local_llc_mask();
            let cpus = llc_span(prev_cpu);
            if allowed == (*p).cpus_ptr && task_affinity_all(p) {
                llc_cpus = cpus;
            } else if !cpus.is_null() && lupos_scx_idle_mask_and(local, allowed, cpus) {
                llc_cpus = local;
            }
        }
        if wake_flags & SCX_WAKE_SYNC != 0 {
            let cpu = lupos_scx_idle_this_cpu();
            if is_prev_allowed && lupos_scx_idle_share_cache(cpu, prev_cpu)
                && scx_idle_test_and_clear_cpu(prev_cpu) {
                return prev_cpu;
            }
            let waker_node = scx_cpu_node_if_enabled(cpu);
            if (*lupos_scx_idle_current()).flags & LUPOS_SCX_IDLE_PF_EXITING as c_uint == 0
                && (*lupos_scx_idle_cpu_rq(cpu)).scx.local_dsq.nr == 0
                && (flags & SCX_PICK_IDLE_IN_NODE == 0 || waker_node == node)
                && !lupos_scx_idle_mask_empty(lupos_scx_idle_cpu_mask(waker_node))
                && lupos_scx_idle_mask_test(cpu, allowed) {
                scx_idle_test_and_clear_cpu(cpu);
                return cpu;
            }
        }
        if lupos_scx_idle_smt_active() {
            if is_prev_allowed && lupos_scx_idle_mask_test(prev_cpu, lupos_scx_idle_smt_mask(node))
                && scx_idle_test_and_clear_cpu(prev_cpu) {
                return prev_cpu;
            }
            if !llc_cpus.is_null() {
                let cpu = pick_idle_cpu_in_node(llc_cpus, node, SCX_PICK_IDLE_CORE);
                if cpu >= 0 { return cpu; }
            }
            if !numa_cpus.is_null() {
                let cpu = pick_idle_cpu_in_node(numa_cpus, node, SCX_PICK_IDLE_CORE);
                if cpu >= 0 { return cpu; }
            }
            let cpu = scx_pick_idle_cpu(allowed, node, flags | SCX_PICK_IDLE_CORE);
            if cpu >= 0 { return cpu; }
            if flags & SCX_PICK_IDLE_CORE != 0 { return -EBUSY; }
        }
        if is_prev_allowed && scx_idle_test_and_clear_cpu(prev_cpu) { return prev_cpu; }
        if lupos_scx_idle_smt_active() {
            let smt = lupos_scx_idle_siblings(prev_cpu);
            let mut cpu = lupos_scx_idle_mask_scan_and(0, smt, allowed);
            while cpu < lupos_scx_idle_mask_iter_limit() {
                if cpu as c_int != prev_cpu && scx_idle_test_and_clear_cpu(cpu as c_int) {
                    return cpu as c_int;
                }
                cpu = lupos_scx_idle_mask_scan_and(cpu + 1, smt, allowed);
            }
        }
        if !llc_cpus.is_null() {
            let cpu = pick_idle_cpu_in_node(llc_cpus, node, 0);
            if cpu >= 0 { return cpu; }
        }
        if !numa_cpus.is_null() {
            let cpu = pick_idle_cpu_in_node(numa_cpus, node, 0);
            if cpu >= 0 { return cpu; }
        }
        scx_pick_idle_cpu(allowed, node, flags)
    }
}

/// Allocate permanent global, possible-node and possible-CPU idle storage.
///
/// # Safety
/// Boot-time single invocation before readers, in a GFP_KERNEL-capable context.
/// Native BUG_ON allocation failures retain the pinned fatal-failure policy.
#[no_mangle]
pub unsafe extern "C" fn scx_idle_init_masks() {
    // SAFETY: Boot serialization prevents readers of partially allocated masks.
    unsafe {
        lupos_scx_idle_alloc_global();
        lupos_scx_idle_alloc_nodes();
        let mut node = lupos_scx_idle_first_node();
        while node < LUPOS_SCX_IDLE_MAX_NUMNODES as c_int {
            lupos_scx_idle_alloc_node(node);
            node = lupos_scx_idle_next_node(node);
        }
        let mut cpu = lupos_scx_idle_possible_scan(0);
        while cpu < lupos_scx_idle_cpu_iter_limit() {
            lupos_scx_idle_alloc_cpu(cpu as c_int);
            cpu = lupos_scx_idle_possible_scan(cpu + 1);
        }
    }
}

unsafe fn update_builtin_idle(cpu: c_int, idle: bool) {
    // SAFETY: CPU's rq lock is held; masks are permanent. SMT mask writes are
    // deliberately racy hints; only the native CPU bit claim is authoritative.
    unsafe {
        let node = scx_cpu_node_if_enabled(cpu);
        let idle_cpus = lupos_scx_idle_cpu_mask(node);
        lupos_scx_idle_mask_assign(cpu, idle_cpus, idle);
        if lupos_scx_idle_smt_active() {
            let smt = lupos_scx_idle_siblings(cpu);
            let idle_smts = lupos_scx_idle_smt_mask(node);
            if idle {
                if !lupos_scx_idle_mask_subset(smt, idle_cpus) { return; }
                lupos_scx_idle_mask_or(idle_smts, idle_smts, smt);
            } else {
                lupos_scx_idle_mask_andnot(idle_smts, idle_smts, smt);
            }
        }
    }
}

unsafe fn scx_idle_notify(rq: *mut rq, idle: bool, do_notify: bool, root_renotify: bool) {
    // SAFETY: rq lock/live-root contract pins scheduler tree; native capability
    // and per-CPU access preserve cfg. SCX_CALL_OP maintains nested callback
    // locked-rq state and the actual C function-pointer ABI.
    unsafe {
        let cpu = lupos_scx_idle_rq_cpu(rq);
        let cid = lupos_scx_idle_cpu_arg(cpu);
        let root = lupos_scx_idle_live_root();
        lupos_scx_idle_assert_rq(rq);
        if !lupos_scx_idle_has_subs() {
            if (do_notify || root_renotify) && lupos_scx_idle_has_update(root)
                && !lupos_scx_idle_bypassing(root, cpu) {
                lupos_scx_idle_call_update(root, rq, cid, idle);
            }
            return;
        }
        let mut pos = lupos_scx_idle_next_descendant(ptr::null_mut(), root);
        while !pos.is_null() {
            if lupos_scx_idle_missing_base(pos, cpu) {
                pos = lupos_scx_idle_skip_subtree(pos, root);
                continue;
            }
            let forced = if (*pos).level == 0 { root_renotify }
                else { lupos_scx_idle_take_renotify(pos, cpu) };
            if (do_notify || forced) && lupos_scx_idle_has_update(pos)
                && !lupos_scx_idle_bypassing(pos, cpu) {
                lupos_scx_idle_call_update(pos, rq, cid, idle);
            }
            pos = lupos_scx_idle_next_descendant(pos, root);
        }
    }
}

/// Refresh idle bits and send real or specifically owed idle notifications.
///
/// # Safety
/// rq is live with its native raw rq lock held and local IRQs disabled. The
/// live scheduler, CID tables and descendant tree remain protected by that
/// native scheduling context (including its RCU read-side protection). Calls
/// obey put/set-next and idle-to-idle re-pick semantics.
#[no_mangle]
pub unsafe extern "C" fn __scx_update_idle(rq: *mut rq, idle: bool, do_notify: bool) {
    // SAFETY: rq lock protects flags and callback ordering. Builtin bits become
    // visible before notifications, preserving enqueue/update interlocking.
    unsafe {
        let cpu = lupos_scx_idle_rq_cpu(rq);
        lupos_scx_idle_assert_rq(rq);
        if lupos_scx_idle_builtin_likely() { update_builtin_idle(cpu, idle); }
        let root_bit = LUPOS_SCX_IDLE_RENOTIFY_ROOT as c_uint;
        let owed_bits = root_bit | LUPOS_SCX_IDLE_RENOTIFY_SUB as c_uint;
        if do_notify || (idle && (*rq).scx.flags & owed_bits != 0) {
            let root_renotify = (*rq).scx.flags & root_bit != 0;
            (*rq).scx.flags &= !owed_bits;
            scx_idle_notify(rq, idle, do_notify, root_renotify);
        }
    }
}

unsafe fn reset_idle_masks(ops: *mut sched_ext_ops) {
    // SAFETY: Enable serialization/bypass excludes live tracking during reset.
    unsafe {
        if (*ops).flags & SCX_OPS_BUILTIN_IDLE_PER_NODE == 0 {
            lupos_scx_idle_mask_clear(lupos_scx_idle_cpu_mask(NUMA_NO_NODE));
            lupos_scx_idle_mask_clear(lupos_scx_idle_smt_mask(NUMA_NO_NODE));
            return;
        }
        let mut node = lupos_scx_idle_first_node();
        while node < LUPOS_SCX_IDLE_MAX_NUMNODES as c_int {
            lupos_scx_idle_mask_clear(lupos_scx_idle_cpu_mask(node));
            lupos_scx_idle_mask_clear(lupos_scx_idle_smt_mask(node));
            node = lupos_scx_idle_next_node(node);
        }
    }
}

/// Configure builtin/per-node tracking and start with all CPUs marked busy.
///
/// # Safety
/// ops and initialized masks are live under enable/hotplug serialization;
/// cpuslocked key updates require the native CPU read lock and bypass ordering.
#[no_mangle]
pub unsafe extern "C" fn scx_idle_enable(ops: *mut sched_ext_ops) {
    // SAFETY: Caller provides the same key-update and reset exclusion as idle.c.
    unsafe {
        lupos_scx_idle_set_builtin((*ops).update_idle.is_none()
            || (*ops).flags & SCX_OPS_KEEP_BUILTIN_IDLE != 0);
        lupos_scx_idle_set_per_node((*ops).flags & SCX_OPS_BUILTIN_IDLE_PER_NODE != 0);
        reset_idle_masks(ops);
    }
}

/// Disable builtin and per-node tracking keys.
///
/// # Safety
/// Caller holds scheduler disable serialization in a context allowed to sleep
/// for native static-key updates, after stopping live scheduling as in ext.c.
#[no_mangle]
pub unsafe extern "C" fn scx_idle_disable() {
    // SAFETY: The native leaf uses non-cpuslocked disable, matching the oracle.
    unsafe { lupos_scx_idle_disable_keys() };
}

unsafe fn validate_node(sch: *mut scx_sched, node: c_int) -> c_int {
    // SAFETY: sch is RCU-protected; validation precedes any node-array access.
    unsafe {
        if !lupos_scx_idle_per_node_likely() {
            lupos_scx_idle_error_per_node_disabled(sch);
            return -EOPNOTSUPP;
        }
        if node == NUMA_NO_NODE { return -ENOENT; }
        if node < 0 || (node as c_uint) >= lupos_scx_idle_nr_nodes() {
            lupos_scx_idle_error_invalid_node(sch, node);
            return -EINVAL;
        }
        if !lupos_scx_idle_node_possible(node) {
            lupos_scx_idle_error_unavailable_node(sch, node);
            return -EINVAL;
        }
        node
    }
}
unsafe fn check_builtin_idle_enabled(sch: *mut scx_sched) -> bool {
    // SAFETY: sch is live under RCU for native error reporting.
    unsafe {
        if lupos_scx_idle_builtin_likely() { return true; }
        lupos_scx_idle_error_disabled(sch);
        false
    }
}
unsafe fn is_bpf_migration_disabled(p: *const task_struct) -> bool {
    // SAFETY: p affinity/migration state is protected by the caller's lock;
    // only CONFIG_PREEMPT_RCU makes current's depth-one BPF prolog ambiguous.
    unsafe {
        if (*p).migration_disabled == 1 {
            if lupos_scx_idle_preempt_rcu() { return p != lupos_scx_idle_current(); }
            return true;
        }
        (*p).migration_disabled != 0
    }
}
unsafe fn select_cpu_from_kfunc(
    sch: *mut scx_sched, p: *mut task_struct, prev_cpu: c_int,
    wake_flags: u64, allowed: *const cpumask, flags: u64,
) -> c_int {
    // SAFETY: Native BPF verifier supplies live p/mask and kfunc context; the
    // outer RCU guard pins sch. Native BPF entry pins this CPU while sampling
    // this_rq and locked-rq context. Lock acquisition is restricted to a truly
    // unlocked call, and PiGuard releases after all selection guards.
    unsafe {
        if !lupos_scx_idle_cpu_valid(sch, prev_cpu) { return -EINVAL; }
        if !check_builtin_idle_enabled(sch) { return -EBUSY; }
        let _pi;
        if (*lupos_scx_idle_this_rq()).scx.in_select_cpu {
            if !lupos_scx_idle_task_ok(sch, p) { return -EINVAL; }
            lupos_scx_idle_assert_pi(p);
            _pi = None;
        } else if !lupos_scx_idle_locked_rq().is_null() {
            if lupos_scx_idle_task_rq(p) != lupos_scx_idle_locked_rq() {
                lupos_scx_idle_error_cross_task(sch, p);
                return -EINVAL;
            }
            _pi = None;
        } else {
            _pi = Some(PiGuard::lock(p));
        }
        let allowed = if allowed.is_null() { (*p).cpus_ptr } else { allowed };
        if (*p).nr_cpus_allowed == 1 || is_bpf_migration_disabled(p) {
            if lupos_scx_idle_mask_test(prev_cpu, allowed) && scx_idle_test_and_clear_cpu(prev_cpu) {
                prev_cpu
            } else { -EBUSY }
        } else {
            scx_select_cpu_dfl(p, prev_cpu, wake_flags, allowed, flags)
        }
    }
}

/// Resolve a validated CPU's NUMA node for BPF.
///
/// # Safety
/// aux is the implicit argument supplied by the native BPF kfunc thunk, in a
/// context permitted by the unchanged context filter. CPU is untrusted input.
#[export_name = "lupos_scx_idle_bpf_cpu_node"]
pub unsafe extern "C" fn scx_bpf_cpu_node(cpu: c_int, aux: *const bpf_prog_aux) -> c_int {
    // SAFETY: RCU pins resolved scheduler; validation precedes topology access.
    unsafe {
        let _rcu = RcuGuard::lock();
        let sch = lupos_scx_idle_prog_sched(aux);
        if lupos_scx_idle_unlikely_cpu_node(sch.is_null()) || !lupos_scx_idle_cpu_valid(sch, cpu) {
            return NUMA_NO_NODE;
        }
        lupos_scx_idle_cpu_node(cpu)
    }
}

/// Default BPF selection; preserve prev_cpu fallback and idle out-parameter.
///
/// # Safety
/// Native thunk/verifier supply live p, writable is_idle, valid implicit aux,
/// and an allowed STRUCT_OPS/SYSCALL context. Task locking is enforced inside.
#[export_name = "lupos_scx_idle_bpf_select_cpu_dfl"]
pub unsafe extern "C" fn scx_bpf_select_cpu_dfl(
    p: *mut task_struct, prev_cpu: c_int, wake_flags: u64,
    is_idle: *mut bool, aux: *const bpf_prog_aux,
) -> c_int {
    // SAFETY: RCU pins sch; the no-scheduler branch intentionally leaves
    // is_idle untouched, exactly as the pinned C source does.
    unsafe {
        let _rcu = RcuGuard::lock();
        let sch = lupos_scx_idle_prog_sched(aux);
        if lupos_scx_idle_unlikely_select_dfl(sch.is_null()) { return -ENODEV; }
        let cpu = select_cpu_from_kfunc(sch, p, prev_cpu, wake_flags, ptr::null(), 0);
        *is_idle = cpu >= 0;
        if cpu >= 0 { cpu } else { prev_cpu }
    }
}

/// Argument-wrapped BPF selection with an explicit allowed mask.
///
/// # Safety
/// Native thunk/verifier supply live p, allowed, args and implicit aux. Context
/// is a registered STRUCT_OPS/SYSCALL entry with KF_RCU and valid task lifetime.
#[export_name = "lupos_scx_idle_bpf_select_cpu_and"]
pub unsafe extern "C" fn __scx_bpf_select_cpu_and(
    p: *mut task_struct, allowed: *const cpumask,
    args: *mut scx_bpf_select_cpu_and_args, aux: *const bpf_prog_aux,
) -> c_int {
    // SAFETY: RCU pins sch; the argument structure uses the native definition.
    unsafe {
        let _rcu = RcuGuard::lock();
        let sch = lupos_scx_idle_prog_sched(aux);
        if lupos_scx_idle_unlikely_select_and(sch.is_null()) { return -ENODEV; }
        select_cpu_from_kfunc(sch, p, (*args).prev_cpu, (*args).wake_flags, allowed, (*args).flags)
    }
}

/// Compatibility selection interface retained until the oracle's v6.22 removal.
///
/// # Safety
/// Native verifier supplies live p/allowed and a permitted KF_RCU selection
/// context. No sub-scheduler may use this scheduler-identity-free interface.
/// If attached children trigger the early rejection, p's pi_lock or rq lock
/// must already be held for the retained native scx_task_sched(p) diagnostic;
/// this path does not reach the later lock acquisition. That precondition for
/// unlocked BPF callers remains an admission blocker, not satisfied by RCU.
#[export_name = "lupos_scx_idle_bpf_select_cpu_compat"]
pub unsafe extern "C" fn scx_bpf_select_cpu_and(
    p: *mut task_struct, prev_cpu: c_int, wake_flags: u64,
    allowed: *const cpumask, flags: u64,
) -> c_int {
    // SAFETY: RCU pins root and its child list; native cfg gates that access.
    // The early compatibility diagnostic additionally requires the caller's
    // task lock documented above; RCU does not replace scx_task_sched's lock.
    unsafe {
        let _rcu = RcuGuard::lock();
        let sch = lupos_scx_idle_root();
        if lupos_scx_idle_unlikely_select_compat(sch.is_null()) { return -ENODEV; }
        #[cfg(CONFIG_EXT_SUB_SCHED)]
        if lupos_scx_idle_unlikely_children(lupos_scx_idle_has_children(sch)) {
            lupos_scx_idle_error_compat(p);
            return -EINVAL;
        }
        select_cpu_from_kfunc(sch, p, prev_cpu, wake_flags, allowed, flags)
    }
}

/// Return a trusted, permanent per-node CPU idle mask.
///
/// # Safety
/// Called through the KF_ACQUIRE native thunk with valid implicit aux/context.
/// The returned native mask must be treated read-only and released via put.
#[export_name = "lupos_scx_idle_bpf_get_idle_cpumask_node"]
pub unsafe extern "C" fn scx_bpf_get_idle_cpumask_node(node: c_int, aux: *const bpf_prog_aux) -> *const cpumask {
    // SAFETY: RCU pins sch; validate_node checks enabled key, bounds and holes.
    unsafe {
        let _rcu = RcuGuard::lock();
        let sch = lupos_scx_idle_prog_sched(aux);
        if lupos_scx_idle_unlikely_get_node(sch.is_null()) { return lupos_scx_idle_none_mask(); }
        let node = validate_node(sch, node);
        if node < 0 { return lupos_scx_idle_none_mask(); }
        lupos_scx_idle_cpu_mask(node)
    }
}

/// Return a trusted, permanent global CPU idle mask.
///
/// # Safety
/// Native KF_ACQUIRE thunk supplies aux/context; caller reads only and pairs
/// this acquire with put. Global lookup rejects per-node tracking mode.
#[export_name = "lupos_scx_idle_bpf_get_idle_cpumask"]
pub unsafe extern "C" fn scx_bpf_get_idle_cpumask(aux: *const bpf_prog_aux) -> *const cpumask {
    // SAFETY: RCU pins sch for key validation and diagnostics.
    unsafe {
        let _rcu = RcuGuard::lock();
        let sch = lupos_scx_idle_prog_sched(aux);
        if lupos_scx_idle_unlikely_get(sch.is_null()) { return lupos_scx_idle_none_mask(); }
        if lupos_scx_idle_per_node_unlikely() {
            lupos_scx_idle_error_per_node_mask(sch);
            return lupos_scx_idle_none_mask();
        }
        if !check_builtin_idle_enabled(sch) { return lupos_scx_idle_none_mask(); }
        lupos_scx_idle_cpu_mask(NUMA_NO_NODE)
    }
}

/// Return a trusted per-node wholly idle core mask (CPU mask on non-SMT).
///
/// # Safety
/// Native KF_ACQUIRE thunk supplies aux/context; returned storage is read-only
/// and permanent, with verifier acquire/release pairing required.
#[export_name = "lupos_scx_idle_bpf_get_idle_smtmask_node"]
pub unsafe extern "C" fn scx_bpf_get_idle_smtmask_node(node: c_int, aux: *const bpf_prog_aux) -> *const cpumask {
    // SAFETY: RCU and node validation precede access to native permanent masks.
    unsafe {
        let _rcu = RcuGuard::lock();
        let sch = lupos_scx_idle_prog_sched(aux);
        if lupos_scx_idle_unlikely_smt_node(sch.is_null()) { return lupos_scx_idle_none_mask(); }
        let node = validate_node(sch, node);
        if node < 0 { return lupos_scx_idle_none_mask(); }
        if lupos_scx_idle_smt_active() { lupos_scx_idle_smt_mask(node) }
        else { lupos_scx_idle_cpu_mask(node) }
    }
}

/// Return a trusted global wholly idle core mask (CPU mask on non-SMT).
///
/// # Safety
/// Native KF_ACQUIRE thunk supplies aux/context; returned mask is read-only,
/// paired with put, and cannot be used as a per-node mask.
#[export_name = "lupos_scx_idle_bpf_get_idle_smtmask"]
pub unsafe extern "C" fn scx_bpf_get_idle_smtmask(aux: *const bpf_prog_aux) -> *const cpumask {
    // SAFETY: RCU pins sch for key validation and diagnostics.
    unsafe {
        let _rcu = RcuGuard::lock();
        let sch = lupos_scx_idle_prog_sched(aux);
        if lupos_scx_idle_unlikely_smt(sch.is_null()) { return lupos_scx_idle_none_mask(); }
        if lupos_scx_idle_per_node_unlikely() {
            lupos_scx_idle_error_per_node_mask(sch);
            return lupos_scx_idle_none_mask();
        }
        if !check_builtin_idle_enabled(sch) { return lupos_scx_idle_none_mask(); }
        if lupos_scx_idle_smt_active() { lupos_scx_idle_smt_mask(NUMA_NO_NODE) }
        else { lupos_scx_idle_cpu_mask(NUMA_NO_NODE) }
    }
}

/// Complete the verifier-only idle-mask reference lifetime.
///
/// # Safety
/// mask is a read-only pointer obtained from a matching native KF_ACQUIRE idle
/// kfunc, consumed by the KF_RELEASE thunk. Storage is permanent, never freed.
#[export_name = "lupos_scx_idle_bpf_put_idle_cpumask"]
pub unsafe extern "C" fn scx_bpf_put_idle_cpumask(_mask: *const cpumask) {
    // Intentionally empty in the pinned C source: no runtime reference exists.
}

/// Validate and atomically claim a CPU's idle bit.
///
/// # Safety
/// Native thunk supplies implicit aux and a registered kfunc context. cpu may
/// be untrusted and is validated before any mask access.
#[export_name = "lupos_scx_idle_bpf_test_and_clear_cpu_idle"]
pub unsafe extern "C" fn scx_bpf_test_and_clear_cpu_idle(cpu: c_int, aux: *const bpf_prog_aux) -> bool {
    // SAFETY: RCU pins sch while validation and diagnostics run.
    unsafe {
        let _rcu = RcuGuard::lock();
        let sch = lupos_scx_idle_prog_sched(aux);
        if lupos_scx_idle_unlikely_claim(sch.is_null()) { return false; }
        if !check_builtin_idle_enabled(sch) { return false; }
        if !lupos_scx_idle_cpu_valid(sch, cpu) { return false; }
        scx_idle_test_and_clear_cpu(cpu)
    }
}

/// Pick/claim an idle CPU starting at a validated NUMA node.
///
/// # Safety
/// Native KF_RCU thunk supplies live allowed mask, valid aux/context. node is
/// validated here; returned CPU is only a racy idle claim, not a dispatch.
#[export_name = "lupos_scx_idle_bpf_pick_idle_cpu_node"]
pub unsafe extern "C" fn scx_bpf_pick_idle_cpu_node(
    allowed: *const cpumask, node: c_int, flags: u64, aux: *const bpf_prog_aux,
) -> c_int {
    // SAFETY: RCU also pins nearest-node topology for a cross-node search.
    unsafe {
        let _rcu = RcuGuard::lock();
        let sch = lupos_scx_idle_prog_sched(aux);
        if lupos_scx_idle_unlikely_pick_node(sch.is_null()) { return -ENODEV; }
        let node = validate_node(sch, node);
        if node < 0 { return node; }
        scx_pick_idle_cpu(allowed, node, flags)
    }
}

/// Pick/claim an idle CPU from the global allowed mask.
///
/// # Safety
/// Native KF_RCU thunk supplies live allowed mask and valid aux/context.
/// Global lookup is unavailable in per-node mode or without builtin tracking.
#[export_name = "lupos_scx_idle_bpf_pick_idle_cpu"]
pub unsafe extern "C" fn scx_bpf_pick_idle_cpu(
    allowed: *const cpumask, flags: u64, aux: *const bpf_prog_aux,
) -> c_int {
    // SAFETY: RCU pins sch; key checks precede global idle-mask access.
    unsafe {
        let _rcu = RcuGuard::lock();
        let sch = lupos_scx_idle_prog_sched(aux);
        if lupos_scx_idle_unlikely_pick(sch.is_null()) { return -ENODEV; }
        if lupos_scx_idle_per_node_maybe() {
            lupos_scx_idle_error_per_node_enabled(sch);
            return -EBUSY;
        }
        if !check_builtin_idle_enabled(sch) { return -EBUSY; }
        scx_pick_idle_cpu(allowed, NUMA_NO_NODE, flags)
    }
}

/// Pick an idle CPU, otherwise any allowed CPU, with node restriction honored.
///
/// # Safety
/// Native KF_RCU thunk supplies live allowed mask and valid aux/context. Node
/// validation precedes idle lookup and native cpumask_of_node access.
#[export_name = "lupos_scx_idle_bpf_pick_any_cpu_node"]
pub unsafe extern "C" fn scx_bpf_pick_any_cpu_node(
    allowed: *const cpumask, node: c_int, flags: u64, aux: *const bpf_prog_aux,
) -> c_int {
    // SAFETY: RCU pins scheduler and NUMA topology throughout both attempts.
    unsafe {
        let _rcu = RcuGuard::lock();
        let sch = lupos_scx_idle_prog_sched(aux);
        if lupos_scx_idle_unlikely_any_node(sch.is_null()) { return -ENODEV; }
        let node = validate_node(sch, node);
        if node < 0 { return node; }
        let cpu = scx_pick_idle_cpu(allowed, node, flags);
        if cpu >= 0 { return cpu; }
        let cpu = if flags & SCX_PICK_IDLE_IN_NODE != 0 {
            lupos_scx_idle_mask_any_and(lupos_scx_idle_node_mask(node), allowed)
        } else { lupos_scx_idle_mask_any(allowed) };
        if cpu < lupos_scx_idle_nr_cpus() { cpu as c_int } else { -EBUSY }
    }
}

/// Pick an idle CPU, otherwise any allowed CPU, in global tracking mode.
///
/// # Safety
/// Native KF_RCU thunk supplies live allowed mask and valid implicit aux. Any
/// returned CPU obeys allowed; busy fallback does not itself issue a kick.
#[export_name = "lupos_scx_idle_bpf_pick_any_cpu"]
pub unsafe extern "C" fn scx_bpf_pick_any_cpu(
    allowed: *const cpumask, flags: u64, aux: *const bpf_prog_aux,
) -> c_int {
    // SAFETY: RCU pins sch; unlike pick_idle, disabled builtin tracking still
    // permits the documented busy fallback without reporting a tracking error.
    unsafe {
        let _rcu = RcuGuard::lock();
        let sch = lupos_scx_idle_prog_sched(aux);
        if lupos_scx_idle_unlikely_any(sch.is_null()) { return -ENODEV; }
        if lupos_scx_idle_per_node_maybe() {
            lupos_scx_idle_error_per_node_enabled(sch);
            return -EBUSY;
        }
        if lupos_scx_idle_builtin_likely() {
            let cpu = scx_pick_idle_cpu(allowed, NUMA_NO_NODE, flags);
            if cpu >= 0 { return cpu; }
        }
        let cpu = lupos_scx_idle_mask_any(allowed);
        if cpu < lupos_scx_idle_nr_cpus() { cpu as c_int } else { -EBUSY }
    }
}

/// Register native idle kfunc and restricted select-CPU metadata sets.
///
/// # Safety
/// Single initialization invocation after native BTF setup, with both sets in
/// the required translation unit. No prior registration may be duplicated.
#[no_mangle]
pub unsafe extern "C" fn scx_idle_init() -> c_int {
    // SAFETY: Native leaves register the exact original tables/program types;
    // preserve first-error short circuit and exclude selection from TRACING.
    unsafe {
        let ret = lupos_scx_idle_register_ops();
        if ret != 0 { return ret; }
        let ret = lupos_scx_idle_register_tracing();
        if ret != 0 { return ret; }
        let ret = lupos_scx_idle_register_syscall();
        if ret != 0 { return ret; }
        let ret = lupos_scx_idle_register_select_ops();
        if ret != 0 { return ret; }
        lupos_scx_idle_register_select_syscall()
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
// Above marker is inherited provenance, not the authority for this repair.
