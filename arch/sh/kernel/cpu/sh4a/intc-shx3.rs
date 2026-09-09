// SPDX-License-Identifier: GPL-2.0
/*
 * Shared support for SH-X3 interrupt controllers.
 *
 *  Copyright (C) 2009 - 2010  Paul Mundt
 */

const INTACK: usize = 0xfe4100b8;
const INTACKCLR: usize = 0xfe4100bc;
const INTC_USERIMASK: usize = 0xfe411000;

extern "C" {
    fn __raw_readl(addr: usize) -> u32;
    fn __raw_writel(value: u32, addr: usize);
    fn irq2evt(irq: u32) -> u32;
    fn register_intc_userimask(addr: usize) -> i32;
}

// Preserved from the C build-time condition: CONFIG_INTC_BALANCING.
#[cfg(CONFIG_INTC_BALANCING)]
pub unsafe fn irq_lookup(irq: u32) -> u32 {
    if __raw_readl(INTACK) & 1 != 0 {
        irq
    } else {
        NO_IRQ_IGNORE
    }
}

// Preserved from the C build-time condition: CONFIG_INTC_BALANCING.
#[cfg(CONFIG_INTC_BALANCING)]
pub unsafe fn irq_finish(irq: u32) {
    __raw_writel(irq2evt(irq), INTACKCLR);
}

unsafe fn shx3_irq_setup() -> i32 {
    register_intc_userimask(INTC_USERIMASK)
}

// arch_initcall(shx3_irq_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
