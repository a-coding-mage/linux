/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding kernel headers:
// <net/act_api.h>, <linux/tc_act/tc_sample.h>, and <net/psample.h>

#[repr(C)]
pub struct tcf_sample {
    pub common: tc_action,
    pub rate: u32,
    pub truncate: bool,
    pub trunc_size: u32,
    pub psample_group: *mut psample_group,
    pub psample_group_num: u32,
    pub tcfm_list: list_head,
}

#[inline]
pub unsafe fn to_sample(a: *mut tc_action) -> *mut tcf_sample {
    a as *mut tcf_sample
}

#[inline]
pub unsafe fn tcf_sample_rate(a: &tc_action) -> u32 {
    (*to_sample(a as *const tc_action as *mut tc_action)).rate
}

#[inline]
pub unsafe fn tcf_sample_truncate(a: &tc_action) -> bool {
    (*to_sample(a as *const tc_action as *mut tc_action)).truncate
}

#[inline]
pub unsafe fn tcf_sample_trunc_size(a: &tc_action) -> i32 {
    (*to_sample(a as *const tc_action as *mut tc_action)).trunc_size as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
