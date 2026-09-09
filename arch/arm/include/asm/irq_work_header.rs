/* SPDX-License-Identifier: GPL-2.0 */

// Dependency provided by <asm/smp_plat.h>.
unsafe extern "C" {
    fn is_smp() -> bool;
}

#[inline]
fn arch_irq_work_has_interrupt() -> bool {
    unsafe { is_smp() }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
