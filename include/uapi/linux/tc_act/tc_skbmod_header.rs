/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Copyright (c) 2016, Jamal Hadi Salim
 */

// Dependency supplied by the corresponding Linux packet classifier definitions.

pub const SKBMOD_F_DMAC: u32 = 0x1;
pub const SKBMOD_F_SMAC: u32 = 0x2;
pub const SKBMOD_F_ETYPE: u32 = 0x4;
pub const SKBMOD_F_SWAPMAC: u32 = 0x8;
pub const SKBMOD_F_ECN: u32 = 0x10;

#[repr(C)]
pub struct tc_skbmod {
    pub tc_gen: tc_gen,
    pub flags: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum tc_skbmod_attr {
    TCA_SKBMOD_UNSPEC = 0,
    TCA_SKBMOD_TM,
    TCA_SKBMOD_PARMS,
    TCA_SKBMOD_DMAC,
    TCA_SKBMOD_SMAC,
    TCA_SKBMOD_ETYPE,
    TCA_SKBMOD_PAD,
    __TCA_SKBMOD_MAX,
}

pub const TCA_SKBMOD_MAX: tc_skbmod_attr =
    tc_skbmod_attr::TCA_SKBMOD_PAD;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
