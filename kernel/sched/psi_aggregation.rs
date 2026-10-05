// SPDX-License-Identifier: GPL-2.0
// psi.c:270-774. Native seqcount, workqueue, timer and wait primitives stay in C.

/// # Safety
/// Native group/per-CPU storage is live; the caller holds this aggregator's mutex.
unsafe fn get_recent_times(group: *mut b::psi_group, cpu: c_int, aggregator: usize,
                           times: *mut u32, changed: *mut u32) {
    unsafe {
        let groupc = b::rust_psi_group_cpu(group, cpu);
        let current_cpu = b::rust_psi_raw_cpu();
        let mut tasks = [0u32; COUNTS];
        let (mut now, mut start, mut mask);
        *changed = 0;
        loop {
            let seq = b::rust_psi_read_begin(cpu);
            now = b::rust_psi_cpu_clock(cpu);
            b::rust_psi_copy(times.cast(), addr_of!((*groupc).times).cast(),
                             core::mem::size_of::<[u32; STATES]>());
            mask = b::rust_psi_read_u32(addr_of!((*groupc).state_mask));
            start = b::rust_psi_read_u64(addr_of!((*groupc).state_start));
            if cpu == current_cpu {
                b::rust_psi_copy(tasks.as_mut_ptr().cast(), addr_of!((*groupc).tasks).cast(),
                                 core::mem::size_of::<[u32; COUNTS]>());
            }
            if !b::rust_psi_read_retry(cpu, seq) { break; }
        }
        for s in 0..STATES {
            if mask & (1u32 << s) != 0 {
                *times.add(s) = (*times.add(s)).wrapping_add(now.wrapping_sub(start) as u32);
            }
            let delta = (*times.add(s)).wrapping_sub((*groupc).times_prev[aggregator][s]);
            (*groupc).times_prev[aggregator][s] = *times.add(s);
            *times.add(s) = delta;
            if delta != 0 { *changed |= 1u32 << s; }
        }
        if b::rust_psi_current_work() == addr_of_mut!((*group).avgs_work.work) {
            let reschedule = if cpu == current_cpu {
                tasks[b::RUST_PSI_NR_RUNNING as usize]
                    .wrapping_add(tasks[b::RUST_PSI_NR_IOWAIT as usize])
                    .wrapping_add(tasks[b::RUST_PSI_NR_MEMSTALL as usize]) > 1
            } else {
                *changed & (1u32 << NONIDLE) != 0
            };
            if reschedule { *changed |= b::RUST_PSI_STATE_RESCHEDULE as u32; }
        }
    }
}

/// # Safety
/// The three average cells are writable under the owning group's avgs_lock.
unsafe fn calc_avgs(avg: *mut c_ulong, missed: c_int, time: u64, period: u64) {
    unsafe {
        let exponents = [b::RUST_PSI_EXP_10S, b::RUST_PSI_EXP_60S, b::RUST_PSI_EXP_300S];
        if missed != 0 {
            for (i, exp) in exponents.iter().enumerate() {
                *avg.add(i) = b::rust_psi_calc_load_n(*avg.add(i), *exp as c_ulong, 0, missed as c_uint);
            }
        }
        // div_u64 takes a u32 divisor in the oracle; preserve that conversion.
        let pct = (b::rust_psi_div_u64(time.wrapping_mul(100), period as u32) as c_ulong)
            .wrapping_mul(b::RUST_PSI_FIXED_1 as c_ulong);
        for (i, exp) in exponents.iter().enumerate() {
            *avg.add(i) = b::rust_psi_calc_load(*avg.add(i), *exp as c_ulong, pct);
        }
    }
}

/// # Safety
/// Caller holds the selected aggregator lock; group and per-CPU allocations live.
unsafe fn collect_percpu_times(group: *mut b::psi_group, aggregator: usize, changed: *mut u32) {
    unsafe {
        let mut deltas = [0u64; STATES - 1];
        let mut nonidle_total: c_ulong = 0;
        let mut changed_states = 0;
        each_possible_cpu(|cpu| {
            let mut times = [0u32; STATES];
            let mut cpu_changed = 0;
            get_recent_times(group, cpu, aggregator, times.as_mut_ptr(), &mut cpu_changed);
            changed_states |= cpu_changed;
            let nonidle = b::rust_psi_nsecs_to_jiffies(times[NONIDLE] as u64) as u32;
            nonidle_total = nonidle_total.wrapping_add(nonidle as c_ulong);
            for s in 0..NONIDLE {
                deltas[s] = deltas[s].wrapping_add((times[s] as u64).wrapping_mul(nonidle as u64));
            }
        });
        for s in 0..STATES - 1 {
            (*group).total[aggregator][s] = (*group).total[aggregator][s].wrapping_add(
                b::rust_psi_div_u64(deltas[s], max(nonidle_total, 1) as u32));
        }
        if !changed.is_null() { *changed = changed_states; }
    }
}

