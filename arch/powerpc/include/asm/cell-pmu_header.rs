/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Cell Broadband Engine Performance Monitor
 *
 * (C) Copyright IBM Corporation 2006
 *
 * Author:
 *   David Erb (djerb@us.ibm.com)
 *   Kevin Corry (kevcorry@us.ibm.com)
 */

/* The Cell PMU has four hardware performance counters, which can be
 * configured as four 32-bit counters or eight 16-bit counters.
 */
pub const NR_PHYS_CTRS: i32 = 4;
pub const NR_CTRS: i32 = NR_PHYS_CTRS * 2;

/* Macros for the pm_control register. */
#[inline]
pub const fn CBE_PM_16BIT_CTR(ctr: i32) -> i32 {
	1 << (24 - (ctr & (NR_PHYS_CTRS - 1)))
}

/* Macros for the trace_address register. */
pub const CBE_PM_TRACE_BUF_EMPTY: i32 = 0x00000400;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum pm_reg_name {
	group_control,
	debug_bus_control,
	trace_address,
	ext_tr_timer,
	pm_status,
	pm_control,
	pm_interval,
	pm_start_stop,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
