/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 by Ralf Baechle (ralf@linux-mips.org)
 */

// Dependency equivalent of: #include <linux/compiler.h>

#[cfg(CONFIG_SYNC_R4K)]
unsafe extern "C" {
    pub fn synchronise_count_slave(cpu: ::core::ffi::c_int);
}

#[cfg(not(CONFIG_SYNC_R4K))]
#[inline]
pub fn synchronise_count_slave(_cpu: ::core::ffi::c_int) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
