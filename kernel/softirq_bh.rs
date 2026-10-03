// SPDX-License-Identifier: GPL-2.0-only
// Mutually exclusive RT/non-RT bottom-half ownership, from softirq.c.

#[cfg(CONFIG_PREEMPT_RT)]
#[no_mangle]
pub unsafe extern "C" fn local_bh_blocked() -> bool {
    lupos_sirq_ctrl_read_raw() != 0
}

#[cfg(CONFIG_PREEMPT_RT)]
#[no_mangle]
pub unsafe extern "C" fn __local_bh_disable_ip(_ip: c_ulong, cnt: c_uint) {
    lupos_sirq_warn_hardirq(in_hardirq(), 0);
    lupos_sirq_bh_acquire(instruction_ip());
    let task = current_task();
    if (*task).softirq_disable_cnt == 0 {
        if lupos_sirq_preemptible() {
            // local_lock's migration-before-address-before-lock algorithm
            // is expanded here rather than hidden in a C local_lock wrapper.
            lupos_sirq_migrate_disable();
            #[cfg(CONFIG_PREEMPT_RT_NEEDS_BH_LOCK)]
            lupos_sirq_spin_lock(lupos_sirq_ctrl_lock());
            lupos_sirq_rcu_lock();
        } else {
            lupos_sirq_debug_warn(lupos_sirq_ctrl_read() != 0);
        }
    }
    #[cfg(CONFIG_PREEMPT_RT_NEEDS_BH_LOCK)]
    {
        let newcnt = lupos_sirq_ctrl_add_return(cnt);
        (*task).softirq_disable_cnt = newcnt;
        #[cfg(CONFIG_TRACE_IRQFLAGS)]
        if newcnt as c_uint == cnt {
            let flags = lupos_sirq_raw_irq_save();
            lupos_sirq_softirqs_off(_ip);
            lupos_sirq_raw_irq_restore(flags);
        }
    }
    #[cfg(not(CONFIG_PREEMPT_RT_NEEDS_BH_LOCK))]
    {
        #[cfg(CONFIG_TRACE_IRQFLAGS)]
        let sirq_dis = (*task).softirq_disable_cnt == 0;
        lupos_sirq_ctrl_add(cnt);
        (*task).softirq_disable_cnt = (*task).softirq_disable_cnt.wrapping_add(cnt as c_int);
        lupos_sirq_warn_count_negative((*task).softirq_disable_cnt < 0, 0);
        #[cfg(CONFIG_TRACE_IRQFLAGS)]
        if sirq_dis {
            let flags = lupos_sirq_raw_irq_save();
            lupos_sirq_softirqs_off(_ip);
            lupos_sirq_raw_irq_restore(flags);
        }
    }
}

#[cfg(CONFIG_PREEMPT_RT)]
unsafe fn local_bh_enable_internal(cnt: c_uint, unlock: bool) {
    let task = current_task();
    #[cfg(CONFIG_PREEMPT_RT_NEEDS_BH_LOCK)]
    lupos_sirq_debug_warn((*task).softirq_disable_cnt != lupos_sirq_ctrl_read());
    #[cfg(all(CONFIG_TRACE_IRQFLAGS, CONFIG_PREEMPT_RT_NEEDS_BH_LOCK))]
    let sirq_en = softirq_count() == cnt;
    #[cfg(all(CONFIG_TRACE_IRQFLAGS, not(CONFIG_PREEMPT_RT_NEEDS_BH_LOCK)))]
    let sirq_en = (*task).softirq_disable_cnt as c_uint == cnt;
    #[cfg(CONFIG_TRACE_IRQFLAGS)]
    if sirq_en {
        let flags = lupos_sirq_raw_irq_save();
        lupos_sirq_softirqs_on(caller_ip());
        lupos_sirq_raw_irq_restore(flags);
    }
    #[cfg(CONFIG_PREEMPT_RT_NEEDS_BH_LOCK)]
    {
        let newcnt = lupos_sirq_ctrl_sub_return(cnt);
        (*task).softirq_disable_cnt = newcnt;
        if newcnt == 0 && unlock {
            lupos_sirq_rcu_unlock();
            lupos_sirq_spin_unlock(lupos_sirq_ctrl_lock());
            lupos_sirq_migrate_enable();
        }
    }
    #[cfg(not(CONFIG_PREEMPT_RT_NEEDS_BH_LOCK))]
    {
        (*task).softirq_disable_cnt = (*task).softirq_disable_cnt.wrapping_sub(cnt as c_int);
        lupos_sirq_ctrl_sub(cnt);
        if unlock && (*task).softirq_disable_cnt == 0 {
            // Preserve the original RT-without-BH-lock ordering.
            lupos_sirq_migrate_enable();
            lupos_sirq_rcu_unlock();
        } else {
            lupos_sirq_warn_count_negative((*task).softirq_disable_cnt < 0, 1);
        }
    }
}

