/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* x_tables module for matching the IPv4/IPv6 DSCP field
 *
 * (C) 2002 Harald Welte <laforge@gnumonks.org>
 * This software is distributed under GNU GPL v2, 1991
 *
 * See RFC2474 for a description of the DSCP field within the IP Header.
 *
 * xt_dscp.h,v 1.3 2002/08/05 19:00:21 laforge Exp
 */

// Dependency intent: __u8 is supplied by the Linux types bindings.

pub const XT_DSCP_MASK: u8 = 0xfc; // 11111100
pub const XT_DSCP_SHIFT: u32 = 2;
pub const XT_DSCP_MAX: u8 = 0x3f; // 00111111

/* match info */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_dscp_info {
    pub dscp: u8,
    pub invert: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_tos_match_info {
    pub tos_mask: u8,
    pub tos_value: u8,
    pub invert: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
