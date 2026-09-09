/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from net/pkt_cls.h. C preprocessor configuration is preserved
 * by the corresponding cfg comments below. */

pub const TC_ACT_CONSUMED: i32 = TC_ACT_VALUE_MAX + 1;

#[repr(C)]
pub struct tcf_walker {
    pub stop: i32,
    pub skip: i32,
    pub count: i32,
    pub nonempty: bool,
    pub cookie: usize,
    pub r#fn: Option<unsafe extern "C" fn(*mut tcf_proto, *mut core::ffi::c_void, *mut tcf_walker) -> i32>,
}

extern "C" {
    pub fn register_tcf_proto_ops(ops: *mut tcf_proto_ops) -> i32;
    pub fn unregister_tcf_proto_ops(ops: *mut tcf_proto_ops);
    pub fn tcf_queue_work(rwork: *mut rcu_work, func: work_func_t) -> bool;
}

pub const NET_CLS_ALIAS_PREFIX: &str = "net-cls-";

#[repr(C)]
pub struct tcf_block_ext_info {
    pub binder_type: flow_block_binder_type,
    pub chain_head_change: Option<unsafe extern "C" fn()>,
    pub chain_head_change_priv: *mut core::ffi::c_void,
    pub block_index: u32,
}
#[repr(C)]
pub struct tcf_qevent {
    pub block: *mut tcf_block,
    pub info: tcf_block_ext_info,
    pub filter_chain: *mut tcf_proto,
}
extern "C" {
    pub fn tcf_block_put_ext(block: *mut tcf_block, q: *mut Qdisc, ei: *mut tcf_block_ext_info);
    pub fn tcf_qevent_init(qe: *mut tcf_qevent, sch: *mut Qdisc, binder_type: flow_block_binder_type, block_index_attr: *mut nlattr, extack: *mut netlink_ext_ack) -> i32;
    pub fn tcf_qevent_destroy(qe: *mut tcf_qevent, sch: *mut Qdisc);
    pub fn tcf_qevent_validate_change(qe: *mut tcf_qevent, block_index_attr: *mut nlattr, extack: *mut netlink_ext_ack) -> i32;
    pub fn tcf_qevent_handle(qe: *mut tcf_qevent, sch: *mut Qdisc, skb: *mut sk_buff, to_free: *mut *mut sk_buff, ret: *mut i32) -> *mut sk_buff;
    pub fn tcf_qevent_dump(skb: *mut sk_buff, attr_name: i32, qe: *mut tcf_qevent) -> i32;
    pub fn tc_setup_cb_call(block: *mut tcf_block, ty: tc_setup_type, type_data: *mut core::ffi::c_void, err_stop: bool, rtnl_held: bool) -> i32;
    pub fn tc_setup_cb_add(block: *mut tcf_block, tp: *mut tcf_proto, ty: tc_setup_type, type_data: *mut core::ffi::c_void, err_stop: bool, flags: *mut u32, in_hw_count: *mut u32, rtnl_held: bool) -> i32;
    pub fn tc_setup_cb_replace(block: *mut tcf_block, tp: *mut tcf_proto, ty: tc_setup_type, type_data: *mut core::ffi::c_void, err_stop: bool, old_flags: *mut u32, old_in_hw_count: *mut u32, new_flags: *mut u32, new_in_hw_count: *mut u32, rtnl_held: bool) -> i32;
    pub fn tc_setup_cb_destroy(block: *mut tcf_block, tp: *mut tcf_proto, ty: tc_setup_type, type_data: *mut core::ffi::c_void, err_stop: bool, flags: *mut u32, in_hw_count: *mut u32, rtnl_held: bool) -> i32;
    pub fn tc_setup_cb_reoffload(block: *mut tcf_block, tp: *mut tcf_proto, add: bool, cb: *mut flow_setup_cb_t, ty: tc_setup_type, type_data: *mut core::ffi::c_void, cb_priv: *mut core::ffi::c_void, flags: *mut u32, in_hw_count: *mut u32) -> i32;
}

