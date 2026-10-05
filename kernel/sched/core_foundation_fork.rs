// SPDX-License-Identifier: GPL-2.0-only
#[no_mangle]
pub unsafe extern "C" fn set_load_weight(p: *mut task_struct, update_load: bool) {
    // SAFETY: The caller keeps p live with a valid scheduler priority and
    // class; required rq/class serialization covers reweighting or load assignment.
    unsafe {
        let prio = ((*p).static_prio - LUPOS_CORE_MAX_RT_PRIO) as usize;
        let mut lw: load_weight;
        if lupos_core_task_has_idle_policy(p) {
            lw = load_weight {
                weight: lupos_core_scale_load(LUPOS_CORE_WEIGHT_IDLEPRIO),
                inv_weight: LUPOS_CORE_WMULT_IDLEPRIO,
            };
        } else {
            lw = load_weight {
                weight: lupos_core_scale_load(sched_prio_to_weight[prio] as c_ulong),
                inv_weight: sched_prio_to_wmult[prio],
            };
        }
        if update_load {
            if let Some(reweight) = (*(*p).sched_class).reweight_task {
                reweight(lupos_core_task_rq(p), p, addr_of_mut!(lw));
                return;
            }
        }
        (*p).se.load = lw;
    }
}
unsafe fn __sched_fork(clone_flags: u64, p: *mut task_struct) {
    // SAFETY: The fork caller initializes an unpublished child. The separate
    // sched_init boot call initializes current before concurrent SCX observers;
    // that static init task is not an unpublished allocation. Neither case
    // justifies reading a different, already-live source task by aggregate copy.
    unsafe {
        (*p).on_rq = 0;
        (*p).se.on_rq = 0;
        (*p).se.exec_start = 0;
        (*p).se.sum_exec_runtime = 0;
        (*p).se.prev_sum_exec_runtime = 0;
        (*p).se.nr_migrations = 0;
        (*p).se.vruntime = 0;
        (*p).se.vlag = 0;
        (*p).se.rel_deadline = 0;
        lupos_core_init_list_head(addr_of_mut!((*p).se.group_node));
        lupos_core_warn_fork_delayed((*p).se.sched_delayed != 0);
        lupos_core_warn_fork_blocked((*p).is_blocked != 0);
        #[cfg(CONFIG_FAIR_GROUP_SCHED)]
        {
            (*p).se.cfs_rq = null_mut();
            #[cfg(CONFIG_CFS_BANDWIDTH)]
            init_cfs_throttle_work(p);
        }
        #[cfg(CONFIG_SCHEDSTATS)]
        core::ptr::write_bytes(addr_of_mut!((*p).stats), 0, 1);
        init_dl_entity(addr_of_mut!((*p).dl));
        lupos_core_init_list_head(addr_of_mut!((*p).rt.run_list));
        (*p).rt.timeout = 0;
        (*p).rt.time_slice = sched_rr_timeslice as c_uint;
        (*p).rt.on_rq = 0;
        (*p).rt.on_list = 0;
        #[cfg(CONFIG_SCHED_CLASS_EXT)]
        init_scx_entity(addr_of_mut!((*p).scx));
        #[cfg(CONFIG_PREEMPT_NOTIFIERS)]
        {
            (*p).preempt_notifiers.first = null_mut();
        }
        #[cfg(CONFIG_COMPACTION)]
        {
            (*p).capture_control = null_mut();
        }
        lupos_core_init_numa_balancing(clone_flags, p);
        lupos_core_set_wake_entry_flags(p, LUPOS_CORE_CSD_TYPE_TTWU);
        (*p).migration_pending = null_mut();
        lupos_core_init_sched_mm(p);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched_fork(clone_flags: u64, p: *mut task_struct) -> c_int {
    // SAFETY: The fork caller owns the unpublished child and keeps current
    // live; priority/policy initialization precedes any child wakeup.
    unsafe {
        __sched_fork(clone_flags, p);
        (*p).__state = LUPOS_CORE_TASK_NEW;
        (*p).prio = (*lupos_core_current()).normal_prio;
        uclamp_fork(p);
        if lupos_core_task_reset_on_fork(p) != 0 {
            if lupos_core_task_has_dl_policy(p) || lupos_core_task_has_rt_policy(p) {
                (*p).policy = LUPOS_CORE_SCHED_NORMAL;
                (*p).static_prio = LUPOS_CORE_NICE_ZERO_PRIO;
                (*p).rt_priority = 0;
                (*p).timer_slack_ns = (*p).default_timer_slack_ns;
            } else if lupos_core_prio_to_nice((*p).static_prio) < 0 {
                (*p).static_prio = LUPOS_CORE_NICE_ZERO_PRIO;
            }
            (*p).normal_prio = (*p).static_prio;
            (*p).prio = (*p).normal_prio;
            set_load_weight(p, false);
            (*p).se.custom_slice = 0;
            (*p).se.slice = sysctl_sched_base_slice as u64;
            lupos_core_set_task_reset_on_fork(p, 0);
        }
        if lupos_core_dl_prio((*p).prio) {
            return -LUPOS_CORE_EAGAIN;
        }
        lupos_core_scx_pre_fork(p);
        if lupos_core_rt_prio((*p).prio) {
            (*p).sched_class = addr_of!(rt_sched_class);
        } else {
            #[cfg(CONFIG_SCHED_CLASS_EXT)]
            {
                (*p).sched_class = if lupos_core_task_should_scx((*p).policy as c_int) {
                    addr_of!(ext_sched_class)
                } else {
                    addr_of!(fair_sched_class)
                };
            }
            #[cfg(not(CONFIG_SCHED_CLASS_EXT))]
            {
                (*p).sched_class = addr_of!(fair_sched_class);
            }
        }
        init_entity_runnable_average(addr_of_mut!((*p).se));
        #[cfg(CONFIG_SCHED_INFO)]
        if lupos_core_sched_info_on() {
            core::ptr::write_bytes(addr_of_mut!((*p).sched_info), 0, 1);
        }
        (*p).on_cpu = 0;
        lupos_core_init_task_preempt_count(p);
        lupos_core_plist_node_init(addr_of_mut!((*p).pushable_tasks), LUPOS_CORE_MAX_PRIO);
        lupos_core_rb_clear_node(addr_of_mut!((*p).pushable_dl_tasks));
        0
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched_cgroup_fork(
    p: *mut task_struct,
    kargs: *mut kernel_clone_args,
) -> c_int {
    // SAFETY: The fork caller owns the unpublished child and live clone
    // arguments/cgroup set. Its pi lock covers initial CPU and class setup.
    unsafe {
        let flags = lupos_core_raw_spin_lock_irqsave(addr_of_mut!((*p).pi_lock));
        #[cfg(CONFIG_CGROUP_SCHED)]
        {
            let css = (*(*kargs).cset).subsys[LUPOS_CORE_CPU_CGRP_ID as usize];
            let tg = css
                .cast::<u8>()
                .sub(offset_of!(task_group, css))
                .cast::<task_group>();
            (*p).sched_task_group = lupos_core_autogroup_task_group(p, tg);
        }
        lupos_core___set_task_cpu(p, lupos_core_smp_processor_id() as c_uint);
        if let Some(task_fork) = (*(*p).sched_class).task_fork {
            task_fork(p);
        }
        lupos_core_raw_spin_unlock_irqrestore(addr_of_mut!((*p).pi_lock), flags);
        lupos_core_scx_fork(p, kargs)
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched_cancel_fork(p: *mut task_struct) {
    // SAFETY: The fork caller keeps the not-yet-published child live while
    // the native configured scheduler-extension cancellation hook runs.
    unsafe {
        lupos_core_scx_cancel_fork(p);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched_post_fork(p: *mut task_struct) {
    // SAFETY: The fork caller keeps p live through MM-CID, utilization and
    // native scheduler-extension post-fork initialization.
    unsafe {
        sched_mm_cid_fork(p);
        uclamp_post_fork(p);
        lupos_core_scx_post_fork(p);
    }
}
#[no_mangle]
pub unsafe extern "C" fn to_ratio(period: u64, runtime: u64) -> u64 {
    // SAFETY: The native division is reached only with a nonzero period;
    // its scalar arguments require no pointer lifetime or shared-state access.
    unsafe {
        if runtime == LUPOS_CORE_RUNTIME_INF {
            return LUPOS_CORE_BW_UNIT;
        }
        if period == 0 {
            return 0;
        }
        lupos_core_div64_u64(runtime.wrapping_shl(LUPOS_CORE_BW_SHIFT), period)
    }
}
#[no_mangle]
pub unsafe extern "C" fn wake_up_new_task(p: *mut task_struct) {
    // SAFETY: The caller keeps the new task live and owns its first wakeup.
    // pi/rq locks protect selection/activation; class callbacks preserve locking.
    unsafe {
        let mut rf = MaybeUninit::<rq_flags>::uninit();
        let mut flags = LUPOS_CORE_WF_FORK;
        (*rf.as_mut_ptr()).flags = lupos_core_raw_spin_lock_irqsave(addr_of_mut!((*p).pi_lock));
        lupos_core_write_once_uint(addr_of_mut!((*p).__state), LUPOS_CORE_TASK_RUNNING);
        (*p).recent_used_cpu = lupos_core_task_cpu(p);
        lupos_core___set_task_cpu(
            p,
            select_task_rq(p, lupos_core_task_cpu(p), addr_of_mut!(flags)) as c_uint,
        );
        let rq = ___task_rq_lock(p, rf.as_mut_ptr());
        update_rq_clock(rq);
        post_init_entity_util_avg(p);
        activate_task(
            rq,
            p,
            LUPOS_CORE_ENQUEUE_NOCLOCK | LUPOS_CORE_ENQUEUE_INITIAL,
        );
        lupos_core_trace_wakeup_new(p);
        wakeup_preempt(rq, p, flags);
        if let Some(woken) = (*(*p).sched_class).task_woken {
            lupos_core_rq_unpin_lock(rq, rf.as_mut_ptr());
            woken(rq, p);
            lupos_core_rq_repin_lock(rq, rf.as_mut_ptr());
        }
        lupos_core_task_rq_unlock(rq, p, rf.as_mut_ptr());
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched_set_stop_task(cpu: c_int, stop: *mut task_struct) {
    // SAFETY: The CPU setup/teardown caller serializes this valid CPU's
    // stop-task replacement and keeps old/new tasks live through class updates.
    unsafe {
        let old_stop = (*lupos_core_cpu_rq(cpu)).stop;
        if !stop.is_null() {
            let mut param = sched_param {
                sched_priority: LUPOS_CORE_MAX_RT_PRIO - 1,
            };
            sched_setscheduler_nocheck(stop, LUPOS_CORE_SCHED_FIFO as c_int, addr_of_mut!(param));
            lupos_core_task_class_write(stop, addr_of!(stop_sched_class));
            lupos_core_lockdep_stop_pi_class(stop);
        }
        (*lupos_core_cpu_rq(cpu)).stop = stop;
        if !old_stop.is_null() {
            lupos_core_task_class_write(old_stop, addr_of!(rt_sched_class));
        }
    }
}
