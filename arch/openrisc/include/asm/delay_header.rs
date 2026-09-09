/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * OpenRISC implementation:
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 */

// Dependency equivalent of: #include <asm-generic/delay.h>

pub static mut loops_per_jiffy: core::ffi::c_ulong;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
