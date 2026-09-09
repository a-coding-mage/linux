/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/dreamcast/sysasic.h
 *
 * Definitions for the Dreamcast System ASIC and related peripherals.
 *
 * Copyright (c) 2001 M. R. Brown <mrbrown@linuxdc.org>
 * Copyright (C) 2003 Paul Mundt <lethal@linux-sh.org>
 *
 * This file is part of the LinuxDC project (www.linuxdc.org)
 */

// Dependency: <asm/irq.h>

/* Hardware events -

   Each of these events correspond to a bit within the Event Mask Registers/
   Event Status Registers.  Because of the virtual IRQ numbering scheme, a
   base offset must be used when calculating the virtual IRQ that each event
   takes.
*/

pub const HW_EVENT_IRQ_BASE: i32 = 48 + 16;

/* IRQ 13 */
pub const HW_EVENT_VSYNC: i32 = HW_EVENT_IRQ_BASE + 5; // VSync
pub const HW_EVENT_MAPLE_DMA: i32 = HW_EVENT_IRQ_BASE + 12; // Maple DMA complete
pub const HW_EVENT_GDROM_DMA: i32 = HW_EVENT_IRQ_BASE + 14; // GD-ROM DMA complete
pub const HW_EVENT_G2_DMA: i32 = HW_EVENT_IRQ_BASE + 15; // G2 DMA complete
pub const HW_EVENT_PVR2_DMA: i32 = HW_EVENT_IRQ_BASE + 19; // PVR2 DMA complete

/* IRQ 11 */
pub const HW_EVENT_GDROM_CMD: i32 = HW_EVENT_IRQ_BASE + 32; // GD-ROM cmd. complete
pub const HW_EVENT_AICA_SYS: i32 = HW_EVENT_IRQ_BASE + 33; // AICA-related
pub const HW_EVENT_EXTERNAL: i32 = HW_EVENT_IRQ_BASE + 35; // Ext. (expansion)

pub const HW_EVENT_IRQ_MAX: i32 = HW_EVENT_IRQ_BASE + 95;

/* arch/sh/boards/mach-dreamcast/irq.c */
extern "C" {
    pub fn systemasic_irq_demux(irq: core::ffi::c_int) -> core::ffi::c_int;
    pub fn systemasic_irq_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
