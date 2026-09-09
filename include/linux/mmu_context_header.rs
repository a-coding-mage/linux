/* SPDX-License-Identifier: GPL-2.0 */

// The architecture-specific declarations supplied by <asm/mmu_context.h>
// and <asm/mmu.h> are external dependencies.

/* Architectures that care about IRQ state in switch_mm can override this. */
// If `switch_mm_irqs_off` is not supplied by the architecture, it aliases
// `switch_mm`.
#[cfg(not(any()))]
macro_rules! switch_mm_irqs_off {
    ($($args:tt)*) => { switch_mm($($args)*) };
}

#[cfg(not(any()))]
#[inline]
pub unsafe fn leave_mm() {}

/*
 * CPUs that are capable of running user task @p. Must contain at least one
 * active CPU. It is assumed that the kernel can run on all CPUs, so calling
 * this for a kernel thread is pointless.
 *
 * By default, we assume a sane, homogeneous system.
 */
// When `task_cpu_possible_mask` is not supplied by the architecture:
#[cfg(not(any()))]
macro_rules! task_cpu_possible_mask {
    ($p:expr) => { cpu_possible_mask };
}

#[cfg(not(any()))]
macro_rules! task_cpu_possible {
    ($cpu:expr, $p:expr) => { true };
}

#[cfg(not(any()))]
macro_rules! task_cpu_fallback_mask {
    ($p:expr) => { housekeeping_cpumask(HK_TYPE_DOMAIN) };
}

// Otherwise, `task_cpu_possible(cpu, p)` is supplied by the architecture as:
// cpumask_test_cpu((cpu), task_cpu_possible_mask(p))

#[cfg(not(any()))]
#[inline]
pub unsafe fn mm_untag_mask(mm: *mut mm_struct) -> c_ulong {
    let _ = mm;
    !0 as c_ulong
}

#[cfg(not(any()))]
#[inline]
pub unsafe fn arch_pgtable_dma_compat(mm: *mut mm_struct) -> bool {
    let _ = mm;
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
