/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of net/act_api.h. External kernel types and functions are supplied elsewhere. */

#[repr(C)]
pub struct tcf_idrinfo {
    pub lock: mutex,
    pub action_idr: idr,
    pub net: *mut net,
}

#[repr(C)]
pub struct tc_action {
    pub ops: *const tc_action_ops,
    pub r#type: __u32,
    pub idrinfo: *mut tcf_idrinfo,
    pub tcfa_index: u32,
    pub tcfa_refcnt: refcount_t,
    pub tcfa_bindcnt: atomic_t,
    pub tcfa_action: ::core::ffi::c_int,
    pub tcfa_tm: tcf_t,
    pub tcfa_bstats: gnet_stats_basic_sync,
    pub tcfa_bstats_hw: gnet_stats_basic_sync,
    pub tcfa_drops: atomic_t,
    pub tcfa_overlimits: atomic_t,
    pub tcfa_rate_est: *mut net_rate_estimator,
    pub tcfa_lock: spinlock_t,
    pub cpu_bstats: *mut gnet_stats_basic_sync,
    pub cpu_bstats_hw: *mut gnet_stats_basic_sync,
    pub cpu_qstats: *mut gnet_stats_queue,
    pub user_cookie: *mut tc_cookie,
    pub goto_chain: *mut tcf_chain,
    pub tcfa_flags: u32,
    pub tcfa_rcu: rcu_head,
    pub hw_stats: u8,
    pub used_hw_stats: u8,
    pub used_hw_stats_valid: bool,
    pub in_hw_count: u32,
}

// C field aliases: tcf_index=tcfa_index, tcf_refcnt=tcfa_refcnt,
// tcf_bindcnt=tcfa_bindcnt, tcf_action=tcfa_action, tcf_tm=tcfa_tm,
// tcf_bstats=tcfa_bstats, tcf_rate_est=tcfa_rate_est, tcf_lock=tcfa_lock.

pub const TCA_ACT_HW_STATS_ANY: u8 = TCA_ACT_HW_STATS_IMMEDIATE | TCA_ACT_HW_STATS_DELAYED;
pub const TCA_ACT_FLAGS_USER_BITS: u32 = 16;
pub const TCA_ACT_FLAGS_USER_MASK: u32 = 0xffff;
pub const TCA_ACT_FLAGS_POLICE: u32 = 1u32 << TCA_ACT_FLAGS_USER_BITS;
pub const TCA_ACT_FLAGS_BIND: u32 = 1u32 << (TCA_ACT_FLAGS_USER_BITS + 1);
pub const TCA_ACT_FLAGS_REPLACE: u32 = 1u32 << (TCA_ACT_FLAGS_USER_BITS + 2);
pub const TCA_ACT_FLAGS_NO_RTNL: u32 = 1u32 << (TCA_ACT_FLAGS_USER_BITS + 3);
pub const TCA_ACT_FLAGS_AT_INGRESS: u32 = 1u32 << (TCA_ACT_FLAGS_USER_BITS + 4);
pub const TCA_ACT_FLAGS_AT_INGRESS_OR_CLSACT: u32 = 1u32 << (TCA_ACT_FLAGS_USER_BITS + 5);

pub unsafe fn tcf_lastuse_update(tm: *mut tcf_t) {
    let now = jiffies;
    if READ_ONCE((*tm).lastuse) != now { WRITE_ONCE(&mut (*tm).lastuse, now); }
    if unlikely(!READ_ONCE((*tm).firstuse)) { WRITE_ONCE(&mut (*tm).firstuse, now); }
}

pub unsafe fn tcf_tm_dump(dtm: *mut tcf_t, stm: *const tcf_t) {
    let now = jiffies;
    (*dtm).install = jiffies_to_clock_t(now.wrapping_sub(READ_ONCE((*stm).install)));
    (*dtm).lastuse = jiffies_to_clock_t(now.wrapping_sub(READ_ONCE((*stm).lastuse)));
    let firstuse = READ_ONCE((*stm).firstuse);
    (*dtm).firstuse = if firstuse { jiffies_to_clock_t(now.wrapping_sub(firstuse)) } else { 0 };
    (*dtm).expires = jiffies_to_clock_t(READ_ONCE((*stm).expires));
}

pub unsafe fn tc_act_hw_stats(hw_stats: u8) -> flow_action_hw_stats {
    if WARN_ON_ONCE(hw_stats > TCA_ACT_HW_STATS_ANY) { FLOW_ACTION_HW_STATS_DONT_CARE }
    else if hw_stats == 0 { FLOW_ACTION_HW_STATS_DISABLED } else { hw_stats }
}

