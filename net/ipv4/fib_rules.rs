// SPDX-License-Identifier: GPL-2.0-or-later
/* IPv4 Forwarding Information Base: policy rules. */

#[repr(C)]
pub struct fib4_rule {
    pub common: fib_rule,
    pub dst_len: u8,
    pub src_len: u8,
    pub dscp: dscp_t,
    pub dscp_mask: dscp_t,
    pub dscp_full: bool,
    pub src: __be32,
    pub srcmask: __be32,
    pub dst: __be32,
    pub dstmask: __be32,
    #[cfg(CONFIG_IP_ROUTE_CLASSID)]
    pub tclassid: u32,
}

unsafe fn fib4_rule_matchall(rule: *const fib_rule) -> bool {
    let r = container_of!(rule, fib4_rule, common);
    if (*r).dst_len != 0 || (*r).src_len != 0 || (*r).dscp != 0 { return false; }
    fib_rule_matchall(rule)
}

pub unsafe fn fib4_rule_default(rule: *const fib_rule) -> bool {
    if !fib4_rule_matchall(rule) || (*rule).action != FR_ACT_TO_TBL || (*rule).l3mdev { return false; }
    if (*rule).table != RT_TABLE_LOCAL && (*rule).table != RT_TABLE_MAIN && (*rule).table != RT_TABLE_DEFAULT { return false; }
    true
}

pub unsafe fn fib4_rules_dump(net: *mut net, nb: *mut notifier_block, extack: *mut netlink_ext_ack) -> i32 { fib_rules_dump(net, nb, AF_INET, extack) }
pub unsafe fn fib4_rules_seq_read(net: *const net) -> u32 { fib_rules_seq_read(net, AF_INET) }

pub unsafe fn __fib_lookup(net: *mut net, flp: *mut flowi4, res: *mut fib_result, flags: u32) -> i32 {
    let mut arg = fib_lookup_arg { result: res, flags, ..core::mem::zeroed() };
    l3mdev_update_flow(net, flowi4_to_flowi(flp));
    let mut err = fib_rules_lookup((*net).ipv4.rules_ops, flowi4_to_flowi(flp), 0, &mut arg);
    #[cfg(CONFIG_IP_ROUTE_CLASSID)]
    { (*res).tclassid = if !arg.rule.is_null() { (*(arg.rule as *mut fib4_rule)).tclassid } else { 0 }; }
    if err == -ESRCH { err = -ENETUNREACH; }
    err
}

pub unsafe fn fib4_rule_action(rule: *mut fib_rule, flp: *mut flowi, _flags: i32, arg: *mut fib_lookup_arg) -> i32 {
    let mut err = -EAGAIN;
    let tbl: *mut fib_table;
    let tb_id: u32;
    match (*rule).action {
        FR_ACT_TO_TBL => (),
        FR_ACT_UNREACHABLE => return -ENETUNREACH,
        FR_ACT_PROHIBIT => return -EACCES,
        _ => return -EINVAL,
    }
    rcu_read_lock();
    tb_id = fib_rule_get_table(rule, arg);
    tbl = fib_get_table((*rule).fr_net, tb_id);
    if !tbl.is_null() { err = fib_table_lookup(tbl, &mut (*flp).u.ip4, (*arg).result as *mut fib_result, (*arg).flags); }
    rcu_read_unlock();
    err
}

pub unsafe fn fib4_rule_suppress(rule: *mut fib_rule, _flags: i32, arg: *mut fib_lookup_arg) -> bool {
    let result = (*arg).result;
    let mut dev: *mut net_device = core::ptr::null_mut();
    if !(*result).fi.is_null() { dev = fib_info_nhc((*result).fi, 0).nhc_dev; }
    if (*result).prefixlen <= (*rule).suppress_prefixlen { return suppress_route(rule, arg); }
    if (*rule).suppress_ifgroup != -1 && !dev.is_null() && (*dev).group == (*rule).suppress_ifgroup { return suppress_route(rule, arg); }
    false
}

unsafe fn suppress_route(_rule: *mut fib_rule, arg: *mut fib_lookup_arg) -> bool {
    if (*arg).flags & FIB_LOOKUP_NOREF == 0 { fib_info_put((*(*arg).result).fi); }
    true
}

