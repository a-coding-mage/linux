// SPDX-License-Identifier: GPL-2.0
// F10 continuation of retained ext.rs, ext.c:5708-6180, pinned to
// 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. No existing Rust identity replaced.
// Native primitives/envelopes are unqualified C runtime, not Rust coverage.
compile_error!("SOURCE ONLY HOLD: sched_ext bypass ABI, concurrency and protection qualification incomplete");

use super::*;
use core::ptr;

/// Load-balance one donor (5708-5812), with native stable cursor storage.
///
/// # Safety
/// sch and its per-CPU DSQs/masks remain live for the timer invocation. Masks
/// have the original timer serialization; donor is a selected online CPU.
/// IRQ/rq/DSQ protection is acquired in the body and released before return.
pub(crate) unsafe fn bypass_lb_cpu(
    sch: *mut scx_sched, donor: i32, donee_mask: *mut cpumask,
    resched_mask: *mut cpumask, nr_donor_target: u32, nr_donee_target: u32,
) -> u32 {
    // SAFETY: The native frame supplies the genuine self-referential cursor
    // and keeps it live across all body lock drops. No cursor is copied.
    unsafe {
        let donor_rq = lupos_scx_core_bypass_cpu_rq(donor);
        let donor_dsq = lupos_scx_core_bypass_dsq(sch, donor);
        lupos_scx_core_bypass_cpu_cursor_frame(
            sch, donor_rq, donor_dsq, donee_mask, resched_mask,
            nr_donor_target, nr_donee_target,
        )
    }
}

/// Rust body under the native INIT_DSQ_LIST_CURSOR frame.
///
/// # Safety
/// Only the matching frame calls this function. Its initialized cursor has a
/// fixed stack address until return; all other arguments satisfy bypass_lb_cpu.
/// No unwind is permitted across the native frame or while locks are held.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_bypass_cpu_cursor_body(
    sch: *mut scx_sched, donor_rq: *mut rq, donor_dsq: *mut scx_dispatch_q,
    cursor: *mut scx_dsq_list_node, donee_mask: *mut cpumask,
    resched_mask: *mut cpumask, nr_donor_target: u32, nr_donee_target: u32,
) -> u32 {
    // SAFETY: The DSQ lock protects real nodes/task embeddings; rq custody
    // excludes migration for accepted donor tasks. Cursor resume traverses
    // its actual list node, never a fabricated task_struct. See source ledger
    // for the native F01 assertion retained on each resume and normal advance.
    unsafe {
        let delta = lupos_scx_core_bypass_nr_read_once(donor_dsq)
            .wrapping_sub(nr_donor_target) as i32;
        let mut nr_balanced = 0u32;
        let min_delta_us = lupos_scx_core_bypass_lb_intv_us_read_once()
            / SCX_BYPASS_LB_MIN_DELTA_DIV as u32;
        // C compares s32 delta to unsigned DIV_ROUND_UP: preserve conversion
        // to u32 rather than accidentally treating negative delta as smaller.
        // Native DIV_ROUND_UP retains its two distinct READ_ONCE slice reads.
        if (delta as u32) < lupos_scx_core_bypass_delta_threshold(min_delta_us) {
            return 0;
        }

        lupos_scx_core_bypass_rq_lock_irq(donor_rq);
        lupos_scx_core_bypass_raw_lock(ptr::addr_of_mut!((*donor_dsq).lock));
        lupos_scx_core_bypass_list_add(
            ptr::addr_of_mut!((*cursor).node), ptr::addr_of_mut!((*donor_dsq).list),
        );
        'resume: loop {
            lupos_scx_core_slice_assert_next_dsq(donor_dsq);
            let mut n = nldsq_next_task_after_node(
                donor_dsq, ptr::addr_of_mut!((*cursor).node), false,
            );
            while !n.is_null() {
                let p = n;
                n = nldsq_next_task(donor_dsq, n, false);
                if (*donor_dsq).nr <= nr_donor_target {
                    break;
                }
                if lupos_scx_core_bypass_mask_empty(donee_mask) {
                    break;
                }
                // A task moved by an earlier donor pass may still have its
                // previous rq. Its unlocked rq excludes it from this move.
                if lupos_scx_core_bypass_task_rq(p) != donor_rq {
                    continue;
                }
                let donee = lupos_scx_core_bypass_any_allowed(donee_mask, p);
                if donee as u32 >= lupos_scx_core_nr_cpu_ids() {
                    continue;
                }
                let donee_dsq = lupos_scx_core_bypass_dsq(sch, donee);
                if !task_can_run_on_remote_rq(
                    sch, p, lupos_scx_core_bypass_cpu_rq(donee), false,
                ) {
                    continue;
                }
                dispatch_dequeue_locked(p, donor_dsq);
                scx_dispatch_enqueue(
                    sch, lupos_scx_core_bypass_cpu_rq(donee), donee_dsq, p,
                    0, 0, SCX_ENQ_NESTED as u64,
                );
                lupos_scx_core_bypass_mask_set_cpu(donee, resched_mask);
                if lupos_scx_core_bypass_nr_read_once(donee_dsq) >= nr_donee_target {
                    lupos_scx_core_bypass_mask_clear_cpu(donee, donee_mask);
                }
                nr_balanced = nr_balanced.wrapping_add(1);
                if nr_balanced % SCX_BYPASS_LB_BATCH as u32 == 0 && !n.is_null() {
                    lupos_scx_core_bypass_list_move_tail(
                        ptr::addr_of_mut!((*cursor).node),
                        ptr::addr_of_mut!((*n).scx.dsq_list.node),
                    );
                    lupos_scx_core_bypass_raw_unlock(ptr::addr_of_mut!((*donor_dsq).lock));
                    scx_rq_lock_drop(donor_rq);
                    lupos_scx_core_bypass_rq_unlock_irq(donor_rq);
                    lupos_scx_core_cpu_relax();
                    lupos_scx_core_bypass_rq_lock_irq(donor_rq);
                    lupos_scx_core_bypass_raw_lock(ptr::addr_of_mut!((*donor_dsq).lock));
                    continue 'resume;
                }
            }
            break;
        }
        lupos_scx_core_bypass_list_del_init(ptr::addr_of_mut!((*cursor).node));
        lupos_scx_core_bypass_raw_unlock(ptr::addr_of_mut!((*donor_dsq).lock));
        scx_rq_lock_drop(donor_rq);
        lupos_scx_core_bypass_rq_unlock_irq(donor_rq);
        nr_balanced
    }
}

