/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from net/fib_rules.h; included C dependencies remain external. */

#[repr(C)]
pub struct fib_kuid_range {
    pub start: kuid_t,
    pub end: kuid_t,
}

#[repr(C)]
pub struct fib_rule {
    pub list: list_head,
    pub iifindex: ::core::ffi::c_int,
    pub oifindex: ::core::ffi::c_int,
    pub mark: u32,
    pub mark_mask: u32,
    pub flags: u32,
    pub table: u32,
    pub action: u8,
    pub l3mdev: u8,
    pub proto: u8,
    pub ip_proto: u8,
    pub target: u32,
    pub tun_id: __be64,
    pub ctarget: *mut fib_rule,
    pub fr_net: *mut net,
    pub refcnt: refcount_t,
    pub pref: u32,
    pub suppress_ifgroup: ::core::ffi::c_int,
    pub suppress_prefixlen: ::core::ffi::c_int,
    pub iifname: [::core::ffi::c_char; IFNAMSIZ],
    pub oifname: [::core::ffi::c_char; IFNAMSIZ],
    pub uid_range: fib_kuid_range,
    pub sport_range: fib_rule_port_range,
    pub dport_range: fib_rule_port_range,
    pub sport_mask: u16,
    pub dport_mask: u16,
    pub iif_is_l3_master: u8,
    pub oif_is_l3_master: u8,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct fib_lookup_arg {
    pub lookup_ptr: *mut ::core::ffi::c_void,
    pub lookup_data: *const ::core::ffi::c_void,
    pub result: *mut ::core::ffi::c_void,
    pub rule: *mut fib_rule,
    pub table: u32,
    pub flags: ::core::ffi::c_int,
}

pub const FIB_LOOKUP_NOREF: ::core::ffi::c_int = 1;
pub const FIB_LOOKUP_IGNORE_LINKSTATE: ::core::ffi::c_int = 2;

#[repr(C)]
pub struct fib_rules_ops {
    pub family: ::core::ffi::c_int,
    pub list: list_head,
    pub rule_size: ::core::ffi::c_int,
    pub addr_size: ::core::ffi::c_int,
    pub unresolved_rules: ::core::ffi::c_int,
    pub nr_goto_rules: ::core::ffi::c_int,
    pub fib_rules_seq: ::core::ffi::c_uint,
    pub action: Option<unsafe extern "C" fn(*mut fib_rule, *mut flowi, ::core::ffi::c_int, *mut fib_lookup_arg) -> ::core::ffi::c_int>,
    pub suppress: Option<unsafe extern "C" fn(*mut fib_rule, ::core::ffi::c_int, *mut fib_lookup_arg) -> bool>,
    pub r#match: Option<unsafe extern "C" fn(*mut fib_rule, *mut flowi, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub configure: Option<unsafe extern "C" fn(*mut fib_rule, *mut sk_buff, *mut fib_rule_hdr, *mut *mut nlattr, *mut netlink_ext_ack) -> ::core::ffi::c_int>,
    pub delete: Option<unsafe extern "C" fn(*mut fib_rule)>,
    pub compare: Option<unsafe extern "C" fn(*mut fib_rule, *mut fib_rule_hdr, *mut *mut nlattr) -> ::core::ffi::c_int>,
    pub fill: Option<unsafe extern "C" fn(*mut fib_rule, *mut sk_buff, *mut fib_rule_hdr) -> ::core::ffi::c_int>,
    pub nlmsg_payload: Option<unsafe extern "C" fn(*mut fib_rule) -> usize>,
    /* Called after modifications to the rules set, must flush the route cache if one exists. */
    pub flush_cache: Option<unsafe extern "C" fn(*mut fib_rules_ops)>,
    pub need_rtnl: Option<unsafe extern "C" fn(*mut net) -> bool>,
    pub nlgroup: ::core::ffi::c_int,
    pub rules_list: list_head,
    pub owner: *mut module,
    pub fro_net: *mut net,
    pub lock: mutex,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct fib_rule_notifier_info {
    pub info: fib_notifier_info,
    pub rule: *mut fib_rule,
}

#[inline]
pub unsafe fn fib_rule_get(rule: *mut fib_rule) {
    refcount_inc(&mut (*rule).refcnt);
}

#[inline]
pub unsafe fn fib_rule_get_safe(rule: *mut fib_rule) -> bool {
    refcount_inc_not_zero(&mut (*rule).refcnt)
}

#[inline]
pub unsafe fn fib_rule_put(rule: *mut fib_rule) {
    if refcount_dec_and_test(&mut (*rule).refcnt) {
        kfree_rcu(rule, rcu);
    }
}

#[cfg(CONFIG_NET_L3_MASTER_DEV)]
#[inline]
pub unsafe fn fib_rule_get_table(rule: *mut fib_rule, arg: *mut fib_lookup_arg) -> u32 {
    if (*rule).l3mdev != 0 { (*arg).table } else { (*rule).table }
}

#[cfg(not(CONFIG_NET_L3_MASTER_DEV))]
#[inline]
pub unsafe fn fib_rule_get_table(rule: *mut fib_rule, _arg: *mut fib_lookup_arg) -> u32 {
    (*rule).table
}

#[inline]
pub unsafe fn frh_get_table(frh: *mut fib_rule_hdr, nla: *mut *mut nlattr) -> u32 {
    if !(*nla.add(FRA_TABLE as usize)).is_null() {
        nla_get_u32(*nla.add(FRA_TABLE as usize))
    } else { (*frh).table }
}

#[inline]
pub unsafe fn fib_rule_port_range_set(range: *const fib_rule_port_range) -> bool {
    (*range).start != 0 && (*range).end != 0
}

#[inline]
pub unsafe fn fib_rule_port_inrange(a: *const fib_rule_port_range, port: __be16) -> bool {
    ntohs(port) >= (*a).start && ntohs(port) <= (*a).end
}

#[inline]
pub unsafe fn fib_rule_port_match(range: *const fib_rule_port_range, port_mask: u16, port: __be16) -> bool {
    if (((*range).start ^ ntohs(port)) & port_mask) != 0 { return false; }
    if port_mask == 0 && fib_rule_port_range_set(range) && !fib_rule_port_inrange(range, port) { return false; }
    true
}

#[inline]
pub unsafe fn fib_rule_port_range_valid(a: *const fib_rule_port_range) -> bool {
    (*a).start != 0 && (*a).end != 0 && (*a).end < 0xffff && (*a).start <= (*a).end
}

#[inline]
pub unsafe fn fib_rule_port_range_compare(a: *mut fib_rule_port_range, b: *mut fib_rule_port_range) -> bool {
    (*a).start == (*b).start && (*a).end == (*b).end
}

#[inline]
pub unsafe fn fib_rule_port_is_range(range: *const fib_rule_port_range) -> bool {
    (*range).start != (*range).end
}

#[inline]
pub unsafe fn fib_rule_requires_fldissect(rule: *mut fib_rule) -> bool {
    (*rule).iifindex != LOOPBACK_IFINDEX && ((*rule).ip_proto != 0 ||
        fib_rule_port_range_set(&(*rule).sport_range) || fib_rule_port_range_set(&(*rule).dport_range))
}

extern "C" {
    pub fn fib_rules_register(ops: *const fib_rules_ops, net: *mut net) -> *mut fib_rules_ops;
    pub fn fib_rules_unregister(ops: *mut fib_rules_ops);
    pub fn fib_rules_lookup(ops: *mut fib_rules_ops, fl: *mut flowi, flags: ::core::ffi::c_int, arg: *mut fib_lookup_arg) -> ::core::ffi::c_int;
    pub fn fib_default_rule_add(ops: *mut fib_rules_ops, pref: u32, table: u32) -> ::core::ffi::c_int;
    pub fn fib_rule_matchall(rule: *const fib_rule) -> bool;
    pub fn fib_rules_dump(net: *mut net, nb: *mut notifier_block, family: ::core::ffi::c_int, extack: *mut netlink_ext_ack) -> ::core::ffi::c_int;
    pub fn fib_rules_seq_read(net: *const net, family: ::core::ffi::c_int) -> ::core::ffi::c_uint;
    pub fn fib_newrule(net: *mut net, skb: *mut sk_buff, nlh: *mut nlmsghdr, extack: *mut netlink_ext_ack, rtnl_held: bool) -> ::core::ffi::c_int;
    pub fn fib_delrule(net: *mut net, skb: *mut sk_buff, nlh: *mut nlmsghdr, extack: *mut netlink_ext_ack, rtnl_held: bool) -> ::core::ffi::c_int;
    pub fn fib6_rule_match(rule: *mut fib_rule, fl: *mut flowi, flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn fib4_rule_match(rule: *mut fib_rule, fl: *mut flowi, flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn fib6_rule_action(rule: *mut fib_rule, flp: *mut flowi, flags: ::core::ffi::c_int, arg: *mut fib_lookup_arg) -> ::core::ffi::c_int;
    pub fn fib4_rule_action(rule: *mut fib_rule, flp: *mut flowi, flags: ::core::ffi::c_int, arg: *mut fib_lookup_arg) -> ::core::ffi::c_int;
    pub fn fib6_rule_suppress(rule: *mut fib_rule, flags: ::core::ffi::c_int, arg: *mut fib_lookup_arg) -> bool;
    pub fn fib4_rule_suppress(rule: *mut fib_rule, flags: ::core::ffi::c_int, arg: *mut fib_lookup_arg) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
