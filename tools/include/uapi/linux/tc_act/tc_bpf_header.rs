/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Copyright (c) 2015 Jiri Pirko <jiri@resnulli.us>
 */

/* Depends on linux/pkt_cls.h for the C tc_gen macro contents. */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tc_act_bpf {
    /* tc_gen; */
}

pub const TCA_ACT_BPF_UNSPEC: u32 = 0;
pub const TCA_ACT_BPF_TM: u32 = 1;
pub const TCA_ACT_BPF_PARMS: u32 = 2;
pub const TCA_ACT_BPF_OPS_LEN: u32 = 3;
pub const TCA_ACT_BPF_OPS: u32 = 4;
pub const TCA_ACT_BPF_FD: u32 = 5;
pub const TCA_ACT_BPF_NAME: u32 = 6;
pub const TCA_ACT_BPF_PAD: u32 = 7;
pub const TCA_ACT_BPF_TAG: u32 = 8;
pub const TCA_ACT_BPF_ID: u32 = 9;
pub const __TCA_ACT_BPF_MAX: u32 = 10;

pub const TCA_ACT_BPF_MAX: u32 = __TCA_ACT_BPF_MAX - 1;