/// Balance online CPUs on one node, then issue kicks and trace (5814-5880).
///
/// # Safety
/// Called from the live scheduler timer; its masks are private to this timer.
/// Native READ_ONCE protects each unsynchronized queue-length observation.
pub(crate) unsafe fn bypass_lb_node(sch: *mut scx_sched, node: c_int) {
    // SAFETY: Native CPU-mask access retains configured representations and
    // exact bitmap bounds. Online membership is reread at every original pass.
    unsafe {
        let node_mask = lupos_scx_core_bypass_node_mask(node);
        let donee_mask = lupos_scx_core_bypass_donee_mask(sch);
        let resched_mask = lupos_scx_core_bypass_resched_mask(sch);
        let (mut nr_tasks, mut nr_cpus, mut nr_balanced) = (0u32, 0u32, 0u32);
        let (mut before_min, mut before_max) = (u32::MAX, 0u32);
        let (mut after_min, mut after_max) = (u32::MAX, 0u32);
        let mut cpu = -1;
        loop {
            cpu = lupos_scx_core_bypass_next_online_node_cpu(cpu, node_mask);
            if cpu as u32 >= lupos_scx_core_bypass_mask_cpu_limit() { break; }
            let nr = lupos_scx_core_bypass_nr_read_once(lupos_scx_core_bypass_dsq(sch, cpu));
            nr_tasks = nr_tasks.wrapping_add(nr);
            nr_cpus = nr_cpus.wrapping_add(1);
            before_min = core::cmp::min(nr, before_min);
            before_max = core::cmp::max(nr, before_max);
        }
        if nr_cpus == 0 { return; }
        let nr_target = nr_tasks.wrapping_add(nr_cpus).wrapping_sub(1) / nr_cpus;
        let nr_donor_target = nr_target.wrapping_mul(SCX_BYPASS_LB_DONOR_PCT as u32)
            .wrapping_add(100).wrapping_sub(1) / 100;

        lupos_scx_core_bypass_mask_clear(donee_mask);
        cpu = -1;
        loop {
            cpu = lupos_scx_core_bypass_next_online_node_cpu(cpu, node_mask);
            if cpu as u32 >= lupos_scx_core_bypass_mask_cpu_limit() { break; }
            if lupos_scx_core_bypass_nr_read_once(lupos_scx_core_bypass_dsq(sch, cpu)) < nr_target {
                lupos_scx_core_bypass_mask_set_cpu(cpu, donee_mask);
            }
        }
        lupos_scx_core_bypass_mask_clear(resched_mask);
        cpu = -1;
        loop {
            cpu = lupos_scx_core_bypass_next_online_node_cpu(cpu, node_mask);
            if cpu as u32 >= lupos_scx_core_bypass_mask_cpu_limit() { break; }
            if lupos_scx_core_bypass_mask_empty(donee_mask) { break; }
            if lupos_scx_core_bypass_mask_test_cpu(cpu, donee_mask) { continue; }
            if lupos_scx_core_bypass_nr_read_once(lupos_scx_core_bypass_dsq(sch, cpu)) <= nr_donor_target {
                continue;
            }
            nr_balanced = nr_balanced.wrapping_add(bypass_lb_cpu(
                sch, cpu, donee_mask, resched_mask, nr_donor_target, nr_target,
            ));
        }
        cpu = -1;
        loop {
            cpu = lupos_scx_core_bypass_next_mask_cpu(cpu, resched_mask);
            if cpu as u32 >= lupos_scx_core_bypass_mask_cpu_limit() { break; }
            resched_cpu(cpu);
        }
        cpu = -1;
        loop {
            cpu = lupos_scx_core_bypass_next_online_node_cpu(cpu, node_mask);
            if cpu as u32 >= lupos_scx_core_bypass_mask_cpu_limit() { break; }
            let nr = lupos_scx_core_bypass_nr_read_once(lupos_scx_core_bypass_dsq(sch, cpu));
            after_min = core::cmp::min(nr, after_min);
            after_max = core::cmp::max(nr, after_max);
        }
        lupos_scx_core_bypass_trace_lb(
            node, nr_cpus, nr_tasks, nr_balanced, before_min, before_max, after_min, after_max,
        );
    }
}

