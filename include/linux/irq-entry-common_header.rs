/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from linux/irq-entry-common.h. Required kernel symbols are external dependencies. */

/* Dummy _TIF work flags are zero when not supplied by the architecture or enabled functionality. */
#[allow(non_upper_case_globals)]
const _TIF_PATCH_PENDING: usize = 0;
#[allow(non_upper_case_globals)]
const ARCH_EXIT_TO_USER_MODE_WORK: usize = 0;

const EXIT_TO_USER_MODE_WORK: usize = _TIF_SIGPENDING | _TIF_NOTIFY_RESUME | _TIF_UPROBE |
    _TIF_NEED_RESCHED | _TIF_NEED_RESCHED_LAZY | _TIF_PATCH_PENDING |
    _TIF_NOTIFY_SIGNAL | _TIF_RSEQ | ARCH_EXIT_TO_USER_MODE_WORK;

#[cfg(feature = "CONFIG_HRTIMER_REARM_DEFERRED")]
const EXIT_TO_USER_MODE_WORK_SYSCALL: usize = EXIT_TO_USER_MODE_WORK;
#[cfg(feature = "CONFIG_HRTIMER_REARM_DEFERRED")]
const EXIT_TO_USER_MODE_WORK_IRQ: usize = EXIT_TO_USER_MODE_WORK | _TIF_HRTIMER_REARM;
#[cfg(not(feature = "CONFIG_HRTIMER_REARM_DEFERRED"))]
const EXIT_TO_USER_MODE_WORK_SYSCALL: usize = EXIT_TO_USER_MODE_WORK;
#[cfg(not(feature = "CONFIG_HRTIMER_REARM_DEFERRED"))]
const EXIT_TO_USER_MODE_WORK_IRQ: usize = EXIT_TO_USER_MODE_WORK;

#[inline(always)]
unsafe fn arch_enter_from_user_mode(_regs: *mut pt_regs) {}

#[inline(always)]
unsafe fn arch_in_rcu_eqs() -> bool { false }

#[inline(always)]
unsafe fn enter_from_user_mode(regs: *mut pt_regs) {
    arch_enter_from_user_mode(regs);
    lockdep_hardirqs_off(CALLER_ADDR0);
    CT_WARN_ON(__ct_state() != CT_STATE_USER);
    user_exit_irqoff();
    instrumentation_begin();
    kmsan_unpoison_entry_regs(regs);
    trace_hardirqs_off_finish();
    instrumentation_end();
}

#[inline]
unsafe fn arch_exit_to_user_mode_work(_regs: *mut pt_regs, _ti_work: c_ulong) {}

#[inline]
unsafe fn arch_exit_to_user_mode_prepare(_regs: *mut pt_regs, _ti_work: c_ulong) {}

#[inline(always)]
unsafe fn arch_exit_to_user_mode() {}

extern "C" {
    fn arch_do_signal_or_restart(regs: *mut pt_regs);
    fn exit_to_user_mode_loop(regs: *mut pt_regs, ti_work: c_ulong) -> c_ulong;
    fn raw_irqentry_exit_cond_resched();
}

#[inline(always)]
unsafe fn __exit_to_user_mode_prepare(regs: *mut pt_regs, work_mask: c_ulong) {
    let mut ti_work: c_ulong;
    lockdep_assert_irqs_disabled();
    tick_nohz_user_enter_prepare();
    ti_work = read_thread_flags();
    if (ti_work & work_mask) != 0 {
        if !hrtimer_rearm_deferred_user_irq(&mut ti_work, work_mask) {
            ti_work = exit_to_user_mode_loop(regs, ti_work);
        }
    }
    arch_exit_to_user_mode_prepare(regs, ti_work);
}

#[inline(always)]
unsafe fn __exit_to_user_mode_validate() {
    kmap_assert_nomap();
    lockdep_assert_irqs_disabled();
    lockdep_sys_exit();
}

#[inline(always)]
unsafe fn syscall_exit_to_user_mode_prepare(regs: *mut pt_regs) {
    __exit_to_user_mode_prepare(regs, EXIT_TO_USER_MODE_WORK_SYSCALL as c_ulong);
    rseq_syscall_exit_to_user_mode();
    __exit_to_user_mode_validate();
}

#[inline(always)]
unsafe fn irqentry_exit_to_user_mode_prepare(regs: *mut pt_regs) {
    __exit_to_user_mode_prepare(regs, EXIT_TO_USER_MODE_WORK_IRQ as c_ulong);
    rseq_irqentry_exit_to_user_mode();
    __exit_to_user_mode_validate();
}

