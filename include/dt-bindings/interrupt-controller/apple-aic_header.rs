/* SPDX-License-Identifier: GPL-2.0+ OR MIT */

// Dependency intent: <dt-bindings/interrupt-controller/irq.h>

pub const AIC_IRQ: u32 = 0;
pub const AIC_FIQ: u32 = 1;

pub const AIC_TMR_HV_PHYS: u32 = 0;
pub const AIC_TMR_HV_VIRT: u32 = 1;
pub const AIC_TMR_GUEST_PHYS: u32 = 2;
pub const AIC_TMR_GUEST_VIRT: u32 = 3;
pub const AIC_CPU_PMU_E: u32 = 4;
pub const AIC_CPU_PMU_P: u32 = 5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
