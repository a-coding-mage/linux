// SPDX-License-Identifier: GPL-2.0
// F09 ext.c:5040-5143,5210-5514,6182-6239,6277-6347,7095-7380.
// Authority 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. Source-only proposal.
// Native leaves are unqualified C runtime, not Rust algorithm coverage.
compile_error!("SOURCE ONLY HOLD: sched_ext object lifetime ABI and protection qualification incomplete");
use super::*;
use core::{mem::MaybeUninit, ptr};
use kernel::ffi::{c_char, c_int, c_ulong};

/// Initialize one exclusively owned DSQ, including every possible CPU's node.
///
/// # Safety
/// dsq is writable native storage not previously live; sch is its stable owner.
/// Allocation is allowed in this context. Failure leaves no pcpu allocation.
#[no_mangle]
pub unsafe extern "C" fn scx_init_dsq(dsq: *mut scx_dispatch_q, id: u64, sch: *mut scx_sched) -> i32 {
    // SAFETY: Initialization precedes publication; native leaves preserve the
    // configured lock/percpu layouts and possible-CPU domain.
    unsafe {
        lupos_scx_core_object_zero_dsq(dsq);
        lupos_scx_core_object_init_dsq_lock(dsq);
        lupos_scx_core_object_init_list(ptr::addr_of_mut!((*dsq).list));
        (*dsq).id = id;
        (*dsq).sched = sch;
        lupos_scx_core_object_alloc_dsq_pcpu(dsq);
        if (*dsq).pcpu.is_null() { return -(ENOMEM as i32); }
        let mut cpu = lupos_scx_core_object_next_cpu(-1);
        while (cpu as u32) < lupos_scx_core_object_possible_cpu_limit() {
            let pcpu = lupos_scx_core_object_dsq_pcpu(dsq, cpu);
            (*pcpu).dsq = dsq;
            lupos_scx_core_object_init_list(ptr::addr_of_mut!((*pcpu).deferred_reenq_user.node));
            cpu = lupos_scx_core_object_next_cpu(cpu);
        }
        0
    }
}

/// Release per-CPU DSQ state after new insertions have stopped and RCU elapsed.
///
/// # Safety
/// dsq was successfully initialized and is either unpublished allocation-unwind
/// storage, or has stopped admitting new references and passed a grace period
/// since its last deferred insertion. Caller owns teardown exactly once and
/// may enter each rq's native deferred-reenqueue IRQ guard.
pub(crate) unsafe fn exit_dsq(dsq: *mut scx_dispatch_q) {
    // SAFETY: The warning preserves the original defensive unlink under its
    // rq lock. pcpu stays allocated until all possible CPUs have been visited.
    unsafe {
        let mut cpu = lupos_scx_core_object_next_cpu(-1);
        while (cpu as u32) < lupos_scx_core_object_possible_cpu_limit() {
            let pcpu = lupos_scx_core_object_dsq_pcpu(dsq, cpu);
            let dru = ptr::addr_of_mut!((*pcpu).deferred_reenq_user);
            let rq = lupos_scx_core_object_cpu_rq(cpu);
            if lupos_scx_core_object_warn_deferred_user(dru) {
                lupos_scx_core_object_deferred_unlink(rq, dru);
            }
            cpu = lupos_scx_core_object_next_cpu(cpu);
        }
        lupos_scx_core_object_free_dsq_pcpu(dsq);
    }
}

/// Native call_rcu continuation; uses the actual callback_head type.
///
/// # Safety
/// rcu is the unique queued DSQ callback after removal and the required grace
/// period. No live task remains queued; this callback owns both allocations.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_object_free_dsq_rcu_body(rcu: *mut callback_head) {
    // SAFETY: container_of uses the native member offset; no guessed layout.
    unsafe {
        let dsq = lupos_scx_core_object_dsq_from_rcu(rcu);
        exit_dsq(dsq);
        lupos_scx_core_object_free(dsq.cast());
    }
}

/// Bounce pending frees out of scheduler locks before scheduling RCU callbacks.
///
/// # Safety
/// Entered by the native free_dsq_irq_work item. Its atomically detached chain
/// contains uniquely owned removed DSQs; each is queued to RCU exactly once.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_object_free_dsq_irq_body(_work: *mut irq_work) {
    // SAFETY: Save next before handing this DSQ to call_rcu, matching safe llist
    // traversal without dereferencing an object after its callback may run.
    unsafe {
        let mut node = lupos_scx_core_object_take_free_dsqs();
        while !node.is_null() {
            let next = (*node).next;
            let dsq = lupos_scx_core_object_dsq_from_free_node(node);
            lupos_scx_core_object_call_dsq_rcu(dsq);
            node = next;
        }
    }
}

