/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 *	mpls tunnel api
 *
 *	Authors:
 *		Roopa Prabhu <roopa@cumulusnetworks.com>
 *
 *	This program is free software; you can redistribute it and/or
 *	modify it under the terms of the GNU General Public License
 *	as published by the Free Software Foundation; either version
 *
 *	of the License, or (at your option) any later version.
 */

/* MPLS tunnel attributes
 * [RTA_ENCAP] = {
 *     [MPLS_IPTUNNEL_DST]
 *     [MPLS_IPTUNNEL_TTL]
 * }
 */
pub const MPLS_IPTUNNEL_UNSPEC: i32 = 0;
pub const MPLS_IPTUNNEL_DST: i32 = 1;
pub const MPLS_IPTUNNEL_TTL: i32 = 2;
pub const __MPLS_IPTUNNEL_MAX: i32 = 3;
pub const MPLS_IPTUNNEL_MAX: i32 = __MPLS_IPTUNNEL_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
