/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1999, 2000, 01, 02, 03 by Ralf Baechle
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 * Copyright (C) 2001 Kanoj Sarcar
 */

// Dependency supplied by the generic IRQ definitions:
// #include <asm/mach-generic/irq.h>

pub const NR_IRQS: usize = 256;

pub const IP27_HUB_PEND0_IRQ: usize = MIPS_CPU_IRQ_BASE + 2;
pub const IP27_HUB_PEND1_IRQ: usize = MIPS_CPU_IRQ_BASE + 3;
pub const IP27_RT_TIMER_IRQ: usize = MIPS_CPU_IRQ_BASE + 4;

pub const IP27_HUB_IRQ_BASE: usize = MIPS_CPU_IRQ_BASE + 8;
pub const IP27_HUB_IRQ_COUNT: usize = 128;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