#[cfg(feature = "CONFIG_NET_CLS")]
extern "C" {
    pub fn tcf_chain_get_by_act(block: *mut tcf_block, chain_index: u32) -> *mut tcf_chain;
    pub fn tcf_chain_put_by_act(chain: *mut tcf_chain);
    pub fn tcf_get_next_chain(block: *mut tcf_block, chain: *mut tcf_chain) -> *mut tcf_chain;
    pub fn tcf_get_next_proto(chain: *mut tcf_chain, tp: *mut tcf_proto) -> *mut tcf_proto;
    pub fn tcf_block_netif_keep_dst(block: *mut tcf_block);
    pub fn tcf_block_get(p_block: *mut *mut tcf_block, p_filter_chain: *mut *mut tcf_proto, q: *mut Qdisc, extack: *mut netlink_ext_ack) -> i32;
    pub fn tcf_block_get_ext(p_block: *mut *mut tcf_block, q: *mut Qdisc, ei: *mut tcf_block_ext_info, extack: *mut netlink_ext_ack) -> i32;
    pub fn tcf_block_put(block: *mut tcf_block);
    pub fn tcf_block_put_ext(block: *mut tcf_block, q: *mut Qdisc, ei: *mut tcf_block_ext_info);
    pub fn tcf_exts_init_ex(exts: *mut tcf_exts, net: *mut net, action: i32, police: i32, tp: *mut tcf_proto, handle: u32, used_action_miss: bool) -> i32;
    pub fn tcf_classify(skb: *mut sk_buff, block: *const tcf_block, tp: *const tcf_proto, res: *mut tcf_result, compat_mode: bool) -> i32;
}

#[inline]
pub unsafe fn tcf_block_shared(block: *mut tcf_block) -> bool { (*block).index != 0 }
#[inline]
pub unsafe fn tcf_block_non_null_shared(block: *mut tcf_block) -> bool { !block.is_null() && (*block).index != 0 }
#[inline]
pub unsafe fn tcf_block_q(block: *mut tcf_block) -> *mut Qdisc { WARN_ON(tcf_block_shared(block)); (*block).q }

