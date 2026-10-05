// SPDX-License-Identifier: GPL-2.0-only
// core.c:8355..8905; CPU hotplug and isolation lock ordering.
#[no_mangle]
pub unsafe extern "C" fn cpuset_cpumask_can_shrink(
    cur: *const cpumask,
    trial: *const cpumask,
) -> c_int {
    // SAFETY: Both native CPU masks remain live through the empty-mask
    // check and deadline-bandwidth admission query.
    unsafe {
        if lupos_core_cpumask_empty(cur) {
            return 1;
        }
        dl_cpuset_cpumask_can_shrink(cur, trial)
    }
}
#[no_mangle]
pub unsafe extern "C" fn task_can_attach(p: *mut task_struct) -> c_int {
    // SAFETY: The caller keeps p live and follows native cpuset attachment
    // serialization while inspecting its no-setaffinity task flag.
    unsafe {
        if (*p).flags & LUPOS_CORE_PF_NO_SETAFFINITY != 0 {
            -(LUPOS_CORE_EINVAL as c_int)
        } else {
            0
        }
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
#[no_mangle]
pub unsafe extern "C" fn migrate_task_to(p: *mut task_struct, target_cpu: c_int) -> c_int {
    // SAFETY: The caller keeps p live; the synchronous stopper completes
    // before the stack migration argument expires and revalidates CPU placement.
    unsafe {
        let mut arg = MaybeUninit::<migration_arg>::zeroed();
        let arg = arg.as_mut_ptr();
        (*arg).task = p;
        (*arg).dest_cpu = target_cpu;
        let cpu = lupos_core_task_cpu(p);
        if cpu == target_cpu {
            return 0;
        }
        if !lupos_core_cpumask_test_cpu(target_cpu, (*p).cpus_ptr) {
            return -(LUPOS_CORE_EINVAL as c_int);
        }
        lupos_core_trace_sched_move_numa(p, cpu, target_cpu);
        lupos_core_header_stop_one_cpu(cpu as c_uint, Some(migration_cpu_stop), arg.cast())
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
#[no_mangle]
pub unsafe extern "C" fn sched_setnuma(p: *mut task_struct, nid: c_int) {
    // SAFETY: The caller keeps p live; acquired task-rq locks cover the
    // matched class-change scope and preferred-node update.
    unsafe {
        let mut rf = MaybeUninit::<rq_flags>::uninit();
        let rf = rf.as_mut_ptr();
        let rq = _task_rq_lock(p, rf);
        let ctx = sched_change_begin(p, LUPOS_CORE_DEQUEUE_SAVE as c_uint);
        (*p).numa_preferred_nid = nid;
        sched_change_end(ctx);
        lupos_core_task_rq_unlock(rq, p, rf);
    }
}
#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn sched_force_init_mm() {
    // SAFETY: The outgoing CPU's hotplug task owns its live active MM.
    // Native IRQ/MM switching and lazy-TLB references preserve the handoff.
    unsafe {
        let current = lupos_core_current();
        let mm = (*current).active_mm;
        if mm != addr_of_mut!(init_mm) {
            lupos_core_mmgrab_lazy_tlb(addr_of_mut!(init_mm));
            lupos_core_local_irq_disable();
            (*current).active_mm = addr_of_mut!(init_mm);
            lupos_core_switch_mm_irqs_off(mm, addr_of_mut!(init_mm), current);
            lupos_core_local_irq_enable();
            lupos_core_finish_arch_post_lock_switch();
            lupos_core_mmdrop_lazy_tlb(mm);
        }
    }
}
#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe extern "C" fn __balance_push_cpu_stop(arg: *mut c_void) -> c_int {
    // SAFETY: The stopper owns p's queued task reference. pi/rq locks
    // serialize fallback/migration, including transfer to the destination rq.
    unsafe {
        let p = arg.cast::<task_struct>();
        let mut rq = lupos_core_this_rq();
        let mut rf = MaybeUninit::<rq_flags>::uninit();
        let rf = rf.as_mut_ptr();
        lupos_core_raw_spin_lock_irq(addr_of_mut!((*p).pi_lock));
        let cpu = select_fallback_rq((*rq).cpu, p);
        lupos_core_rq_lock(rq, rf);
        update_rq_clock(rq);
        if lupos_core_task_rq(p) == rq && lupos_core_task_on_rq_queued(p) {
            rq = __migrate_task(rq, rf, p, cpu);
        }
        lupos_core_rq_unlock(rq, rf);
        lupos_core_raw_spin_unlock_irq(addr_of_mut!((*p).pi_lock));
        lupos_core_put_task_struct(p);
        0
    }
}
#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe extern "C" fn balance_push(rq: *mut rq) {
    // SAFETY: The caller owns the outgoing live rq lock. A task reference
    // and preempt/IRQ exclusion span stopper publication before rq reacquisition.
    unsafe {
        let push_task = lupos_core_rq_curr(rq);
        lupos_core_assert_rq_held(rq);
        (*rq).balance_callback = addr_of_mut!(balance_push_callback);
        if !lupos_core_cpu_dying((*rq).cpu) || rq != lupos_core_this_rq() {
            return;
        }
        if kthread_is_per_cpu(push_task) || lupos_core_is_migration_disabled(push_task) {
            if (*rq).nr_running == 0
                && !rq_has_pinned_tasks(rq)
                && lupos_core_rcuwait_active(addr_of_mut!((*rq).hotplug_wait))
            {
                lupos_core_raw_spin_rq_unlock(rq);
                rcuwait_wake_up(addr_of_mut!((*rq).hotplug_wait));
                raw_spin_rq_lock_nested(rq, 0);
            }
            return;
        }
        lupos_core_get_task_struct(push_task);
        lupos_core_preempt_disable();
        lupos_core_raw_spin_rq_unlock(rq);
        lupos_core_header_stop_one_cpu_nowait(
            (*rq).cpu as c_uint,
            Some(__balance_push_cpu_stop),
            push_task.cast(),
            lupos_core_this_push_work(),
        );
        lupos_core_preempt_enable();
        raw_spin_rq_lock_nested(rq, 0);
    }
}
#[cfg(not(CONFIG_HOTPLUG_CPU))]
unsafe extern "C" fn balance_push(_rq: *mut rq) {}
#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn balance_push_set(cpu: c_int, on: bool) {
    // SAFETY: cpu selects a live permanent rq. The IRQ-saving rq lock
    // serializes installation/removal of the special native balance callback.
    unsafe {
        let rq = lupos_core_cpu_rq(cpu);
        let mut rf = MaybeUninit::<rq_flags>::uninit();
        let rf = rf.as_mut_ptr();
        lupos_core_rq_lock_irqsave(rq, rf);
        if on {
            lupos_core_warn_once_site_8532(!(*rq).balance_callback.is_null());
            (*rq).balance_callback = addr_of_mut!(balance_push_callback);
        } else if (*rq).balance_callback == addr_of_mut!(balance_push_callback) {
            (*rq).balance_callback = null_mut();
        }
        lupos_core_rq_unlock_irqrestore(rq, rf);
    }
}
#[cfg(not(CONFIG_HOTPLUG_CPU))]
unsafe fn balance_push_set(_cpu: c_int, _on: bool) {}
#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn balance_hotplug_wait() {
    // SAFETY: The outgoing CPU's hotplug thread pins this live rq. Native
    // rcuwait publication/state ordering spans the wait and matching finish.
    unsafe {
        let rq = lupos_core_this_rq();
        // Expand rcuwait_wait_event sequencing in Rust; native leaves are single
        // prepare/state/signal/finish operations, not a C predicate/algorithm loop.
        lupos_core_prepare_to_rcuwait(addr_of_mut!((*rq).hotplug_wait));
        loop {
            lupos_core_set_current_state(LUPOS_CORE_TASK_UNINTERRUPTIBLE);
            if (*rq).nr_running == 1 && !rq_has_pinned_tasks(rq) {
                break;
            }
            if lupos_core_signal_pending_state(
                LUPOS_CORE_TASK_UNINTERRUPTIBLE as c_ulong,
                lupos_core_current(),
            ) {
                break;
            }
            schedule();
        }
        lupos_core_finish_rcuwait(addr_of_mut!((*rq).hotplug_wait));
    }
}
#[cfg(not(CONFIG_HOTPLUG_CPU))]
unsafe fn balance_hotplug_wait() {}
#[no_mangle]
pub unsafe extern "C" fn set_rq_online(rq: *mut rq) {
    // SAFETY: The caller holds the live rq's lock with a valid root domain;
    // mask publication precedes class callbacks as required by native hotplug.
    unsafe {
        if (*rq).online == 0 {
            lupos_core_cpumask_set_cpu((*rq).cpu as c_uint, lupos_core_root_domain_online((*rq).rd));
            (*rq).online = 1;
            let mut class = lupos_core_first_class();
            while !class.is_null() {
                if let Some(cb) = (*class).rq_online {
                    cb(rq);
                }
                class = lupos_core_next_class(class);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn set_rq_offline(rq: *mut rq) {
    // SAFETY: The caller holds rq's lock and keeps its root domain live;
    // class offline callbacks finish before clearing native online membership.
    unsafe {
        if (*rq).online != 0 {
            update_rq_clock(rq);
            let mut class = lupos_core_first_class();
            while !class.is_null() {
                if let Some(cb) = (*class).rq_offline {
                    cb(rq);
                }
                class = lupos_core_next_class(class);
            }
            lupos_core_cpumask_clear_cpu((*rq).cpu as c_uint, lupos_core_root_domain_online((*rq).rd));
            (*rq).online = 0;
        }
    }
}
unsafe fn sched_set_rq_online(rq: *mut rq, cpu: c_int) {
    // SAFETY: rq and cpu identify the same live runqueue. Its acquired
    // IRQ-saving lock protects root-domain validation and online publication.
    unsafe {
        let mut rf = MaybeUninit::<rq_flags>::uninit();
        let rf = rf.as_mut_ptr();
        lupos_core_rq_lock_irqsave(rq, rf);
        if !(*rq).rd.is_null() {
            lupos_core_bug_on_site_8619(!lupos_core_cpumask_test_cpu(
                cpu,
                lupos_core_root_domain_span((*rq).rd),
            ));
            set_rq_online(rq);
        }
        lupos_core_rq_unlock_irqrestore(rq, rf);
    }
}
unsafe fn sched_set_rq_offline(rq: *mut rq, cpu: c_int) {
    // SAFETY: rq and cpu identify the same live runqueue. Its acquired
    // IRQ-saving lock protects domain validation and class offline transitions.
    unsafe {
        let mut rf = MaybeUninit::<rq_flags>::uninit();
        let rf = rf.as_mut_ptr();
        lupos_core_rq_lock_irqsave(rq, rf);
        if !(*rq).rd.is_null() {
            lupos_core_bug_on_site_8631(!lupos_core_cpumask_test_cpu(
                cpu,
                lupos_core_root_domain_span((*rq).rd),
            ));
            set_rq_offline(rq);
        }
        lupos_core_rq_unlock_irqrestore(rq, rf);
    }
}
unsafe fn cpuset_cpu_active() {
    // SAFETY: The CPU-hotplug caller serializes frozen-CPU counts and
    // cpuset/domain updates; the native header accessor supplies configured state.
    unsafe {
        if lupos_core_header_cpuhp_tasks_frozen() {
            lupos_core_header_cpuset_reset_sched_domains();
            let count = lupos_core_num_cpus_frozen();
            *count = (*count).wrapping_sub(1);
            if *count != 0 {
                return;
            }
            lupos_core_header_cpuset_force_rebuild();
        }
        lupos_core_header_cpuset_update_active_cpus();
    }
}
unsafe fn cpuset_cpu_inactive(_cpu: c_uint) {
    // SAFETY: The CPU-hotplug caller serializes frozen-CPU counts and
    // cpuset/domain updates; all branch decisions use native configured state.
    unsafe {
        if !lupos_core_header_cpuhp_tasks_frozen() {
            lupos_core_header_cpuset_update_active_cpus();
        } else {
            let count = lupos_core_num_cpus_frozen();
            *count = (*count).wrapping_add(1);
            lupos_core_header_cpuset_reset_sched_domains();
        }
    }
}
unsafe fn sched_smt_present_inc(cpu: c_int) {
    // SAFETY: The hotplug caller holds CPU-hotplug topology serialization;
    // the native cpuslocked static-key operation follows sibling-mask inspection.
    unsafe {
        if lupos_core_cpumask_weight(lupos_core_cpu_smt_mask(cpu)) == 2 {
            lupos_core_sched_smt_present_inc_cpuslocked();
        }
    }
}
unsafe fn sched_smt_present_dec(cpu: c_int) {
    // SAFETY: The hotplug caller holds CPU-hotplug topology serialization;
    // the native cpuslocked key decrement matches the sibling-state transition.
    unsafe {
        if lupos_core_cpumask_weight(lupos_core_cpu_smt_mask(cpu)) == 2 {
            lupos_core_sched_smt_present_dec_cpuslocked();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched_cpu_activate(cpu: c_uint) -> c_int {
    // SAFETY: The native hotplug caller serializes this valid CPU's
    // activation; permanent rq state and topology remain live through callbacks.
    unsafe {
        let rq = lupos_core_cpu_rq(cpu as c_int);
        balance_push_set(cpu as c_int, false);
        sched_smt_present_inc(cpu as c_int);
        lupos_core_set_cpu_active(cpu, true);
        if sched_smp_initialized {
            lupos_core_header_sched_update_numa(cpu as c_int, true);
            lupos_core_header_sched_domains_numa_masks_set(cpu);
            cpuset_cpu_active();
        }
        lupos_core_header_scx_rq_activate(rq);
        sched_set_rq_online(rq, cpu as c_int);
        0
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched_cpu_deactivate(cpu: c_uint) -> c_int {
    // SAFETY: The hotplug caller serializes this valid CPU's deactivation.
    // RCU synchronization follows active-mask removal before domain/class teardown.
    unsafe {
        let rq = lupos_core_cpu_rq(cpu as c_int);
        let ret = dl_bw_deactivate(cpu as c_int);
        if ret != 0 {
            return ret;
        }
        lupos_core_rcu_read_lock();
        lupos_core_header_nohz_balance_exit_idle(rq);
        lupos_core_rcu_read_unlock();
        lupos_core_set_cpu_active(cpu, false);
        balance_push_set(cpu as c_int, true);
        synchronize_rcu();
        sched_domains_free_llc_id(cpu as c_int);
        sched_set_rq_offline(rq, cpu as c_int);
        lupos_core_header_scx_rq_deactivate(rq);
        sched_smt_present_dec(cpu as c_int);
        sched_core_cpu_deactivate(cpu);
        if !sched_smp_initialized {
            return 0;
        }
        lupos_core_header_sched_update_numa(cpu as c_int, false);
        cpuset_cpu_inactive(cpu);
        lupos_core_header_sched_domains_numa_masks_clear(cpu);
        0
    }
}
unsafe fn sched_rq_cpu_starting(cpu: c_uint) {
    // SAFETY: The hotplug/boot caller initializes this valid CPU's
    // permanent runqueue load-accounting epoch before normal scheduling.
    unsafe {
        (*lupos_core_cpu_rq(cpu as c_int)).calc_load_update = calc_load_update;
        update_max_interval();
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched_cpu_starting(cpu: c_uint) -> c_int {
    // SAFETY: The boot/hotplug caller serializes this valid CPU's startup;
    // core leadership, runqueue accounting and tick state are initialized in order.
    unsafe {
        sched_core_cpu_starting(cpu);
        sched_rq_cpu_starting(cpu);
        sched_tick_start(cpu as c_int);
        0
    }
}
#[cfg(CONFIG_HOTPLUG_CPU)]
#[no_mangle]
pub unsafe extern "C" fn sched_cpu_wait_empty(_cpu: c_uint) -> c_int {
    // SAFETY: The outgoing CPU's hotplug thread has parked/unbound other
    // per-CPU tasks and owns the native wait/MM cleanup sequence.
    unsafe {
        balance_hotplug_wait();
        sched_force_init_mm();
        0
    }
}
#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn calc_load_migrate(rq: *mut rq) {
    // SAFETY: The CPU stopper is the last running task on this live rq;
    // its active-load count is stable while the native global atomic is updated.
    unsafe {
        let delta = calc_load_fold_active(rq, 1);
        if delta != 0 {
            lupos_core_atomic_long_add(delta, addr_of_mut!(calc_load_tasks));
        }
    }
}
#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn dump_rq_tasks(rq: *mut rq, loglvl: *const c_char) {
    // SAFETY: The caller holds the live outgoing rq lock and supplies native
    // task-lifetime protection for traversal; loglvl is a live log-prefix string.
    unsafe {
        let cpu = lupos_core_cpu_of(rq);
        lupos_core_assert_rq_held(rq);
        _printk(
            b"%sCPU%d enqueued tasks (%u total):\n\0".as_ptr().cast(),
            loglvl,
            cpu,
            (*rq).nr_running,
        );
        let mut g = lupos_core_next_task(addr_of_mut!(init_task));
        while g != addr_of_mut!(init_task) {
            let mut p = lupos_core_first_thread(g);
            while !p.is_null() {
                if lupos_core_task_cpu(p) == cpu && lupos_core_task_on_rq_queued(p) {
                    _printk(
                        b"%s\tpid: %d, name: %s\n\0".as_ptr().cast(),
                        loglvl,
                        (*p).pid,
                        (*p).comm.as_ptr(),
                    );
                }
                p = lupos_core_next_thread_in_group(g, p);
            }
            g = lupos_core_next_task(g);
        }
    }
}
#[cfg(CONFIG_HOTPLUG_CPU)]
#[no_mangle]
pub unsafe extern "C" fn sched_cpu_dying(cpu: c_uint) -> c_int {
    // SAFETY: The native stopper/hotplug caller serializes this valid
    // CPU's final teardown; rq locking covers class/server state before timer
    // and core-leader cleanup.
    unsafe {
        let rq = lupos_core_cpu_rq(cpu as c_int);
        let mut rf = MaybeUninit::<rq_flags>::uninit();
        let rf = rf.as_mut_ptr();
        sched_tick_stop(cpu as c_int);
        lupos_core_rq_lock_irqsave(rq, rf);
        update_rq_clock(rq);
        if (*rq).nr_running != 1 || rq_has_pinned_tasks(rq) {
            lupos_core_warn_message_site_8878(
                true,
                b"Dying CPU not properly vacated!\0".as_ptr().cast(),
            );
            dump_rq_tasks(rq, b"\x014\0".as_ptr().cast());
        }
        dl_server_stop(addr_of_mut!((*rq).fair_server));
        #[cfg(CONFIG_SCHED_CLASS_EXT)]
        dl_server_stop(addr_of_mut!((*rq).ext_server));
        lupos_core_rq_unlock_irqrestore(rq, rf);
        calc_load_migrate(rq);
        update_max_interval();
        hrtick_clear(rq);
        sched_core_cpu_dying(cpu);
        0
    }
}