/// Remove an empty user DSQ and enqueue its deferred release.
///
/// # Safety
/// sch is live and dsq_hash initialized. Caller may acquire RCU and the DSQ IRQ
/// lock; it need not own the DSQ or already hold RCU. Caller retains sch lifetime.
pub(crate) unsafe fn destroy_dsq(sch: *mut scx_sched, id: u64) {
    // SAFETY: RCU pins the lookup through unlock; it does not exclude F07's
    // preempt/RCU-only id reader. Both runtime access sides use native plain
    // field leaves, with native race qualification still held. Invalidation
    // stays DSQ-locked, after hash removal and before free-list publication.
    unsafe {
        lupos_scx_core_object_rcu_read_lock();
        let dsq = find_user_dsq(sch, id);
        if !dsq.is_null() {
            let lock = ptr::addr_of_mut!((*dsq).lock);
            let flags = lupos_scx_core_object_lock_irqsave(lock);
            if (*dsq).nr != 0 {
                lupos_scx_core_object_error_dsq_busy(sch, dsq);
            } else if lupos_scx_core_object_remove_dsq(sch, dsq) == 0 {
                scx_shared_dsq_id_write(dsq, LUPOS_SCX_CORE_OBJECT_DSQ_INVALID);
                if lupos_scx_core_object_add_free_dsq(dsq) {
                    lupos_scx_core_object_queue_free_dsq();
                }
            }
            lupos_scx_core_object_unlock_irqrestore(lock, flags);
        }
        lupos_scx_core_object_rcu_read_unlock();
    }
}

/// Allocate ext-owned set_cmask scratch; partial failure stays owned by sch.
///
/// # Safety
/// sch is exclusively initialized, not published to callbacks, and has no
/// existing scratch allocation. Its arena pool is live for arena allocation.
/// Caller must arrange eventual scratch_free even after partial -ENOMEM.
#[no_mangle]
pub unsafe extern "C" fn scx_set_cmask_scratch_alloc(sch: *mut scx_sched) -> i32 {
    // SAFETY: Zeroed per-CPU pointer slots make partial unwind safe; the arena
    // owner supplies allocation and cmask native inline supplies initialization.
    unsafe {
        let size = lupos_scx_core_object_scratch_size();
        if !(*sch).is_cid_type || (*sch).arena_pool.is_null() { return 0; }
        lupos_scx_core_object_alloc_scratch_slots(sch);
        if (*sch).set_cmask_scratch.is_null() { return -(ENOMEM as i32); }
        let mut cpu = lupos_scx_core_object_next_cpu(-1);
        while (cpu as u32) < lupos_scx_core_object_possible_cpu_limit() {
            let slot = lupos_scx_core_object_scratch_slot(sch, cpu);
            *slot = scx_arena_alloc(sch, size).cast();
            if (*slot).is_null() { return -(ENOMEM as i32); }
            lupos_scx_core_object_cmask_init(*slot, lupos_scx_core_num_possible_cpus());
            cpu = lupos_scx_core_object_next_cpu(cpu);
        }
        0
    }
}

/// Release all scratch slots before destroying the arena pool.
///
/// # Safety
/// sch is exclusively tearing down; no set_cmask callback can run. Its arena
/// pool remains live and scratch is either NULL or a partially/full allocation.
pub(crate) unsafe fn scx_set_cmask_scratch_free(sch: *mut scx_sched) {
    // SAFETY: The arena's free contract accepts each allocated slot including
    // NULL partial slots. The per-CPU pointer allocation is released last.
    unsafe {
        let size = lupos_scx_core_object_scratch_size();
        if (*sch).set_cmask_scratch.is_null() { return; }
        let mut cpu = lupos_scx_core_object_next_cpu(-1);
        while (cpu as u32) < lupos_scx_core_object_possible_cpu_limit() {
            let slot = lupos_scx_core_object_scratch_slot(sch, cpu);
            scx_arena_free(sch, (*slot).cast(), size);
            cpu = lupos_scx_core_object_next_cpu(cpu);
        }
        lupos_scx_core_object_free_scratch_slots(sch);
        (*sch).set_cmask_scratch = ptr::null_mut();
    }
}

/// Queue the final scheduler release behind an RCU grace period.
///
/// # Safety
/// Called only by the native kobj_type release callback after the last kobject
/// reference. The containing scheduler is fully initialized and inactive; this
/// callback transfers its final ownership to the RCU work exactly once.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_object_release_body(kobj: *mut kobject) {
    // SAFETY: Native container conversion and work initialization use original
    // callback types; no ordinary kfree may race or replace the queued work.
    unsafe {
        let sch = lupos_scx_core_object_kobj_sched(kobj);
        lupos_scx_core_object_init_rcu_work(sch);
        lupos_scx_core_object_queue_rcu_work(sch);
    }
}