/// # Safety
/// Caller exclusively owns this window or holds its aggregator lock.
unsafe fn window_reset(win: *mut b::psi_window, now: u64, value: u64, previous: u64) {
    unsafe {
        (*win).start_time = now;
        (*win).start_value = value;
        (*win).prev_growth = previous;
    }
}

/// # Safety
/// Window is initialized with a nonzero size and stabilized by its aggregator lock.
unsafe fn window_update(win: *mut b::psi_window, now: u64, value: u64) -> u64 {
    unsafe {
        let elapsed = now.wrapping_sub((*win).start_time);
        let mut growth = value.wrapping_sub((*win).start_value);
        if elapsed > (*win).size {
            window_reset(win, now, value, growth);
        } else {
            // C deliberately narrows remaining to u32 (maximum window is 10s).
            let remaining = (*win).size.wrapping_sub(elapsed) as u32;
            growth = growth.wrapping_add(b::rust_psi_div64_u64(
                (*win).prev_growth.wrapping_mul(remaining as u64), (*win).size));
        }
        growth
    }
}

/// # Safety
/// Caller holds the selected aggregator lock and stabilizes all trigger lifetimes.
unsafe fn update_triggers(group: *mut b::psi_group, now: u64, aggregator: usize) {
    unsafe {
        let (head, previous) = if aggregator == AVGS {
            (addr_of_mut!((*group).avg_triggers), addr_of!((*group).avg_total).cast::<u64>())
        } else {
            (addr_of_mut!((*group).rtpoll_triggers), addr_of!((*group).rtpoll_total).cast::<u64>())
        };
        each_trigger(head, |trigger| {
            let state = (*trigger).state as usize;
            let total = (*group).total[aggregator][state];
            let new_stall = *previous.add(state) != total;
            if !new_stall && !(*trigger).pending_event { return; }
            if new_stall {
                let growth = window_update(addr_of_mut!((*trigger).win), now, total);
                if !(*trigger).pending_event {
                    if growth < (*trigger).threshold { return; }
                    (*trigger).pending_event = true;
                }
            }
            if now < (*trigger).last_event_time.wrapping_add((*trigger).win.size) { return; }
            if b::rust_psi_cmpxchg_event(trigger, 0, 1) == 0 { notify_trigger(trigger); }
            (*trigger).last_event_time = now;
            (*trigger).pending_event = false;
        });
    }
}

/// # Safety
/// Caller holds avgs_lock, and now is at or beyond the next update deadline.
unsafe fn update_averages(group: *mut b::psi_group, now: u64) -> u64 {
    unsafe {
        let expires = (*group).avg_next_update;
        let psi_period = b::rust_psi_period;
        let mut missed: c_ulong = 0;
        if now.wrapping_sub(expires) >= psi_period {
            missed = b::rust_psi_div_u64(now.wrapping_sub(expires), psi_period as u32) as c_ulong;
        }
        let next = expires.wrapping_add((missed.wrapping_add(1) as u64).wrapping_mul(psi_period));
        let period = now.wrapping_sub((*group).avg_last_update.wrapping_add((missed as u64).wrapping_mul(psi_period)));
        (*group).avg_last_update = now;
        for s in 0..STATES - 1 {
            let mut sample = (*group).total[AVGS][s].wrapping_sub((*group).avg_total[s]) as u32;
            if sample as u64 > period { sample = period as u32; }
            (*group).avg_total[s] = (*group).avg_total[s].wrapping_add(sample as u64);
            calc_avgs(addr_of_mut!((*group).avg[s]).cast(), missed as c_int, sample as u64, period);
        }
        next
    }
}

/// # Safety
/// Native workqueue calls with a live psi_group.avgs_work.work embedding.
#[no_mangle]
pub unsafe extern "C" fn rust_psi_avgs_work(work: *mut b::work_struct) {
    unsafe {
        let dwork = container_of!(work, b::delayed_work, work);
        let group = container_of!(dwork, b::psi_group, avgs_work);
        b::rust_psi_mutex_lock(addr_of_mut!((*group).avgs_lock));
        let now = b::rust_psi_sched_clock();
        let mut changed = 0;
        collect_percpu_times(group, AVGS, &mut changed);
        if now >= (*group).avg_next_update {
            update_triggers(group, now, AVGS);
            (*group).avg_next_update = update_averages(group, now);
        }
        if changed & b::RUST_PSI_STATE_RESCHEDULE as u32 != 0 {
            b::rust_psi_schedule_delayed(dwork, b::rust_psi_nsecs_to_jiffies(
                (*group).avg_next_update.wrapping_sub(now)).wrapping_add(1));
        }
        b::rust_psi_mutex_unlock(addr_of_mut!((*group).avgs_lock));
    }
}

