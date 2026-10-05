// SPDX-License-Identifier: GPL-2.0
// F16 query/event owners, pinned ext.c:10070-10658 at
// 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. Existing ext.rs identities stay put.
// Native primitives and guard/kfunc envelopes remain unqualified C runtime.
compile_error!("SOURCE ONLY HOLD: sched_ext query/event ABI, race and protection qualification incomplete");

use super::*;
use core::{mem::MaybeUninit, ptr};
use kernel::ffi::c_int;

/// Maximum relative CPU capacity, with the original invalid-scheduler fallback.
///
/// # Safety
/// The native kfunc wrapper holds guard(rcu) over this entire body. aux is the
/// verifier-supplied live association and the CPU argument may be invalid.
#[export_name = "lupos_scx_core_qe_cpuperf_cap_body"]
pub unsafe extern "C" fn scx_bpf_cpuperf_cap(cpu: i32, aux: *const bpf_prog_aux) -> u32 {
    // SAFETY: Validate the CPU before invoking its native architecture hook.
    unsafe {
        let sch = lupos_scx_core_qe_prog_sched(aux);
        if lupos_scx_core_qe_likely_sched(sch) && scx_cpu_valid(sch, cpu, ptr::null()) {
            lupos_scx_core_qe_arch_capacity(cpu)
        } else {
            SCX_CPUPERF_ONE as u32
        }
    }
}

/// CID-addressed maximum relative capacity.
///
/// # Safety
/// Called only inside the native kfunc's RCU guard, pinning scheduler and CID
/// tables. The CID owner performs its original validation and diagnostics.
#[export_name = "lupos_scx_core_qe_cidperf_cap_body"]
pub unsafe extern "C" fn scx_bpf_cidperf_cap(cid: i32, aux: *const bpf_prog_aux) -> u32 {
    // SAFETY: No table or architecture read precedes scheduler/CID validation.
    unsafe {
        let sch = lupos_scx_core_qe_prog_sched(aux);
        if lupos_scx_core_qe_unlikely_no_sched(sch) {
            return SCX_CPUPERF_ONE as u32;
        }
        let cpu = lupos_scx_core_qe_cid_to_cpu(sch, cid);
        if cpu < 0 {
            return SCX_CPUPERF_ONE as u32;
        }
        lupos_scx_core_qe_arch_capacity(cpu)
    }
}

/// Current relative CPU performance.
///
/// # Safety
/// The native kfunc wrapper holds its original RCU guard and supplies live aux.
#[export_name = "lupos_scx_core_qe_cpuperf_cur_body"]
pub unsafe extern "C" fn scx_bpf_cpuperf_cur(cpu: i32, aux: *const bpf_prog_aux) -> u32 {
    // SAFETY: Invalid scheduler/CPU keeps the original SCX_CPUPERF_ONE result.
    unsafe {
        let sch = lupos_scx_core_qe_prog_sched(aux);
        if lupos_scx_core_qe_likely_sched(sch) && scx_cpu_valid(sch, cpu, ptr::null()) {
            lupos_scx_core_qe_arch_frequency(cpu)
        } else {
            SCX_CPUPERF_ONE as u32
        }
    }
}

/// CID-addressed current relative performance.
///
/// # Safety
/// The native kfunc RCU guard pins the association and CID table for this body.
#[export_name = "lupos_scx_core_qe_cidperf_cur_body"]
pub unsafe extern "C" fn scx_bpf_cidperf_cur(cid: i32, aux: *const bpf_prog_aux) -> u32 {
    // SAFETY: Preserve CID error handling before the frequency-capacity hook.
    unsafe {
        let sch = lupos_scx_core_qe_prog_sched(aux);
        if lupos_scx_core_qe_unlikely_no_sched(sch) {
            return SCX_CPUPERF_ONE as u32;
        }
        let cpu = lupos_scx_core_qe_cid_to_cpu(sch, cid);
        if cpu < 0 {
            return SCX_CPUPERF_ONE as u32;
        }
        lupos_scx_core_qe_arch_frequency(cpu)
    }
}

