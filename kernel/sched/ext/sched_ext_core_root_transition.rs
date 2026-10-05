// SPDX-License-Identifier: GPL-2.0
// F12 additive source repair: ext.c:6380-6543,7382-7918 at
// 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. The existing ext.rs identities
// remain with their owners. Native leaves are unqualified C runtime.
compile_error!("SOURCE ONLY HOLD: root transition ABI, stack, races and protection remain unqualified");

use super::*;
use core::{mem::MaybeUninit, ptr};
use kernel::ffi::{c_int, c_uint, c_ulonglong};

/// Preserve the F00 state owner at active native WARN expression sites.
///
/// # Safety
/// Caller owns the original root transition protocol and supplies a native
/// enable-state enumerator. This bridge adds no lock or ordering of its own.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_root_set_enable_state(to: scx_enable_state) -> scx_enable_state {
    unsafe { super::scx_set_enable_state(to) }
}

/// Recover all tasks, stop callbacks, then retire the root and CID publication.
///
/// # Safety
/// F11's disable worker pins sch and has claimed its exit. This may sleep and
/// must run outside scx_enable_mutex. All owner functions retain their actual
/// task/cgroup/RCU protocols; no old-C teardown is available as a fallback.
pub(crate) unsafe fn scx_root_disable(sch: *mut scx_sched) {
    // SAFETY: Bypass/drain precedes any blocking lock. Task references and
    // iterator locks are managed by F06; sch stays pinned through the last
    // bypass release, including the already-disabled early path.
    unsafe {
        scx_bypass(sch, true);
        lupos_scx_root_drain_descendants(sch);
        match scx_set_enable_state(SCX_DISABLING) {
            SCX_DISABLING => lupos_scx_root_warn_duplicate_disable(),
            SCX_DISABLED => {
                lupos_scx_root_warn_no_ops(sch);
                lupos_scx_root_warn_disabled_without_ops();
                scx_bypass(sch, false);
                return;
            }
            _ => {}
        }
        lupos_scx_root_enable_mutex_lock();
        let was_switched_all = lupos_scx_root_switched_all();
        lupos_scx_core_switched_all_disable();
        lupos_scx_core_switching_all_write_once(false);

        scx_cgroup_lock();
        lupos_scx_core_cgroup_enabled_write(false);
        scx_cgroup_exit(sch);
        scx_cgroup_unlock();

        lupos_scx_root_fork_down_write();
        lupos_scx_core_init_task_enabled_write(false);
        let mut iter = MaybeUninit::<scx_task_iter>::uninit();
        let sti = iter.as_mut_ptr();
        scx_task_iter_start(sti, ptr::null_mut());
        loop {
            let p = scx_task_iter_next_locked(sti);
            if p.is_null() { break; }
            let mut queue_flags = (DEQUEUE_SAVE | DEQUEUE_MOVE | DEQUEUE_NOCLOCK) as c_uint;
            let old_class = (*p).sched_class;
            let new_class = scx_setscheduler_class(p);
            lupos_scx_root_update_rq_clock(lupos_scx_core_task_rq(p));
            if old_class != new_class { queue_flags |= DEQUEUE_CLASS as c_uint; }
            lupos_scx_root_disable_change(p, queue_flags, new_class);
            scx_disable_and_exit_task(lupos_scx_core_task_sched(p), p);
        }
        scx_task_iter_stop(sti);
        scx_disable_dump(sch);
        scx_cgroup_lock();
        lupos_scx_root_set_cgroup_sched(sch, ptr::null_mut());
        scx_cgroup_unlock();
        lupos_scx_root_fork_up_write();

        // Recovery cannot fail: a swap error warns with both servers detached.
        let mut cpu = lupos_scx_root_next_possible_cpu(-1);
        while (cpu as c_uint) < lupos_scx_root_possible_cpu_limit() {
            let rq = lupos_scx_root_cpu_rq(cpu);
            lupos_scx_root_restore_bw(rq, was_switched_all, cpu);
            cpu = lupos_scx_root_next_possible_cpu(cpu);
        }
        lupos_scx_core_enabled_disable();
        lupos_scx_core_cid_type_disable();
        if (*lupos_scx_root_sched_ops(sch)).flags & SCX_OPS_TID_TO_TASK as u64 != 0 {
            lupos_scx_core_tid_to_task_disable();
        }
        lupos_scx_root_zero_has_op(sch);
        scx_idle_disable();
        lupos_scx_root_synchronize_rcu();
        if (*lupos_scx_root_sched_ops(sch)).flags & SCX_OPS_TID_TO_TASK as u64 != 0 {
            lupos_scx_root_tid_hash_free();
        }
        scx_log_sched_disable(sch);
        if (*lupos_scx_root_sched_ops(sch)).exit.is_some() {
            // sch->exit_info remains evaluated within native SCX_CALL_OP.
            lupos_scx_root_call_exit(sch);
        }
        lupos_scx_root_mark_dead(sch);
        lupos_scx_root_synchronize_rcu();
        scx_unlink_sched(sch);
        lupos_scx_root_cpus_read_lock();
        lupos_scx_core_root_clear();
        scx_cid_retire_tables();
        lupos_scx_root_cpus_read_unlock();

        #[cfg(CONFIG_EXT_SUB_SCHED)]
        if !(*sch).sub_kset.is_null() {
            lupos_scx_root_del_kobj(lupos_scx_root_sub_kobj(sch));
        }
        if lupos_scx_root_in_sysfs(sch) {
            lupos_scx_root_del_kobj(ptr::addr_of_mut!((*sch).kobj));
        }
        free_kick_syncs();
        lupos_scx_root_enable_mutex_unlock();
        lupos_scx_root_warn_disabled_at_tail();
        scx_bypass(sch, false);
    }
}