/// F09 timer callback's Rust owner body (5891-5906).
///
/// # Safety
/// timer is the live sch->bypass_lb_timer embedding; F09 shutdown_sync keeps
/// sch and its masks/DSQs live until this callback completes. Timer context
/// permits the original IRQ/rq locking; the interval knob is read afresh.
#[export_name = "lupos_scx_core_bypass_lb_timer_body"]
pub unsafe extern "C" fn scx_bypass_lb_timerfn(timer: *mut timer_list) {
    // SAFETY: Native container conversion and dispatch-depth read retain the
    // actual embedding and atomic/branch semantics. No extra RCU scope is added.
    unsafe {
        let sch = lupos_scx_core_bypass_timer_sched(timer);
        if !lupos_scx_core_bypass_dsp_enabled(sch) { return; }
        let mut node = -1;
        loop {
            node = lupos_scx_core_bypass_next_cpu_node(node);
            if node as u32 >= lupos_scx_core_bypass_cpu_node_limit() { break; }
            bypass_lb_node(sch, node);
        }
        let intv_us = lupos_scx_core_bypass_lb_intv_us_read_once();
        if intv_us != 0 {
            lupos_scx_core_bypass_mod_timer(timer, intv_us);
        }
    }
}

/// Enter nested bypass, accounting only 0 -> 1 (5908-5921).
///
/// # Safety
/// sch is live and the common bypass lock is held with original IRQ state.
pub(crate) unsafe fn inc_bypass_depth(sch: *mut scx_sched) -> bool {
    // SAFETY: Native warnings are site-distinct; plain reads and WRITE_ONCE
    // stores preserve the original synchronization and transition ordering.
    unsafe {
        lupos_scx_core_bypass_assert_inc_lock();
        lupos_scx_core_bypass_warn_inc_depth(sch);
        lupos_scx_core_bypass_depth_write_once(sch, (*sch).bypass_depth.wrapping_add(1));
        if (*sch).bypass_depth != 1 { return false; }
        // NSEC_PER_USEC is native 1000L: keep native-long multiplication
        // width before converting the result to slice_dfl's u64. On 64-bit
        // the unsigned-int operand fits signed long; its product also fits.
        let slice = (lupos_scx_core_slice_bypass_us_read_once() as c_ulong)
            .wrapping_mul(NSEC_PER_USEC as c_ulong) as u64;
        lupos_scx_core_bypass_slice_write_once(
            sch, slice,
        );
        (*sch).bypass_timestamp = lupos_scx_core_bypass_ktime_get_ns();
        lupos_scx_core_bypass_event_activate(sch);
        true
    }
}