/// Validate and apply a cpuperf target without acquiring a second rq lock.
///
/// # Safety
/// sch is RCU-pinned by the calling kfunc. Current SCX locked-rq tracking must
/// match the native callback context; arbitrary supplied CPU/perf are checked.
/// The unlocked path is allowed to acquire the target rq with IRQ save/restore.
pub(crate) unsafe fn scx_cpuperf_set(sch: *mut scx_sched, cpu: i32, perf: u32) -> i32 {
    // SAFETY: Reject perf, invalid CPU and ABBA lock combinations before writes.
    // The native unlocked envelope uses a stack-native rq_flags and the original
    // explicit rq_lock_irqsave/update_rq_clock/rq_unlock_irqrestore sequence.
    unsafe {
        if lupos_scx_core_qe_unlikely_bad_perf(perf) {
            lupos_scx_core_qe_error_perf(sch, perf, cpu);
            return -(EINVAL as i32);
        }
        if !scx_cpu_valid(sch, cpu, ptr::null()) {
            return -(EINVAL as i32);
        }
        let rq = lupos_scx_core_qe_cpu_rq(cpu);
        let locked_rq = lupos_scx_core_locked_rq();
        if !locked_rq.is_null() && rq != locked_rq {
            lupos_scx_core_qe_error_target(sch, cpu);
            return -(EINVAL as i32);
        }
        if locked_rq.is_null() {
            lupos_scx_core_qe_cpuperf_unlocked(sch, cpu, perf, rq)
        } else {
            lupos_scx_core_qe_cpuperf_locked_body(sch, cpu, perf, rq)
        }
    }
}

/// Original capability test and write while the corresponding rq is locked.
///
/// # Safety
/// sch is live, CPU/perf are validated, rq is cpu_rq(cpu), and its lock is held.
/// IRQ/preempt context is unchanged from the existing-lock or explicit-lock
/// native path; ecaps revocation is serialized by this same rq lock.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_qe_cpuperf_locked_body(
    sch: *mut scx_sched, cpu: i32, perf: u32, rq: *mut rq,
) -> i32 {
    // SAFETY: Capability check remains under rq lock, followed immediately by
    // the native plain target assignment and cpufreq util update. The rq lock
    // does not exclude schedutil's sibling-CPU read under a policy lock. Both
    // access sides stay native; their race semantics remain unqualified.
    // Denial accounts the original current-CPU event and native trace macro.
    unsafe {
        if lupos_scx_core_qe_likely_perf_allowed(sch, cpu) {
            lupos_scx_core_qe_cpuperf_target_write(rq, perf);
            lupos_scx_core_qe_cpufreq_update(rq);
            0
        } else {
            lupos_scx_core_qe_event_perf_denied(sch);
            -(EACCES as i32)
        }
    }
}

/// CPU-addressed void setter: intentionally discards the helper's error code.
///
/// # Safety
/// Called only by the native kfunc wrapper holding its RCU guard around aux.
#[export_name = "lupos_scx_core_qe_cpuperf_set_body"]
pub unsafe extern "C" fn scx_bpf_cpuperf_set(cpu: i32, perf: u32, aux: *const bpf_prog_aux) {
    // SAFETY: No scheduler means no validation/error or mutation, as in C.
    unsafe {
        let sch = lupos_scx_core_qe_prog_sched(aux);
        if lupos_scx_core_qe_unlikely_no_sched(sch) {
            return;
        }
        scx_cpuperf_set(sch, cpu, perf);
    }
}

/// CID setter retains ENODEV, CID conversion errors and capability errors.
///
/// # Safety
/// The native kfunc RCU guard pins the scheduler association and CID tables.
#[export_name = "lupos_scx_core_qe_cidperf_set_body"]
pub unsafe extern "C" fn scx_bpf_cidperf_set(cid: i32, perf: u32, aux: *const bpf_prog_aux) -> i32 {
    // SAFETY: CID resolution precedes perf validation, matching the C order.
    unsafe {
        let sch = lupos_scx_core_qe_prog_sched(aux);
        if lupos_scx_core_qe_unlikely_no_sched(sch) {
            return -(ENODEV as i32);
        }
        let cpu = lupos_scx_core_qe_cid_to_cpu(sch, cid);
        if cpu < 0 {
            return cpu;
        }
        scx_cpuperf_set(sch, cpu, perf)
    }
}

