// SPDX-License-Identifier: GPL-2.0
// psi.c:1259-1559: reporting, trigger lifetime, worker publication and polling.

/// # Safety
/// Trigger is live under its aggregator lock or exclusive teardown ownership.
unsafe fn notify_trigger(trigger: *mut b::psi_trigger) {
    unsafe {
        if !(*trigger).of.is_null() { b::rust_psi_kernfs_notify((*trigger).of); }
        else { b::rust_psi_wake_interruptible(addr_of_mut!((*trigger).event_wait)); }
    }
}

/// # Safety
/// seq_file and group are live; res is a configured native PSI resource.
#[no_mangle]
pub unsafe extern "C" fn psi_show(m: *mut b::seq_file, group: *mut b::psi_group, res: b::psi_res) -> c_int {
    unsafe {
        if b::rust_psi_disabled() { return -(b::RUST_PSI_EOPNOTSUPP as c_int); }
        #[cfg(CONFIG_IRQ_TIME_ACCOUNTING)]
        if !b::rust_psi_irqtime_enabled() && res == b::RUST_PSI_IRQ as b::psi_res {
            return -(b::RUST_PSI_EOPNOTSUPP as c_int);
        }
        b::rust_psi_mutex_lock(addr_of_mut!((*group).avgs_lock));
        let now = b::rust_psi_sched_clock();
        collect_percpu_times(group, AVGS, null_mut());
        if now >= (*group).avg_next_update { (*group).avg_next_update = update_averages(group, now); }
        b::rust_psi_mutex_unlock(addr_of_mut!((*group).avgs_lock));
        #[cfg(CONFIG_IRQ_TIME_ACCOUNTING)]
        let only_full = res == b::RUST_PSI_IRQ as b::psi_res;
        #[cfg(not(CONFIG_IRQ_TIME_ACCOUNTING))]
        let only_full = false;
        for full in 0..2 - only_full as usize {
            let mut avg = [0 as c_ulong; 3];
            let mut total = 0;
            if !(group == addr_of_mut!(b::psi_system) && res == b::RUST_PSI_CPU as b::psi_res && full != 0) {
                let state = res as usize * 2 + full;
                for w in 0..3 { avg[w] = (*group).avg[state][w]; }
                total = b::rust_psi_div_u64((*group).total[AVGS][state], b::RUST_PSI_NSEC_PER_USEC as u32);
            }
            b::rust_psi_seq_report(m, full != 0 || only_full, avg.as_ptr(), total);
        }
        0
    }
}

/// # Safety
/// Group stays live. Caller has dropped cgroup_mutex and other fork-dependent locks.
#[no_mangle]
pub unsafe extern "C" fn psi_trigger_create_rtpoll_worker(group: *mut b::psi_group) -> c_int {
    unsafe {
        let task = b::rust_psi_create_worker(group);
        if b::rust_psi_is_err(task.cast()) { return b::rust_psi_ptr_err(task.cast()) as c_int; }
        b::rust_psi_mutex_lock(addr_of_mut!((*group).rtpoll_trigger_lock));
        if b::rust_psi_access_rtpoll_task(group).is_null() {
            b::rust_psi_atomic_set(addr_of_mut!((*group).rtpoll_wakeup), 0);
            b::rust_psi_wake_process(task);
            b::rust_psi_assign_rtpoll_task(group, task);
            psi_schedule_rtpoll_work(group, 1, true);
            b::rust_psi_mutex_unlock(addr_of_mut!((*group).rtpoll_trigger_lock));
            return 0;
        }
        b::rust_psi_mutex_unlock(addr_of_mut!((*group).rtpoll_trigger_lock));
        b::rust_psi_kthread_stop(task);
        0
    }
}