#[cfg(CONFIG_PREEMPT_RT)]
#[no_mangle]
pub unsafe extern "C" fn __local_bh_enable_ip(_ip: c_ulong, mut cnt: c_uint) {
    let preempt_on = lupos_sirq_preemptible();
    lupos_sirq_warn_hardirq(in_hardirq(), 1);
    lupos_sirq_assert_irqs_enabled();
    lupos_sirq_bh_release(instruction_ip());
    let flags = lupos_sirq_irq_save();
    #[cfg(CONFIG_PREEMPT_RT_NEEDS_BH_LOCK)]
    let curcnt = lupos_sirq_ctrl_read();
    #[cfg(not(CONFIG_PREEMPT_RT_NEEDS_BH_LOCK))]
    let curcnt = (*current_task()).softirq_disable_cnt;
    if curcnt as c_uint == cnt && lupos_sirq_pending() != 0 {
        if !preempt_on {
            wakeup_softirqd();
        } else {
            cnt = RUST_SIRQ_SOFTIRQ_OFFSET;
            local_bh_enable_internal(cnt, false);
            __do_softirq();
        }
    }
    local_bh_enable_internal(cnt, preempt_on);
    lupos_sirq_irq_restore(flags);
}

#[cfg(CONFIG_PREEMPT_RT)]
unsafe fn ksoftirqd_run_begin() {
    __local_bh_disable_ip(caller_ip(), RUST_SIRQ_SOFTIRQ_OFFSET);
    lupos_sirq_irq_disable();
}
#[cfg(CONFIG_PREEMPT_RT)]
unsafe fn ksoftirqd_run_end() {
    lupos_sirq_bh_release(instruction_ip());
    local_bh_enable_internal(RUST_SIRQ_SOFTIRQ_OFFSET, true);
    lupos_sirq_warn_interrupt(in_interrupt());
    lupos_sirq_irq_enable();
}
#[cfg(CONFIG_PREEMPT_RT)]
unsafe fn softirq_handle_begin() {}
#[cfg(CONFIG_PREEMPT_RT)]
unsafe fn softirq_handle_end() {}
#[cfg(CONFIG_PREEMPT_RT)]
unsafe fn should_wake_ksoftirqd() -> bool {
    lupos_sirq_ctrl_read() == 0
}
#[cfg(CONFIG_PREEMPT_RT)]
unsafe fn invoke_softirq() {
    if should_wake_ksoftirqd() {
        wakeup_softirqd();
    }
}
#[cfg(CONFIG_PREEMPT_RT)]
#[no_mangle]
pub unsafe extern "C" fn do_softirq_post_smp_call_flush(was_pending: c_uint) {
    let is_pending = lupos_sirq_pending();
    if was_pending != is_pending {
        lupos_sirq_warn_flush(was_pending != (is_pending & !(1u32 << SCHED_SOFTIRQ)));
        invoke_softirq();
    }
}

#[cfg(all(not(CONFIG_PREEMPT_RT), CONFIG_TRACE_IRQFLAGS))]
#[no_mangle]
pub unsafe extern "C" fn __local_bh_disable_ip(ip: c_ulong, cnt: c_uint) {
    lupos_sirq_warn_hardirq(in_hardirq(), 0);
    let flags = lupos_sirq_raw_irq_save();
    // Never call the preempt tracer while SOFTIRQ_OFFSET and lockdep's
    // softirq-enabled bit disagree.
    lupos_sirq_preempt_add_raw(cnt);
    if softirq_count() == (cnt & RUST_SIRQ_SOFTIRQ_MASK) {
        lupos_sirq_softirqs_off(ip);
    }
    lupos_sirq_raw_irq_restore(flags);
    if lupos_sirq_preempt_count() == cnt {
        #[cfg(CONFIG_DEBUG_PREEMPT)]
        {
            (*current_task()).preempt_disable_ip = lock_parent_ip();
        }
        lupos_sirq_preempt_off(caller_ip(), lock_parent_ip());
    }
}

