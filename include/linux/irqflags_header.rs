/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust translation of include/linux/irqflags.h.
 * Includes and build-time configuration supplied by the surrounding kernel
 * translation are intentionally represented by cfg gates and external names.
 */

#[repr(C)]
pub struct task_struct;

#[cfg(feature = "CONFIG_PROVE_LOCKING")]
extern "C" {
    pub fn lockdep_softirqs_on(ip: libc::c_ulong);
    pub fn lockdep_softirqs_off(ip: libc::c_ulong);
    pub fn lockdep_hardirqs_on_prepare();
    pub fn lockdep_hardirqs_on(ip: libc::c_ulong);
    pub fn lockdep_hardirqs_off(ip: libc::c_ulong);
    pub fn lockdep_cleanup_dead_cpu(cpu: libc::c_uint, idle: *mut task_struct);
}

#[cfg(not(feature = "CONFIG_PROVE_LOCKING"))]
#[inline(always)] pub unsafe fn lockdep_softirqs_on(_ip: libc::c_ulong) {}
#[cfg(not(feature = "CONFIG_PROVE_LOCKING"))]
#[inline(always)] pub unsafe fn lockdep_softirqs_off(_ip: libc::c_ulong) {}
#[cfg(not(feature = "CONFIG_PROVE_LOCKING"))]
#[inline(always)] pub unsafe fn lockdep_hardirqs_on_prepare() {}
#[cfg(not(feature = "CONFIG_PROVE_LOCKING"))]
#[inline(always)] pub unsafe fn lockdep_hardirqs_on(_ip: libc::c_ulong) {}
#[cfg(not(feature = "CONFIG_PROVE_LOCKING"))]
#[inline(always)] pub unsafe fn lockdep_hardirqs_off(_ip: libc::c_ulong) {}
#[cfg(not(feature = "CONFIG_PROVE_LOCKING"))]
#[inline(always)] pub unsafe fn lockdep_cleanup_dead_cpu(_cpu: libc::c_uint, _idle: *mut task_struct) {}

#[cfg(feature = "CONFIG_TRACE_IRQFLAGS")]
extern "C" {
    pub static mut hardirqs_enabled: libc::c_int;
    pub static mut hardirq_context: libc::c_int;
    pub fn trace_hardirqs_on_prepare();
    pub fn trace_hardirqs_off_finish();
    pub fn trace_hardirqs_on();
    pub fn trace_hardirqs_off();
}

/* The following macro translations preserve the original call-site behavior. */
#[cfg(feature = "CONFIG_TRACE_IRQFLAGS")]
#[macro_export] macro_rules! lockdep_hardirq_context { () => { unsafe { $crate::raw_cpu_read($crate::hardirq_context) } }; }
#[cfg(feature = "CONFIG_TRACE_IRQFLAGS")]
#[macro_export] macro_rules! lockdep_softirq_context { ($p:expr) => { unsafe { (*$p).softirq_context } }; }
#[cfg(feature = "CONFIG_TRACE_IRQFLAGS")]
#[macro_export] macro_rules! lockdep_hardirqs_enabled { () => { unsafe { $crate::this_cpu_read($crate::hardirqs_enabled) } }; }
#[cfg(feature = "CONFIG_TRACE_IRQFLAGS")]
#[macro_export] macro_rules! lockdep_softirqs_enabled { ($p:expr) => { unsafe { (*$p).softirqs_enabled } }; }

#[cfg(not(feature = "CONFIG_TRACE_IRQFLAGS"))]
#[macro_export] macro_rules! trace_hardirqs_on_prepare { () => {} }
#[cfg(not(feature = "CONFIG_TRACE_IRQFLAGS"))]
#[macro_export] macro_rules! trace_hardirqs_off_finish { () => {} }
#[cfg(not(feature = "CONFIG_TRACE_IRQFLAGS"))]
#[macro_export] macro_rules! trace_hardirqs_on { () => {} }
#[cfg(not(feature = "CONFIG_TRACE_IRQFLAGS"))]
#[macro_export] macro_rules! trace_hardirqs_off { () => {} }

#[cfg(not(feature = "CONFIG_TRACE_IRQFLAGS"))]
#[macro_export] macro_rules! lockdep_hardirq_context { () => { 0 }; }
#[cfg(not(feature = "CONFIG_TRACE_IRQFLAGS"))]
#[macro_export] macro_rules! lockdep_softirq_context { ($p:expr) => { 0 }; }
#[cfg(not(feature = "CONFIG_TRACE_IRQFLAGS"))]
#[macro_export] macro_rules! lockdep_hardirqs_enabled { () => { 0 }; }
#[cfg(not(feature = "CONFIG_TRACE_IRQFLAGS"))]
#[macro_export] macro_rules! lockdep_softirqs_enabled { ($p:expr) => { 0 }; }

#[macro_export] macro_rules! lockdep_hardirq_enter { () => { unsafe { $crate::__this_cpu_inc_return($crate::hardirq_context); if $crate::__this_cpu_inc_return($crate::hardirq_context) == 1 { (*$crate::current).hardirq_threaded = 0; } } }; }
#[macro_export] macro_rules! lockdep_hardirq_threaded { () => { unsafe { (*$crate::current).hardirq_threaded = 1; } }; }
#[macro_export] macro_rules! lockdep_hardirq_exit { () => { unsafe { $crate::__this_cpu_dec($crate::hardirq_context); } }; }

