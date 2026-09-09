// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// Dependencies supplied by the kernel's interrupt and SMP subsystems.

unsafe extern "C" {
    fn irqchip_init();
    fn setup_smp_ipi();
}

/// Kernel initialization entry point (`__init`).
pub unsafe fn init_IRQ() {
    unsafe {
        irqchip_init();
    }

    #[cfg(CONFIG_SMP)]
    unsafe {
        setup_smp_ipi();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
