/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding kernel headers are intentionally
// left external to this translation.

extern "C" {
    pub fn synchronize_irq(irq: ::core::ffi::c_uint);
    pub fn synchronize_hardirq(irq: ::core::ffi::c_uint) -> bool;
}

// Under CONFIG_NO_HZ_FULL this is an external function; otherwise it is empty.
#[cfg(CONFIG_NO_HZ_FULL)]
extern "C" {
    pub fn __rcu_irq_enter_check_tick();
}

#[cfg(not(CONFIG_NO_HZ_FULL))]
#[inline(always)]
pub unsafe fn __rcu_irq_enter_check_tick() {}

#[inline(always)]
pub unsafe fn rcu_irq_enter_check_tick() {
    if context_tracking_enabled() {
        __rcu_irq_enter_check_tick();
    }
}

/*
 * It is safe to do non-atomic ops on ->hardirq_context,
 * because NMI handlers may not preempt and the ops are
 * always balanced, so the interrupted value of ->hardirq_context
 * will always be restored.
 */
#[inline(always)]
pub unsafe fn __irq_enter() {
    preempt_count_add(HARDIRQ_OFFSET);
    lockdep_hardirq_enter();
    account_hardirq_enter(current);
}

/*
 * Like __irq_enter() without time accounting for fast
 * interrupts, e.g. reschedule IPI where time accounting
 * is more expensive than the actual interrupt.
 */
#[inline(always)]
pub unsafe fn __irq_enter_raw() {
    preempt_count_add(HARDIRQ_OFFSET);
    lockdep_hardirq_enter();
}

/* Enter irq context (on NO_HZ, update jiffies): */
extern "C" {
    pub fn irq_enter();
}

/* Like irq_enter(), but RCU is already watching. */
extern "C" {
    pub fn irq_enter_rcu();
}

/* Exit irq context without processing softirqs: */
#[inline(always)]
pub unsafe fn __irq_exit() {
    account_hardirq_exit(current);
    lockdep_hardirq_exit();
    preempt_count_sub(HARDIRQ_OFFSET);
}

/* Like __irq_exit() without time accounting */
#[inline(always)]
pub unsafe fn __irq_exit_raw() {
    lockdep_hardirq_exit();
    preempt_count_sub(HARDIRQ_OFFSET);
}

/* Exit irq context and process softirqs if needed: */
extern "C" {
    pub fn irq_exit();
}

/* Like irq_exit(), but return with RCU watching. */
extern "C" {
    pub fn irq_exit_rcu();
}

// If the architecture does not provide these hooks, they are empty.
#[inline(always)]
pub unsafe fn arch_nmi_enter() {}

#[inline(always)]
pub unsafe fn arch_nmi_exit() {}

#[cfg(CONFIG_HAS_SEPARATE_PREEMPT_RESCHED_BITS)]
#[inline(always)]
pub unsafe fn __preempt_count_nmi_enter() {
    __preempt_count_add(NMI_OFFSET + HARDIRQ_OFFSET);
}

#[cfg(CONFIG_HAS_SEPARATE_PREEMPT_RESCHED_BITS)]
#[inline(always)]
pub unsafe fn __preempt_count_nmi_exit() {
    __preempt_count_sub(NMI_OFFSET + HARDIRQ_OFFSET);
}

#[cfg(not(CONFIG_HAS_SEPARATE_PREEMPT_RESCHED_BITS))]
extern "C" {
    // DECLARE_PER_CPU(unsigned int, nmi_nesting)
    pub static mut nmi_nesting: ::core::ffi::c_uint;
}

#[cfg(not(CONFIG_HAS_SEPARATE_PREEMPT_RESCHED_BITS))]
#[inline(always)]
pub unsafe fn __preempt_count_nmi_enter() {
    __preempt_count_add(HARDIRQ_OFFSET);
    // Maximum NMI nesting is 15.
    BUG_ON(__this_cpu_read(nmi_nesting) >= 15);
    __this_cpu_inc(nmi_nesting);
    preempt_count_set(preempt_count() | NMI_MASK);
}

#[cfg(not(CONFIG_HAS_SEPARATE_PREEMPT_RESCHED_BITS))]
#[inline(always)]
pub unsafe fn __preempt_count_nmi_exit() {
    __preempt_count_sub(HARDIRQ_OFFSET);
    if __this_cpu_dec_return(nmi_nesting) == 0 {
        preempt_count_set(preempt_count() & !NMI_MASK);
    }
}

/*
 * NMI vs Tracing
 * --------------
 *
 * We must not land in a tracer until (or after) we've changed preempt_count
 * such that in_nmi() becomes true. To that effect all NMI C entry points must
 * be marked 'notrace' and call nmi_enter() as soon as possible.
 */

/* nmi_enter() can nest - nesting is tracked in a per-CPU counter. */
#[inline(always)]
pub unsafe fn __nmi_enter() {
    lockdep_off();
    arch_nmi_enter();
    __preempt_count_nmi_enter();
}

#[inline(always)]
pub unsafe fn nmi_enter() {
    __nmi_enter();
    lockdep_hardirq_enter();
    ct_nmi_enter();
    instrumentation_begin();
    ftrace_nmi_enter();
    instrumentation_end();
}

#[inline(always)]
pub unsafe fn __nmi_exit() {
    BUG_ON(!in_nmi());
    __preempt_count_nmi_exit();
    arch_nmi_exit();
    lockdep_on();
}

#[inline(always)]
pub unsafe fn nmi_exit() {
    instrumentation_begin();
    ftrace_nmi_exit();
    instrumentation_end();
    ct_nmi_exit();
    lockdep_hardirq_exit();
    __nmi_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