pub type tc_action_priv_destructor = unsafe extern "C" fn(*mut ::core::ffi::c_void);

#[repr(C)]
pub struct tc_action_ops {
    pub head: list_head,
    pub kind: [::core::ffi::c_char; IFNAMSIZ],
    pub id: tca_id,
    pub net_id: ::core::ffi::c_uint,
    pub size: usize,
    pub owner: *mut module,
    pub act: Option<unsafe extern "C" fn(*mut sk_buff, *const tc_action, *mut tcf_result) -> ::core::ffi::c_int>,
    pub dump: Option<unsafe extern "C" fn(*mut sk_buff, *mut tc_action, ::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub cleanup: Option<unsafe extern "C" fn(*mut tc_action)>,
    pub lookup: Option<unsafe extern "C" fn(*mut net, *mut *mut tc_action, u32) -> ::core::ffi::c_int>,
    pub init: Option<unsafe extern "C" fn(*mut net, *mut nlattr, *mut nlattr, *mut *mut tc_action, *mut tcf_proto, u32, *mut netlink_ext_ack) -> ::core::ffi::c_int>,
    pub walk: Option<unsafe extern "C" fn(*mut net, *mut sk_buff, *mut netlink_callback, ::core::ffi::c_int, *const tc_action_ops, *mut netlink_ext_ack) -> ::core::ffi::c_int>,
    pub stats_update: Option<unsafe extern "C" fn(*mut tc_action, u64, u64, u64, u64, bool)>,
    pub get_fill_size: Option<unsafe extern "C" fn(*const tc_action) -> usize>,
    pub get_dev: Option<unsafe extern "C" fn(*const tc_action, *mut tc_action_priv_destructor) -> *mut net_device>,
    pub get_psample_group: Option<unsafe extern "C" fn(*const tc_action, *mut tc_action_priv_destructor) -> *mut psample_group>,
    pub offload_act_setup: Option<unsafe extern "C" fn(*mut tc_action, *mut ::core::ffi::c_void, *mut u32, bool, *mut netlink_ext_ack) -> ::core::ffi::c_int>,
}

#[repr(C)]
pub struct tc_action_net { pub idrinfo: *mut tcf_idrinfo, pub ops: *const tc_action_ops }

pub const ACT_P_BOUND: ::core::ffi::c_int = 0;
pub const ACT_P_CREATED: ::core::ffi::c_int = 1;
pub const ACT_P_DELETED: ::core::ffi::c_int = 1;

pub unsafe fn tcf_action_valid(action: ::core::ffi::c_int) -> bool {
    let opcode = TC_ACT_EXT_OPCODE(action);
    if opcode == 0 { action <= TC_ACT_VALUE_MAX } else { opcode <= TC_ACT_EXT_OPCODE_MAX || action == TC_ACT_UNSPEC }
}

extern "C" {
    pub fn tcf_idrinfo_destroy(ops: *const tc_action_ops, idrinfo: *mut tcf_idrinfo);
    pub fn tcf_generic_walker(tn: *mut tc_action_net, skb: *mut sk_buff, cb: *mut netlink_callback, r#type: ::core::ffi::c_int, ops: *const tc_action_ops, extack: *mut netlink_ext_ack) -> ::core::ffi::c_int;
    pub fn tcf_idr_search(tn: *mut tc_action_net, a: *mut *mut tc_action, index: u32) -> ::core::ffi::c_int;
    pub fn tcf_action_update_stats(a: *mut tc_action, bytes: u64, packets: u64, drops: u64, hw: bool);
    pub fn tcf_action_copy_stats(skb: *mut sk_buff, a: *mut tc_action, r#type: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn tcf_action_update_hw_stats(action: *mut tc_action) -> ::core::ffi::c_int;
    pub fn tcf_action_reoffload_cb(cb: *mut flow_indr_block_bind_cb_t, cb_priv: *mut ::core::ffi::c_void, add: bool) -> ::core::ffi::c_int;
    pub fn tcf_action_check_ctrlact(action: ::core::ffi::c_int, tp: *mut tcf_proto, handle: *mut *mut tcf_chain, newchain: *mut netlink_ext_ack) -> ::core::ffi::c_int;
    pub fn tcf_action_set_ctrlact(a: *mut tc_action, action: ::core::ffi::c_int, newchain: *mut tcf_chain) -> *mut tcf_chain;
    pub fn tcf_dev_queue_xmit(skb: *mut sk_buff, xmit: Option<unsafe extern "C" fn(*mut sk_buff) -> ::core::ffi::c_int>) -> ::core::ffi::c_int;
    pub fn tcf_idr_create(tn: *mut tc_action_net, index: u32, est: *mut nlattr, a: *mut *mut tc_action, ops: *const tc_action_ops, bind: ::core::ffi::c_int, cpustats: bool, flags: u32) -> ::core::ffi::c_int;
    pub fn tcf_idr_create_from_flags(tn: *mut tc_action_net, index: u32, est: *mut nlattr, a: *mut *mut tc_action, ops: *const tc_action_ops, bind: ::core::ffi::c_int, flags: u32) -> ::core::ffi::c_int;
    pub fn tcf_idr_insert_many(actions: *mut *mut tc_action, init_res: *mut ::core::ffi::c_int);
    pub fn tcf_idr_cleanup(tn: *mut tc_action_net, index: u32);
    pub fn tcf_idr_check_alloc(tn: *mut tc_action_net, index: *mut u32, a: *mut *mut tc_action, bind: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn tcf_idr_release(a: *mut tc_action, bind: bool) -> ::core::ffi::c_int;
    pub fn tcf_register_action(a: *mut tc_action_ops, ops: *mut pernet_operations) -> ::core::ffi::c_int;
    pub fn tcf_unregister_action(a: *mut tc_action_ops, ops: *mut pernet_operations) -> ::core::ffi::c_int;
    pub fn tcf_action_destroy(actions: *mut *mut tc_action, bind: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn tcf_action_exec(skb: *mut sk_buff, actions: *mut *mut tc_action, nr_actions: ::core::ffi::c_int, res: *mut tcf_result) -> ::core::ffi::c_int;
    pub fn tcf_action_init(net: *mut net, tp: *mut tcf_proto, nla: *mut nlattr, est: *mut nlattr, actions: *mut *mut tc_action, init_res: *mut ::core::ffi::c_int, attr_size: *mut usize, flags: u32, fl_flags: u32, extack: *mut netlink_ext_ack) -> ::core::ffi::c_int;
    pub fn tc_action_load_ops(nla: *mut nlattr, flags: u32, extack: *mut netlink_ext_ack) -> *mut tc_action_ops;
    pub fn tcf_action_init_1(net: *mut net, tp: *mut tcf_proto, nla: *mut nlattr, est: *mut nlattr, a_o: *mut tc_action_ops, init_res: *mut ::core::ffi::c_int, flags: u32, extack: *mut netlink_ext_ack) -> *mut tc_action;
    pub fn tcf_action_dump(skb: *mut sk_buff, actions: *mut *mut tc_action, bind: ::core::ffi::c_int, r#ref: ::core::ffi::c_int, terse: bool) -> ::core::ffi::c_int;
    pub fn tcf_action_dump_old(skb: *mut sk_buff, a: *mut tc_action, bind: ::core::ffi::c_int, r#ref: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

pub unsafe fn tcf_action_reoffload_cb_disabled(_cb: *mut flow_indr_block_bind_cb_t, _cb_priv: *mut ::core::ffi::c_void, _add: bool) -> ::core::ffi::c_int { 0 }

pub unsafe fn tcf_action_stats_update(a: *mut tc_action, bytes: u64, packets: u64, drops: u64, lastuse: u64, hw: bool) {
    if !(*a).ops.is_null() && (*(*a).ops).stats_update.is_some() { ((*(*a).ops).stats_update.unwrap())(a, bytes, packets, drops, lastuse, hw); }
}

pub unsafe fn tcf_action_update_bstats(a: *mut tc_action, skb: *mut sk_buff) {
    if !(*a).cpu_bstats.is_null() { bstats_update(this_cpu_ptr((*a).cpu_bstats), skb); return; }
    spin_lock(&mut (*a).tcfa_lock); bstats_update(&mut (*a).tcfa_bstats, skb); spin_unlock(&mut (*a).tcfa_lock);
}
pub unsafe fn tcf_action_inc_drop_qstats(a: *mut tc_action) {
    if !(*a).cpu_qstats.is_null() { qstats_cpu_drop_inc((*a).cpu_qstats); return; }
    atomic_inc(&mut (*a).tcfa_drops);
}
pub unsafe fn tcf_action_inc_overlimit_qstats(a: *mut tc_action) {
    if !(*a).cpu_qstats.is_null() { qstats_cpu_overlimit_inc((*a).cpu_qstats); return; }
    atomic_inc(&mut (*a).tcfa_overlimits);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