/// Reject a root attachment whose advertised hotplug sequence is stale.
///
/// # Safety
/// sch and ops stay live. Caller holds the original CPUs read lock, and the
/// hotplug atomic read retains native signed-long conversion to unsigned long
/// long without strengthening ordering or snapshotting the ops field early.
unsafe fn check_hotplug_seq(sch: *mut scx_sched, ops: *const sched_ext_ops) -> c_int {
    unsafe {
        if (*ops).hotplug_seq != 0 {
            let global_hotplug_seq = lupos_scx_core_hotplug_seq_read() as c_ulonglong;
            if (*ops).hotplug_seq != global_hotplug_seq {
                lupos_scx_root_exit_hotplug(sch, ops, global_hotplug_seq);
                return -(EBUSY as c_int);
            }
        }
        0
    }
}

/// Validate callback/flag combinations and CPU-versus-CID form dependencies.
///
/// # Safety
/// sch, ops and any ancestors remain pinned by the caller's enable protocol.
/// ops may point to the shorter CID allocation: its CPU-only tail MUST remain
/// unread until !sch->is_cid_type is established, including diagnostics.
#[no_mangle]
pub unsafe extern "C" fn scx_validate_ops(sch: *mut scx_sched, ops: *const sched_ext_ops) -> c_int {
    unsafe {
        if (*ops).flags & SCX_OPS_ENQ_LAST as u64 != 0 && (*ops).enqueue.is_none() {
            lupos_scx_root_error_enq_last(sch);
            return -(EINVAL as c_int);
        }
        if (*ops).flags & SCX_OPS_TID_TO_TASK as u64 != 0
            && !lupos_scx_core_parent(sch).is_null()
            && (*lupos_scx_root_sched_ops(lupos_scx_core_ancestor_at(sch, 0))).flags
                & SCX_OPS_TID_TO_TASK as u64 == 0 {
            lupos_scx_root_error_tid_dependency(sch);
            return -(EINVAL as c_int);
        }
        if (*ops).flags & SCX_OPS_BUILTIN_IDLE_PER_NODE as u64 != 0
            && ((*ops).update_idle.is_some()
                && (*ops).flags & SCX_OPS_KEEP_BUILTIN_IDLE as u64 == 0) {
            lupos_scx_root_error_idle_per_node(sch);
            return -(EINVAL as c_int);
        }
        if !(*sch).is_cid_type && ((*ops).cpu_acquire.is_some() || (*ops).cpu_release.is_some()) {
            lupos_scx_root_warn_deprecated_cpu_ops();
        }
        if !(*sch).is_cid_type {
            if !lupos_scx_core_parent(sch).is_null() {
                lupos_scx_root_error_sub_cpu_form(sch);
                return -(EINVAL as c_int);
            }
            if (*ops).sub_attach.is_some() || (*ops).sub_detach.is_some() {
                lupos_scx_root_error_attach_cpu_form(sch);
                return -(EINVAL as c_int);
            }
        }
        0
    }
}