/// # Safety
/// Caller holds rtpoll_trigger_lock and stabilizes all triggers.
unsafe fn init_rtpoll_triggers(group: *mut b::psi_group, now: u64) {
    unsafe {
        each_trigger(addr_of_mut!((*group).rtpoll_triggers), |trigger| {
            window_reset(addr_of_mut!((*trigger).win), now, (*group).total[POLL][(*trigger).state as usize], 0);
        });
        for s in 0..STATES - 1 { (*group).rtpoll_total[s] = (*group).total[POLL][s]; }
        (*group).rtpoll_next_update = now.wrapping_add((*group).rtpoll_min_period);
    }
}

/// # Safety
/// Live group; native RCU protects worker publication and timer lifetime.
unsafe fn psi_schedule_rtpoll_work(group: *mut b::psi_group, delay: c_ulong, force: bool) {
    unsafe {
        // This exchange must execute even if force is false: it supplies the MB.
        if b::rust_psi_atomic_xchg(addr_of_mut!((*group).rtpoll_scheduled), 1) != 0 && !force { return; }
        b::rust_psi_rcu_read_lock();
        if !b::rust_psi_dereference_rtpoll_task(group).is_null() {
            b::rust_psi_mod_timer(addr_of_mut!((*group).rtpoll_timer), b::rust_psi_jiffies().wrapping_add(delay));
        } else {
            b::rust_psi_atomic_set(addr_of_mut!((*group).rtpoll_scheduled), 0);
        }
        b::rust_psi_rcu_read_unlock();
    }
}

/// # Safety
/// Called by this group's live worker; group remains allocated until worker stops.
unsafe fn psi_rtpoll_work(group: *mut b::psi_group) {
    unsafe {
        b::rust_psi_mutex_lock(addr_of_mut!((*group).rtpoll_trigger_lock));
        let now = b::rust_psi_sched_clock();
        let force = now <= (*group).rtpoll_until;
        if !force {
            b::rust_psi_atomic_set(addr_of_mut!((*group).rtpoll_scheduled), 0);
            b::rust_psi_smp_mb();
        }
        let mut changed = 0;
        collect_percpu_times(group, POLL, &mut changed);
        if changed & (*group).rtpoll_states != 0 {
            if now > (*group).rtpoll_until { init_rtpoll_triggers(group, now); }
            (*group).rtpoll_until = now.wrapping_add((*group).rtpoll_min_period.wrapping_mul(UPDATES_PER_WINDOW as u64));
        }
        if now > (*group).rtpoll_until {
            (*group).rtpoll_next_update = b::RUST_PSI_U64_MAX as u64;
        } else {
            if now >= (*group).rtpoll_next_update {
                if changed & (*group).rtpoll_states != 0 {
                    update_triggers(group, now, POLL);
                    for s in 0..STATES - 1 { (*group).rtpoll_total[s] = (*group).total[POLL][s]; }
                }
                (*group).rtpoll_next_update = now.wrapping_add((*group).rtpoll_min_period);
            }
            psi_schedule_rtpoll_work(group, b::rust_psi_nsecs_to_jiffies(
                (*group).rtpoll_next_update.wrapping_sub(now)).wrapping_add(1), force);
        }
        b::rust_psi_mutex_unlock(addr_of_mut!((*group).rtpoll_trigger_lock));
    }
}

/// # Safety
/// Native kthread data points to a group that survives until kthread_stop returns.
#[no_mangle]
pub unsafe extern "C" fn rust_psi_rtpoll_worker(data: *mut c_void) -> c_int {
    unsafe {
        let group = data.cast::<b::psi_group>();
        b::rust_psi_sched_set_fifo_low(b::rust_psi_current());
        loop {
            b::rust_psi_wait_rtpoll(group);
            if b::rust_psi_kthread_should_stop() { break; }
            psi_rtpoll_work(group);
        }
        0
    }
}

/// # Safety
/// Native timer callback holds a live psi_group.rtpoll_timer embedding.
#[no_mangle]
pub unsafe extern "C" fn rust_psi_poll_timer(timer: *mut b::timer_list) {
    unsafe {
        let group = container_of!(timer, b::psi_group, rtpoll_timer);
        b::rust_psi_atomic_set(addr_of_mut!((*group).rtpoll_wakeup), 1);
        b::rust_psi_wake_interruptible(addr_of_mut!((*group).rtpoll_wait));
    }
}
