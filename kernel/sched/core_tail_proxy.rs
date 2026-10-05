// SPDX-License-Identifier: GPL-2.0-only
// core.c:6746..7067. Native helpers only expose bitfields, mutex/header primitives.
unsafe fn try_to_block_task(
    rq: *mut rq,
    p: *mut task_struct,
    state_p: *mut c_ulong,
    should_block: bool,
) -> bool {
    // SAFETY: The caller holds the live rq's lock, keeps p live until
    // blocking releases its runqueue ownership, and supplies writable state storage.
    unsafe {
        let state = *state_p;
        lupos_core_warn_once_site_6754(lupos_core_task_is_blocked_flag(p));
        if lupos_core_signal_pending_state(state, p) {
            lupos_core_write_once_uint(addr_of_mut!((*p).__state), LUPOS_CORE_TASK_RUNNING);
            *state_p = LUPOS_CORE_TASK_RUNNING as c_ulong;
            lupos_core_clear_task_blocked_on(p);
            return false;
        }
        lupos_core_set_task_is_blocked(p, true);
        if !should_block {
            return false;
        }
        block_task(rq, p, state);
        true
    }
}
#[cfg(CONFIG_SCHED_PROXY_EXEC)]
unsafe fn proxy_set_task_cpu(p: *mut task_struct, cpu: c_int) {
    // SAFETY: The caller owns p's proxy migration under rq synchronization;
    // native CPU publication must preserve the saved wake CPU for later return.
    unsafe {
        let wake_cpu = (*p).wake_cpu;
        lupos_core___set_task_cpu(p, cpu as c_uint);
        (*p).wake_cpu = wake_cpu;
    }
}
#[cfg(CONFIG_SCHED_PROXY_EXEC)]
unsafe fn proxy_resched_idle(rq: *mut rq) -> *mut task_struct {
    // SAFETY: The caller holds rq's lock and keeps current donor/idle live
    // while class ownership is transferred to the permanent idle task.
    unsafe {
        let idle = (*rq).idle;
        lupos_core_put_prev_set_next_task(rq, lupos_core_rq_donor(rq), idle);
        (*rq).next_class = addr_of!(idle_sched_class);
        lupos_core_rq_set_donor(rq, idle);
        lupos_core_set_tsk_need_resched(idle);
        idle
    }
}
#[cfg(CONFIG_SCHED_PROXY_EXEC)]
unsafe fn proxy_deactivate(rq: *mut rq, donor: *mut task_struct) {
    // SAFETY: The caller holds rq's lock and keeps donor live through
    // reference detachment; no donor access is permitted after block_task.
    unsafe {
        let state = lupos_core_read_once_uint(addr_of!((*donor).__state)) as c_ulong;
        lupos_core_warn_once_site_6806(state == LUPOS_CORE_TASK_RUNNING as c_ulong);
        lupos_core_warn_once_site_6807(!(*donor).blocked_on.is_null());
        // Drop every rq donor reference BEFORE on_rq becomes zero and remote ttwu
        // can move/deallocate donor. Nothing below block_task may dereference donor.
        proxy_resched_idle(rq);
        block_task(rq, donor, state);
    }
}
#[cfg(CONFIG_SCHED_PROXY_EXEC)]
unsafe fn proxy_release_rq_lock(rq: *mut rq, rf: *mut rq_flags) {
    // SAFETY: The caller owns the live rq lock with matching pin flags;
    // callback detachment precedes unpinning and native unlock.
    unsafe {
        zap_balance_callbacks(rq);
        lupos_core_rq_unpin_lock(rq, rf);
        lupos_core_raw_spin_rq_unlock(rq);
    }
}
#[cfg(CONFIG_SCHED_PROXY_EXEC)]
unsafe fn proxy_reacquire_rq_lock(rq: *mut rq, rf: *mut rq_flags) {
    // SAFETY: The scheduler caller keeps rq/flags live and IRQs disabled;
    // native reacquisition restores the pin before refreshing the clock.
    unsafe {
        raw_spin_rq_lock_nested(rq, 0);
        lupos_core_rq_repin_lock(rq, rf);
        update_rq_clock(rq);
    }
}
#[cfg(CONFIG_SCHED_PROXY_EXEC)]
unsafe fn proxy_migrate_task(
    rq: *mut rq,
    rf: *mut rq_flags,
    p: *mut task_struct,
    target_cpu: c_int,
) {
    // SAFETY: The caller owns rq and p's blocked migration transition.
    // Donor references are detached before releasing rq; native destination
    // attachment completes before reacquiring the original rq and pin.
    unsafe {
        let target_rq = lupos_core_cpu_rq(target_cpu);
        lupos_core_assert_rq_held(rq);
        lupos_core_warn_site_6869(p == lupos_core_rq_curr(rq));
        proxy_resched_idle(rq);
        deactivate_task(rq, p, LUPOS_CORE_DEQUEUE_NOCLOCK as c_int);
        proxy_set_task_cpu(p, target_cpu);
        proxy_release_rq_lock(rq, rf);
        lupos_core_header_attach_one_task(target_rq, p);
        proxy_reacquire_rq_lock(rq, rf);
    }
}
#[cfg(CONFIG_SCHED_PROXY_EXEC)]
unsafe fn find_proxy_task(
    rq: *mut rq,
    donor: *mut task_struct,
    rf: *mut rq_flags,
) -> *mut task_struct {
    // SAFETY: The caller owns rq's lock and a live blocked donor. Nested
    // mutex wait/blocked locks stabilize each owner until rq membership is checked;
    // actions leave both inner locks before deactivation or migration.
    unsafe {
        // The action enum is Rust control flow only, never an ABI/layout mirror.
        enum Action {
            Continue,
            Return(*mut task_struct),
            Deactivate,
            Migrate(c_int),
        }
        let mut owner: *mut task_struct = null_mut();
        let mut curr_in_chain = false;
        let this_cpu = lupos_core_cpu_of(rq);
        let mut p = donor;
        while lupos_core_task_is_blocked_flag(p) {
            let mutex = (*p).blocked_on;
            if mutex.is_null() {
                lupos_core_clear_task_blocked_on(p);
                if lupos_core_task_current(rq, p) {
                    lupos_core_set_task_is_blocked(p, false);
                    return p;
                }
                proxy_deactivate(rq, p);
                return null_mut();
            }
            lupos_core_raw_spin_lock(addr_of_mut!((*mutex).wait_lock));
            lupos_core_raw_spin_lock(addr_of_mut!((*p).blocked_lock));
            let action = (|| {
                if mutex != lupos_core_get_task_blocked_on_locked(p) {
                    return Action::Return(null_mut());
                }
                if lupos_core_task_current(rq, p) {
                    curr_in_chain = true;
                }
                owner = lupos_core_mutex_owner(mutex);
                if owner.is_null() {
                    lupos_core_clear_task_blocked_on_locked(p);
                    if lupos_core_task_current(rq, p) {
                        lupos_core_set_task_is_blocked(p, false);
                        return Action::Return(p);
                    }
                    return Action::Deactivate;
                }
                if lupos_core_read_once_u8(addr_of!((*owner).on_rq)) == 0
                    || lupos_core_se_sched_delayed(addr_of!((*owner).se))
                {
                    if curr_in_chain {
                        return Action::Return(proxy_resched_idle(rq));
                    }
                    lupos_core_clear_task_blocked_on_locked(p);
                    return Action::Deactivate;
                }
                let owner_cpu = lupos_core_task_cpu(owner);
                if owner_cpu != this_cpu {
                    if curr_in_chain {
                        return Action::Return(proxy_resched_idle(rq));
                    }
                    return Action::Migrate(owner_cpu);
                }
                if lupos_core_task_on_rq_migrating(owner) {
                    return Action::Return(proxy_resched_idle(rq));
                }
                if !lupos_core_task_on_rq_queued(owner) || lupos_core_task_cpu(owner) != this_cpu {
                    return Action::Return(null_mut());
                }
                if owner == p {
                    return Action::Return(proxy_resched_idle(rq));
                }
                (*owner).blocked_donor = p;
                Action::Continue
            })();
            // C cleanup guards release these on return/goto/iteration, inner first.
            lupos_core_raw_spin_unlock(addr_of_mut!((*p).blocked_lock));
            lupos_core_raw_spin_unlock(addr_of_mut!((*mutex).wait_lock));
            match action {
                Action::Continue => p = owner,
                Action::Return(result) => return result,
                Action::Deactivate => {
                    proxy_deactivate(rq, p);
                    return null_mut();
                }
                Action::Migrate(cpu) => {
                    proxy_migrate_task(rq, rf, p, cpu);
                    return null_mut();
                }
            }
        }
        lupos_core_warn_once_site_7051(!owner.is_null() && (*owner).on_rq == 0);
        owner
    }
}
#[cfg(not(CONFIG_SCHED_PROXY_EXEC))]
unsafe fn find_proxy_task(
    _rq: *mut rq,
    donor: *mut task_struct,
    _rf: *mut rq_flags,
) -> *mut task_struct {
    // SAFETY: The native disabled-config path emits its original diagnostic;
    // donor is returned unchanged and is not dereferenced here.
    unsafe {
        lupos_core_warn_once_message_site_7064(
            true,
            b"This should never be called in the !SCHED_PROXY_EXEC case\n\0"
                .as_ptr()
                .cast(),
        );
        donor
    }
}
