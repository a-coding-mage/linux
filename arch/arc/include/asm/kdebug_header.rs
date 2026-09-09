/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

#[repr(C)]
pub enum die_val {
	DIE_UNUSED,
	DIE_TRAP,
	DIE_IERR,
	DIE_OOPS,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
