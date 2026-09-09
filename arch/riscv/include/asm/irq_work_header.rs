/* SPDX-License-Identifier: GPL-2.0 */

#[inline]
fn arch_irq_work_has_interrupt() -> bool {
    IS_ENABLED!(CONFIG_SMP)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
