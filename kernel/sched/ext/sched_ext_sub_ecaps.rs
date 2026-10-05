// SPDX-License-Identifier: GPL-2.0

unsafe fn caps_updated_record(ps: *mut scx_pshard, cids: *const scx_cmask, caps: u64, to_deliver: *mut list_head) {
    // SAFETY: IRQs are disabled, caller owns the shard cap update and the
    // callback list. Nested accumulator lock serializes coalescing/in-flight.
    unsafe {
        let cu = ptr::addr_of_mut!((*ps).caps_updated);
        lupos_scx_sub_updated_lock(cu);
        scx_cmask_or(lupos_scx_sub_updated_cmask(cu), cids);
        (*cu).caps |= caps;
        if lupos_scx_sub_list_empty(ptr::addr_of!((*cu).node_in_flight)) {
            lupos_scx_sub_list_add_tail(ptr::addr_of_mut!((*cu).node_in_flight), to_deliver);
        }
        lupos_scx_sub_updated_unlock(cu);
    }
}

unsafe fn caps_updated_deliver(to_deliver: *mut list_head) {
    // SAFETY: The caller's IRQ-disabled scheduler read-side lifetime pins each
    // cu's owning shard; list membership alone does not take a reference. The
    // accumulator lock protects payload copying into scheduler arena storage.
    // No raw-spin lock is held across a callback, allowing nested grants.
    unsafe {
        let mut cu = lupos_scx_sub_updated_first(to_deliver);
        while !cu.is_null() {
            let next = lupos_scx_sub_updated_next(to_deliver, cu);
            let ps = lupos_scx_sub_updated_owner(cu);
            let sch = (*ps).sch;
            loop {
                let mut caps = 0;
                lupos_scx_sub_updated_lock(cu);
                if (*cu).caps != 0 && lupos_scx_sub_has_caps_op(sch)
                    && lupos_scx_sub_likely_caps_deliver(!lupos_scx_sub_aborting(sch))
                {
                    let mut storage = core::mem::MaybeUninit::<scx_cmask_ref>::uninit();
                    let reference = storage.as_mut_ptr();
                    caps = (*cu).caps;
                    scx_cmask_ref_init_kern(sch, (*cu).cmask_arena_out,
                        (*ps).base, (*ps).nr_cids, reference);
                    scx_cmask_ref_copy(reference, lupos_scx_sub_updated_cmask(cu));
                    scx_cmask_clear(lupos_scx_sub_updated_cmask(cu));
                    (*cu).caps = 0;
                } else {
                    lupos_scx_sub_list_del_init(ptr::addr_of_mut!((*cu).node_in_flight));
                }
                lupos_scx_sub_updated_unlock(cu);
                if caps == 0 {
                    break;
                }
                lupos_scx_sub_call_caps_op(sch, (*cu).cmask_arena_out, caps);
            }
            cu = next;
        }
    }
}

unsafe fn scx_sub_seed_caps(sch: *mut scx_sched) {
    // SAFETY: Called once enable makes has_op visible. The adapter keeps IRQs
    // disabled and a native automatic LIST_HEAD live through this continuation.
    unsafe { lupos_scx_sub_with_seed_list(sch) };
}

/// Deliver caps accumulated before the scheduler's ops became live.
///
/// # Safety
/// Only the native seed adapter calls this with IRQs disabled, a live enabled
/// scheduler, and a private empty initialized list valid for this whole call.
#[export_name = "lupos_scx_sub_seed_with_list"]
pub unsafe extern "C" fn scx_sub_seed_caps_list(sch: *mut scx_sched, to_deliver: *mut list_head) {
    // SAFETY: Enable serialization pins every shard. Accumulator locks protect
    // transfer to the private list, which is fully drained before returning.
    unsafe {
        for si in 0..(*sch).nr_pshards as usize {
            let ps = *(*sch).pshard.add(si);
            let cu = ptr::addr_of_mut!((*ps).caps_updated);
            lupos_scx_sub_updated_lock(cu);
            if (*cu).caps != 0 && lupos_scx_sub_list_empty(ptr::addr_of!((*cu).node_in_flight)) {
                lupos_scx_sub_list_add_tail(ptr::addr_of_mut!((*cu).node_in_flight), to_deliver);
            }
            lupos_scx_sub_updated_unlock(cu);
        }
        caps_updated_deliver(to_deliver);
    }
}

