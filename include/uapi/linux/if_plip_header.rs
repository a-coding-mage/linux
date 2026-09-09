/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 *	NET3	PLIP tuning facilities for the new Niibe PLIP.
 *
 *	This program is free software; you can redistribute it and/or
 *	modify it under the terms of the GNU General Public License
 *	as published by the Free Software Foundation; either version
 *	2 of the License, or (at your option) any later version.
 *
 */

/* Dependency supplied by linux/sockios.h in the original header. */
pub const SIOCDEVPLIP: ::core::ffi::c_ulong = SIOCDEVPRIVATE;

#[repr(C)]
pub struct plipconf {
    pub pcmd: u16,
    pub nibble: ::core::ffi::c_ulong,
    pub trigger: ::core::ffi::c_ulong,
}

pub const PLIP_GET_TIMEOUT: ::core::ffi::c_ulong = 0x1;
pub const PLIP_SET_TIMEOUT: ::core::ffi::c_ulong = 0x2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