/// Leave nested bypass, accounting only 1 -> 0 (5923-5936).
///
/// # Safety
/// Same lifetime and lock contract as inc_bypass_depth.
pub(crate) unsafe fn dec_bypass_depth(sch: *mut scx_sched) -> bool {
    // SAFETY: Timestamp access is protected by the bypass lock; event update
    // retains the native per-CPU/preemption protocol under IRQ disablement.
    unsafe {
        lupos_scx_core_bypass_assert_dec_lock();
        lupos_scx_core_bypass_warn_dec_depth(sch);
        lupos_scx_core_bypass_depth_write_once(sch, (*sch).bypass_depth.wrapping_sub(1));
        if (*sch).bypass_depth != 0 { return false; }
        lupos_scx_core_bypass_slice_write_once(sch, SCX_SLICE_DFL as u64);
        // Keep both macro-expanded time reads inside the native event leaf.
        lupos_scx_core_bypass_event_duration(sch);
        true
    }
}

/// Claim dispatch on this scheduler and its parent host (5938-5982).
///
/// # Safety
/// Called after this scheduler's outermost bypass-depth transition while the
/// bypass lock is held; scheduler and parent lifetimes are pinned by caller.
pub(crate) unsafe fn enable_bypass_dsp(sch: *mut scx_sched) {
    // SAFETY: Parent is evaluated once for ?: host selection. Atomic enable
    // references are added before checking/starting the host's load-balance timer.
    unsafe {
        let parent = lupos_scx_core_parent(sch);
        let host = if parent.is_null() { sch } else { parent };
        let intv_us = lupos_scx_core_bypass_lb_intv_us_read_once();
        if lupos_scx_core_bypass_claim_enable(sch) { return; }
        let ret = lupos_scx_core_bypass_dsp_inc(sch);
        lupos_scx_core_bypass_warn_enable_self(ret);
        if host != sch {
            let ret = lupos_scx_core_bypass_dsp_inc(host);
            lupos_scx_core_bypass_warn_enable_host(ret);
        }
        if intv_us != 0
            && !lupos_scx_core_bypass_timer_pending(ptr::addr_of_mut!((*host).bypass_lb_timer))
        {
            lupos_scx_core_bypass_mod_timer(ptr::addr_of_mut!((*host).bypass_lb_timer), intv_us);
        }
    }
}

/// Release a dispatch claim, including its ancestor-host reference (5985-5999).
///
/// # Safety
/// sch and parent are live through return. May run without bypass lock; the
/// native test-and-clear and atomic decrements serialize claim/reference state.
#[no_mangle]
pub unsafe extern "C" fn scx_disable_bypass_dsp(sch: *mut scx_sched) {
    // SAFETY: A cleared claim makes a repeated disable a no-op exactly as C.
    // Parent is reread in the decrement expression, preserving the two calls.
    unsafe {
        if !lupos_scx_core_bypass_claim_disable(sch) { return; }
        let ret = lupos_scx_core_bypass_dsp_dec(sch);
        lupos_scx_core_bypass_warn_disable_self(ret);
        if !lupos_scx_core_parent(sch).is_null() {
            let ret = lupos_scx_core_bypass_dsp_dec(lupos_scx_core_parent(sch));
            lupos_scx_core_bypass_warn_disable_parent(ret);
        }
    }
}

/// Arm original root/sub idle renotification flags (6020-6031).
///
/// # Safety
/// rq and scheduler-tree locks are held; pcpu is pos's state on rq's CPU.
pub(crate) unsafe fn unbypass_renotify_idle(
    rq: *mut rq, pos: *mut scx_sched, pcpu: *mut scx_sched_pcpu,
) {
    // SAFETY: Sub-only field accesses stay under the exact native cfg; root
    // idle renotification remains available with CONFIG_EXT_SUB_SCHED disabled.
    unsafe {
        if (*pos).level == 0 {
            (*rq).scx.flags |= SCX_RQ_ROOT_IDLE_RENOTIFY as u32;
            return;
        }
        #[cfg(CONFIG_EXT_SUB_SCHED)]
        {
            (*pcpu).idle_renotify = true;
            (*rq).scx.flags |= SCX_RQ_SUB_IDLE_RENOTIFY as u32;
        }
        #[cfg(not(CONFIG_EXT_SUB_SCHED))]
        let _ = pcpu;
    }
}