/// # Safety
/// Caller owns writable result storage and a NUL-terminated kernel buffer; file,
/// optional kernfs open file and group remain live through trigger destruction.
#[no_mangle]
pub unsafe extern "C" fn psi_trigger_create(group: *mut b::psi_group, buf: *mut c_char,
    res: b::psi_res, file: *mut b::file, of: *mut b::kernfs_open_file,
    need_worker: *mut bool) -> *mut b::psi_trigger {
    unsafe {
        *need_worker = false;
        if b::rust_psi_disabled() { return b::rust_psi_err_ptr(-(b::RUST_PSI_EOPNOTSUPP as kernel::ffi::c_long)).cast(); }
        let privileged = b::rust_psi_file_privileged(file);
        let mut threshold_us = 0;
        let mut window_us = 0;
        let state = if b::rust_psi_scan_some(buf, &mut threshold_us, &mut window_us) == 2 {
            b::RUST_PSI_IO_SOME as b::psi_states + res * 2
        } else if b::rust_psi_scan_full(buf, &mut threshold_us, &mut window_us) == 2 {
            b::RUST_PSI_IO_FULL as b::psi_states + res * 2
        } else {
            return b::rust_psi_err_ptr(-(b::RUST_PSI_EINVAL as kernel::ffi::c_long)).cast();
        };
        #[cfg(CONFIG_IRQ_TIME_ACCOUNTING)]
        let state = if res == b::RUST_PSI_IRQ as b::psi_res {
            let adjusted = state.wrapping_sub(1);
            if adjusted != b::RUST_PSI_IRQ_FULL as b::psi_states {
                return b::rust_psi_err_ptr(-(b::RUST_PSI_EINVAL as kernel::ffi::c_long)).cast();
            }
            adjusted
        } else { state };
        if state as usize >= NONIDLE || window_us == 0 || window_us > b::RUST_PSI_WINDOW_MAX_US as u32 ||
           (!privileged && window_us % 2_000_000 != 0) || threshold_us == 0 || threshold_us > window_us {
            return b::rust_psi_err_ptr(-(b::RUST_PSI_EINVAL as kernel::ffi::c_long)).cast();
        }
        let trigger = b::rust_psi_alloc_trigger();
        if trigger.is_null() { return b::rust_psi_err_ptr(-(b::RUST_PSI_ENOMEM as kernel::ffi::c_long)).cast(); }
        (*trigger).group = group;
        (*trigger).state = state;
        // Match the native unsigned-long arithmetic width of NSEC_PER_USEC.
        (*trigger).threshold = (threshold_us as c_ulong).wrapping_mul(b::RUST_PSI_NSEC_PER_USEC as c_ulong) as u64;
        (*trigger).win.size = (window_us as c_ulong).wrapping_mul(b::RUST_PSI_NSEC_PER_USEC as c_ulong) as u64;
        window_reset(addr_of_mut!((*trigger).win), b::rust_psi_sched_clock(), (*group).total[POLL][state as usize], 0);
        (*trigger).event = 0;
        (*trigger).last_event_time = 0;
        (*trigger).of = of;
        if of.is_null() { b::rust_psi_init_trigger_wait(trigger); }
        (*trigger).pending_event = false;
        (*trigger).aggregator = (if privileged { POLL } else { AVGS }) as b::psi_aggregators;
        if privileged {
            b::rust_psi_mutex_lock(addr_of_mut!((*group).rtpoll_trigger_lock));
            b::rust_psi_list_add(addr_of_mut!((*trigger).node), addr_of_mut!((*group).rtpoll_triggers));
            (*group).rtpoll_min_period = min((*group).rtpoll_min_period,
                b::rust_psi_div_u64((*trigger).win.size, UPDATES_PER_WINDOW));
            (*group).rtpoll_nr_triggers[state as usize] = (*group).rtpoll_nr_triggers[state as usize].wrapping_add(1);
            (*group).rtpoll_states |= 1u32 << state;
            *need_worker = b::rust_psi_access_rtpoll_task(group).is_null();
            b::rust_psi_mutex_unlock(addr_of_mut!((*group).rtpoll_trigger_lock));
        } else {
            b::rust_psi_mutex_lock(addr_of_mut!((*group).avgs_lock));
            b::rust_psi_list_add(addr_of_mut!((*trigger).node), addr_of_mut!((*group).avg_triggers));
            (*group).avg_nr_triggers[state as usize] = (*group).avg_nr_triggers[state as usize].wrapping_add(1);
            b::rust_psi_mutex_unlock(addr_of_mut!((*group).avgs_lock));
        }
        trigger
    }
}