unsafe fn calc_effective_caps(ps: *mut scx_pshard, cid: c_int) -> u64 {
    // SAFETY: ps is live; cmask test retains native READ_ONCE word accesses.
    // Rq owns the transposed ecaps copy; cap writers serialize separately.
    unsafe {
        let mut ecaps = 0;
        for bit in 0..__SCX_NR_CAPS {
            if lupos_scx_sub_cmask_test(cid as u32, lupos_scx_sub_cap_cmask(ps, bit)) {
                let cap = 1u64 << bit;
                ecaps |= cap | lupos_scx_sub_caps_implied(cap);
            }
        }
        ecaps
    }
}

unsafe fn queue_sync_ecaps(sch: *mut scx_sched, cid: c_int) {
    // SAFETY: Caller holds this CID's pshard lock with IRQs disabled, excluding
    // duplicate producers. Full barriers pair with consumption before cap reads.
    unsafe {
        let cpu = lupos_scx_sub_cid_cpu(cid);
        let pcpu = lupos_scx_sub_pcpu(sch, cpu);
        let node = ptr::addr_of_mut!((*pcpu).ecaps_to_sync_node);
        lupos_scx_sub_mb();
        if lupos_scx_sub_llist_on(node) {
            return;
        }
        let rq = lupos_scx_sub_cpu_rq(cpu);
        if lupos_scx_sub_llist_add(node, ptr::addr_of_mut!((*rq).scx.ecaps_to_sync)) {
            scx_kick_cpu(lupos_scx_sub_ancestor(sch, 0), cpu, 0);
        }
    }
}

unsafe fn discard_queued_syncs(rq: *mut rq) {
    // SAFETY: Rq lock held, all removed nodes remain allocated by their sched
    // owners. Snapshot next before init resets the node's on-list sentinel.
    unsafe {
        lupos_scx_sub_assert_discard_syncs_rq(rq);
        let mut pos = lupos_scx_sub_llist_del_all(ptr::addr_of_mut!((*rq).scx.ecaps_to_sync));
        while !pos.is_null() {
            let next = (*pos).next;
            lupos_scx_sub_llist_init(pos);
            pos = next;
        }
    }
}

/// Transpose queued capability changes into this CPU's effective cap state.
///
/// # Safety
/// Caller holds rq lock and pins prev for the dispatch invocation. Live CID and
/// scheduler storage must survive callbacks, including their temporary rq unlocks.
#[no_mangle]
pub unsafe extern "C" fn scx_process_sync_ecaps(rq: *mut rq, prev: *mut task_struct) {
    // SAFETY: Rq lock owns ecaps and private batch. Node reset/barrier ordering
    // preserves producer races. Native callback dispatch retains SCX_CALL_OP.
    unsafe {
        let cpu = lupos_scx_sub_cpu(rq);
        lupos_scx_sub_assert_process_sync_rq(rq);
        let pending = ptr::addr_of_mut!((*rq).scx.ecaps_to_sync);
        if !lupos_scx_sub_has_subs() || lupos_scx_sub_likely_sync_empty(lupos_scx_sub_llist_empty(pending)) {
            return;
        }
        if lupos_scx_sub_unlikely_sync_inactive(!lupos_scx_sub_cpu_active(cpu)) {
            discard_queued_syncs(rq);
            return;
        }
        let cid = lupos_scx_sub_cpu_cid(cpu);
        let shard = lupos_scx_sub_cid_shard(cid);
        let mut lost_all = 0;
        let mut pos = lupos_scx_sub_llist_del_all(pending);
        while !pos.is_null() {
            let next = (*pos).next;
            let pcpu = lupos_scx_sub_pcpu_from_sync(pos);
            let sch = (*pcpu).sch;
            let ps = *(*sch).pshard.add(shard as usize);
            lupos_scx_sub_llist_init(pos);
            lupos_scx_sub_mb();
            let old = lupos_scx_sub_read_ecaps(pcpu);
            let ecaps = calc_effective_caps(ps, cid);
            lupos_scx_sub_write_ecaps(pcpu, ecaps);
            let lost = old & !ecaps;
            let gained = ecaps & !old;
            lost_all |= lost;
            if ecaps != (*pcpu).reported_ecaps && lupos_scx_sub_has_ecaps_op(sch)
                && !lupos_scx_sub_bypassing(sch, cpu)
            {
                let dspc = lupos_scx_sub_this_dsp_ctx(sch);
                (*dspc).rq = rq;
                (*rq).scx.sub_dispatch_prev = prev;
                lupos_scx_sub_call_ecaps_op(sch, rq, cpu, (*pcpu).reported_ecaps, ecaps);
                (*rq).scx.sub_dispatch_prev = ptr::null_mut();
                scx_flush_dispatch_buf(sch, rq);
                (*pcpu).reported_ecaps = ecaps;
            }
            if gained & SCX_CAP_BASE as u64 != 0 {
                (*pcpu).idle_renotify = true;
                (*rq).scx.flags |= SCX_RQ_SUB_IDLE_RENOTIFY;
            } else if lost & SCX_CAP_BASE as u64 != 0 {
                (*pcpu).idle_renotify = false;
            }
            pos = next;
        }
        if lost_all & SCX_CAPS_REENQ_ON_LOSS as u64 != 0 {
            lupos_scx_sub_schedule_reenq_local(rq, SCX_REENQ_CAP_REVOKE);
        }
    }
}