#[inline(always)]
unsafe fn exit_to_user_mode() {
    instrumentation_begin();
    unwind_reset_info();
    trace_hardirqs_on_prepare();
    lockdep_hardirqs_on_prepare();
    instrumentation_end();
    user_enter_irqoff();
    arch_exit_to_user_mode();
    lockdep_hardirqs_on(CALLER_ADDR0);
}

#[inline(always)]
unsafe fn irqentry_enter_from_user_mode(regs: *mut pt_regs) {
    enter_from_user_mode(regs);
    rseq_note_user_irq_entry();
}

#[inline(always)]
unsafe fn irqentry_exit_to_user_mode(regs: *mut pt_regs) {
    lockdep_assert_irqs_disabled();
    instrumentation_begin();
    irqentry_exit_to_user_mode_prepare(regs);
    instrumentation_end();
    exit_to_user_mode();
}

#[repr(C)]
pub union irqentry_state_union {
    pub exit_rcu: bool,
    pub lockdep: bool,
}

#[repr(C)]
pub struct irqentry_state {
    pub state: irqentry_state_union,
}
pub type irqentry_state_t = irqentry_state;

#[cfg(feature = "CONFIG_PREEMPT_DYNAMIC")]
#[cfg(feature = "CONFIG_HAVE_PREEMPT_DYNAMIC_CALL")]
#[inline]
unsafe fn irqentry_exit_cond_resched() { static_call_irqentry_exit_cond_resched(); }
#[cfg(feature = "CONFIG_PREEMPT_DYNAMIC")]
#[cfg(all(not(feature = "CONFIG_HAVE_PREEMPT_DYNAMIC_CALL"), feature = "CONFIG_HAVE_PREEMPT_DYNAMIC_KEY"))]
#[inline]
unsafe fn irqentry_exit_cond_resched() { dynamic_irqentry_exit_cond_resched(); }
#[cfg(not(feature = "CONFIG_PREEMPT_DYNAMIC"))]
#[inline]
unsafe fn irqentry_exit_cond_resched() { raw_irqentry_exit_cond_resched(); }

#[inline(always)]
unsafe fn irqentry_enter_from_kernel_mode(regs: *mut pt_regs) -> irqentry_state_t {
    let mut ret = irqentry_state_t { state: irqentry_state_union { exit_rcu: false } };
    if !cfg!(feature = "CONFIG_TINY_RCU") && (is_idle_task(current) || arch_in_rcu_eqs()) {
        lockdep_hardirqs_off(CALLER_ADDR0);
        ct_irq_enter();
        instrumentation_begin();
        kmsan_unpoison_entry_regs(regs);
        trace_hardirqs_off_finish();
        instrumentation_end();
        ret.state.exit_rcu = true;
        return ret;
    }
    lockdep_hardirqs_off(CALLER_ADDR0);
    instrumentation_begin();
    kmsan_unpoison_entry_regs(regs);
    rcu_irq_enter_check_tick();
    trace_hardirqs_off_finish();
    instrumentation_end();
    ret
}

#[inline]
unsafe fn irqentry_exit_to_kernel_mode_preempt(regs: *mut pt_regs, state: irqentry_state_t) {
    if regs_irqs_disabled(regs) || state.state.exit_rcu { return; }
    if cfg!(feature = "CONFIG_PREEMPTION") { irqentry_exit_cond_resched(); }
}

#[inline(always)]
unsafe fn irqentry_exit_to_kernel_mode_after_preempt(regs: *mut pt_regs, state: irqentry_state_t) {
    if !regs_irqs_disabled(regs) {
        if state.state.exit_rcu {
            instrumentation_begin();
            hrtimer_rearm_deferred();
            trace_hardirqs_on_prepare();
            lockdep_hardirqs_on_prepare();
            instrumentation_end();
            ct_irq_exit();
            lockdep_hardirqs_on(CALLER_ADDR0);
            return;
        }
        instrumentation_begin();
        hrtimer_rearm_deferred();
        trace_hardirqs_on();
        instrumentation_end();
    } else if state.state.exit_rcu {
        ct_irq_exit();
    }
}

#[inline(always)]
unsafe fn irqentry_exit_to_kernel_mode(regs: *mut pt_regs, state: irqentry_state_t) {
    lockdep_assert_irqs_disabled();
    instrumentation_begin();
    irqentry_exit_to_kernel_mode_preempt(regs, state);
    instrumentation_end();
    irqentry_exit_to_kernel_mode_after_preempt(regs, state);
}

extern "C" {
    fn irqentry_enter(regs: *mut pt_regs) -> irqentry_state_t;
    fn irqentry_exit(regs: *mut pt_regs, state: irqentry_state_t);
    fn irqentry_nmi_enter(regs: *mut pt_regs) -> irqentry_state_t;
    fn irqentry_nmi_exit(regs: *mut pt_regs, irq_state: irqentry_state_t);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
