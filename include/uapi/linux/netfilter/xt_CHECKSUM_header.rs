/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Header file for iptables ipt_CHECKSUM target
 *
 * (C) 2002 by Harald Welte <laforge@gnumonks.org>
 * (C) 2010 Red Hat Inc
 * Author: Michael S. Tsirkin <mst@redhat.com>
 *
 * This software is distributed under GNU GPL v2, 1991
 */

// Dependency supplied by the surrounding Linux type definitions.

pub const XT_CHECKSUM_OP_FILL: u8 = 0x01; /* fill in checksum in IP header */

#[repr(C)]
pub struct xt_CHECKSUM_info {
    pub operation: u8, /* bitset of operations */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