/// Apply the outermost transition while holding the global bypass lock.
///
/// # Safety
/// scx_bypass has acquired its explicit irqsave lock; sch/tree lifetimes are
/// pinned by that protocol. All returns leave bypass unlock to the caller.
unsafe fn bypass_locked(sch: *mut scx_sched, bypass: bool) {
    // SAFETY: Descendant walks hold the common scheduler lock. Per-CPU state
    // and runnable-list changes hold the rq lock, always followed by lock_drop
    // before unlocking. No cpus_read_lock/mutex/rwsem is introduced.
    unsafe {
        if bypass {
            if !inc_bypass_depth(sch) { return; }
            enable_bypass_dsp(sch);
        } else if !dec_bypass_depth(sch) {
            return;
        }
        lupos_scx_core_bypass_raw_lock(lupos_scx_core_sched_lock());
        let mut pos = ptr::null_mut();
        loop {
            pos = lupos_scx_core_bypass_next_descendant(pos, sch);
            if pos.is_null() { break; }
            if pos == sch { continue; }
            if bypass { inc_bypass_depth(pos); } else { dec_bypass_depth(pos); }
        }
        lupos_scx_core_bypass_raw_unlock(lupos_scx_core_sched_lock());

        let mut cpu = -1;
        loop {
            cpu = lupos_scx_core_bypass_next_possible_cpu(cpu);
            if cpu as u32 >= lupos_scx_core_bypass_possible_cpu_limit() { break; }
            let rq = lupos_scx_core_bypass_cpu_rq(cpu);
            lupos_scx_core_raw_spin_rq_lock(rq);
            lupos_scx_core_bypass_raw_lock(lupos_scx_core_sched_lock());
            pos = ptr::null_mut();
            loop {
                pos = lupos_scx_core_bypass_next_descendant(pos, sch);
                if pos.is_null() { break; }
                let pcpu = lupos_scx_core_bypass_pcpu(pos, cpu);
                let was_bypassing = (*pcpu).flags & SCX_SCHED_PCPU_BYPASSING as u64 != 0;
                if (*pos).bypass_depth != 0 {
                    (*pcpu).flags |= SCX_SCHED_PCPU_BYPASSING as u64;
                } else {
                    (*pcpu).flags &= !(SCX_SCHED_PCPU_BYPASSING as u64);
                    if was_bypassing {
                        unbypass_renotify_idle(rq, pos, pcpu);
                        lupos_scx_core_bypass_replay_ecaps(rq, pos);
                    }
                }
            }
            lupos_scx_core_bypass_raw_unlock(lupos_scx_core_sched_lock());
            if !lupos_scx_core_bypass_enabled() {
                scx_rq_lock_drop(rq);
                lupos_scx_core_raw_spin_rq_unlock(rq);
                continue;
            }
            // Exact safe-reverse mutation order, expressed with real list
            // nodes so the sentinel never becomes a fabricated Rust task.
            let head = ptr::addr_of_mut!((*rq).scx.runnable_list);
            let mut node = (*head).prev;
            // The C safe-reverse initializer/step fetches the predecessor
            // before testing the current node, including the final sentinel.
            let mut prev = (*node).prev;
            while node != head {
                let p = lupos_scx_core_bypass_runnable_task(node);
                if scx_is_descendant(lupos_scx_core_bypass_task_sched(p), sch) {
                    if bypass && lupos_scx_core_bypass_task_current(rq, p) {
                        scx_task_slice_ended(rq, p);
                    }
                    lupos_scx_core_bypass_cycle_task(p);
                }
                node = prev;
                prev = (*node).prev;
            }
            if lupos_scx_core_bypass_cpu_online(cpu)
                || cpu == lupos_scx_core_bypass_processor_id()
            {
                resched_curr(rq);
            }
            scx_rq_lock_drop(rq);
            lupos_scx_core_raw_spin_rq_unlock(rq);
        }
        // All tasks have left bypass DSQs before this dispatch reference drops.
        if !bypass { scx_disable_bypass_dsp(sch); }
    }
}

/// Bypass/unbypass with the original outer IRQ/rq/tree ordering (6065-6180).
///
/// # Safety
/// sch is live under the caller's original scheduler lifetime protocol. This
/// may not block on scheduler-dependent mutexes/rwsems. The original call
/// context permits raw irqsave locking and task cycling; no Rust unwind is
/// permitted across these native lock boundaries.
#[no_mangle]
pub unsafe extern "C" fn scx_bypass(sch: *mut scx_sched, bypass: bool) {
    // SAFETY: Each early logical transition return goes through the one native
    // irqrestore, including nested-depth paths. No guard API is substituted.
    unsafe {
        let flags = lupos_scx_core_bypass_lock_irqsave(lupos_scx_core_bypass_lock());
        bypass_locked(sch, bypass);
        lupos_scx_core_bypass_unlock_irqrestore(lupos_scx_core_bypass_lock(), flags);
    }
}
