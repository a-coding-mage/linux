/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

/* Number of first-level interrupts associated with the CPU core. */
pub const HEXAGON_CPUINTS: i32 = 32;

/*
 * Must define NR_IRQS before including <asm-generic/irq.h>
 * 64 == the two SIRC's, 176 == the two gpio's
 *
 * IRQ configuration is still in flux; defining this to a comfortably
 * large number.
 */
pub const NR_IRQS: i32 = 512;

/* C dependency: <asm-generic/irq.h> */

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

pub unsafe extern "C" {
    pub fn arch_do_IRQ(regs: *mut pt_regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