#[inline]
pub unsafe fn tc_cls_stats_dump(tp: *mut tcf_proto, arg: *mut tcf_walker, filter: *mut core::ffi::c_void) -> bool {
    if (*arg).count >= (*arg).skip && ((*arg).r#fn.unwrap())(tp, filter, arg) < 0 { (*arg).stop = 1; return false; }
    (*arg).count += 1; true
}
#[inline] pub unsafe fn tcf_classify_qdisc(skb: *mut sk_buff, tp: *const tcf_proto, res: *mut tcf_result, compat_mode: bool) -> i32 { let mut ret = tcf_classify(skb, core::ptr::null(), tp, res, compat_mode); if ret == TC_ACT_REDIRECT { ret = TC_ACT_SHOT; } ret }
#[inline] pub unsafe fn __cls_set_class(clp: *mut usize, cl: usize) -> usize { core::ptr::replace(clp, cl) }
#[inline] pub unsafe fn __tcf_bind_filter(q: *mut Qdisc, r: *mut tcf_result, base: usize) { let cl = q_bind_tcf(q, base, (*r).classid); let old = __cls_set_class(&mut (*r).class, cl); if old != 0 { q_unbind_tcf(q, old); } }
#[inline] pub unsafe fn tcf_bind_filter(tp: *mut tcf_proto, r: *mut tcf_result, base: usize) { let q = (*(*tp).chain).block_q; if !q.is_null() { __tcf_bind_filter(q, r, base); } }
#[inline] pub unsafe fn __tcf_unbind_filter(q: *mut Qdisc, r: *mut tcf_result) { let cl = __cls_set_class(&mut (*r).class, 0); if cl != 0 { q_unbind_tcf(q, cl); } }
#[inline] pub unsafe fn tcf_unbind_filter(tp: *mut tcf_proto, r: *mut tcf_result) { let q = (*(*tp).chain).block_q; if !q.is_null() { __tcf_unbind_filter(q, r); } }
#[inline] pub unsafe fn tc_cls_bind_class(classid: u32, cl: usize, q: *mut Qdisc, res: *mut tcf_result, base: usize) { if (*res).classid == classid { if cl != 0 { __tcf_bind_filter(q, res, base); } else { __tcf_unbind_filter(q, res); } } }

#[repr(C)]
pub struct tcf_exts {
    pub r#type: u32,
    pub nr_actions: i32,
    pub actions: *mut *mut tc_action,
    pub net: *mut net,
    pub ns_tracker: netns_tracker,
    pub miss_cookie_node: *mut tcf_exts_miss_cookie_node,
    pub action: i32,
    pub police: i32,
}

#[inline] pub unsafe fn tcf_exts_get_net(exts: *mut tcf_exts) -> bool { !(*exts).net.is_null() }
#[inline] pub unsafe fn tcf_exts_put_net(_exts: *mut tcf_exts) {}

#[inline] pub unsafe fn tcf_em_tree_validate(tp: *mut tcf_proto, tb: *mut nlattr, t: *mut tcf_ematch_tree) -> i32 { __tcf_em_tree_validate(tp, tb, t) }

#[inline]
pub unsafe fn tcf_exts_init(exts: *mut tcf_exts, net: *mut net, action: i32, police: i32) -> i32 {
    #[cfg(feature = "CONFIG_NET_CLS")] { return tcf_exts_init_ex(exts, net, action, police, core::ptr::null_mut(), 0, false); }
    #[cfg(not(feature = "CONFIG_NET_CLS"))] { let _ = (exts, net, action, police); return -EOPNOTSUPP; }
}

#[inline] pub unsafe fn tc_act_in_hw(act: *mut tc_action) -> bool { (*act).in_hw_count != 0 }
#[inline] pub unsafe fn tcf_exts_has_actions(exts: *mut tcf_exts) -> bool { (*exts).nr_actions != 0 }
#[inline] pub unsafe fn tcf_exts_exec(skb: *mut sk_buff, exts: *mut tcf_exts, res: *mut tcf_result) -> i32 { tcf_action_exec(skb, (*exts).actions, (*exts).nr_actions, res) }
#[inline] pub unsafe fn tcf_exts_exec_ex(skb: *mut sk_buff, exts: *mut tcf_exts, act_index: isize, res: *mut tcf_result) -> i32 { tcf_action_exec(skb, (*exts).actions.offset(act_index), (*exts).nr_actions - act_index as i32, res) }

extern "C" {
    pub fn tcf_exts_validate(net: *mut net, tp: *mut tcf_proto, tb: *mut *mut nlattr, rate_tlv: *mut nlattr, exts: *mut tcf_exts, flags: u32, extack: *mut netlink_ext_ack) -> i32;
    pub fn tcf_exts_validate_ex(net: *mut net, tp: *mut tcf_proto, tb: *mut *mut nlattr, rate_tlv: *mut nlattr, exts: *mut tcf_exts, flags: u32, fl_flags: u32, extack: *mut netlink_ext_ack) -> i32;
    pub fn tcf_exts_destroy(exts: *mut tcf_exts);
    pub fn tcf_exts_change(dst: *mut tcf_exts, src: *mut tcf_exts);
    pub fn tcf_exts_dump(skb: *mut sk_buff, exts: *mut tcf_exts) -> i32;
    pub fn tcf_exts_terse_dump(skb: *mut sk_buff, exts: *mut tcf_exts) -> i32;
    pub fn tcf_exts_dump_stats(skb: *mut sk_buff, exts: *mut tcf_exts) -> i32;
    pub fn tcf_em_register(ops: *mut tcf_ematch_ops) -> i32;
    pub fn tcf_em_unregister(ops: *mut tcf_ematch_ops);
    pub fn __tcf_em_tree_match(skb: *mut sk_buff, tree: *mut tcf_ematch_tree, info: *mut tcf_pkt_info) -> i32;
    pub fn __tcf_em_tree_validate(tp: *mut tcf_proto, tb: *mut nlattr, tree: *mut tcf_ematch_tree) -> i32;
    pub fn tcf_em_tree_destroy(tree: *mut tcf_ematch_tree);
    pub fn tcf_em_tree_dump(skb: *mut sk_buff, tree: *mut tcf_ematch_tree, tlv: i32) -> i32;
}

#[repr(C)] pub struct tcf_pkt_info { pub ptr: *mut u8, pub nexthdr: i32 }

#[cfg(feature = "CONFIG_NET_EMATCH")]
#[repr(C)] pub struct tcf_ematch { pub ops: *mut tcf_ematch_ops, pub data: usize, pub datalen: u32, pub matchid: u16, pub flags: u16, pub net: *mut net }
#[cfg(feature = "CONFIG_NET_EMATCH")]
#[inline] pub unsafe fn tcf_em_is_container(em: *mut tcf_ematch) -> bool { (*em).ops.is_null() }
#[cfg(feature = "CONFIG_NET_EMATCH")]
#[inline] pub unsafe fn tcf_em_is_simple(em: *mut tcf_ematch) -> i32 { ((*em).flags & TCF_EM_SIMPLE) as i32 }
#[cfg(feature = "CONFIG_NET_EMATCH")]
#[inline] pub unsafe fn tcf_em_is_inverted(em: *mut tcf_ematch) -> i32 { ((*em).flags & TCF_EM_INVERT) as i32 }
#[cfg(feature = "CONFIG_NET_EMATCH")]
#[inline] pub unsafe fn tcf_em_last_match(em: *mut tcf_ematch) -> i32 { (((*em).flags & TCF_EM_REL_MASK) == TCF_EM_REL_END) as i32 }
#[cfg(feature = "CONFIG_NET_EMATCH")]
#[inline] pub unsafe fn tcf_em_early_end(em: *mut tcf_ematch, result: i32) -> i32 { if tcf_em_last_match(em) != 0 || (result == 0 && (*em).flags & TCF_EM_REL_AND != 0) || (result != 0 && (*em).flags & TCF_EM_REL_OR != 0) { 1 } else { 0 } }

#[cfg(feature = "CONFIG_NET_EMATCH")]
#[repr(C)] pub struct tcf_ematch_tree { pub hdr: tcf_ematch_tree_hdr, pub matches: *mut tcf_ematch }
#[cfg(feature = "CONFIG_NET_EMATCH")]
#[repr(C)] pub struct tcf_ematch_ops { pub kind: i32, pub datalen: i32, pub change: Option<unsafe extern "C" fn(*mut net, *mut core::ffi::c_void, i32, *mut tcf_ematch) -> i32>, pub r#match: Option<unsafe extern "C" fn(*mut sk_buff, *mut tcf_ematch, *mut tcf_pkt_info) -> i32>, pub destroy: Option<unsafe extern "C" fn(*mut tcf_ematch)>, pub dump: Option<unsafe extern "C" fn(*mut sk_buff, *mut tcf_ematch) -> i32>, pub owner: *mut module, pub link: list_head }
#[cfg(feature = "CONFIG_NET_EMATCH")]
#[inline] pub unsafe fn tcf_em_tree_match(skb: *mut sk_buff, tree: *mut tcf_ematch_tree, info: *mut tcf_pkt_info) -> i32 { if (*tree).hdr.nmatches != 0 { __tcf_em_tree_match(skb, tree, info) } else { 1 } }

#[inline] pub unsafe fn tcf_get_base_ptr(skb: *mut sk_buff, layer: i32) -> *mut u8 { match layer { TCF_LAYER_LINK => skb_mac_header(skb), TCF_LAYER_NETWORK => skb_network_header(skb), TCF_LAYER_TRANSPORT => if skb_transport_header_was_set(skb) { skb_transport_header(skb) } else { core::ptr::null_mut() }, _ => core::ptr::null_mut() } }
#[inline] pub unsafe fn tcf_valid_offset(skb: *const sk_buff, ptr: *const u8, len: isize) -> i32 { ((ptr.offset(len) <= skb_tail_pointer(skb) as *const u8) && ptr >= (*skb).head && ptr <= ptr.offset(len)) as i32 }
#[inline] pub unsafe fn tcf_match_indev(skb: *mut sk_buff, ifindex: i32) -> bool { if ifindex == 0 { true } else if (*skb).skb_iif == 0 { false } else { ifindex == (*skb).skb_iif } }

extern "C" {
    pub fn tc_setup_offload_action(flow_action: *mut flow_action, exts: *const tcf_exts, extack: *mut netlink_ext_ack) -> i32;
    pub fn tc_cleanup_offload_action(flow_action: *mut flow_action);
    pub fn tc_setup_action(flow_action: *mut flow_action, actions: *mut *mut tc_action, miss_cookie_base: u32, extack: *mut netlink_ext_ack) -> i32;
    pub fn tcf_exts_num_actions(exts: *mut tcf_exts) -> u32;
}

#[repr(C)] pub struct tc_cls_u32_knode { pub exts: *mut tcf_exts, pub res: *mut tcf_result, pub sel: *mut tc_u32_sel, pub handle: u32, pub val: u32, pub mask: u32, pub link_handle: u32, pub fshift: u8 }
#[repr(C)] pub struct tc_cls_u32_hnode { pub handle: u32, pub prio: u32, pub divisor: u32 }
#[repr(C)] pub enum tc_clsu32_command { TC_CLSU32_NEW_KNODE, TC_CLSU32_REPLACE_KNODE, TC_CLSU32_DELETE_KNODE, TC_CLSU32_NEW_HNODE, TC_CLSU32_REPLACE_HNODE, TC_CLSU32_DELETE_HNODE }
#[repr(C)] pub union tc_cls_u32_offload_data { pub knode: tc_cls_u32_knode, pub hnode: tc_cls_u32_hnode }
#[repr(C)] pub struct tc_cls_u32_offload { pub common: flow_cls_common_offload, pub command: tc_clsu32_command, pub data: tc_cls_u32_offload_data }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