/// Native configured node-ID bound; no mask weight or topology substitute.
///
/// # Safety
/// Native topology initialization required by the original kfunc is complete.
#[export_name = "lupos_scx_core_qe_nr_node_ids_body"]
pub unsafe extern "C" fn scx_bpf_nr_node_ids() -> u32 {
    // SAFETY: Native nr_node_ids handles all configured node-map alternatives.
    unsafe { lupos_scx_core_qe_nr_node_ids() }
}

/// Native CPU-ID bound.
///
/// # Safety
/// CPU topology is initialized as required by the original kfunc.
#[export_name = "lupos_scx_core_qe_nr_cpu_ids_body"]
pub unsafe extern "C" fn scx_bpf_nr_cpu_ids() -> u32 {
    // SAFETY: Single F00 primitive reads the original configured nr_cpu_ids.
    unsafe { lupos_scx_core_nr_cpu_ids() }
}

/// CID-space size is num_possible_cpus(), not nr_cpu_ids.
///
/// # Safety
/// The original native CPU topology initialization is complete.
#[export_name = "lupos_scx_core_qe_nr_cids_body"]
pub unsafe extern "C" fn scx_bpf_nr_cids() -> u32 {
    // SAFETY: F00 uses the exact native helper, including the UP alternative.
    unsafe { lupos_scx_core_num_possible_cpus() }
}

/// Current online CID count retains the native raw atomic / UP read.
///
/// # Safety
/// Caller accepts the original unsnapshotted hotplug-observable count.
#[export_name = "lupos_scx_core_qe_nr_online_cids_body"]
pub unsafe extern "C" fn scx_bpf_nr_online_cids() -> u32 {
    // SAFETY: The native helper is not replaced by an iteration or cached count.
    unsafe { lupos_scx_core_qe_num_online_cpus() }
}

/// Read this CPU's CID, returning EINVAL before table publication.
///
/// # Safety
/// Called in the original BPF CPU context under the native kfunc RCU guard.
/// The guard pins table retirement; raw_smp_processor_id is intentionally kept.
#[export_name = "lupos_scx_core_qe_this_cid_body"]
pub unsafe extern "C" fn scx_bpf_this_cid() -> i32 {
    // SAFETY: Index only a non-NULL table; native CID initialization sets its
    // complete CPU geometry. The signed s16 result is sign-extended to s32.
    unsafe {
        let tbl = lupos_scx_core_qe_this_cid_table_rcu();
        if tbl.is_null() {
            return -(EINVAL as i32);
        }
        *tbl.add(lupos_scx_core_qe_raw_cpu() as usize) as i32
    }
}

/// Return the immortal possible-CPU mask with native kfunc acquire metadata.
///
/// # Safety
/// Native global CPU masks are initialized; caller must not mutate this mask.
#[export_name = "lupos_scx_core_qe_get_possible_cpumask_body"]
pub unsafe extern "C" fn scx_bpf_get_possible_cpumask() -> *const cpumask {
    // SAFETY: Returning this pointer does not take a runtime reference.
    unsafe { lupos_scx_core_qe_possible_cpumask() }
}

/// Return the native online mask, with its original hotplug visibility.
///
/// # Safety
/// Caller follows the verifier's trusted-pointer rules and never writes it.
#[export_name = "lupos_scx_core_qe_get_online_cpumask_body"]
pub unsafe extern "C" fn scx_bpf_get_online_cpumask() -> *const cpumask {
    // SAFETY: Native global mask storage is immortal and only borrowed here.
    unsafe { lupos_scx_core_qe_online_cpumask() }
}

/// Release only the verifier's trust/reference token for an immortal mask.
///
/// # Safety
/// The pointer satisfies this kfunc's KF_RELEASE contract. The pinned original
/// function is deliberately empty: there is no runtime refcount to decrement.
#[export_name = "lupos_scx_core_qe_put_cpumask_body"]
pub unsafe extern "C" fn scx_bpf_put_cpumask(_cpumask: *const cpumask) {}

/// Plain task_rq(p)->curr comparison, preserving the configured rq union.
///
/// # Safety
/// The task is live under KF_RCU. This observes the native race-tolerant plain
/// query and does not claim a stable run-state snapshot or add synchronization.
#[export_name = "lupos_scx_core_qe_task_running_body"]
pub unsafe extern "C" fn scx_bpf_task_running(p: *const task_struct) -> bool {
    // SAFETY: Native field access preserves task_rq evaluation and a plain curr
    // read. It must not become the RCU or READ_ONCE current-task query below.
    unsafe { lupos_scx_core_qe_task_rq_curr_plain(p) == p as *mut task_struct }
}

