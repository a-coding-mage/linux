/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Root interrupt controller for the BCM2836 (Raspberry Pi 2).
 *
 * Copyright 2015 Broadcom
 */

pub const LOCAL_CONTROL: u32 = 0x000;
pub const LOCAL_PRESCALER: u32 = 0x008;

/*
 * The low 2 bits identify the CPU that the GPU IRQ goes to, and the
 * next 2 bits identify the CPU that the GPU FIQ goes to.
 */
pub const LOCAL_GPU_ROUTING: u32 = 0x00c;
/* When setting bits 0-3, enables PMU interrupts on that CPU. */
pub const LOCAL_PM_ROUTING_SET: u32 = 0x010;
/* When setting bits 0-3, disables PMU interrupts on that CPU. */
pub const LOCAL_PM_ROUTING_CLR: u32 = 0x014;
/*
 * The low 4 bits of this are the CPU's timer IRQ enables, and the
 * next 4 bits are the CPU's timer FIQ enables (which override the IRQ
 * bits).
 */
pub const LOCAL_TIMER_INT_CONTROL0: u32 = 0x040;
/*
 * The low 4 bits of this are the CPU's per-mailbox IRQ enables, and
 * the next 4 bits are the CPU's per-mailbox FIQ enables (which
 * override the IRQ bits).
 */
pub const LOCAL_MAILBOX_INT_CONTROL0: u32 = 0x050;
/*
 * The CPU's interrupt status register.  Bits are defined by the
 * LOCAL_IRQ_* bits below.
 */
pub const LOCAL_IRQ_PENDING0: u32 = 0x060;
/* Same status bits as above, but for FIQ. */
pub const LOCAL_FIQ_PENDING0: u32 = 0x070;
/*
 * Mailbox write-to-set bits.  There are 16 mailboxes, 4 per CPU, and
 * these bits are organized by mailbox number and then CPU number.  We
 * use mailbox 0 for IPIs.  The mailbox's interrupt is raised while
 * any bit is set.
 */
pub const LOCAL_MAILBOX0_SET0: u32 = 0x080;
pub const LOCAL_MAILBOX3_SET0: u32 = 0x08c;
/* Mailbox write-to-clear bits. */
pub const LOCAL_MAILBOX0_CLR0: u32 = 0x0c0;
pub const LOCAL_MAILBOX3_CLR0: u32 = 0x0cc;

pub const LOCAL_IRQ_CNTPSIRQ: u32 = 0;
pub const LOCAL_IRQ_CNTPNSIRQ: u32 = 1;
pub const LOCAL_IRQ_CNTHPIRQ: u32 = 2;
pub const LOCAL_IRQ_CNTVIRQ: u32 = 3;
pub const LOCAL_IRQ_MAILBOX0: u32 = 4;
pub const LOCAL_IRQ_MAILBOX1: u32 = 5;
pub const LOCAL_IRQ_MAILBOX2: u32 = 6;
pub const LOCAL_IRQ_MAILBOX3: u32 = 7;
pub const LOCAL_IRQ_GPU_FAST: u32 = 8;
pub const LOCAL_IRQ_PMU_FAST: u32 = 9;
pub const LAST_IRQ: u32 = LOCAL_IRQ_PMU_FAST;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
