/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * (C) Copyright 2005 Tundra Semiconductor Corp.
 * Alex Bounine, <alexandreb at tundra.com>.
 *
 * See file CREDITS for list of people who contributed to this
 * project.
 */

/*
 * definitions for interrupt controller initialization and external interrupt
 * demultiplexing on TSI108EMU/SVB boards.
 */

/*
 * Tsi108 interrupts
 */
pub const TSI108_IRQ_REG_BASE: u32 = 0;

pub const fn tsi108_irq(x: u32) -> u32 {
    TSI108_IRQ_REG_BASE + x
}

pub const TSI108_MAX_VECTORS: u32 = 36 + 4; /* 36 sources + PCI INT demux */
pub const MAX_TASK_PRIO: u32 = 0xF;

pub const TSI108_IRQ_SPURIOUS: u32 = TSI108_MAX_VECTORS;

pub const DEFAULT_PRIO_LVL: u32 = 10; /* initial priority level */

/* Interrupt vectors assignment to external and internal
 * sources of requests. */

/* EXTERNAL INTERRUPT SOURCES */

pub const IRQ_TSI108_EXT_INT0: u32 = tsi108_irq(0); /* External Source at INT[0] */
pub const IRQ_TSI108_EXT_INT1: u32 = tsi108_irq(1); /* External Source at INT[1] */
pub const IRQ_TSI108_EXT_INT2: u32 = tsi108_irq(2); /* External Source at INT[2] */
pub const IRQ_TSI108_EXT_INT3: u32 = tsi108_irq(3); /* External Source at INT[3] */

/* INTERNAL INTERRUPT SOURCES */

pub const IRQ_TSI108_RESERVED0: u32 = tsi108_irq(4); /* Reserved IRQ */
pub const IRQ_TSI108_RESERVED1: u32 = tsi108_irq(5); /* Reserved IRQ */
pub const IRQ_TSI108_RESERVED2: u32 = tsi108_irq(6); /* Reserved IRQ */
pub const IRQ_TSI108_RESERVED3: u32 = tsi108_irq(7); /* Reserved IRQ */
pub const IRQ_TSI108_DMA0: u32 = tsi108_irq(8); /* DMA0 */
pub const IRQ_TSI108_DMA1: u32 = tsi108_irq(9); /* DMA1 */
pub const IRQ_TSI108_DMA2: u32 = tsi108_irq(10); /* DMA2 */
pub const IRQ_TSI108_DMA3: u32 = tsi108_irq(11); /* DMA3 */
pub const IRQ_TSI108_UART0: u32 = tsi108_irq(12); /* UART0 */
pub const IRQ_TSI108_UART1: u32 = tsi108_irq(13); /* UART1 */
pub const IRQ_TSI108_I2C: u32 = tsi108_irq(14); /* I2C */
pub const IRQ_TSI108_GPIO: u32 = tsi108_irq(15); /* GPIO */
pub const IRQ_TSI108_GIGE0: u32 = tsi108_irq(16); /* GIGE0 */
pub const IRQ_TSI108_GIGE1: u32 = tsi108_irq(17); /* GIGE1 */
pub const IRQ_TSI108_RESERVED4: u32 = tsi108_irq(18); /* Reserved IRQ */
pub const IRQ_TSI108_HLP: u32 = tsi108_irq(19); /* HLP */
pub const IRQ_TSI108_SDRAM: u32 = tsi108_irq(20); /* SDC */
pub const IRQ_TSI108_PROC_IF: u32 = tsi108_irq(21); /* Processor IF */
pub const IRQ_TSI108_RESERVED5: u32 = tsi108_irq(22); /* Reserved IRQ */
pub const IRQ_TSI108_PCI: u32 = tsi108_irq(23); /* PCI/X block */

pub const IRQ_TSI108_MBOX0: u32 = tsi108_irq(24); /* Mailbox 0 register */
pub const IRQ_TSI108_MBOX1: u32 = tsi108_irq(25); /* Mailbox 1 register */
pub const IRQ_TSI108_MBOX2: u32 = tsi108_irq(26); /* Mailbox 2 register */
pub const IRQ_TSI108_MBOX3: u32 = tsi108_irq(27); /* Mailbox 3 register */

pub const IRQ_TSI108_DBELL0: u32 = tsi108_irq(28); /* Doorbell 0 */
pub const IRQ_TSI108_DBELL1: u32 = tsi108_irq(29); /* Doorbell 1 */
pub const IRQ_TSI108_DBELL2: u32 = tsi108_irq(30); /* Doorbell 2 */
pub const IRQ_TSI108_DBELL3: u32 = tsi108_irq(31); /* Doorbell 3 */

pub const IRQ_TSI108_TIMER0: u32 = tsi108_irq(32); /* Global Timer 0 */
pub const IRQ_TSI108_TIMER1: u32 = tsi108_irq(33); /* Global Timer 1 */
pub const IRQ_TSI108_TIMER2: u32 = tsi108_irq(34); /* Global Timer 2 */
pub const IRQ_TSI108_TIMER3: u32 = tsi108_irq(35); /* Global Timer 3 */

/*
 * PCI bus INTA# - INTD# lines demultiplexor
 */
pub const IRQ_PCI_INTAD_BASE: u32 = tsi108_irq(36);
pub const IRQ_PCI_INTA: u32 = IRQ_PCI_INTAD_BASE + 0;
pub const IRQ_PCI_INTB: u32 = IRQ_PCI_INTAD_BASE + 1;
pub const IRQ_PCI_INTC: u32 = IRQ_PCI_INTAD_BASE + 2;
pub const IRQ_PCI_INTD: u32 = IRQ_PCI_INTAD_BASE + 3;
pub const NUM_PCI_IRQS: u32 = 4;

/* number of entries in vector dispatch table */
pub const IRQ_TSI108_TAB_SIZE: u32 = TSI108_MAX_VECTORS + 1;

/* Mapping of MPIC outputs to processors' interrupt pins */

pub const IDIR_INT_OUT0: u32 = 0x1;
pub const IDIR_INT_OUT1: u32 = 0x2;
pub const IDIR_INT_OUT2: u32 = 0x4;
pub const IDIR_INT_OUT3: u32 = 0x8;

/*---------------------------------------------------------------
 * IRQ line configuration parameters */

/* Interrupt delivery modes */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TSI108_IRQ_MODE {
    TSI108_IRQ_DIRECTED,
    TSI108_IRQ_DISTRIBUTED,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
