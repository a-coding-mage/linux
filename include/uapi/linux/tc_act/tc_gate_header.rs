/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Copyright 2020 NXP */

// Dependency: tc_gen is supplied by linux/pkt_cls.h.

#[repr(C)]
pub struct tc_gate {
    pub tc_gen: tc_gen,
}

#[repr(i32)]
pub enum tc_gate_entry_attr {
    TCA_GATE_ENTRY_UNSPEC,
    TCA_GATE_ENTRY_INDEX,
    TCA_GATE_ENTRY_GATE,
    TCA_GATE_ENTRY_INTERVAL,
    TCA_GATE_ENTRY_IPV,
    TCA_GATE_ENTRY_MAX_OCTETS,
    __TCA_GATE_ENTRY_MAX,
}

pub const TCA_GATE_ENTRY_MAX: i32 = __TCA_GATE_ENTRY_MAX as i32 - 1;

#[repr(i32)]
pub enum tc_gate_one_entry_attr {
    TCA_GATE_ONE_ENTRY_UNSPEC,
    TCA_GATE_ONE_ENTRY,
    __TCA_GATE_ONE_ENTRY_MAX,
}

pub const TCA_GATE_ONE_ENTRY_MAX: i32 = __TCA_GATE_ONE_ENTRY_MAX as i32 - 1;

#[repr(i32)]
pub enum tc_gate_attr {
    TCA_GATE_UNSPEC,
    TCA_GATE_TM,
    TCA_GATE_PARMS,
    TCA_GATE_PAD,
    TCA_GATE_PRIORITY,
    TCA_GATE_ENTRY_LIST,
    TCA_GATE_BASE_TIME,
    TCA_GATE_CYCLE_TIME,
    TCA_GATE_CYCLE_TIME_EXT,
    TCA_GATE_FLAGS,
    TCA_GATE_CLOCKID,
    __TCA_GATE_MAX,
}

pub const TCA_GATE_MAX: i32 = __TCA_GATE_MAX as i32 - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
