/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Equalizer Load-balancer for serial network interfaces.
 *
 * (c) Copyright 1995 Simon "Guru Aleph-Null" Janes
 * NCM: Network and Communications Management, Inc.
 *
 *
 *	This software may be used and distributed according to the terms
 *	of the GNU General Public License, incorporated herein by reference.
 * 
 * The author may be reached as simon@ncm.com, or C/O
 *    NCM
 *    Attn: Simon Janes
 *    6803 Whittier Ave
 *    McLean VA 22101
 *    Phone: 1-703-847-0040 ext 103
 */

use core::ffi::{c_char, c_int, c_long, c_ulong};

// HZ and SIOCDEVPRIVATE are supplied by the surrounding Linux UAPI bindings.
pub const EQL_DEFAULT_SLAVE_PRIORITY: c_int = 28800;
pub const EQL_DEFAULT_MAX_SLAVES: c_int = 4;
pub const EQL_DEFAULT_MTU: c_int = 576;
pub const EQL_DEFAULT_RESCHED_IVAL: c_int = HZ;

pub const EQL_ENSLAVE: c_ulong = SIOCDEVPRIVATE;
pub const EQL_EMANCIPATE: c_ulong = SIOCDEVPRIVATE + 1;

pub const EQL_GETSLAVECFG: c_ulong = SIOCDEVPRIVATE + 2;
pub const EQL_SETSLAVECFG: c_ulong = SIOCDEVPRIVATE + 3;

pub const EQL_GETMASTRCFG: c_ulong = SIOCDEVPRIVATE + 4;
pub const EQL_SETMASTRCFG: c_ulong = SIOCDEVPRIVATE + 5;

#[repr(C)]
pub struct master_config {
    pub master_name: [c_char; 16],
    pub max_slaves: c_int,
    pub min_slaves: c_int,
}

pub type master_config_t = master_config;

#[repr(C)]
pub struct slave_config {
    pub slave_name: [c_char; 16],
    pub priority: c_long,
}

pub type slave_config_t = slave_config;

#[repr(C)]
pub struct slaving_request {
    pub slave_name: [c_char; 16],
    pub priority: c_long,
}

pub type slaving_request_t = slaving_request;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
