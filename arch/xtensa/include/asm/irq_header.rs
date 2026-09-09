/*
 * include/asm-xtensa/irq.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

// Dependency intent: the original header includes <linux/init.h> and
// <asm/core.h>; their supplied Rust symbols are referenced below.

// Build-time condition preserved from CONFIG_PLATFORM_NR_IRQS.
// When CONFIG_PLATFORM_NR_IRQS is defined by the surrounding build, this
// value should be replaced with that configuration value.
pub const PLATFORM_NR_IRQS: u32 = 0;
pub const XTENSA_NR_IRQS: u32 = XCHAL_NUM_INTERRUPTS;
pub const NR_IRQS: u32 = XTENSA_NR_IRQS + PLATFORM_NR_IRQS + 1;

#[inline]
pub const fn XTENSA_PIC_LINUX_IRQ(hwirq: u32) -> u32 {
    hwirq + 1
}

#[inline]
pub const fn irq_canonicalize(irq: i32) -> i32 {
    irq
}

#[repr(C)]
pub struct irqaction {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_domain {
    _private: [u8; 0],
}

extern "C" {
    pub fn migrate_irqs();
    pub fn xtensa_irq_domain_xlate(
        intspec: *const u32,
        intsize: u32,
        int_irq: usize,
        ext_irq: usize,
        out_hwirq: *mut usize,
        out_type: *mut u32,
    ) -> i32;
    pub fn xtensa_irq_map(d: *mut irq_domain, irq: u32, hw: irq_hw_number_t) -> i32;
    pub fn xtensa_map_ext_irq(ext_irq: u32) -> u32;
    pub fn xtensa_get_ext_irq_no(irq: u32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