pub unsafe fn fib4_rule_match(rule: *mut fib_rule, fl: *mut flowi, _flags: i32) -> i32 {
    let r = rule as *mut fib4_rule; let fl4 = &mut (*fl).u.ip4;
    if (((fl4.saddr ^ (*r).src) & (*r).srcmask) != 0) || (((fl4.daddr ^ (*r).dst) & (*r).dstmask) != 0) { return 0; }
    if ((*r).dscp_full && (((*r).dscp ^ fl4.flowi4_dscp) & (*r).dscp_mask) != 0) || (!(*r).dscp_full && (*r).dscp != 0 && !fib_dscp_masked_match((*r).dscp, fl4)) { return 0; }
    if (*rule).ip_proto != 0 && (*rule).ip_proto != fl4.flowi4_proto { return 0; }
    if !fib_rule_port_match(&(*rule).sport_range, (*rule).sport_mask, fl4.fl4_sport) || !fib_rule_port_match(&(*rule).dport_range, (*rule).dport_mask, fl4.fl4_dport) { return 0; }
    1
}

unsafe fn fib_empty_table(net: *mut net) -> *mut fib_table {
    let mut id = 1u32;
    loop { if fib_get_table(net, id).is_null() { return fib_new_table(net, id); } if id == RT_TABLE_MAX { break; } id += 1; }
    core::ptr::null_mut()
}

unsafe fn fib4_nl2rule_dscp(nla: *const nlattr, rule4: *mut fib4_rule, extack: *mut netlink_ext_ack) -> i32 {
    if (*rule4).dscp != 0 { NL_SET_ERR_MSG(extack, "Cannot specify both TOS and DSCP"); return -EINVAL; }
    (*rule4).dscp = inet_dsfield_to_dscp(nla_get_u8(nla) << 2); (*rule4).dscp_mask = inet_dsfield_to_dscp(INET_DSCP_MASK); (*rule4).dscp_full = true; 0
}

unsafe fn fib4_nl2rule_dscp_mask(nla: *const nlattr, rule4: *mut fib4_rule, extack: *mut netlink_ext_ack) -> i32 {
    if !(*rule4).dscp_full { NL_SET_ERR_MSG_ATTR(extack, nla, "Cannot specify DSCP mask without DSCP value"); return -EINVAL; }
    let mask = inet_dsfield_to_dscp(nla_get_u8(nla) << 2); if (*rule4).dscp & !mask != 0 { NL_SET_ERR_MSG_ATTR(extack, nla, "Invalid DSCP mask"); return -EINVAL; } (*rule4).dscp_mask = mask; 0
}

// Remaining callbacks retain the kernel ABI and are declared against external definitions.
extern "C" {
    fn fib4_rule_configure(rule: *mut fib_rule, skb: *mut sk_buff, frh: *mut fib_rule_hdr, tb: *mut *mut nlattr, extack: *mut netlink_ext_ack) -> i32;
    fn fib4_rule_delete(rule: *mut fib_rule);
    fn fib4_rule_compare(rule: *mut fib_rule, frh: *mut fib_rule_hdr, tb: *mut *mut nlattr) -> i32;
    fn fib4_rule_fill(rule: *mut fib_rule, skb: *mut sk_buff, frh: *mut fib_rule_hdr) -> i32;
    fn fib4_rule_nlmsg_payload(rule: *mut fib_rule) -> usize;
    fn fib4_rule_flush_cache(ops: *mut fib_rules_ops);
    fn fib4_rule_need_rtnl(net: *mut net) -> bool;
}

unsafe fn fib_default_rules_init(ops: *mut fib_rules_ops) -> i32 {
    let mut err = fib_default_rule_add(ops, 0, RT_TABLE_LOCAL); if err < 0 { return err; }
    err = fib_default_rule_add(ops, 0x7FFE, RT_TABLE_MAIN); if err < 0 { return err; }
    err = fib_default_rule_add(ops, 0x7FFF, RT_TABLE_DEFAULT); if err < 0 { return err; } 0
}

pub unsafe fn fib4_rules_init(net: *mut net) -> i32 {
    let ops = fib_rules_register(&fib4_rules_ops_template, net); if IS_ERR(ops) { return PTR_ERR(ops); }
    let err = fib_default_rules_init(ops); if err < 0 { fib_rules_unregister(ops); return err; }
    (*net).ipv4.rules_ops = ops; (*net).ipv4.fib_has_custom_rules = false; (*net).ipv4.fib_rules_require_fldissect = 0; 0
}

pub unsafe fn fib4_rules_exit(net: *mut net) { fib_rules_unregister((*net).ipv4.rules_ops); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
