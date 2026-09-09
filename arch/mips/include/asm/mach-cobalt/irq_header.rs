/*
 * Cobalt IRQ definitions.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1997 Cobalt Microserver
 * Copyright (C) 1997, 2003 Ralf Baechle
 * Copyright (C) 2001-2003 Liam Davies (ldavies@agile.tv)
 * Copyright (C) 2007 Yoichi Yuasa <yuasa@linux-mips.org>
 */

/*
 * i8259 interrupts used on Cobalt:
 *
 *	8  - RTC
 *	9  - PCI slot
 *	14 - IDE0
 *	15 - IDE1(no connector on board)
 */
pub const I8259A_IRQ_BASE: i32 = 0;

pub const PCISLOT_IRQ: i32 = I8259A_IRQ_BASE + 9;

/*
 * CPU interrupts used on Cobalt:
 *
 *	0 - Software interrupt 0 (unused)
 *	1 - Software interrupt 0 (unused)
 *	2 - cascade GT64111
 *	3 - ethernet or SCSI host controller
 *	4 - ethernet
 *	5 - 16550 UART
 *	6 - cascade i8259
 *	7 - CP0 counter
 */
pub const MIPS_CPU_IRQ_BASE: i32 = 16;

pub const GT641XX_CASCADE_IRQ: i32 = MIPS_CPU_IRQ_BASE + 2;
pub const RAQ2_SCSI_IRQ: i32 = MIPS_CPU_IRQ_BASE + 3;
pub const ETH0_IRQ: i32 = MIPS_CPU_IRQ_BASE + 3;
pub const QUBE1_ETH0_IRQ: i32 = MIPS_CPU_IRQ_BASE + 4;
pub const ETH1_IRQ: i32 = MIPS_CPU_IRQ_BASE + 4;
pub const SERIAL_IRQ: i32 = MIPS_CPU_IRQ_BASE + 5;
pub const SCSI_IRQ: i32 = MIPS_CPU_IRQ_BASE + 5;
pub const I8259_CASCADE_IRQ: i32 = MIPS_CPU_IRQ_BASE + 6;

pub const GT641XX_IRQ_BASE: i32 = 24;

/* Dependency supplied by the corresponding asm/irq_gt641xx.h translation. */
pub const NR_IRQS: i32 = GT641XX_PCI_INT3_IRQ + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
