/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Copyright (c) 2015 Jiri Pirko <jiri@resnulli.us>
 */

// Dependency intent: `tc_gen` is supplied by the translated linux/pkt_cls.h.

#[repr(C)]
pub struct tc_act_bpf {
    pub tc_gen: tc_gen,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TcaActBpf {
    TCA_ACT_BPF_UNSPEC,
    TCA_ACT_BPF_TM,
    TCA_ACT_BPF_PARMS,
    TCA_ACT_BPF_OPS_LEN,
    TCA_ACT_BPF_OPS,
    TCA_ACT_BPF_FD,
    TCA_ACT_BPF_NAME,
    TCA_ACT_BPF_PAD,
    TCA_ACT_BPF_TAG,
    TCA_ACT_BPF_ID,
    __TCA_ACT_BPF_MAX,
}

pub const TCA_ACT_BPF_MAX: i32 = TcaActBpf::__TCA_ACT_BPF_MAX as i32 - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
