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

/* FIXME: How can we replace these with values from the CPU...
 * they shouldn't be hard-coded!
 */

/* C macro: __ro_after_init expands to __read_mostly. */

pub const L1_CACHE_BYTES: usize = 16;
pub const L1_CACHE_SHIFT: usize = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
