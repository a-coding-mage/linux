/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2004 Fujitsu Siemens Computers GmbH
 * Author: Bodo Stroesser <bstroesser@siemens.com>
 */

/* For compilation on a host that doesn't support AT_SYSINFO (Linux 2.4) */

// These preprocessor definitions are represented as Rust constants. If the
// surrounding build supplies equivalent definitions, those definitions should
// take precedence in the consuming translation unit.
#[allow(dead_code)]
pub const AT_SYSINFO: i32 = 32;

#[allow(dead_code)]
pub const AT_SYSINFO_EHDR: i32 = 33;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
