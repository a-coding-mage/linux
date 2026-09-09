/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

/*
 * ARCv2 can support 240 interrupts in the core interrupts controllers and
 * 128 interrupts in IDU. Thus 512 virtual IRQs must be enough for most
 * configurations of boards.
 * This doesn't affect ARCompact, but we change it to same value
 */
pub const NR_IRQS: u32 = 512;

/* Platform Independent IRQs */
#[cfg(CONFIG_ISA_ARCV2)]
pub const IPI_IRQ: u32 = 19;

#[cfg(CONFIG_ISA_ARCV2)]
pub const SOFTIRQ_IRQ: u32 = 21;

#[cfg(CONFIG_ISA_ARCV2)]
pub const FIRST_EXT_IRQ: u32 = 24;

/* Dependencies supplied by the surrounding kernel translation. */
pub struct pt_regs;

extern "C" {
    pub fn arc_init_IRQ();
    pub fn arch_do_IRQ(irq: u32, regs: *mut pt_regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
