/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Copyright (C) 2008-2010 Gabor Juhos <juhosg@openwrt.org>
 *  Copyright (C) 2008 Imre Kaloz <kaloz@openwrt.org>
 */

pub const MIPS_CPU_IRQ_BASE: i32 = 0;
pub const NR_IRQS: i32 = 51;

pub const fn ATH79_CPU_IRQ(x: i32) -> i32 {
    MIPS_CPU_IRQ_BASE + x
}

pub const ATH79_MISC_IRQ_BASE: i32 = 8;
pub const ATH79_MISC_IRQ_COUNT: i32 = 32;
pub const fn ATH79_MISC_IRQ(x: i32) -> i32 {
    ATH79_MISC_IRQ_BASE + x
}

pub const ATH79_PCI_IRQ_BASE: i32 = ATH79_MISC_IRQ_BASE + ATH79_MISC_IRQ_COUNT;
pub const ATH79_PCI_IRQ_COUNT: i32 = 6;
pub const fn ATH79_PCI_IRQ(x: i32) -> i32 {
    ATH79_PCI_IRQ_BASE + x
}

pub const ATH79_IP2_IRQ_BASE: i32 = ATH79_PCI_IRQ_BASE + ATH79_PCI_IRQ_COUNT;
pub const ATH79_IP2_IRQ_COUNT: i32 = 2;
pub const fn ATH79_IP2_IRQ(x: i32) -> i32 {
    ATH79_IP2_IRQ_BASE + x
}

pub const ATH79_IP3_IRQ_BASE: i32 = ATH79_IP2_IRQ_BASE + ATH79_IP2_IRQ_COUNT;
pub const ATH79_IP3_IRQ_COUNT: i32 = 3;
pub const fn ATH79_IP3_IRQ(x: i32) -> i32 {
    ATH79_IP3_IRQ_BASE + x
}

// Dependency provided by <asm/mach-generic/irq.h> in the original header.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
