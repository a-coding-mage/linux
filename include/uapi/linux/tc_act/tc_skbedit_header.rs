/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (c) 2008, Intel Corporation.
 *
 * Author: Alexander Duyck <alexander.h.duyck@intel.com>
 */

// Dependency: tc_gen is supplied by <linux/pkt_cls.h>.

pub const SKBEDIT_F_PRIORITY: u32 = 0x1;
pub const SKBEDIT_F_QUEUE_MAPPING: u32 = 0x2;
pub const SKBEDIT_F_MARK: u32 = 0x4;
pub const SKBEDIT_F_PTYPE: u32 = 0x8;
pub const SKBEDIT_F_MASK: u32 = 0x10;
pub const SKBEDIT_F_INHERITDSFIELD: u32 = 0x20;
pub const SKBEDIT_F_TXQ_SKBHASH: u32 = 0x40;

#[repr(C)]
pub struct tc_skbedit {
    pub tc_gen: tc_gen,
}

pub const TCA_SKBEDIT_UNSPEC: u32 = 0;
pub const TCA_SKBEDIT_TM: u32 = 1;
pub const TCA_SKBEDIT_PARMS: u32 = 2;
pub const TCA_SKBEDIT_PRIORITY: u32 = 3;
pub const TCA_SKBEDIT_QUEUE_MAPPING: u32 = 4;
pub const TCA_SKBEDIT_MARK: u32 = 5;
pub const TCA_SKBEDIT_PAD: u32 = 6;
pub const TCA_SKBEDIT_PTYPE: u32 = 7;
pub const TCA_SKBEDIT_MASK: u32 = 8;
pub const TCA_SKBEDIT_FLAGS: u32 = 9;
pub const TCA_SKBEDIT_QUEUE_MAPPING_MAX: u32 = 10;
pub const __TCA_SKBEDIT_MAX: u32 = 11;
pub const TCA_SKBEDIT_MAX: u32 = __TCA_SKBEDIT_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