/// Requeue a cap notification suppressed while a sub-scheduler was bypassing.
///
/// # Safety
/// Caller holds rq lock as sch leaves bypass; CID tables, sch and its pshards
/// are pinned through this call. IRQs are already disabled by rq locking.
#[no_mangle]
pub unsafe extern "C" fn scx_unbypass_replay_ecaps(rq: *mut rq, sch: *mut scx_sched) {
    // SAFETY: Only a proper sub uses ecaps; shard lock serializes the producer.
    unsafe {
        let cpu = lupos_scx_sub_cpu(rq);
        let pcpu = lupos_scx_sub_pcpu(sch, cpu);
        lupos_scx_sub_assert_unbypass_replay_rq(rq);
        if (*sch).level == 0 || lupos_scx_sub_read_ecaps(pcpu) == (*pcpu).reported_ecaps {
            return;
        }
        let cid = lupos_scx_sub_cpu_cid(cpu);
        let ps = *(*sch).pshard.add(lupos_scx_sub_cid_shard(cid) as usize);
        lupos_scx_sub_ps_lock(ps);
        queue_sync_ecaps(sch, cid);
        lupos_scx_sub_ps_unlock(ps);
    }
}

/// Queue a full sub-hierarchy cap reseed when a CPU becomes active.
///
/// # Safety
/// Called by the serialized hotplug owner, which prevents root disable from
/// retiring tables. rq is live, and caller permits its irqsave rq lock.
#[no_mangle]
pub unsafe extern "C" fn scx_online_ecaps(rq: *mut rq) {
    // SAFETY: The enabled gate precedes table access, preserving failed-enable
    // handling. Native rq_flags has generated layout and stays on this stack.
    unsafe {
        if !lupos_scx_sub_enabled() {
            return;
        }
        let mut rf = core::mem::MaybeUninit::<rq_flags>::uninit();
        lupos_scx_sub_rq_lock(rq, rf.as_mut_ptr());
        let root = lupos_scx_sub_root_protected();
        let cid = lupos_scx_sub_cpu_cid(lupos_scx_sub_cpu(rq));
        let shard = lupos_scx_sub_cid_shard(cid);
        let mut pos = scx_next_descendant_pre(ptr::null_mut(), root);
        while !pos.is_null() {
            if (*pos).level != 0 {
                let ps = *(*pos).pshard.add(shard as usize);
                lupos_scx_sub_ps_lock(ps);
                queue_sync_ecaps(pos, cid);
                lupos_scx_sub_ps_unlock(ps);
            }
            pos = scx_next_descendant_pre(pos, root);
        }
        lupos_scx_sub_rq_unlock(rq, rf.as_mut_ptr());
    }
}