/// Read the global enable state for the native sysfs attribute.
///
/// # Safety
/// Native sysfs pins the attribute and supplies its writable PAGE_SIZE buffer;
/// F00's state enum is a valid index in its authoritative state-name table.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_object_state_body(_kobj: *mut kobject, _ka: *mut kobj_attribute, buf: *mut c_char) -> isize {
    // SAFETY: No reference to mutable storage escapes the synchronous formatter.
    unsafe { lupos_scx_core_object_emit_state(buf, lupos_scx_core_enable_state_name(scx_enable_state())) }
}
/// Read the original READ_ONCE switch flag for sysfs.
///
/// # Safety
/// Native sysfs supplies a live attribute and writable PAGE_SIZE buffer.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_object_switch_all_body(_kobj: *mut kobject, _ka: *mut kobj_attribute, buf: *mut c_char) -> isize {
    // SAFETY: F00 owns the synchronized read; emit uses the sysfs-owned buffer.
    unsafe { lupos_scx_core_object_emit_switch(buf, lupos_scx_core_switching_all_read_once()) }
}
/// Read the rejected-task atomic counter for sysfs.
///
/// # Safety
/// Native sysfs supplies a live attribute and writable PAGE_SIZE buffer.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_object_nr_rejected_body(_kobj: *mut kobject, _ka: *mut kobj_attribute, buf: *mut c_char) -> isize {
    // SAFETY: Atomic-long access/format preserve native signed-long width.
    unsafe { lupos_scx_core_object_emit_long(buf, lupos_scx_core_rejected_read()) }
}
/// Read the hotplug atomic sequence for sysfs.
///
/// # Safety
/// Native sysfs supplies a live attribute and writable PAGE_SIZE buffer.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_object_hotplug_seq_body(_kobj: *mut kobject, _ka: *mut kobj_attribute, buf: *mut c_char) -> isize {
    // SAFETY: Atomic-long access/format preserve native signed-long width.
    unsafe { lupos_scx_core_object_emit_long(buf, lupos_scx_core_hotplug_seq_read()) }
}
/// Read the enable atomic sequence for sysfs.
///
/// # Safety
/// Native sysfs supplies a live attribute and writable PAGE_SIZE buffer.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_object_enable_seq_body(_kobj: *mut kobject, _ka: *mut kobj_attribute, buf: *mut c_char) -> isize {
    // SAFETY: Atomic-long access/format preserve native signed-long width.
    unsafe { lupos_scx_core_object_emit_long(buf, lupos_scx_core_enable_seq_read()) }
}
/// Read the immutable registered scheduler name for sysfs.
///
/// # Safety
/// kobj is this family's scx_ktype object, pinned by sysfs for the call, with
/// initialized immutable ops; buf is the writable PAGE_SIZE sysfs buffer.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_object_ops_body(kobj: *mut kobject, _ka: *mut kobj_attribute, buf: *mut c_char) -> isize {
    // SAFETY: Native container offset and native union view are authoritative.
    unsafe { lupos_scx_core_object_emit_ops(buf, lupos_scx_core_object_kobj_sched(kobj)) }
}
/// Read scheduler events and invoke the original native format expansion.
///
/// # Safety
/// kobj is a sysfs-pinned initialized scheduler; buf is a PAGE_SIZE sysfs
/// buffer. F16 scx_read_events must fill all native event fields before read.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_object_events_body(kobj: *mut kobject, _ka: *mut kobj_attribute, buf: *mut c_char) -> isize {
    // SAFETY: MaybeUninit storage is only read after the owner fills it. Native
    // SCX_EVENTS_LIST formatting owns field names/order and int accumulation;
    // that formatting expansion remains native runtime work, not Rust coverage.
    unsafe {
        let mut events = MaybeUninit::<scx_event_stats>::uninit();
        scx_read_events(lupos_scx_core_object_kobj_sched(kobj), events.as_mut_ptr());
        lupos_scx_core_object_emit_events(buf, events.as_ptr()) as isize
    }
}

/// Allocate aggregation scratch and emit each capability's CPU bitmap.
///
/// # Safety
/// kobj is a sysfs-pinned scheduler whose pshards are initialized for its
/// lifetime; buf is the sysfs PAGE_SIZE buffer and sleeping allocation is legal.
#[cfg(CONFIG_EXT_SUB_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_object_caps_body(kobj: *mut kobject, _ka: *mut kobj_attribute, buf: *mut c_char) -> isize {
    // SAFETY: Both allocations are attempted, as in the source. The original
    // __free(bitmap)/__free(kfree) predicates are honored in reverse cleanup
    // order on every path: non-NULL bitmap, then non-ERR/non-NULL aggregate.
    unsafe {
        let sch = lupos_scx_core_object_kobj_sched(kobj);
        let nr = lupos_scx_core_num_possible_cpus();
        let agg = lupos_scx_core_object_alloc_agg(nr);
        let bitmap = lupos_scx_core_object_alloc_bitmap(nr);
        let mut count = -(ENOMEM as isize);
        if !agg.is_null() && !bitmap.is_null() {
            count = 0;
            let mut cap: c_int = 0;
            while cap < __SCX_NR_CAPS as c_int {
                count += lupos_scx_core_object_with_caps_snap(sch, buf, agg, bitmap, cap, count, nr);
                cap += 1;
            }
        }
        if !bitmap.is_null() { lupos_scx_core_object_free_bitmap(bitmap); }
        if !lupos_scx_core_object_is_err_or_null(agg.cast()) {
            lupos_scx_core_object_free(agg.cast());
        }
        count
    }
}

/// Aggregate one capability using the original native stack-cmask allocation.
///
/// # Safety
/// Native with_caps_snap supplies a fresh SCX_CMASK_DEFINE allocation, alive
/// for this synchronous call only. sch, agg, bitmap, buf have caps_body's
/// contracts, cap is in range, and at is the accumulated sysfs byte count.
#[cfg(CONFIG_EXT_SUB_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_object_caps_one_body(sch: *mut scx_sched, buf: *mut c_char, agg: *mut scx_cmask, bitmap: *mut c_ulong, cap: c_int, at: isize, nr: u32, snap: *mut scx_cmask) -> isize {
    // SAFETY: Every source cmask belongs to a live pshard; its recorded range
    // fits the authoritative shard-sized snapshot. Snapshot never escapes.
    unsafe {
        lupos_scx_core_object_cmask_init(agg, nr);
        let mut si: c_int = 0;
        while (si as u32) < (*sch).nr_pshards {
            let cm = lupos_scx_core_object_shard_cap(sch, si, cap);
            lupos_scx_core_object_cmask_reframe(snap, (*cm).base, (*cm).nr_cids);
            lupos_scx_core_object_cmask_copy(snap, cm);
            lupos_scx_core_object_cmask_or(agg, snap);
            si += 1;
        }
        lupos_scx_core_object_bitmap_from_cmask(bitmap, agg, nr);
        lupos_scx_core_object_emit_cap(buf, at, cap, nr, bitmap)
    }
}

