/* SPDX-License-Identifier: GPL-2.0 */

// Declarations supplied by asm-generic/delay.h and linux/init.h are external
// dependencies of this header.

pub unsafe extern "C" fn use_tsc_delay();
pub unsafe extern "C" fn use_tpause_delay();
pub unsafe extern "C" fn use_mwaitx_delay();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
