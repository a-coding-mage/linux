/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Constant for device tree bindings for Turris Mox module configuration bus
 *
 * Copyright (C) 2019 Marek Behún <kabel@kernel.org>
 */

pub const MOXTET_IRQ_PCI: i32 = 0;
pub const MOXTET_IRQ_USB3: i32 = 4;
pub const MOXTET_IRQ_TOPAZ: i32 = 12;

pub const fn moxtet_irq_peridot(n: i32) -> i32 {
    8 + n
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