/// Emit scheduler uevents while rejecting kset objects encountered up the chain.
///
/// # Safety
/// Native kset dispatch pins kobj and env for this call; env accepts uevent
/// writes. The container conversion is valid only after the exact ktype test.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_object_uevent_body(kobj: *const kobject, env: *mut kobj_uevent_env) -> c_int {
    // SAFETY: No cast of kset_ktype objects occurs; the scheduler is obtained
    // only after matching the one native scx_ktype identity.
    unsafe {
        if !lupos_scx_core_object_is_sched_kobj(kobj) { return 0; }
        lupos_scx_core_object_add_uevent(env, lupos_scx_core_object_const_kobj_sched(kobj))
    }
}

/// Complete scheduler destruction after the kobject RCU-work grace period.
///
/// # Safety
/// work is this scheduler's uniquely queued rcu_work. Scheduler bypass and
/// disable have blocked new deferrals/kicks and removed external publication;
/// no external reference can be acquired. This sleepable worker owns all
/// resources, may flush work/timers, and must run exactly once.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_object_sched_free_body(work: *mut work_struct) {
    // SAFETY: The original flush/free ordering is explicit below. Per-CPU
    // deferred and kick lists are quiesced before their storage disappears;
    // scratch/pshards precede arena pool destruction, map put and final kfree.
    unsafe {
        let sch = lupos_scx_core_object_work_sched(work);
        lupos_scx_core_object_irq_sync(ptr::addr_of_mut!((*sch).propagate_exit_irq_work));
        lupos_scx_core_object_irq_sync(ptr::addr_of_mut!((*sch).disable_irq_work));
        lupos_scx_core_object_destroy_helper(sch);
        lupos_scx_core_object_shutdown_timer(sch);
        lupos_scx_core_object_free_donee(sch);
        lupos_scx_core_object_free_resched(sch);
        lupos_scx_core_object_free_stall(sch);
        #[cfg(CONFIG_EXT_SUB_SCHED)]
        {
            lupos_scx_core_object_free((*sch).cgrp_path.cast());
            if !lupos_scx_core_object_sch_cgroup(sch).is_null() {
                lupos_scx_core_object_cgroup_put(lupos_scx_core_object_sch_cgroup(sch));
            }
            if !(*sch).sub_kset.is_null() {
                lupos_scx_core_object_kobject_put(ptr::addr_of_mut!((*(*sch).sub_kset).kobj));
            }
            let parent = lupos_scx_core_parent(sch);
            if !parent.is_null() {
                lupos_scx_core_object_kobject_put(ptr::addr_of_mut!((*parent).kobj));
            }
        }
        let mut cpu = lupos_scx_core_object_next_cpu(-1);
        while (cpu as u32) < lupos_scx_core_object_possible_cpu_limit() {
            let pcpu = lupos_scx_core_object_sched_pcpu(sch, cpu);
            lupos_scx_core_object_warn_deferred_local(pcpu);
            lupos_scx_core_object_discard_ecaps(cpu, pcpu);
            let rq = lupos_scx_core_object_cpu_rq(cpu);
            lupos_scx_core_object_irq_sync(ptr::addr_of_mut!((*rq).scx.kick_cpus_irq_work));
            lupos_scx_core_object_warn_kick_node(pcpu);
            free_pcpu_masks(pcpu);
            exit_dsq(lupos_scx_core_bypass_dsq(sch, cpu));
            cpu = lupos_scx_core_object_next_cpu(cpu);
        }
        lupos_scx_core_object_free_sched_pcpu(sch);
        free_all_pnodes(sch);
        lupos_scx_core_object_free_pshards(sch);

        let mut iter = MaybeUninit::<rhashtable_iter>::uninit();
        lupos_scx_core_object_walk_enter(sch, iter.as_mut_ptr());
        loop {
            lupos_scx_core_object_walk_start(iter.as_mut_ptr());
            let last = loop {
                let dsq = lupos_scx_core_object_walk_next(iter.as_mut_ptr());
                if lupos_scx_core_object_is_err_or_null(dsq.cast()) { break dsq; }
                destroy_dsq(sch, scx_shared_dsq_id_read(dsq));
            };
            lupos_scx_core_object_walk_stop(iter.as_mut_ptr());
            if lupos_scx_core_object_ptr_err(last.cast()) != -(EAGAIN as kernel::ffi::c_long) { break; }
        }
        lupos_scx_core_object_walk_exit(iter.as_mut_ptr());
        lupos_scx_core_object_free_hash(sch);
        free_exit_info((*sch).exit_info);
        scx_set_cmask_scratch_free(sch);
        scx_arena_pool_destroy(sch);
        if !(*sch).arena_map.is_null() { lupos_scx_core_object_put_map((*sch).arena_map); }
        lupos_scx_core_object_dec_has_subs(sch);
        lupos_scx_core_object_free(sch.cast());
    }
}

