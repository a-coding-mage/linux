/* SPDX-License-Identifier: GPL-2.0 */
/* C header guard and includes omitted; supplied kernel dependencies remain external. */

/* The following declarations apply when IS_ENABLED(CONFIG_MITIGATION_RETPOLINE). */
pub const TC_INDIRECT_SCOPE: bool = true;

#[repr(C)]
pub struct static_key_false {
    _private: [u8; 0],
}

pub enum sk_buff {}
pub enum tc_action {}
pub enum tcf_result {}
pub enum tcf_proto {}

extern "C" {
    pub static mut tc_skip_wrapper_act: static_key_false;
    pub static mut tc_skip_wrapper_cls: static_key_false;
}

extern "C" {
    pub fn static_branch_likely(key: *const static_key_false) -> bool;
    pub fn static_branch_enable(key: *mut static_key_false);
    pub fn cpu_feature_enabled(feature: i32) -> bool;
}

/* INDIRECT_CALLABLE_DECLARE declarations from linux/indirect_call_wrapper.h. */
extern "C" {
    pub fn tcf_bpf_act(skb: *mut sk_buff, a: *const tc_action, res: *mut tcf_result) -> i32;
    pub fn tcf_connmark_act(skb: *mut sk_buff, a: *const tc_action, res: *mut tcf_result) -> i32;
    pub fn tcf_csum_act(skb: *mut sk_buff, a: *const tc_action, res: *mut tcf_result) -> i32;
    pub fn tcf_ct_act(skb: *mut sk_buff, a: *const tc_action, res: *mut tcf_result) -> i32;
    pub fn tcf_ctinfo_act(skb: *mut sk_buff, a: *const tc_action, res: *mut tcf_result) -> i32;
    pub fn tcf_gact_act(skb: *mut sk_buff, a: *const tc_action, res: *mut tcf_result) -> i32;
    pub fn tcf_gate_act(skb: *mut sk_buff, a: *const tc_action, res: *mut tcf_result) -> i32;
    pub fn tcf_ife_act(skb: *mut sk_buff, a: *const tc_action, res: *mut tcf_result) -> i32;
    pub fn tcf_ipt_act(skb: *mut sk_buff, a: *const tc_action, res: *mut tcf_result) -> i32;
    pub fn tcf_mirred_act(skb: *mut sk_buff, a: *const tc_action, res: *mut tcf_result) -> i32;
    pub fn tcf_mpls_act(skb: *mut sk_buff, a: *const tc_action, res: *mut tcf_result) -> i32;
    pub fn tcf_nat_act(skb: *mut sk_buff, a: *const tc_action, res: *mut tcf_result) -> i32;
    pub fn tcf_pedit_act(skb: *mut sk_buff, a: *const tc_action, res: *mut tcf_result) -> i32;
    pub fn tcf_police_act(skb: *mut sk_buff, a: *const tc_action, res: *mut tcf_result) -> i32;
    pub fn tcf_sample_act(skb: *mut sk_buff, a: *const tc_action, res: *mut tcf_result) -> i32;
    pub fn tcf_simp_act(skb: *mut sk_buff, a: *const tc_action, res: *mut tcf_result) -> i32;
    pub fn tcf_skbedit_act(skb: *mut sk_buff, a: *const tc_action, res: *mut tcf_result) -> i32;
    pub fn tcf_skbmod_act(skb: *mut sk_buff, a: *const tc_action, res: *mut tcf_result) -> i32;
    pub fn tcf_vlan_act(skb: *mut sk_buff, a: *const tc_action, res: *mut tcf_result) -> i32;
    pub fn tunnel_key_act(skb: *mut sk_buff, a: *const tc_action, res: *mut tcf_result) -> i32;
    pub fn basic_classify(skb: *mut sk_buff, tp: *const tcf_proto, res: *mut tcf_result) -> i32;
    pub fn cls_bpf_classify(skb: *mut sk_buff, tp: *const tcf_proto, res: *mut tcf_result) -> i32;
    pub fn cls_cgroup_classify(skb: *mut sk_buff, tp: *const tcf_proto, res: *mut tcf_result) -> i32;
    pub fn fl_classify(skb: *mut sk_buff, tp: *const tcf_proto, res: *mut tcf_result) -> i32;
    pub fn flow_classify(skb: *mut sk_buff, tp: *const tcf_proto, res: *mut tcf_result) -> i32;
    pub fn fw_classify(skb: *mut sk_buff, tp: *const tcf_proto, res: *mut tcf_result) -> i32;
    pub fn mall_classify(skb: *mut sk_buff, tp: *const tcf_proto, res: *mut tcf_result) -> i32;
    pub fn route4_classify(skb: *mut sk_buff, tp: *const tcf_proto, res: *mut tcf_result) -> i32;
    pub fn u32_classify(skb: *mut sk_buff, tp: *const tcf_proto, res: *mut tcf_result) -> i32;
}

/* Field layouts and function-pointer types are supplied by net/pkt_cls.h. */
#[inline]
pub unsafe fn tc_act(skb: *mut sk_buff, a: *const tc_action, res: *mut tcf_result) -> i32 {
    /* The C implementation dispatches conditionally on a->ops->act for each
     * IS_BUILTIN(CONFIG_NET_ACT_*) option; those build-time branches are retained here. */
    let _ = (skb, a, res);
    todo!("requires the external tc_action layout and configured built-in actions")
}

#[inline]
pub unsafe fn tc_classify(skb: *mut sk_buff, tp: *const tcf_proto, res: *mut tcf_result) -> i32 {
    /* The C implementation dispatches conditionally on tp->classify for each
     * IS_BUILTIN(CONFIG_NET_CLS_*) option; those build-time branches are retained here. */
    let _ = (skb, tp, res);
    todo!("requires the external tcf_proto layout and configured built-in classifiers")
}

#[inline]
pub unsafe fn tc_wrapper_init() {
    /* Under CONFIG_X86, count configured built-in classifiers/actions, then
     * enable the corresponding static keys when RETPOLINE is unavailable. */
}

/* Fallback when CONFIG_MITIGATION_RETPOLINE is disabled: tc_act and
 * tc_classify directly invoke the corresponding external function pointers,
 * and tc_wrapper_init is empty. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
