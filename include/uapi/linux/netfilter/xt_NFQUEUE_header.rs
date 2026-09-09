/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* iptables module for using NFQUEUE mechanism
 *
 * (C) 2005 Harald Welte <laforge@netfilter.org>
 *
 * This software is distributed under GNU GPL v2, 1991
 *
 */

// C header guard: _XT_NFQ_TARGET_H

/* target info */
#[repr(C)]
pub struct xt_NFQ_info {
    pub queuenum: u16,
}

#[repr(C)]
pub struct xt_NFQ_info_v1 {
    pub queuenum: u16,
    pub queues_total: u16,
}

#[repr(C)]
pub struct xt_NFQ_info_v2 {
    pub queuenum: u16,
    pub queues_total: u16,
    pub bypass: u16,
}

#[repr(C)]
pub struct xt_NFQ_info_v3 {
    pub queuenum: u16,
    pub queues_total: u16,
    pub flags: u16,
}

pub const NFQ_FLAG_BYPASS: u16 = 0x01; // for compatibility with v2
pub const NFQ_FLAG_CPU_FANOUT: u16 = 0x02; // use current CPU (no hashing)
pub const NFQ_FLAG_MASK: u16 = 0x03;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