/// Query the native task CPU (including its SMP/UP helper alternatives).
///
/// # Safety
/// p is live under the original kfunc KF_RCU lifetime contract.
#[export_name = "lupos_scx_core_qe_task_cpu_body"]
pub unsafe extern "C" fn scx_bpf_task_cpu(p: *const task_struct) -> i32 {
    // SAFETY: Preserve the native task_cpu inline, including its access policy.
    unsafe { lupos_scx_core_qe_task_cpu(p) as i32 }
}

/// CID of the task's current CPU under a separate table-lifetime RCU guard.
///
/// # Safety
/// The native wrapper holds guard(rcu) even for sleepable callers: KF_RCU alone
/// pins p, not the global CID table. p remains live over this synchronous call.
#[export_name = "lupos_scx_core_qe_task_cid_body"]
pub unsafe extern "C" fn scx_bpf_task_cid(p: *const task_struct) -> i32 {
    // SAFETY: Check publication before task_cpu evaluation and signed indexing.
    unsafe {
        let tbl = lupos_scx_core_qe_task_cid_table_rcu();
        if tbl.is_null() {
            return -(EINVAL as i32);
        }
        *tbl.add(lupos_scx_core_qe_task_cpu(p) as usize) as i32
    }
}

/// Return the SCX-tracked locked rq or issue the original scheduler error.
///
/// # Safety
/// The native wrapper holds guard(preempt), not a newly added RCU guard. aux
/// and per-CPU lock tracking obey the original BPF/SCX callback lifetime rules.
#[export_name = "lupos_scx_core_qe_locked_rq_body"]
pub unsafe extern "C" fn scx_bpf_locked_rq(aux: *const bpf_prog_aux) -> *mut rq {
    // SAFETY: Association validation precedes per-CPU tracking access.
    unsafe {
        let sch = lupos_scx_core_qe_prog_sched(aux);
        if lupos_scx_core_qe_unlikely_no_sched(sch) {
            return ptr::null_mut();
        }
        let rq = lupos_scx_core_locked_rq();
        if rq.is_null() {
            lupos_scx_core_qe_error_unlocked_rq(sch);
            return ptr::null_mut();
        }
        rq
    }
}

/// RCU-observe a remote CPU's curr after scheduler/CPU validation.
///
/// # Safety
/// Native wrapper holds the original inner RCU guard; the caller's outer
/// KF_RCU_PROTECTED lifetime must keep the returned task usable after return.
#[export_name = "lupos_scx_core_qe_cpu_curr_body"]
pub unsafe extern "C" fn scx_bpf_cpu_curr(cpu: i32, aux: *const bpf_prog_aux) -> *mut task_struct {
    // SAFETY: This uses rcu_dereference on the actual native rq curr member.
    unsafe {
        let sch = lupos_scx_core_qe_prog_sched(aux);
        if lupos_scx_core_qe_unlikely_no_sched(sch) {
            return ptr::null_mut();
        }
        if !scx_cpu_valid(sch, cpu, ptr::null()) {
            return ptr::null_mut();
        }
        lupos_scx_core_qe_cpu_curr_rcu(cpu)
    }
}

/// CID-addressed remote current-task query.
///
/// # Safety
/// Same native inner/outer RCU contract as scx_bpf_cpu_curr, plus CID tables
/// pinned by the inner guard until the native curr dereference completes.
#[export_name = "lupos_scx_core_qe_cid_curr_body"]
pub unsafe extern "C" fn scx_bpf_cid_curr(cid: i32, aux: *const bpf_prog_aux) -> *mut task_struct {
    // SAFETY: Mapping errors return NULL without accessing a runqueue.
    unsafe {
        let sch = lupos_scx_core_qe_prog_sched(aux);
        if lupos_scx_core_qe_unlikely_no_sched(sch) {
            return ptr::null_mut();
        }
        let cpu = lupos_scx_core_qe_cid_to_cpu(sch, cid);
        if cpu < 0 {
            return ptr::null_mut();
        }
        lupos_scx_core_qe_cid_curr_rcu(cpu)
    }
}

