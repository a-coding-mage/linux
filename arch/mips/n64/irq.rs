// SPDX-License-Identifier: GPL-2.0
/*
 *  N64 IRQ
 *
 *  Copyright (C) 2021 Lauri Kasanen
 */

// Declarations supplied by the Linux and MIPS headers included by the C source.

unsafe extern "C" {
    fn mips_cpu_irq_init();
}

/// `__init` function: initialize the MIPS CPU interrupt controller.
pub unsafe extern "C" fn arch_init_irq() {
    unsafe {
        mips_cpu_irq_init();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
