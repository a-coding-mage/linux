/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *	Swansea University Computer Society	NET3
 *	
 *	This file declares the constants of special use with the SLIP/CSLIP/
 *	KISS TNC driver.
 */

// C header guard: _UAPI__LINUX_SLIP_H

pub const SL_MODE_SLIP: i32 = 0;
pub const SL_MODE_CSLIP: i32 = 1;
pub const SL_MODE_KISS: i32 = 4;

pub const SL_OPT_SIXBIT: i32 = 2;
pub const SL_OPT_ADAPTIVE: i32 = 8;

/*
 *	VSV = ioctl for keepalive & outfill in SLIP driver
 */

pub const SIOCSKEEPALIVE: i32 = SIOCDEVPRIVATE; // Set keepalive timeout in sec
pub const SIOCGKEEPALIVE: i32 = SIOCDEVPRIVATE + 1; // Get keepalive timeout
pub const SIOCSOUTFILL: i32 = SIOCDEVPRIVATE + 2; // Set outfill timeout
pub const SIOCGOUTFILL: i32 = SIOCDEVPRIVATE + 3; // Get outfill timeout
pub const SIOCSLEASE: i32 = SIOCDEVPRIVATE + 4; // Set "leased" line type
pub const SIOCGLEASE: i32 = SIOCDEVPRIVATE + 5; // Get line type

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