/// Run the serialized root attachment transaction from its native work shim.
///
/// # Safety
/// work is the queued member of a live scx_enable_cmd, and cmd/ops outlive the
/// synchronous queue/flush. Runs on the dedicated FIFO helper and may sleep.
/// Its raw iterator storage never moves between start, unlock/relock and stop.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_root_enable_work_body(work: *mut kthread_work) {
    // SAFETY: Early failures precede arming disable and use direct cleanup.
    // After ENABLING, every failure releases owned locks before F11 teardown;
    // cmd reports success after the detailed ops.exit notification is flushed.
    unsafe {
        let cmd = lupos_scx_root_work_cmd(work);
        let ops = lupos_scx_root_cmd_ops(cmd);
        let cgrp = root_cgroup();
        lupos_scx_root_enable_mutex_lock();
        if scx_enable_state() != SCX_DISABLED {
            lupos_scx_root_enable_mutex_unlock();
            (*cmd).ret = -(EBUSY as c_int);
            return;
        }
        // Do not let an older unregistration clobber a newly published priv.
        if lupos_scx_root_ops_priv_present(ops) {
            lupos_scx_root_enable_mutex_unlock();
            (*cmd).ret = -(EBUSY as c_int);
            return;
        }
        let ret = alloc_kick_syncs();
        if ret != 0 {
            lupos_scx_root_enable_mutex_unlock();
            (*cmd).ret = ret;
            return;
        }
        if (*ops).flags & SCX_OPS_TID_TO_TASK as u64 != 0 {
            let ret = lupos_scx_root_tid_hash_init();
            if ret != 0 {
                free_kick_syncs();
                lupos_scx_root_enable_mutex_unlock();
                (*cmd).ret = ret;
                return;
            }
        }
        #[cfg(CONFIG_EXT_SUB_SCHED)]
        lupos_scx_root_cgroup_get(cgrp);
        let sch = scx_alloc_and_add_sched(cmd, cgrp, ptr::null_mut());
        if lupos_scx_root_is_err_sched(sch) {
            let ret = lupos_scx_root_sched_ptr_err(sch) as c_int;
            if (*ops).flags & SCX_OPS_TID_TO_TASK as u64 != 0 {
                lupos_scx_root_tid_hash_free();
            }
            free_kick_syncs();
            lupos_scx_root_enable_mutex_unlock();
            (*cmd).ret = ret;
            return;
        }
        if (*sch).is_cid_type { lupos_scx_core_cid_type_enable(); }
        lupos_scx_root_warn_set_enabling();
        lupos_scx_root_warn_root_present();
        lupos_scx_core_rejected_reset();
        let mut cpu = lupos_scx_root_next_possible_cpu(-1);
        while (cpu as c_uint) < lupos_scx_root_possible_cpu_limit() {
            let rq = lupos_scx_root_cpu_rq(cpu);
            (*rq).scx.local_dsq.sched = sch;
            (*rq).scx.cpuperf_target = SCX_CPUPERF_ONE as u32;
            cpu = lupos_scx_root_next_possible_cpu(cpu);
        }
        lupos_scx_root_discard_stale_ecaps();
        lupos_scx_root_rescue_set_knobs(sch);

        // This flag represents only err_disable_unlock_all's two held locks.
        // It does not infer cleanup from object contents or bypass depth.
        let mut all_locked = false;
        let ret = 'enable: {
            lupos_scx_root_cpus_read_lock();
            let ret = scx_cid_init(sch);
            if ret != 0 { lupos_scx_root_cpus_read_unlock(); break 'enable ret; }
            lupos_scx_core_root_publish(sch);
            let ret = scx_link_sched(sch);
            if ret != 0 { lupos_scx_root_cpus_read_unlock(); break 'enable ret; }
            scx_idle_enable(ops);
            if (*sch).is_cid_type && lupos_scx_root_has_init_cids(sch) {
                let ret = lupos_scx_root_call_init_cids(sch);
                if ret != 0 {
                    let ret = lupos_scx_root_sanitize_init_cids(sch, ret);
                    lupos_scx_root_cpus_read_unlock();
                    lupos_scx_root_error_init_cids(sch, ret);
                    break 'enable ret;
                }
            }
            scx_cid_publish_tables();
            let ret = scx_arena_pool_init(sch);
            if ret != 0 { lupos_scx_root_cpus_read_unlock(); break 'enable ret; }
            let ret = scx_set_cmask_scratch_alloc(sch);
            if ret != 0 { lupos_scx_root_cpus_read_unlock(); break 'enable ret; }
            let ret = lupos_scx_root_alloc_pshards(sch);
            if ret != 0 { lupos_scx_root_cpus_read_unlock(); break 'enable ret; }
            lupos_scx_root_init_caps(sch);
            if (*lupos_scx_root_sched_ops(sch)).init.is_some() {
                let ret = lupos_scx_root_call_init(sch);
                if ret != 0 {
                    let ret = lupos_scx_root_sanitize_init(sch, ret);
                    lupos_scx_root_cpus_read_unlock();
                    lupos_scx_root_error_init(sch, ret);
                    break 'enable ret;
                }
                (*(*sch).exit_info).flags |= SCX_EFLAG_INITIALIZED as u64;
            }
            let ret = scx_sched_sysfs_add(sch);
            if ret != 0 { lupos_scx_root_cpus_read_unlock(); break 'enable ret; }
            let mut i = SCX_OPI_CPU_HOTPLUG_BEGIN as c_int;
            while i < SCX_OPI_CPU_HOTPLUG_END as c_int {
                if lupos_scx_root_ops_slot_present(ops, i) { lupos_scx_root_set_has_op(sch, i); }
                i += 1;
            }
            let ret = check_hotplug_seq(sch, ops);
            if ret != 0 { lupos_scx_root_cpus_read_unlock(); break 'enable ret; }
            scx_idle_update_selcpu_topology(ops);
            lupos_scx_root_cpus_read_unlock();
            let ret = scx_validate_ops(sch, ops);
            if ret != 0 { break 'enable ret; }

            cpu = lupos_scx_root_next_possible_cpu(-1);
            while (cpu as c_uint) < lupos_scx_root_possible_cpu_limit() {
                let rq = lupos_scx_root_cpu_rq(cpu);
                let ret = lupos_scx_root_attach_bw(rq);
                if ret != 0 {
                    lupos_scx_root_warn_attach_bw(cpu, ret);
                    break 'enable ret;
                }
                cpu = lupos_scx_root_next_possible_cpu(cpu);
            }
            scx_bypass(sch, true);
            i = SCX_OPI_NORMAL_BEGIN as c_int;
            while i < SCX_OPI_NORMAL_END as c_int {
                if lupos_scx_root_ops_slot_present(ops, i) { lupos_scx_root_set_has_op(sch, i); }
                i += 1;
            }
            // sch owns a full zero-allocated union; unlike external ops above,
            // reading its CPU tail is intentional even for the CID form.
            let sch_ops = lupos_scx_root_sched_ops(sch);
            if (*sch_ops).cpu_acquire.is_some() || (*sch_ops).cpu_release.is_some() {
                (*sch_ops).flags |= SCX_OPS_HAS_CPU_PREEMPT as u64;
            }
            lupos_scx_root_fork_down_write();
            lupos_scx_root_warn_init_task_enabled();
            lupos_scx_core_init_task_enabled_write(true);
            if (*ops).flags & SCX_OPS_TID_TO_TASK as u64 != 0 {
                lupos_scx_core_tid_to_task_enable();
            }
            scx_cgroup_lock();
            all_locked = true;
            lupos_scx_root_set_cgroup_sched(sch, sch);
            let ret = scx_cgroup_init(sch);
            if ret != 0 { break 'enable ret; }
            lupos_scx_root_warn_cgroup_enabled();
            lupos_scx_core_cgroup_enabled_write(true);

            let mut iter = MaybeUninit::<scx_task_iter>::uninit();
            let sti = iter.as_mut_ptr();
            scx_task_iter_start(sti, ptr::null_mut());
            loop {
                let p = scx_task_iter_next_locked(sti);
                if p.is_null() { break; }
                // The list lock proves usage > 0. INIT_BEGIN must be stored
                // under rq before dropping iterator locks for init_task.
                lupos_scx_root_get_task(p);
                scx_set_task_state(p, SCX_TASK_INIT_BEGIN);
                scx_task_iter_unlock(sti);
                let ret = __scx_init_task(sch, p, ptr::null_mut(), false);
                scx_task_iter_relock(sti, p);
                if lupos_scx_root_unlikely_init_error(ret) {
                    if scx_get_task_state(p) != SCX_TASK_DEAD {
                        scx_set_task_state(p, SCX_TASK_NONE);
                    }
                    scx_task_iter_stop(sti);
                    lupos_scx_root_error_init_task(sch, ret, p);
                    lupos_scx_root_put_task(p);
                    break 'enable ret;
                }
                if scx_get_task_state(p) == SCX_TASK_DEAD {
                    // Cancellation targets the scheduler that actually ran
                    // init_task, even if death removed p while locks were down.
                    scx_sub_init_cancel_task(sch, p);
                } else {
                    scx_set_task_state(p, SCX_TASK_INIT);
                    lupos_scx_core_task_set_sched(p, sch);
                    scx_set_task_state(p, SCX_TASK_READY);
                }
                if scx_tid_to_task_enabled() && !lupos_scx_root_tasks_node_empty(p) {
                    scx_tid_hash_insert(p);
                }
                lupos_scx_root_put_task(p);
            }
            scx_task_iter_stop(sti);
            scx_cgroup_unlock();
            lupos_scx_root_fork_up_write();
            all_locked = false;

            lupos_scx_core_switching_all_write_once((*ops).flags & SCX_OPS_SWITCH_PARTIAL as u64 == 0);
            lupos_scx_core_enabled_enable();
            lupos_scx_root_fork_down_write();
            // The same stable iterator storage is freshly initialized for
            // READY -> ENABLED, synchronizing against task exit via its lock.
            scx_task_iter_start(sti, ptr::null_mut());
            loop {
                let p = scx_task_iter_next_locked(sti);
                if p.is_null() { break; }
                let mut queue_flags = (DEQUEUE_SAVE | DEQUEUE_MOVE) as c_uint;
                let old_class = (*p).sched_class;
                let new_class = scx_setscheduler_class(p);
                if scx_get_task_state(p) != SCX_TASK_READY { continue; }
                if old_class != new_class { queue_flags |= DEQUEUE_CLASS as c_uint; }
                lupos_scx_root_enable_change(sch, p, queue_flags, new_class);
            }
            scx_task_iter_stop(sti);
            lupos_scx_root_fork_up_write();
            scx_bypass(sch, false);
            if !scx_tryset_enable_state(SCX_ENABLED, SCX_ENABLING) {
                lupos_scx_root_warn_exit_none(sch);
                break 'enable -(EBUSY as c_int);
            }
            if (*ops).flags & SCX_OPS_SWITCH_PARTIAL as u64 == 0 {
                lupos_scx_core_switched_all_enable();
            }
            if lupos_scx_root_switched_all() {
                cpu = lupos_scx_root_next_possible_cpu(-1);
                while (cpu as c_uint) < lupos_scx_root_possible_cpu_limit() {
                    lupos_scx_root_detach_fair(lupos_scx_root_cpu_rq(cpu));
                    cpu = lupos_scx_root_next_possible_cpu(cpu);
                }
            }
            lupos_scx_root_log_enabled(sch);
            lupos_scx_root_uevent_add(sch);
            lupos_scx_root_enable_mutex_unlock();
            lupos_scx_core_enable_seq_inc();
            (*cmd).ret = 0;
            return;
        };
        if all_locked {
            scx_cgroup_unlock();
            lupos_scx_root_fork_up_write();
        }
        lupos_scx_root_enable_mutex_unlock();
        lupos_scx_root_error_enable(sch, ret);
        scx_flush_disable_work(sch);
        (*cmd).ret = 0;
    }
}

/// Submit root/sub attachment to the singleton FIFO helper and wait for it.
///
/// # Safety
/// Native scx_enable supplies its original static helper slot/mutex; cmd and
/// link obey the struct_ops registration lifetime. The helper slot is shared
/// only under its original READ_ONCE/WRITE_ONCE and mutex protocol. No reference
/// is formed across unlocked access. FFI race semantics remain unqualified.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_root_enable_body(
    cmd: *mut scx_enable_cmd, _link: *mut bpf_link,
    helper: *mut *mut kthread_worker, helper_mutex: *mut mutex,
) -> i32 {
    unsafe {
        if lupos_scx_root_housekeeping_domain_boot() {
            lupos_scx_root_error_isolation();
            return -(EINVAL as i32);
        }
        if lupos_scx_root_helper_read_once(helper).is_null() {
            lupos_scx_root_helper_mutex_lock(helper_mutex);
            if (*helper).is_null() {
                let w = lupos_scx_root_run_worker();
                if lupos_scx_root_is_err_or_null_worker(w) {
                    lupos_scx_root_helper_mutex_unlock(helper_mutex);
                    return -(ENOMEM as i32);
                }
                lupos_scx_root_helper_fifo(w);
                lupos_scx_root_helper_write_once(helper, w);
            }
            lupos_scx_root_helper_mutex_unlock(helper_mutex);
        }
        let work = ptr::addr_of_mut!((*cmd).work);
        #[cfg(CONFIG_EXT_SUB_SCHED)]
        if (*lupos_scx_root_cmd_ops(cmd)).sub_cgroup_id > 1 {
            lupos_scx_root_init_sub_work(work);
        } else {
            lupos_scx_root_init_root_work(work);
        }
        #[cfg(not(CONFIG_EXT_SUB_SCHED))]
        lupos_scx_root_init_root_work(work);
        lupos_scx_root_queue_work(lupos_scx_root_helper_read_once(helper), work);
        lupos_scx_root_flush_work(work);
        (*cmd).ret
    }
}

/// Native sched_change continuation for root teardown.
/// # Safety
/// Native scoped_guard owns the original task sched_change context. The rq
/// remains locked; new_class is the pre-guard original class-selection result.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_root_disable_change_body(p: *mut task_struct, new_class: *const sched_class) {
    unsafe { (*p).sched_class = new_class; }
}

/// Native sched_change continuation for READY -> ENABLED switching.
/// # Safety
/// The original native guard is live, rq/task/sch are pinned, and reading the
/// default slice occurs here after sched_change_begin, never before entry.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_root_enable_change_body(
    sch: *mut scx_sched, p: *mut task_struct, new_class: *const sched_class,
) {
    unsafe {
        scx_set_task_slice(p, lupos_scx_root_slice_read_once(sch));
        (*p).sched_class = new_class;
    }
}

/// Attach ext bandwidth while the native rq_lock_irqsave guard is live.
/// # Safety
/// The synchronous native caller owns rq's exact guard and IRQ context.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_root_attach_bw_body(rq: *mut rq) -> c_int {
    unsafe {
        lupos_scx_root_update_rq_clock(rq);
        lupos_scx_root_dl_attach_ext(rq)
    }
}

/// Restore fair bandwidth or detach ext under the original recovery guard.
/// # Safety
/// rq's exact native guard is live. was_switched_all is the pre-disable static
/// key snapshot; cpu is this rq's original possible-CPU loop index.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_root_restore_bw_body(rq: *mut rq, was_switched_all: bool, cpu: c_int) {
    unsafe {
        lupos_scx_root_update_rq_clock(rq);
        if was_switched_all {
            if lupos_scx_root_warn_dl_swap(rq) { lupos_scx_root_warn_restore_bw(cpu); }
        } else {
            lupos_scx_root_dl_detach_ext(rq);
        }
    }
}

/// Release fair's reservation after the root full-mode switch is committed.
/// # Safety
/// Native guard(rq_lock_irqsave) is live for the entire call and rq stays pinned.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_root_detach_fair_body(rq: *mut rq) {
    unsafe {
        lupos_scx_root_update_rq_clock(rq);
        lupos_scx_root_dl_detach_fair(rq);
    }
}