/// Release the four per-CPU cpumask_var_t allocations in original order.
///
/// # Safety
/// pcpu is exclusively owned initialized/zeroed per-CPU state. No callback
/// uses these masks; each is NULL/embedded or an allocation owned by pcpu.
unsafe fn free_pcpu_masks(pcpu: *mut scx_sched_pcpu) {
    // SAFETY: Native leaves preserve CONFIG_CPUMASK_OFFSTACK alternatives.
    unsafe {
        lupos_scx_core_object_free_kick(pcpu);
        lupos_scx_core_object_free_kick_idle(pcpu);
        lupos_scx_core_object_free_preempt(pcpu);
        lupos_scx_core_object_free_wait(pcpu);
    }
}

/// Free all successfully allocated pnodes, followed by their pointer array.
///
/// # Safety
/// sch owns a non-NULL zero-initialized nr_node_ids array. Possible-node slots
/// are NULL or fully initialized pnodes; DSQs satisfy exit_dsq quiescence.
unsafe fn free_all_pnodes(sch: *mut scx_sched) {
    // SAFETY: The native iterator preserves both MAX_NUMNODES alternatives;
    // zero slots are accepted by free_pnode during allocation failure unwind.
    unsafe {
        let mut node = lupos_scx_core_object_next_node(-1);
        while node < lupos_scx_core_object_node_limit() {
            free_pnode(lupos_scx_core_object_pnode(sch, node));
            node = lupos_scx_core_object_next_node(node);
        }
        lupos_scx_core_object_free((*sch).pnode.cast());
    }
}

/// Free exit-info buffers and their container.
///
/// # Safety
/// ei is non-NULL exclusively owned native exit info; every buffer is either
/// NULL or its matching allocator's live allocation, no readers remain.
pub(crate) unsafe fn free_exit_info(ei: *mut scx_exit_info) {
    // SAFETY: dump uses kvfree; msg, native unsigned-long backtrace, and ei
    // use kfree. NULL partial buffers are valid for these kernel primitives.
    unsafe {
        lupos_scx_core_object_kvfree((*ei).dump.cast());
        lupos_scx_core_object_free((*ei).msg.cast());
        lupos_scx_core_object_free((*ei).bt.cast());
        lupos_scx_core_object_free(ei.cast());
    }
}

/// Allocate exit info including all buffers, retaining the original unwind.
///
/// # Safety
/// Sleeping allocation is permitted. The returned non-NULL allocation has
/// one owner responsible for free_exit_info; exit_dump_len is the validated
/// requested allocation length from the native ops structure.
pub(crate) unsafe fn alloc_exit_info(exit_dump_len: usize) -> *mut scx_exit_info {
    // SAFETY: Container allocation is zeroed. All three buffers are attempted
    // before checking failure, and partial allocations are released together.
    unsafe {
        let ei = lupos_scx_core_object_alloc_ei();
        if ei.is_null() { return ptr::null_mut(); }
        (*ei).exit_cpu = -1;
        (*ei).bt = lupos_scx_core_object_alloc_bt();
        (*ei).msg = lupos_scx_core_object_alloc_msg();
        (*ei).dump = lupos_scx_core_object_alloc_dump(exit_dump_len);
        if (*ei).bt.is_null() || (*ei).msg.is_null() || (*ei).dump.is_null() {
            free_exit_info(ei);
            return ptr::null_mut();
        }
        ei
    }
}

/// Return the pinned source's static NUL-terminated exit-reason string.
pub(crate) fn scx_exit_reason(kind: scx_exit_kind) -> *const c_char {
    let reason: &'static [u8] = match kind {
        SCX_EXIT_UNREG => b"unregistered from user space\0",
        SCX_EXIT_UNREG_BPF => b"unregistered from BPF\0",
        SCX_EXIT_UNREG_KERN => b"unregistered from the main kernel\0",
        SCX_EXIT_SYSRQ => b"disabled by sysrq-S\0",
        SCX_EXIT_PARENT => b"parent exiting\0",
        SCX_EXIT_PARENT_KILL => b"killed by parent scheduler\0",
        SCX_EXIT_ERROR => b"runtime error\0",
        SCX_EXIT_ERROR_BPF => b"scx_bpf_error\0",
        SCX_EXIT_ERROR_STALL => b"runnable task stall\0",
        SCX_EXIT_ERROR_REENQ => b"reenqueue limit\0",
        SCX_EXIT_ERROR_RESCUE => b"rescue bandwidth overload\0",
        _ => b"<UNKNOWN>\0",
    };
    reason.as_ptr().cast()
}

/// Link a scheduler while serializing parent bypass/exit and global membership.
///
/// # Safety
/// sch is initialized with its parent pinned, unlinked and exclusively in the
/// enable path. Context may enter the bypass IRQ guard then scheduler guard.
/// Its list/hash nodes are not currently linked; no caller destroys it here.
#[no_mangle]
pub unsafe extern "C" fn scx_link_sched(sch: *mut scx_sched) -> i32 {
    // SAFETY: Native scoped_guard retains the pinned kernel's counted IRQ
    // semantics and cleanup/context attributes, releasing both locks before
    // the watchdog call even when the Rust continuation reports failure.
    unsafe {
        let ret = lupos_scx_core_object_with_link_locks(sch);
        if ret != 0 { return ret; }
        refresh_watchdog();
        0
    }
}