#[cfg(all(not(CONFIG_PREEMPT_RT), not(CONFIG_TRACE_IRQFLAGS)))]
#[inline(always)]
unsafe fn __local_bh_disable_ip(_ip: c_ulong, cnt: c_uint) {
    // This configured branch lives in bottom_half.h in C; keep it private
    // because the original configuration does not export a symbol.
    lupos_sirq_preempt_add(cnt);
    core::arch::asm!("", options(nostack, preserves_flags));
}

#[cfg(not(CONFIG_PREEMPT_RT))]
unsafe fn local_bh_enable_internal(cnt: c_uint) {
    lupos_sirq_assert_irqs_disabled();
    if lupos_sirq_preempt_count() == cnt {
        lupos_sirq_preempt_on(caller_ip(), lock_parent_ip());
    }
    if softirq_count() == (cnt & RUST_SIRQ_SOFTIRQ_MASK) {
        lupos_sirq_softirqs_on(caller_ip());
    }
    lupos_sirq_preempt_sub_raw(cnt);
}
#[cfg(not(CONFIG_PREEMPT_RT))]
#[no_mangle]
pub unsafe extern "C" fn _local_bh_enable() {
    lupos_sirq_warn_hardirq(in_hardirq(), 2);
    local_bh_enable_internal(RUST_SIRQ_SOFTIRQ_DISABLE_OFFSET);
}
#[cfg(not(CONFIG_PREEMPT_RT))]
#[no_mangle]
pub unsafe extern "C" fn __local_bh_enable_ip(ip: c_ulong, cnt: c_uint) {
    lupos_sirq_warn_hardirq(in_hardirq(), 1);
    lupos_sirq_assert_irqs_enabled();
    #[cfg(CONFIG_TRACE_IRQFLAGS)]
    lupos_sirq_irq_disable();
    if softirq_count() == RUST_SIRQ_SOFTIRQ_DISABLE_OFFSET {
        lupos_sirq_softirqs_on(ip);
    }
    // The last preempt unit remains until nested processing has completed.
    lupos_sirq_preempt_sub_raw(cnt.wrapping_sub(1));
    if !in_interrupt() && lupos_sirq_pending() != 0 {
        do_softirq();
    }
    lupos_sirq_preempt_dec();
    #[cfg(CONFIG_TRACE_IRQFLAGS)]
    lupos_sirq_irq_enable();
    lupos_sirq_preempt_check_resched();
}
#[cfg(not(CONFIG_PREEMPT_RT))]
unsafe fn softirq_handle_begin() {
    __local_bh_disable_ip(caller_ip(), RUST_SIRQ_SOFTIRQ_OFFSET);
}
#[cfg(not(CONFIG_PREEMPT_RT))]
unsafe fn softirq_handle_end() {
    local_bh_enable_internal(RUST_SIRQ_SOFTIRQ_OFFSET);
    lupos_sirq_warn_interrupt(in_interrupt());
}
#[cfg(not(CONFIG_PREEMPT_RT))]
unsafe fn ksoftirqd_run_begin() {
    lupos_sirq_irq_disable();
}
#[cfg(not(CONFIG_PREEMPT_RT))]
unsafe fn ksoftirqd_run_end() {
    lupos_sirq_irq_enable();
}
#[cfg(not(CONFIG_PREEMPT_RT))]
unsafe fn should_wake_ksoftirqd() -> bool {
    true
}
#[cfg(not(CONFIG_PREEMPT_RT))]
unsafe fn invoke_softirq() {
    if !lupos_sirq_force_irqthreads() || lupos_sirq_ksoftirqd_read().is_null() {
        #[cfg(CONFIG_HAVE_IRQ_EXIT_ON_IRQ_STACK)]
        __do_softirq();
        #[cfg(not(CONFIG_HAVE_IRQ_EXIT_ON_IRQ_STACK))]
        lupos_sirq_own_stack();
    } else {
        wakeup_softirqd();
    }
}
#[cfg(not(CONFIG_PREEMPT_RT))]
#[no_mangle]
pub unsafe extern "C" fn do_softirq() {
    if in_interrupt() {
        return;
    }
    let flags = lupos_sirq_irq_save();
    if lupos_sirq_pending() != 0 {
        lupos_sirq_own_stack();
    }
    lupos_sirq_irq_restore(flags);
}
