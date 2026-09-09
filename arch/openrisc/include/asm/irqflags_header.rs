/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * OpenRISC implementation:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 * et al.
 */

// Dependency supplied by asm/spr_defs.h in the source header.
// The generic irqflags declarations are supplied externally.

pub const ARCH_IRQ_DISABLED: u32 = 0x00;
pub const ARCH_IRQ_ENABLED: u32 = SPR_SR_IEE | SPR_SR_TEE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