/// Lookup by SCX TID without taking a task reference or an extra RCU guard.
///
/// # Safety
/// KF_RCU_PROTECTED supplies the caller's RCU section, covering both the
/// hash lookup and subsequent use of a non-NULL borrowed result.
#[export_name = "lupos_scx_core_qe_tid_to_task_body"]
pub unsafe extern "C" fn scx_bpf_tid_to_task(tid: u64) -> *mut task_struct {
    // SAFETY: F00 owns the key/hash/params. Native container_of uses the actual
    // task_struct scx offset. Disabled lookup emits an error only if root exists.
    unsafe {
        if !scx_tid_to_task_enabled() {
            let sch = lupos_scx_core_qe_root_rcu();
            if !sch.is_null() {
                lupos_scx_core_qe_error_tid_disabled(sch);
            }
            return ptr::null_mut();
        }
        let scx = lupos_scx_core_tid_lookup(tid);
        if scx.is_null() {
            return ptr::null_mut();
        }
        lupos_scx_core_qe_entity_task(scx)
    }
}

/// Read a valid cached rq clock or sample a fresh CPU scheduler clock.
///
/// # Safety
/// Caller is on rq's CPU with preemption disabled, or holds its native rq lock.
/// rq remains live. Cross-CPU clock monotonicity is deliberately not promised.
#[no_mangle]
pub unsafe extern "C" fn __scx_bpf_now(rq: *mut rq) -> u64 {
    // SAFETY: The exact diagnostic precedes the acquire flags read. Cached
    // clock uses READ_ONCE; fallback does not set CLK_VALID or write the cache.
    unsafe {
        lupos_scx_core_qe_assert_now(rq);
        if lupos_scx_core_qe_flags_acquire(rq) & SCX_RQ_CLK_VALID as u32 != 0 {
            lupos_scx_core_qe_clock_read_once(rq)
        } else {
            lupos_scx_core_qe_sched_clock_cpu(rq)
        }
    }
}

/// High-performance current-CPU time under the native preemption guard.
///
/// # Safety
/// Called only by the native kfunc while guard(preempt) pins this CPU.
#[export_name = "lupos_scx_core_qe_now_body"]
pub unsafe extern "C" fn scx_bpf_now() -> u64 {
    // SAFETY: this_rq is evaluated after preemption is disabled, as in C.
    unsafe { __scx_bpf_now(lupos_scx_core_qe_this_rq()) }
}

/// Aggregate all possible-CPU event counters in native SCX_EVENTS_LIST order.
///
/// # Safety
/// sch and every possible-CPU pcpu allocation are pinned by the original
/// caller's scheduler lifetime protocol. events is writable native-sized output
/// storage, disjoint from the per-CPU sources. No atomic global snapshot is
/// promised. Native memset fully initializes every byte before any field read.
pub(crate) unsafe fn scx_read_events(sch: *mut scx_sched, events: *mut scx_event_stats) {
    // SAFETY: Iterate the exact native for_each_possible_cpu condition, including
    // NR_CPUS==1. Each source field is one READ_ONCE(s64), then added in Rust at
    // native s64 width using explicit kernel-style signed wrapping semantics.
    unsafe {
        lupos_scx_core_qe_zero_events(events);
        let mut cpu: c_int = 0;
        while lupos_scx_core_qe_possible_cpu_condition(&mut cpu) {
            let e_cpu = lupos_scx_core_qe_cpu_events(sch, cpu);
            macro_rules! add_event {
                ($name:ident) => {
                    (*events).$name = (*events).$name.wrapping_add(
                        lupos_scx_core_qe_read_event(ptr::addr_of!((*e_cpu).$name)),
                    );
                };
            }
            // Exact internal.h:1262-1283 SCX_EVENTS_LIST, not an array-layout cast.
            add_event!(SCX_EV_SELECT_CPU_FALLBACK);
            add_event!(SCX_EV_DISPATCH_LOCAL_DSQ_OFFLINE);
            add_event!(SCX_EV_DISPATCH_KEEP_LAST);
            add_event!(SCX_EV_ENQ_SKIP_EXITING);
            add_event!(SCX_EV_ENQ_SKIP_MIGRATION_DISABLED);
            add_event!(SCX_EV_REENQ_IMMED);
            add_event!(SCX_EV_REENQ_REPEAT);
            add_event!(SCX_EV_REFILL_SLICE_DFL);
            add_event!(SCX_EV_SLICE_CLAMPED);
            add_event!(SCX_EV_SLICE_DENIED);
            add_event!(SCX_EV_BYPASS_DURATION);
            add_event!(SCX_EV_BYPASS_DISPATCH);
            add_event!(SCX_EV_BYPASS_ACTIVATE);
            add_event!(SCX_EV_INSERT_NOT_OWNED);
            add_event!(SCX_EV_SUB_BYPASS_DISPATCH);
            add_event!(SCX_EV_SUB_FORCED_ADMIT);
            add_event!(SCX_EV_SUB_PREEMPT_DENIED);
            add_event!(SCX_EV_SUB_KICK_DENIED);
            add_event!(SCX_EV_SUB_REENQ_DENIED);
            add_event!(SCX_EV_SUB_CIDPERF_DENIED);
            add_event!(SCX_EV_SUB_RESCUE);
            cpu = cpu.wrapping_add(1);
        }
    }
}

