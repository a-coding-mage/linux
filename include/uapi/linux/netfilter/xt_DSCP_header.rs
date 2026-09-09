/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* x_tables module for setting the IPv4/IPv6 DSCP field
 *
 * (C) 2002 Harald Welte <laforge@gnumonks.org>
 * based on ipt_FTOS.c (C) 2000 by Matthew G. Marsh <mgm@paktronix.com>
 * This software is distributed under GNU GPL v2, 1991
 *
 * See RFC2474 for a description of the DSCP field within the IP Header.
 *
 * xt_DSCP.h,v 1.7 2002/03/14 12:03:13 laforge Exp
 */

/* Dependency equivalent of <linux/netfilter/xt_dscp.h>. */
/* Dependency equivalent of <linux/types.h>. */

/* target info */
#[repr(C)]
pub struct xt_DSCP_info {
    pub dscp: __u8,
}

#[repr(C)]
pub struct xt_tos_target_info {
    pub tos_value: __u8,
    pub tos_mask: __u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