/// Perform membership publication under the exact native link guard scopes.
///
/// # Safety
/// Only with_link_locks may enter this continuation; both F00 locks are held
/// in source order and sch obeys scx_link_sched's exclusive lifetime contract.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_object_link_locked_body(sch: *mut scx_sched) -> i32 {
    // SAFETY: The native enclosing scope owns both guards across all returns.
    // The full barrier pairs with F11 claim_exit's aborting publication.
    unsafe {
            #[cfg(CONFIG_EXT_SUB_SCHED)]
            {
                let parent = lupos_scx_core_parent(sch);
                if !parent.is_null() {
                    if lupos_scx_core_object_bypass_depth(parent) != 0 {
                        lupos_scx_core_object_error_parent_bypass(sch);
                        return -(EBUSY as i32);
                    }
                    let ret = lupos_scx_core_object_insert_sched(sch);
                    if ret != 0 {
                        lupos_scx_core_object_error_insert_sched(sch, ret);
                        return ret;
                    }
                    lupos_scx_core_object_list_add_tail_rcu(ptr::addr_of_mut!((*sch).sibling), ptr::addr_of_mut!((*parent).children));
                    lupos_scx_core_object_mb();
                    if lupos_scx_core_object_parent_aborting(parent) {
                        lupos_scx_core_object_remove_sched(sch);
                        lupos_scx_core_object_list_del_rcu(ptr::addr_of_mut!((*sch).sibling));
                        lupos_scx_core_object_error_parent_disabled(sch);
                        return -(ENOENT as i32);
                    }
                    (*sch).linked = true;
                }
            }
            lupos_scx_core_object_list_add_tail_rcu(ptr::addr_of_mut!((*sch).all), lupos_scx_core_sched_all());
            0
    }
}

/// Unlink global and optional parent/hash membership before watchdog refresh.
///
/// # Safety
/// sch is live in the disable path and globally linked exactly once; caller
/// permits the native scoped raw_spinlock_irq guard. Membership is lock-owned.
#[no_mangle]
pub unsafe extern "C" fn scx_unlink_sched(sch: *mut scx_sched) {
    // SAFETY: Native scope preserves counted IRQ semantics and releases the
    // single scheduler lock before refreshing watchdog state.
    unsafe {
        lupos_scx_core_object_with_unlink_lock(sch);
        refresh_watchdog();
    }
}

/// Perform membership removal inside the exact native unlink guard scope.
///
/// # Safety
/// Only with_unlink_lock may enter; its native scheduler IRQ guard is held
/// and sch has scx_unlink_sched's live linked membership contract.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_object_unlink_locked_body(sch: *mut scx_sched) {
    // SAFETY: Native guard owns lock/IRQ release; RCU deletion preserves live
    // readers until the subsequent lifecycle grace period.
    unsafe {
        #[cfg(CONFIG_EXT_SUB_SCHED)]
        if (*sch).linked {
            lupos_scx_core_object_remove_sched(sch);
            lupos_scx_core_object_list_del_rcu(ptr::addr_of_mut!((*sch).sibling));
            (*sch).linked = false;
        }
        lupos_scx_core_object_list_del_rcu(ptr::addr_of_mut!((*sch).all));
    }
}

/// Free one optional initialized per-node global DSQ container.
///
/// # Safety
/// pnode is NULL or exclusively owned; its DSQ is quiesced and initialized,
/// satisfying exit_dsq's RCU/deferred-node requirements.
pub(crate) unsafe fn free_pnode(pnode: *mut scx_sched_pnode) {
    // SAFETY: NULL is the original partial-allocation case.
    unsafe {
        if pnode.is_null() { return; }
        exit_dsq(ptr::addr_of_mut!((*pnode).global_dsq));
        lupos_scx_core_object_free(pnode.cast());
    }
}

/// Allocate a NUMA-local pnode and initialize its global DSQ.
///
/// # Safety
/// sch is the live exclusive scheduler owner, node is a possible node, and
/// sleeping allocation is allowed. Return transfers pnode ownership.
pub(crate) unsafe fn alloc_pnode(sch: *mut scx_sched, node: c_int) -> *mut scx_sched_pnode {
    // SAFETY: A failed DSQ init has not allocated pcpu and needs only kfree.
    unsafe {
        let pnode = lupos_scx_core_object_alloc_pnode(node);
        if pnode.is_null() { return ptr::null_mut(); }
        if scx_init_dsq(ptr::addr_of_mut!((*pnode).global_dsq), LUPOS_SCX_CORE_OBJECT_DSQ_GLOBAL, sch) != 0 {
            lupos_scx_core_object_free(pnode.cast());
            return ptr::null_mut();
        }
        pnode
    }
}

