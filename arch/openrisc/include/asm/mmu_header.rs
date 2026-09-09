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

// C header guard: __ASM_OPENRISC_MMU_H

// C condition: this declaration is omitted when assembling.
#[cfg(not(assembler))]
pub type mm_context_t = usize;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
