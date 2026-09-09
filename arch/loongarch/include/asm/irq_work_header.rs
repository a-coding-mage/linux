/* SPDX-License-Identifier: GPL-2.0 */

/// Return whether the architecture has an interrupt mechanism for irq_work.
#[inline]
pub unsafe fn arch_irq_work_has_interrupt() -> bool {
    is_enabled(CONFIG_SMP) && cpu_opt(LOONGARCH_CPU_CSRIPI)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