/// Fill the original stack event snapshot while explicit RCU protection is held.
///
/// # Safety
/// Called only by the native explicit rcu_read_lock/unlock envelope. e_sys is
/// writable complete native storage; aux lives for this synchronous BPF call.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_qe_events_rcu_body(
    e_sys: *mut scx_event_stats, aux: *const bpf_prog_aux,
) {
    // SAFETY: Both branches initialize all bytes, including native padding.
    unsafe {
        let sch = lupos_scx_core_qe_prog_sched(aux);
        if !sch.is_null() {
            scx_read_events(sch, e_sys);
        } else {
            lupos_scx_core_qe_zero_events(e_sys);
        }
    }
}

/// Read scheduler events and copy only the ABI-compatible requested prefix.
///
/// # Safety
/// events points to the verifier-authorized writable events__sz buffer. aux is
/// valid for the kfunc. Prefix copy occurs after the original RCU unlock.
#[export_name = "lupos_scx_core_qe_events_body"]
pub unsafe extern "C" fn scx_bpf_events(
    events: *mut scx_event_stats, events__sz: usize, aux: *const bpf_prog_aux,
) {
    // SAFETY: The native sizeof determines the bound; no fabricated layout or
    // reference to uninitialized data is formed. Native RCU fill initializes
    // the whole snapshot before this possibly zero-length native memcpy.
    unsafe {
        let mut e_sys = MaybeUninit::<scx_event_stats>::uninit();
        lupos_scx_core_qe_events_rcu(e_sys.as_mut_ptr(), aux);
        let events__sz = core::cmp::min(events__sz, lupos_scx_core_qe_events_size());
        lupos_scx_core_qe_copy_events(events, e_sys.as_ptr(), events__sz);
    }
}

/// Task cgroup with the original pre-guard task_group/default-cgroup snapshots.
///
/// # Safety
/// The native wrapper snapshots p->sched_task_group and default cgroup before
/// guard(rcu), then supplies them here with live p/aux. KF_RCU pins p; SCX's task
/// argument check gates tg_cgrp. The returned pointer gains one cgroup reference.
#[cfg(CONFIG_CGROUP_SCHED)]
#[export_name = "lupos_scx_core_qe_task_cgroup_body"]
pub unsafe extern "C" fn scx_bpf_task_cgroup(
    p: *mut task_struct, aux: *const bpf_prog_aux,
    tg: *mut task_group, mut cgrp: *mut cgroup,
) -> *mut cgroup {
    // SAFETY: Hidden default-y EXT_GROUP_SCHED under SCHED_CLASS_EXT &&
    // CGROUP_SCHED supplies F06's single tg_cgrp body. No duplicate fallback.
    // Every path, including unresolved/invalid scheduler/task, refs its cgroup.
    unsafe {
        let sch = lupos_scx_core_qe_prog_sched(aux);
        if !lupos_scx_core_qe_unlikely_no_sched(sch)
            && lupos_scx_core_qe_task_arg_ok(sch, p)
        {
            cgrp = tg_cgrp(tg);
        }
        lupos_scx_core_qe_cgroup_get(cgrp);
        cgrp
    }
}
