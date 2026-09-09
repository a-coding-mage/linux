// SPDX-License-Identifier: GPL-2.0
/*
 * preemptoff and irqoff tracepoints
 *
 * Copyright (C) Joel Fernandes (Google) <joel@joelfernandes.org>
 */

// Dependency intent preserved from the original Linux kernel includes:
// linux/kallsyms.h, linux/uaccess.h, linux/module.h, linux/ftrace.h,
// linux/kprobes.h, linux/hardirq.h, trace.h, and trace/events/preemptirq.h.

type c_ulong = usize;

/*
 * Use regular trace points on architectures that implement noinstr
 * tooling: these calls will only happen with RCU enabled, which can
 * use a regular tracepoint.
 *
 * On older architectures, RCU may not be watching in idle. In that
 * case, wake up RCU to watch while calling the tracepoint. These
 * aren't NMI-safe - so exclude NMI contexts:
 */

#[cfg(CONFIG_ARCH_WANTS_NO_INSTR)]
macro_rules! trace {
    (irq_enable, $args:expr) => { trace_irq_enable($args) };
    (irq_disable, $args:expr) => { trace_irq_disable($args) };
    (preempt_enable, $args:expr) => { trace_preempt_enable($args) };
    (preempt_disable, $args:expr) => { trace_preempt_disable($args) };
}

#[cfg(not(CONFIG_ARCH_WANTS_NO_INSTR))]
macro_rules! trace {
    (irq_enable, $args:expr) => {{
        if __trace_irq_enable_enabled() {
            let mut exit_rcu: bool = false;
            if in_nmi() {
                return;
            }
            if !cfg!(CONFIG_TINY_RCU) && is_idle_task(current) {
                ct_irq_enter();
                exit_rcu = true;
            }
            trace_irq_enable($args);
            if exit_rcu {
                ct_irq_exit();
            }
        }
    }};
    (irq_disable, $args:expr) => {{
        if __trace_irq_disable_enabled() {
            let mut exit_rcu: bool = false;
            if in_nmi() {
                return;
            }
            if !cfg!(CONFIG_TINY_RCU) && is_idle_task(current) {
                ct_irq_enter();
                exit_rcu = true;
            }
            trace_irq_disable($args);
            if exit_rcu {
                ct_irq_exit();
            }
        }
    }};
}

#[cfg(CONFIG_TRACE_IRQFLAGS)]
static mut tracing_irq_cpu: i32 = 0;

#[cfg(CONFIG_TRACE_IRQFLAGS)]
extern "C" {
    fn __trace_irq_enable_enabled() -> bool;
    fn __trace_irq_disable_enabled() -> bool;
    fn trace_irq_enable(args: (c_ulong, c_ulong));
    fn trace_irq_disable(args: (c_ulong, c_ulong));
    fn tracer_hardirqs_on(a0: c_ulong, a1: c_ulong);
    fn tracer_hardirqs_off(a0: c_ulong, a1: c_ulong);
    fn lockdep_hardirqs_on_prepare();
    fn lockdep_hardirqs_on(a0: c_ulong);
    fn lockdep_hardirqs_off(a0: c_ulong);
    fn caller_addr0() -> c_ulong;
    fn caller_addr1() -> c_ulong;
}

#[cfg(CONFIG_TRACE_IRQFLAGS)]
#[inline]
unsafe fn this_cpu_read_tracing_irq_cpu() -> i32 {
    tracing_irq_cpu
}

#[cfg(CONFIG_TRACE_IRQFLAGS)]
#[inline]
unsafe fn this_cpu_write_tracing_irq_cpu(value: i32) {
    tracing_irq_cpu = value;
}

#[cfg(CONFIG_TRACE_IRQFLAGS)]
#[no_mangle]
pub unsafe extern "C" fn trace_hardirqs_on_prepare() {
    if this_cpu_read_tracing_irq_cpu() != 0 {
        trace!(irq_enable, (caller_addr0(), caller_addr1()));
        tracer_hardirqs_on(caller_addr0(), caller_addr1());
        this_cpu_write_tracing_irq_cpu(0);
    }
}

#[cfg(CONFIG_TRACE_IRQFLAGS)]
#[no_mangle]
pub unsafe extern "C" fn trace_hardirqs_on() {
    if this_cpu_read_tracing_irq_cpu() != 0 {
        trace!(irq_enable, (caller_addr0(), caller_addr1()));
        tracer_hardirqs_on(caller_addr0(), caller_addr1());
        this_cpu_write_tracing_irq_cpu(0);
    }

    lockdep_hardirqs_on_prepare();
    lockdep_hardirqs_on(caller_addr0());
}

#[cfg(CONFIG_TRACE_IRQFLAGS)]
#[no_mangle]
pub unsafe extern "C" fn trace_hardirqs_off_finish() {
    if this_cpu_read_tracing_irq_cpu() == 0 {
        this_cpu_write_tracing_irq_cpu(1);
        tracer_hardirqs_off(caller_addr0(), caller_addr1());
        trace!(irq_disable, (caller_addr0(), caller_addr1()));
    }
}

#[cfg(CONFIG_TRACE_IRQFLAGS)]
#[no_mangle]
pub unsafe extern "C" fn trace_hardirqs_off() {
    lockdep_hardirqs_off(caller_addr0());

    if this_cpu_read_tracing_irq_cpu() == 0 {
        this_cpu_write_tracing_irq_cpu(1);
        tracer_hardirqs_off(caller_addr0(), caller_addr1());
        trace!(irq_disable, (caller_addr0(), caller_addr1()));
    }
}

#[cfg(CONFIG_TRACE_PREEMPT_TOGGLE)]
extern "C" {
    fn trace_preempt_enable(args: (c_ulong, c_ulong));
    fn trace_preempt_disable(args: (c_ulong, c_ulong));
    fn tracer_preempt_on(a0: c_ulong, a1: c_ulong);
    fn tracer_preempt_off(a0: c_ulong, a1: c_ulong);
}

#[cfg(CONFIG_TRACE_PREEMPT_TOGGLE)]
#[no_mangle]
pub unsafe extern "C" fn trace_preempt_on(a0: c_ulong, a1: c_ulong) {
    trace!(preempt_enable, (a0, a1));
    tracer_preempt_on(a0, a1);
}

#[cfg(CONFIG_TRACE_PREEMPT_TOGGLE)]
#[no_mangle]
pub unsafe extern "C" fn trace_preempt_off(a0: c_ulong, a1: c_ulong) {
    trace!(preempt_disable, (a0, a1));
    tracer_preempt_off(a0, a1);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
