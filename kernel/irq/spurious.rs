// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 1992, 1998-2004 Linus Torvalds, Ingo Molnar
 *
 * This file contains spurious interrupt handling.
 */

// Dependencies supplied by the kernel translation unit.

static mut irqfixup: i32 = 0;

const POLL_SPURIOUS_IRQ_INTERVAL: u64 = HZ / 10;
static mut irq_poll_cpu: i32 = 0;
static irq_poll_active: atomic_t = atomic_t::new(0);

/*
 * Recovery handler for misrouted interrupts.
 */
unsafe fn try_one_irq(desc: *mut irq_desc, force: bool) -> bool {
    let mut action: *mut irqaction;
    let mut ret = false;

    let _guard = guard_raw_spinlock(&mut (*desc).lock);

    /*
     * PER_CPU, nested thread interrupts and interrupts explicitly
     * marked polled are excluded from polling.
     */
    if irq_settings_is_per_cpu(desc) || irq_settings_is_nested_thread(desc) ||
        irq_settings_is_polled(desc) { return false; }

    /* Do not poll disabled interrupts unless the spurious disabled poller asks explicitly. */
    if irqd_irq_disabled(&(*desc).irq_data) && !force { return false; }

    /* All handlers must agree on IRQF_SHARED, so we test just the first. */
    action = (*desc).action;
    if action.is_null() || ((*action).flags & IRQF_SHARED) == 0 ||
        ((*action).flags & __IRQF_TIMER) != 0 { return false; }

    /* Already running on another processor */
    if irqd_irq_inprogress(&(*desc).irq_data) {
        /* Already running: If it is shared get the other CPU to go looking for our mystery interrupt too */
        (*desc).istate |= IRQS_PENDING;
        return false;
    }

    /* Mark it poll in progress */
    (*desc).istate |= IRQS_POLL_INPROGRESS;
    loop {
        if handle_irq_event(desc) == IRQ_HANDLED { ret = true; }
        /* Make sure that there is still a valid action */
        action = (*desc).action;
        if ((*desc).istate & IRQS_PENDING) == 0 || action.is_null() { break; }
    }
    (*desc).istate &= !IRQS_POLL_INPROGRESS;
    ret
}

unsafe fn misrouted_irq(irq: i32) -> i32 {
    let mut desc: *mut irq_desc = core::ptr::null_mut();
    let mut ok = 0;
    if atomic_inc_return(&irq_poll_active) != 1 { goto_out!(); }
    irq_poll_cpu = smp_processor_id();
    for_each_irq_desc!(i, desc, {
        if i == 0 { continue; }
        if i == irq { continue; }
        if try_one_irq(desc, false) { ok = 1; }
    });
    goto_out!();
    atomic_dec(&irq_poll_active);
    ok
}

unsafe fn poll_spurious_irqs(_unused: *mut timer_list) {
    let mut desc: *mut irq_desc = core::ptr::null_mut();
    if atomic_inc_return(&irq_poll_active) != 1 { goto_out!(); }
    irq_poll_cpu = smp_processor_id();
    for_each_irq_desc!(i, desc, {
        let state: u32;
        if i == 0 { continue; }
        /* Racy but it doesn't matter */
        state = READ_ONCE((*desc).istate);
        if (state & IRQS_SPURIOUS_DISABLED) == 0 { continue; }
        local_irq_disable();
        try_one_irq(desc, true);
        local_irq_enable();
    });
    goto_out!();
    atomic_dec(&irq_poll_active);
    mod_timer(&mut poll_spurious_irq_timer, jiffies + POLL_SPURIOUS_IRQ_INTERVAL);
}

unsafe fn bad_action_ret(action_ret: irqreturn_t) -> i32 {
    let r = action_ret as u32;
    if likely(r <= (IRQ_HANDLED | IRQ_WAKE_THREAD)) { 0 } else { 1 }
}

unsafe fn __report_bad_irq(desc: *mut irq_desc, action_ret: irqreturn_t) {
    let irq = irq_desc_get_irq(desc);
    let mut action: *mut irqaction;
    if bad_action_ret(action_ret) != 0 { pr_err!("irq event %d: bogus return value %x\n", irq, action_ret); }
    else { pr_err!("irq %d: nobody cared (try booting with the \\"irqpoll\\" option)\n", irq); }
    dump_stack!();
    pr_err!("handlers:\n");
    let _guard = guard_raw_spinlock_irqsave(&mut (*desc).lock);
    for_each_action_of_desc!(desc, action, {
        pr_err!("[<%p>] %ps", (*action).handler, (*action).handler);
        if !(*action).thread_fn.is_null() { pr_cont!(" threaded [<%p>] %ps", (*action).thread_fn, (*action).thread_fn); }
        pr_cont!("\n");
    });
}

