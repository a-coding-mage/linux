/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * Power Switch is wired via BaseIO BRIDGE slot #6.
 *
 * ACFail is wired via BaseIO BRIDGE slot #7.
 */

pub const IP30_POWER_IRQ: _ = HEART_L2_INT_POWER_BTN;

pub const IP30_HEART_L0_IRQ: _ = MIPS_CPU_IRQ_BASE + 2;
pub const IP30_HEART_L1_IRQ: _ = MIPS_CPU_IRQ_BASE + 3;
pub const IP30_HEART_L2_IRQ: _ = MIPS_CPU_IRQ_BASE + 4;
pub const IP30_HEART_TIMER_IRQ: _ = MIPS_CPU_IRQ_BASE + 5;
pub const IP30_HEART_ERR_IRQ: _ = MIPS_CPU_IRQ_BASE + 6;

extern "C" {
    // C declaration carried the __init annotation.
    pub fn ip30_install_ipi();
    pub static mut ip30_smp_ops: plat_smp_ops;
    // C declaration carried the __init annotation.
    pub fn ip30_per_cpu_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
