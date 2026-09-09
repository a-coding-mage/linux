// SPDX-License-Identifier: GPL-2.0-only
/*
 * Joshua Henderson <joshua.henderson@microchip.com>
 * Copyright (C) 2015 Microchip Technology Inc.  All rights reserved.
 */

// Dependencies corresponding to <linux/init.h>, <linux/irqchip.h>, and
// <asm/irq.h> are supplied externally.

extern "C" {
    fn irqchip_init();
}

/// Equivalent to the Linux `__init`-annotated architecture IRQ initializer.
pub unsafe fn arch_init_irq() {
    irqchip_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