#[cfg(all(feature = "CONFIG_TRACE_IRQFLAGS", not(feature = "CONFIG_PREEMPT_RT")))]
#[macro_export] macro_rules! lockdep_softirq_enter { () => { unsafe { (*$crate::current).softirq_context += 1; } }; }
#[cfg(all(feature = "CONFIG_TRACE_IRQFLAGS", not(feature = "CONFIG_PREEMPT_RT")))]
#[macro_export] macro_rules! lockdep_softirq_exit { () => { unsafe { (*$crate::current).softirq_context -= 1; } }; }
#[cfg(any(not(feature = "CONFIG_TRACE_IRQFLAGS"), feature = "CONFIG_PREEMPT_RT"))]
#[macro_export] macro_rules! lockdep_softirq_enter { () => {} }
#[cfg(any(not(feature = "CONFIG_TRACE_IRQFLAGS"), feature = "CONFIG_PREEMPT_RT"))]
#[macro_export] macro_rules! lockdep_softirq_exit { () => {} }

#[macro_export] macro_rules! lockdep_hrtimer_enter { ($h:expr) => {{ let mut expires_hardirq = true; unsafe { if !(*$h).is_hard { (*$crate::current).irq_config = 1; expires_hardirq = false; } } expires_hardirq }}; }
#[macro_export] macro_rules! lockdep_hrtimer_exit { ($e:expr) => { unsafe { if !$e { (*$crate::current).irq_config = 0; } } }; }
#[macro_export] macro_rules! lockdep_posixtimer_enter { () => { unsafe { (*$crate::current).irq_config = 1; } }; }
#[macro_export] macro_rules! lockdep_posixtimer_exit { () => { unsafe { (*$crate::current).irq_config = 0; } }; }
#[macro_export] macro_rules! lockdep_irq_work_enter { ($f:expr) => { unsafe { if !($f & $crate::IRQ_WORK_HARD_IRQ) { (*$crate::current).irq_config = 1; } } }; }
#[macro_export] macro_rules! lockdep_irq_work_exit { ($f:expr) => { unsafe { if !($f & $crate::IRQ_WORK_HARD_IRQ) { (*$crate::current).irq_config = 0; } } }; }

#[cfg(any(feature = "CONFIG_IRQSOFF_TRACER", feature = "CONFIG_PREEMPT_TRACER"))]
extern "C" { pub fn stop_critical_timings(); pub fn start_critical_timings(); }
#[cfg(not(any(feature = "CONFIG_IRQSOFF_TRACER", feature = "CONFIG_PREEMPT_TRACER")))]
#[macro_export] macro_rules! stop_critical_timings { () => {} }
#[cfg(not(any(feature = "CONFIG_IRQSOFF_TRACER", feature = "CONFIG_PREEMPT_TRACER")))]
#[macro_export] macro_rules! start_critical_timings { () => {} }

#[cfg(feature = "CONFIG_DEBUG_IRQFLAGS")]
extern "C" { pub fn warn_bogus_irq_restore(); }
#[macro_export] macro_rules! raw_check_bogus_irq_restore { () => { #[cfg(feature = "CONFIG_DEBUG_IRQFLAGS")] unsafe { if !$crate::arch_irqs_disabled() { $crate::warn_bogus_irq_restore(); } } }; }

#[macro_export] macro_rules! raw_local_irq_disable { () => { $crate::arch_local_irq_disable() }; }
#[macro_export] macro_rules! raw_local_irq_enable { () => { $crate::arch_local_irq_enable() }; }
#[macro_export] macro_rules! raw_local_irq_save { ($f:expr) => { $f = $crate::arch_local_irq_save() }; }
#[macro_export] macro_rules! raw_local_irq_restore { ($f:expr) => { raw_check_bogus_irq_restore!(); $crate::arch_local_irq_restore($f); }; }
#[macro_export] macro_rules! raw_local_save_flags { ($f:expr) => { $f = $crate::arch_local_save_flags() }; }
#[macro_export] macro_rules! raw_irqs_disabled_flags { ($f:expr) => { $crate::arch_irqs_disabled_flags($f) }; }
#[macro_export] macro_rules! raw_irqs_disabled { () => { $crate::arch_irqs_disabled() }; }
#[macro_export] macro_rules! raw_safe_halt { () => { $crate::arch_safe_halt() }; }

#[macro_export] macro_rules! local_irq_enable { () => { trace_hardirqs_on!(); raw_local_irq_enable!(); }; }
#[macro_export] macro_rules! local_irq_disable { () => {{ let was_disabled = raw_irqs_disabled!(); raw_local_irq_disable!(); if !was_disabled { trace_hardirqs_off!(); } }}; }
#[macro_export] macro_rules! local_irq_save { ($f:expr) => { raw_local_irq_save!($f); if !raw_irqs_disabled_flags!($f) { trace_hardirqs_off!(); } }; }
#[macro_export] macro_rules! local_irq_restore { ($f:expr) => { if !raw_irqs_disabled_flags!($f) { trace_hardirqs_on!(); } raw_local_irq_restore!($f); }; }
#[macro_export] macro_rules! safe_halt { () => { trace_hardirqs_on!(); raw_safe_halt!(); }; }
#[macro_export] macro_rules! local_save_flags { ($f:expr) => { raw_local_save_flags!($f) }; }
#[macro_export] macro_rules! irqs_disabled { () => {{ let mut flags: libc::c_ulong = 0; raw_local_save_flags!(flags); raw_irqs_disabled_flags!(flags) }}; }
#[macro_export] macro_rules! irqs_disabled_flags { ($f:expr) => { raw_irqs_disabled_flags!($f) }; }

/* DEFINE_LOCK_GUARD_0(irq, ...) and DEFINE_LOCK_GUARD_0(irqsave, ...) are
 * represented by the corresponding local lock operations at their call sites. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