/// Allocate and initialize a scheduler, consuming cgrp on success or failure.
///
/// # Safety
/// cmd and its correct CPU/CID ops allocation are live and exclusively managed
/// by the serialized enable/registration path; common ops fields are validated
/// by their owner. cgrp is the reference this function consumes under SUB.
/// parent is NULL or pinned and has a valid bounded ancestry. Sleeping
/// allocation, helper creation and kobject operations are permitted. On
/// success only the kobject release path may free the published scheduler;
/// cmd's arena_map reference transfers only after publication and kobj init.
#[no_mangle]
pub unsafe extern "C" fn scx_alloc_and_add_sched(cmd: *mut scx_enable_cmd, cgrp: *mut cgroup, parent: *mut scx_sched) -> *mut scx_sched {
    // SAFETY: The booleans record exact reached unwind labels, not inferred
    // allocation validity. All pre-publication failures unwind in source order;
    // no failure edge or direct free exists after rcu_assign_pointer(ops->priv).
    unsafe {
        let ops = lupos_scx_core_object_cmd_ops(cmd);
        #[cfg(not(CONFIG_EXT_SUB_SCHED))]
        let _ = cgrp;
        let level = if parent.is_null() { 0 } else { (*parent).level + 1 };
        let mut bypass_fail_cpu = lupos_scx_core_nr_cpu_ids() as i32;
        let sch = lupos_scx_core_object_alloc_sched(level);
        if sch.is_null() {
            #[cfg(CONFIG_EXT_SUB_SCHED)]
            lupos_scx_core_object_cgroup_put(cgrp);
            return lupos_scx_core_object_err_sched(-(ENOMEM as kernel::ffi::c_long));
        }
        let mut hash_ready = false;
        let mut helper_ready = false;
        let mut donee_ready = false;
        let mut resched_ready = false;
        #[cfg(CONFIG_EXT_SUB_SCHED)]
        let mut stall_ready = false;
        let ret = 'build: {
            (*sch).exit_info = alloc_exit_info((*ops).exit_dump_len as usize);
            if (*sch).exit_info.is_null() { break 'build -(ENOMEM as i32); }
            let ret = lupos_scx_core_object_init_hash(sch);
            if ret < 0 { break 'build ret; }
            hash_ready = true;
            lupos_scx_core_object_alloc_pnodes(sch);
            if (*sch).pnode.is_null() { break 'build -(ENOMEM as i32); }
            let mut node = lupos_scx_core_object_next_node(-1);
            while node < lupos_scx_core_object_node_limit() {
                let pnode = alloc_pnode(sch, node);
                lupos_scx_core_object_set_pnode(sch, node, pnode);
                if pnode.is_null() { break 'build -(ENOMEM as i32); }
                node = lupos_scx_core_object_next_node(node);
            }
            (*sch).dsp_max_batch = if (*ops).dispatch_max_batch != 0 {
                (*ops).dispatch_max_batch
            } else { SCX_DSP_DFL_MAX_BATCH as u32 };
            lupos_scx_core_object_alloc_sched_pcpu(sch);
            if (*sch).pcpu.is_null() { break 'build -(ENOMEM as i32); }
            let mut cpu = lupos_scx_core_object_next_cpu(-1);
            while (cpu as u32) < lupos_scx_core_object_possible_cpu_limit() {
                let ret = scx_init_dsq(lupos_scx_core_bypass_dsq(sch, cpu), LUPOS_SCX_CORE_OBJECT_DSQ_BYPASS, sch);
                if ret != 0 {
                    bypass_fail_cpu = cpu;
                    break 'build ret;
                }
                cpu = lupos_scx_core_object_next_cpu(cpu);
            }
            cpu = lupos_scx_core_object_next_cpu(-1);
            while (cpu as u32) < lupos_scx_core_object_possible_cpu_limit() {
                let pcpu = lupos_scx_core_object_sched_pcpu(sch, cpu);
                let node = lupos_scx_core_cpu_to_node(cpu);
                (*pcpu).sch = sch;
                lupos_scx_core_object_init_list(ptr::addr_of_mut!((*pcpu).deferred_reenq_local.node));
                #[cfg(CONFIG_EXT_SUB_SCHED)]
                lupos_scx_core_object_init_ecaps_node(pcpu);
                lupos_scx_core_object_init_list(ptr::addr_of_mut!((*pcpu).to_kick_node));
                if !lupos_scx_core_object_alloc_kick(pcpu, node)
                    || !lupos_scx_core_object_alloc_kick_idle(pcpu, node)
                    || !lupos_scx_core_object_alloc_preempt(pcpu, node)
                    || !lupos_scx_core_object_alloc_wait(pcpu, node) {
                    break 'build -(ENOMEM as i32);
                }
                cpu = lupos_scx_core_object_next_cpu(cpu);
            }
            lupos_scx_core_object_run_helper(sch);
            if lupos_scx_core_object_is_err((*sch).helper.cast()) {
                break 'build lupos_scx_core_object_ptr_err((*sch).helper.cast()) as i32;
            }
            helper_ready = true;
            lupos_scx_core_object_helper_fifo(sch);
            if !parent.is_null() { lupos_scx_core_object_copy_ancestors(sch, parent, level); }
            lupos_scx_core_object_set_ancestor(sch, level);
            (*sch).level = level;
            (*sch).id = lupos_scx_core_sched_id_inc();
            (*sch).watchdog_timeout = if (*ops).timeout_ms != 0 {
                lupos_scx_core_object_msecs_to_jiffies((*ops).timeout_ms)
            } else { SCX_WATCHDOG_MAX_TIMEOUT as c_ulong };
            (*sch).slice_dfl = LUPOS_SCX_CORE_OBJECT_SLICE_DFL;
            lupos_scx_core_object_init_exit_kind(sch);
            lupos_scx_core_object_init_disable_irq(sch);
            lupos_scx_core_object_init_propagate_irq(sch);
            lupos_scx_core_object_init_disable_work(sch);
            lupos_scx_core_object_init_bypass_timer(sch);
            if !lupos_scx_core_object_alloc_donee(sch) { break 'build -(ENOMEM as i32); }
            donee_ready = true;
            if !lupos_scx_core_object_alloc_resched(sch) { break 'build -(ENOMEM as i32); }
            resched_ready = true;
            if !lupos_scx_core_object_alloc_stall(sch) { break 'build -(ENOMEM as i32); }
            #[cfg(CONFIG_EXT_SUB_SCHED)]
            { stall_ready = true; }
            // Native union assignment preserves CID's shorter tail; zeroed
            // cpu_acquire/release are not read/copied from a CID allocation.
            if (*cmd).is_cid_type {
                lupos_scx_core_object_copy_cid_ops(sch, cmd);
                (*sch).is_cid_type = true;
            } else {
                lupos_scx_core_object_copy_cpu_ops(sch, cmd);
            }
            #[cfg(CONFIG_EXT_SUB_SCHED)]
            {
                let buf = lupos_scx_core_object_alloc_path();
                if buf.is_null() { break 'build -(ENOMEM as i32); }
                lupos_scx_core_object_cgroup_path(cgrp, buf);
                (*sch).cgrp_path = lupos_scx_core_object_dup_path(buf);
                lupos_scx_core_object_free(buf.cast());
                if (*sch).cgrp_path.is_null() { break 'build -(ENOMEM as i32); }
                (*sch).cgrp = cgrp;
                lupos_scx_core_object_init_list(ptr::addr_of_mut!((*sch).children));
                lupos_scx_core_object_init_list(ptr::addr_of_mut!((*sch).sibling));
            }
            0
        };
        if ret != 0 {
            #[cfg(CONFIG_EXT_SUB_SCHED)]
            if stall_ready { lupos_scx_core_object_free_stall(sch); }
            if resched_ready { lupos_scx_core_object_free_resched(sch); }
            if donee_ready { lupos_scx_core_object_free_donee(sch); }
            if helper_ready { lupos_scx_core_object_destroy_helper(sch); }
            if !(*sch).pcpu.is_null() {
                let mut cpu = lupos_scx_core_object_next_cpu(-1);
                while (cpu as u32) < lupos_scx_core_object_possible_cpu_limit() {
                    free_pcpu_masks(lupos_scx_core_object_sched_pcpu(sch, cpu));
                    cpu = lupos_scx_core_object_next_cpu(cpu);
                }
                cpu = lupos_scx_core_object_next_cpu(-1);
                while (cpu as u32) < lupos_scx_core_object_possible_cpu_limit() {
                    if cpu == bypass_fail_cpu { break; }
                    exit_dsq(lupos_scx_core_bypass_dsq(sch, cpu));
                    cpu = lupos_scx_core_object_next_cpu(cpu);
                }
                lupos_scx_core_object_free_sched_pcpu(sch);
            }
            if !(*sch).pnode.is_null() { free_all_pnodes(sch); }
            if hash_ready { lupos_scx_core_object_free_hash(sch); }
            if !(*sch).exit_info.is_null() { free_exit_info((*sch).exit_info); }
            lupos_scx_core_object_free(sch.cast());
            #[cfg(CONFIG_EXT_SUB_SCHED)]
            lupos_scx_core_object_cgroup_put(cgrp);
            return lupos_scx_core_object_err_sched(ret as kernel::ffi::c_long);
        }
        // Publication is the lifetime boundary: readers can now see sch.
        lupos_scx_core_object_publish_priv(ops, sch);
        (*sch).kobj.kset = lupos_scx_core_kset();
        lupos_scx_core_object_init_list(ptr::addr_of_mut!((*sch).all));
        #[cfg(CONFIG_EXT_SUB_SCHED)]
        if !parent.is_null() {
            lupos_scx_core_object_kobject_get(ptr::addr_of_mut!((*parent).kobj));
        }
        lupos_scx_core_object_init_kobj(sch);
        (*sch).arena_map = (*cmd).arena_map;
        #[cfg(all(CONFIG_MMU, CONFIG_64BIT))]
        if !(*sch).arena_map.is_null() { lupos_scx_core_object_arena_kern_base(sch); }
        (*cmd).arena_map = ptr::null_mut();
        sch
    }
}

/// Publish initialized scheduler sysfs state and optional sub-scheduler kset.
///
/// # Safety
/// sch owns an initialized kobject not yet added to sysfs; all sysfs-visible
/// state is ready. A non-NULL parent's sub_kset is live and pinned. On failure
/// the enable owner must use kobject cleanup, never direct scheduler free.
#[no_mangle]
pub unsafe extern "C" fn scx_sched_sysfs_add(sch: *mut scx_sched) -> c_int {
    // SAFETY: The same native kobj_type and callback records are used; creating
    // the sub kset follows successful add and retains native failure semantics.
    unsafe {
        #[cfg(CONFIG_EXT_SUB_SCHED)]
        {
            let parent = lupos_scx_core_parent(sch);
            let ret = if !parent.is_null() {
                lupos_scx_core_object_add_sub_kobj(sch, parent)
            } else { lupos_scx_core_object_add_root_kobj(sch) };
            if ret < 0 { return ret; }
            if lupos_scx_core_object_has_sub_attach(sch) {
                (*sch).sub_kset = lupos_scx_core_object_create_sub_kset(sch);
                if (*sch).sub_kset.is_null() { return -(ENOMEM as c_int); }
            }
            0
        }
        #[cfg(not(CONFIG_EXT_SUB_SCHED))]
        { lupos_scx_core_object_add_root_kobj(sch) }
    }
}