/// Clear each sub's effective access while leaving its reported caps intact.
///
/// # Safety
/// Caller is the serialized CPU-offline owner with a live protected hierarchy
/// and rq, and permits acquiring rq's irqsave lock.
#[no_mangle]
pub unsafe extern "C" fn scx_offline_ecaps(rq: *mut rq) {
    // SAFETY: Native rq lock pins the effective-copy writes. No callback fires.
    unsafe {
        let cpu = lupos_scx_sub_cpu(rq);
        let mut rf = core::mem::MaybeUninit::<rq_flags>::uninit();
        lupos_scx_sub_rq_lock(rq, rf.as_mut_ptr());
        let root = lupos_scx_sub_root_protected();
        let mut pos = scx_next_descendant_pre(ptr::null_mut(), root);
        while !pos.is_null() {
            if (*pos).level != 0 {
                lupos_scx_sub_write_ecaps(lupos_scx_sub_pcpu(pos, cpu), 0);
            }
            pos = scx_next_descendant_pre(pos, root);
        }
        lupos_scx_sub_rq_unlock(rq, rf.as_mut_ptr());
    }
}

/// Remove a retiring scheduler's queued node and wait out a private sync batch.
///
/// # Safety
/// pcpu's scheduler has been unhashed, bypassed and passed its reader grace
/// period so no producer can requeue this node. Its allocation remains alive
/// until this function returns. Caller permits locking cpu's rq and spinning.
#[no_mangle]
pub unsafe extern "C" fn scx_discard_ecaps_to_sync(cpu: c_int, pcpu: *mut scx_sched_pcpu) {
    // SAFETY: The node remains on-list throughout private-batch processing;
    // only observing off-list while holding rq proves no further pcpu access.
    unsafe {
        let rq = lupos_scx_sub_cpu_rq(cpu);
        let node = ptr::addr_of_mut!((*pcpu).ecaps_to_sync_node);
        let pending = ptr::addr_of_mut!((*rq).scx.ecaps_to_sync);
        let mut rf = core::mem::MaybeUninit::<rq_flags>::uninit();
        if lupos_scx_sub_llist_on(node) {
            let mut head = ptr::null_mut();
            let mut tail = ptr::null_mut();
            lupos_scx_sub_rq_lock(rq, rf.as_mut_ptr());
            let mut pos = lupos_scx_sub_llist_del_all(pending);
            while !pos.is_null() {
                let next = (*pos).next;
                if pos == node {
                    lupos_scx_sub_llist_init(pos);
                } else {
                    (*pos).next = head;
                    head = pos;
                    if tail.is_null() {
                        tail = pos;
                    }
                }
                pos = next;
            }
            if !head.is_null() {
                lupos_scx_sub_llist_add_batch(head, tail, pending);
            }
            lupos_scx_sub_rq_unlock(rq, rf.as_mut_ptr());
        }
        loop {
            lupos_scx_sub_rq_lock(rq, rf.as_mut_ptr());
            let done = !lupos_scx_sub_llist_on(node);
            lupos_scx_sub_rq_unlock(rq, rf.as_mut_ptr());
            if done {
                return;
            }
            lupos_scx_sub_cpu_relax();
        }
    }
}

/// Drain sync nodes from old scheduler instances before new root publication.
///
/// # Safety
/// Caller holds root-enable serialization before the root is live. Every old
/// node still has allocated backing storage until its own rq-locked discard.
#[no_mangle]
pub unsafe extern "C" fn scx_discard_stale_ecaps_syncs() {
    // SAFETY: Native possible-CPU scan preserves configured UP/SMP bounds;
    // each private native rq_flags slot is initialized and consumed by leaves.
    unsafe {
        let limit = lupos_scx_sub_possible_limit();
        let mut cpu = lupos_scx_sub_possible_scan(0);
        while cpu < limit {
            let rq = lupos_scx_sub_cpu_rq(cpu as c_int);
            let mut rf = core::mem::MaybeUninit::<rq_flags>::uninit();
            lupos_scx_sub_rq_lock(rq, rf.as_mut_ptr());
            discard_queued_syncs(rq);
            lupos_scx_sub_rq_unlock(rq, rf.as_mut_ptr());
            cpu = lupos_scx_sub_possible_scan(cpu + 1);
        }
    }
}