/// # Safety
/// Caller exclusively destroys a live trigger (or NULL) after preventing new users.
/// The group remains alive across its RCU grace period and worker teardown.
#[no_mangle]
pub unsafe extern "C" fn psi_trigger_destroy(trigger: *mut b::psi_trigger) {
    unsafe {
        if trigger.is_null() { return; }
        let group = (*trigger).group;
        let state = (*trigger).state as usize;
        let mut task_to_destroy = null_mut();
        notify_trigger(trigger);
        if (*trigger).aggregator as usize == AVGS {
            b::rust_psi_mutex_lock(addr_of_mut!((*group).avgs_lock));
            if !b::rust_psi_list_empty(addr_of!((*trigger).node)) {
                b::rust_psi_list_del(addr_of_mut!((*trigger).node));
                (*group).avg_nr_triggers[state] = (*group).avg_nr_triggers[state].wrapping_sub(1);
            }
            b::rust_psi_mutex_unlock(addr_of_mut!((*group).avgs_lock));
        } else {
            b::rust_psi_mutex_lock(addr_of_mut!((*group).rtpoll_trigger_lock));
            if !b::rust_psi_list_empty(addr_of!((*trigger).node)) {
                b::rust_psi_list_del(addr_of_mut!((*trigger).node));
                (*group).rtpoll_nr_triggers[state] = (*group).rtpoll_nr_triggers[state].wrapping_sub(1);
                if (*group).rtpoll_nr_triggers[state] == 0 { (*group).rtpoll_states &= !(1u32 << state); }
                if (*group).rtpoll_min_period == b::rust_psi_div_u64((*trigger).win.size, UPDATES_PER_WINDOW) {
                    let mut period = b::RUST_PSI_U64_MAX as u64;
                    each_trigger(addr_of_mut!((*group).rtpoll_triggers), |other| {
                        period = min(period, b::rust_psi_div_u64((*other).win.size, UPDATES_PER_WINDOW));
                    });
                    (*group).rtpoll_min_period = period;
                }
                if (*group).rtpoll_states == 0 {
                    (*group).rtpoll_until = 0;
                    task_to_destroy = b::rust_psi_protected_rtpoll_task(group);
                    b::rust_psi_assign_rtpoll_task(group, null_mut());
                    b::rust_psi_timer_delete(addr_of_mut!((*group).rtpoll_timer));
                }
            }
            b::rust_psi_mutex_unlock(addr_of_mut!((*group).rtpoll_trigger_lock));
        }
        b::rust_psi_synchronize_rcu();
        if !task_to_destroy.is_null() {
            b::rust_psi_kthread_stop(task_to_destroy);
            b::rust_psi_atomic_set(addr_of_mut!((*group).rtpoll_scheduled), 0);
        }
        b::rust_psi_free(trigger.cast());
    }
}

/// # Safety
/// Native file lifetime protects the trigger slot and its acquired trigger; wait
/// points to the caller's poll table. Slot publication uses store-release.
#[no_mangle]
pub unsafe extern "C" fn psi_trigger_poll(trigger_ptr: *mut *mut c_void,
    file: *mut b::file, wait: *mut b::poll_table) -> b::__poll_t {
    unsafe {
        let mut ret = b::RUST_PSI_DEFAULT_POLLMASK as b::__poll_t;
        let error = ret | b::RUST_PSI_EPOLLERR as b::__poll_t | b::RUST_PSI_EPOLLPRI as b::__poll_t;
        if b::rust_psi_disabled() { return error; }
        let trigger = b::rust_psi_load_trigger(trigger_ptr);
        if trigger.is_null() { return error; }
        if !(*trigger).of.is_null() { b::rust_psi_kernfs_poll((*trigger).of, wait); }
        else { b::rust_psi_poll_wait(file, addr_of_mut!((*trigger).event_wait), wait); }
        if b::rust_psi_cmpxchg_event(trigger, 1, 0) == 1 { ret |= b::RUST_PSI_EPOLLPRI as b::__poll_t; }
        ret
    }
}