unsafe fn report_bad_irq(desc: *mut irq_desc, action_ret: irqreturn_t) {
    static mut count: i32 = 100;
    if count > 0 { count -= 1; __report_bad_irq(desc, action_ret); }
}

unsafe fn try_misrouted_irq(irq: u32, desc: *mut irq_desc, action_ret: irqreturn_t) -> bool {
    if irqfixup == 0 { return false; }
    if action_ret == IRQ_NONE { return true; }
    if irqfixup < 2 { return false; }
    if irq == 0 { return true; }
    let action = READ_ONCE((*desc).action);
    !action.is_null() && ((*action).flags & IRQF_IRQPOLL) != 0
}

const SPURIOUS_DEFERRED: u32 = 0x80000000;

pub unsafe fn note_interrupt(desc: *mut irq_desc, mut action_ret: irqreturn_t) {
    let irq: u32;
    if ((*desc).istate & IRQS_POLL_INPROGRESS) != 0 || irq_settings_is_polled(desc) { return; }
    if bad_action_ret(action_ret) != 0 { report_bad_irq(desc, action_ret); return; }
    if (action_ret & IRQ_WAKE_THREAD) != 0 {
        if action_ret == IRQ_WAKE_THREAD {
            let handled: i32;
            if ((*desc).threads_handled_last & SPURIOUS_DEFERRED) == 0 { (*desc).threads_handled_last |= SPURIOUS_DEFERRED; return; }
            handled = atomic_read(&(*desc).threads_handled) | SPURIOUS_DEFERRED as i32;
            if handled != (*desc).threads_handled_last { action_ret = IRQ_HANDLED; (*desc).threads_handled_last = handled; }
            else { action_ret = IRQ_NONE; }
        } else { (*desc).threads_handled_last &= !SPURIOUS_DEFERRED; }
    }
    if unlikely(action_ret == IRQ_NONE) {
        if time_after(jiffies, (*desc).last_unhandled + HZ / 10) { (*desc).irqs_unhandled = 1; }
        else { (*desc).irqs_unhandled += 1; }
        (*desc).last_unhandled = jiffies;
    }
    irq = irq_desc_get_irq(desc);
    if unlikely(try_misrouted_irq(irq, desc, action_ret)) { let ok = misrouted_irq(irq as i32); if action_ret == IRQ_NONE { (*desc).irqs_unhandled -= ok; } }
    if likely((*desc).irqs_unhandled == 0) { return; }
    (*desc).irq_count += 1;
    if likely((*desc).irq_count < 100000) { return; }
    (*desc).irq_count = 0;
    if unlikely((*desc).irqs_unhandled > 99900) {
        __report_bad_irq(desc, action_ret);
        pr_emerg!("Disabling IRQ #%d\n", irq);
        (*desc).istate |= IRQS_SPURIOUS_DISABLED;
        (*desc).depth += 1;
        irq_disable(desc);
        mod_timer(&mut poll_spurious_irq_timer, jiffies + POLL_SPURIOUS_IRQ_INTERVAL);
    }
    (*desc).irqs_unhandled = 0;
}

pub static mut noirqdebug: bool = false;
pub unsafe fn noirqdebug_setup(_str: *mut i8) -> i32 { noirqdebug = true; pr_info!("IRQ lockup detection disabled\n"); 1 }
// __setup("noirqdebug", noirqdebug_setup); module_param(noirqdebug, bool, 0644);
// MODULE_PARM_DESC(noirqdebug, "Disable irq lockup detection when true");

unsafe fn irqfixup_setup(_str: *mut i8) -> i32 {
    if IS_ENABLED!(CONFIG_PREEMPT_RT) { pr_warn!("irqfixup boot option not supported with PREEMPT_RT\n"); return 1; }
    irqfixup = 1; pr_warn!("Misrouted IRQ fixup support enabled.\n"); pr_warn!("This may impact system performance.\n"); 1
}
// __setup("irqfixup", irqfixup_setup); module_param(irqfixup, int, 0644);

unsafe fn irqpoll_setup(_str: *mut i8) -> i32 {
    if IS_ENABLED!(CONFIG_PREEMPT_RT) { pr_warn!("irqpoll boot option not supported with PREEMPT_RT\n"); return 1; }
    irqfixup = 2; pr_warn!("Misrouted IRQ fixup and polling support enabled\n"); pr_warn!("This may significantly impact system performance\n"); 1
}
// __setup("irqpoll", irqpoll_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
