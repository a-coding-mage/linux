// SPDX-License-Identifier: GPL-2.0-only
/* net/ipv6/fib6_rules.c IPv6 Routing Policy Rules */

#[repr(C)]
pub struct fib6_rule {
    pub common: fib_rule,
    pub src: rt6key,
    pub dst: rt6key,
    pub flowlabel: __be32,
    pub flowlabel_mask: __be32,
    pub dscp: dscp_t,
    pub dscp_mask: dscp_t,
    pub dscp_full: u8,
}

unsafe fn fib6_rule_matchall(rule: *const fib_rule) -> bool {
    let r = container_of!(rule, fib6_rule, common);
    if (*r).dst.plen != 0 || (*r).src.plen != 0 || (*r).dscp != 0 || (*r).flowlabel_mask != 0 { return false; }
    fib_rule_matchall(rule)
}

pub unsafe fn fib6_rule_default(rule: *const fib_rule) -> bool {
    if !fib6_rule_matchall(rule) || (*rule).action != FR_ACT_TO_TBL || (*rule).l3mdev { return false; }
    if (*rule).table != RT6_TABLE_LOCAL && (*rule).table != RT6_TABLE_MAIN { return false; }
    true
}

pub unsafe fn fib6_rules_dump(net: *mut net, nb: *mut notifier_block, extack: *mut netlink_ext_ack) -> c_int { fib_rules_dump(net, nb, AF_INET6, extack) }
pub unsafe fn fib6_rules_seq_read(net: *const net) -> c_uint { fib_rules_seq_read(net, AF_INET6) }

pub unsafe fn fib6_lookup(net: *mut net, oif: c_int, fl6: *mut flowi6, res: *mut fib6_result, flags: c_int) -> c_int {
    let mut err;
    if (*net).ipv6.fib6_has_custom_rules {
        let mut arg = fib_lookup_arg { lookup_ptr: fib6_table_lookup, lookup_data: &oif as *const _ as *mut _, result: res, flags: FIB_LOOKUP_NOREF };
        l3mdev_update_flow(net, flowi6_to_flowi(fl6));
        err = fib_rules_lookup((*net).ipv6.fib6_rules_ops, flowi6_to_flowi(fl6), flags, &mut arg);
    } else {
        err = fib6_table_lookup(net, (*net).ipv6.fib6_local_tbl, oif, fl6, res, flags);
        if err != 0 || (*res).f6i == (*net).ipv6.fib6_null_entry { err = fib6_table_lookup(net, (*net).ipv6.fib6_main_tbl, oif, fl6, res, flags); }
    }
    err
}

pub unsafe fn fib6_rule_lookup(net: *mut net, fl6: *mut flowi6, skb: *const sk_buff, flags: c_int, lookup: pol_lookup_t) -> *mut dst_entry {
    if (*net).ipv6.fib6_has_custom_rules {
        let mut res: fib6_result = core::mem::zeroed();
        let mut arg = fib_lookup_arg { lookup_ptr: lookup, lookup_data: skb as *mut _, result: &mut res, flags: FIB_LOOKUP_NOREF };
        l3mdev_update_flow(net, flowi6_to_flowi(fl6));
        fib_rules_lookup((*net).ipv6.fib6_rules_ops, flowi6_to_flowi(fl6), flags, &mut arg);
        if !res.rt6.is_null() { return &mut (*res.rt6).dst; }
    } else {
        let mut rt = pol_lookup_func(lookup, net, (*net).ipv6.fib6_local_tbl, fl6, skb, flags);
        if rt != (*net).ipv6.ip6_null_entry && (*rt).dst.error != -EAGAIN { return &mut (*rt).dst; }
        ip6_rt_put_flags(rt, flags);
        rt = pol_lookup_func(lookup, net, (*net).ipv6.fib6_main_tbl, fl6, skb, flags);
        if (*rt).dst.error != -EAGAIN { return &mut (*rt).dst; }
        ip6_rt_put_flags(rt, flags);
    }
    if flags & RT6_LOOKUP_F_DST_NOREF == 0 { dst_hold(&mut (*net).ipv6.ip6_null_entry.as_mut().unwrap().dst); }
    &mut (*net).ipv6.ip6_null_entry.as_mut().unwrap().dst
}

unsafe fn fib6_rule_saddr(net: *mut net, rule: *mut fib_rule, flags: c_int, flp6: *mut flowi6, dev: *const net_device) -> c_int {
    let r = rule as *mut fib6_rule;
    if (*rule).flags & FIB_RULE_FIND_SADDR != 0 && (*r).src.plen != 0 && flags & RT6_LOOKUP_F_HAS_SADDR == 0 {
        let mut saddr: in6_addr = core::mem::zeroed();
        if ipv6_dev_get_saddr(net, dev, &(*flp6).daddr, rt6_flags2srcprefs(flags), &mut saddr) != 0 { return -EAGAIN; }
        if !ipv6_prefix_equal(&saddr, &(*r).src.addr, (*r).src.plen) { return -EAGAIN; }
        (*flp6).saddr = saddr;
    }
    0
}

unsafe fn fib6_rule_action_alt(rule: *mut fib_rule, flp: *mut flowi, flags: c_int, arg: *mut fib_lookup_arg) -> c_int {
    let res = (*arg).result; let flp6 = &mut (*flp).u.ip6; let net = (*rule).fr_net; let mut table; let mut err; let oif;
    match (*rule).action { FR_ACT_TO_TBL => (), FR_ACT_UNREACHABLE => return -ENETUNREACH, FR_ACT_PROHIBIT => return -EACCES, _ => return -EINVAL }
    let tb_id = fib_rule_get_table(rule, arg); table = fib6_get_table(net, tb_id); if table.is_null() { return -EAGAIN; }
    oif = (*arg).lookup_data as *mut c_int; err = fib6_table_lookup(net, table, *oif, flp6, res, flags);
    if err == 0 && (*res).f6i != (*net).ipv6.fib6_null_entry { err = fib6_rule_saddr(net, rule, flags, flp6, (*res).nh.as_ref().unwrap().fib_nh_dev); } else { err = -EAGAIN; } err
}

unsafe fn __fib6_rule_action(rule: *mut fib_rule, flp: *mut flowi, flags: c_int, arg: *mut fib_lookup_arg) -> c_int {
    let res = (*arg).result; let flp6 = &mut (*flp).u.ip6; let mut rt: *mut rt6_info = core::ptr::null_mut(); let net = (*rule).fr_net; let lookup = (*arg).lookup_ptr; let mut err = 0;
    match (*rule).action { FR_ACT_TO_TBL => (), FR_ACT_UNREACHABLE => { err=-ENETUNREACH; rt=(*net).ipv6.ip6_null_entry; }, FR_ACT_PROHIBIT => { err=-EACCES; rt=(*net).ipv6.ip6_prohibit_entry; }, _ => { err=-EINVAL; rt=(*net).ipv6.ip6_blk_hole_entry; } }
    if !rt.is_null() { if flags & RT6_LOOKUP_F_DST_NOREF == 0 { dst_hold(&mut (*rt).dst); } (*res).rt6=rt; return err; }
    let table = fib6_get_table(net, fib_rule_get_table(rule,arg)); if table.is_null() { (*res).rt6=rt; return -EAGAIN; }
    rt = pol_lookup_func(lookup, net, table, flp6, (*arg).lookup_data, flags);
    if rt != (*net).ipv6.ip6_null_entry { let idev=ip6_dst_idev(&(*rt).dst); if !idev.is_null() { err=fib6_rule_saddr(net,rule,flags,flp6,(*idev).dev); if err != -EAGAIN { err=(*rt).dst.error; if err != -EAGAIN { (*res).rt6=rt; return err; } } } }
    ip6_rt_put_flags(rt,flags); (*res).rt6=core::ptr::null_mut(); -EAGAIN
}

pub unsafe fn fib6_rule_action(rule: *mut fib_rule, flp: *mut flowi, flags: c_int, arg: *mut fib_lookup_arg) -> c_int { if (*arg).lookup_ptr == fib6_table_lookup { fib6_rule_action_alt(rule,flp,flags,arg) } else { __fib6_rule_action(rule,flp,flags,arg) } }

pub unsafe fn fib6_rule_suppress(rule: *mut fib_rule, flags: c_int, arg: *mut fib_lookup_arg) -> bool {
    let res=(*arg).result; let rt=(*res).rt6; if rt.is_null(){return false;} let dev=if !(*rt).rt6i_idev.is_null(){(*rt).rt6i_idev.as_ref().unwrap().dev}else{core::ptr::null_mut()};
    if (*rt).rt6i_dst.plen <= (*rule).suppress_prefixlen || ((*rule).suppress_ifgroup != -1 && !dev.is_null() && (*dev).group == (*rule).suppress_ifgroup) { ip6_rt_put_flags(rt,flags); (*res).rt6=core::ptr::null_mut(); return true; } false
}

pub unsafe fn fib6_rule_match(rule: *mut fib_rule, fl: *mut flowi, flags: c_int) -> c_int {
    let r=rule as *mut fib6_rule; let fl6=&mut (*fl).u.ip6;
    if (*r).dst.plen != 0 && !ipv6_prefix_equal(&fl6.daddr,&(*r).dst.addr,(*r).dst.plen){return 0;}
    if (*r).src.plen != 0 { if flags & RT6_LOOKUP_F_HAS_SADDR != 0 {if !ipv6_prefix_equal(&fl6.saddr,&(*r).src.addr,(*r).src.plen){return 0;}} else if (*r).common.flags & FIB_RULE_FIND_SADDR == 0{return 0;} }
    if (((*r).dscp ^ ip6_dscp(fl6.flowlabel)) & (*r).dscp_mask) != 0 || (((*r).flowlabel ^ flowi6_get_flowlabel(fl6)) & (*r).flowlabel_mask) != 0{return 0;}
    if (*rule).ip_proto != 0 && (*rule).ip_proto != fl6.flowi6_proto{return 0;} if !fib_rule_port_match(&(*rule).sport_range,(*rule).sport_mask,fl6.fl6_sport){return 0;} if !fib_rule_port_match(&(*rule).dport_range,(*rule).dport_mask,fl6.fl6_dport){return 0;} 1
}

unsafe fn fib6_nl2rule_dscp(nla:*const nlattr, r:*mut fib6_rule, extack:*mut netlink_ext_ack)->c_int { if (*r).dscp!=0{return -EINVAL;} (*r).dscp=inet_dsfield_to_dscp(nla_get_u8(nla)<<2);(*r).dscp_mask=inet_dsfield_to_dscp(INET_DSCP_MASK);(*r).dscp_full=1;0 }
unsafe fn fib6_nl2rule_dscp_mask(nla:*const nlattr,r:*mut fib6_rule,extack:*mut netlink_ext_ack)->c_int { if (*r).dscp_full==0{return -EINVAL;} let m=inet_dsfield_to_dscp(nla_get_u8(nla)<<2);if (*r).dscp & !m != 0{return -EINVAL;}(*r).dscp_mask=m;0 }
unsafe fn fib6_nl2rule_flowlabel(tb:*mut *mut nlattr,r:*mut fib6_rule,extack:*mut netlink_ext_ack)->c_int { let v=nla_get_be32(*tb.add(FRA_FLOWLABEL as usize));let m=nla_get_be32(*tb.add(FRA_FLOWLABEL_MASK as usize));if m & !IPV6_FLOWLABEL_MASK != 0 || v & !m != 0{return -EINVAL;}(*r).flowlabel=v;(*r).flowlabel_mask=m;0 }
unsafe fn fib6_rule_configure(rule:*mut fib_rule,skb:*mut sk_buff,frh:*mut fib_rule_hdr,tb:*mut *mut nlattr,extack:*mut netlink_ext_ack)->c_int { let r=rule as *mut fib6_rule;let net=(*rule).fr_net;if !inet_validate_dscp((*frh).tos){return -EINVAL;}(*r).dscp=inet_dsfield_to_dscp((*frh).tos);(*r).dscp_mask=if (*frh).tos!=0{inet_dsfield_to_dscp(INET_DSCP_MASK)}else{0};if !(*tb.add(FRA_DSCP as usize)).is_null(){if fib6_nl2rule_dscp(*tb.add(FRA_DSCP as usize),r,extack)<0{return -EINVAL;}}if !(*tb.add(FRA_DSCP_MASK as usize)).is_null(){if fib6_nl2rule_dscp_mask(*tb.add(FRA_DSCP_MASK as usize),r,extack)<0{return -EINVAL;}}if !(*tb.add(FRA_FLOWLABEL as usize)).is_null()||!(*tb.add(FRA_FLOWLABEL_MASK as usize)).is_null(){if fib6_nl2rule_flowlabel(tb,r,extack)<0{return -EINVAL;}}if (*frh).src_len!=0{(*r).src.addr=nla_get_in6_addr(*tb.add(FRA_SRC as usize));}if (*frh).dst_len!=0{(*r).dst.addr=nla_get_in6_addr(*tb.add(FRA_DST as usize));}(*r).src.plen=(*frh).src_len;(*r).dst.plen=(*frh).dst_len;(*net).ipv6.fib6_has_custom_rules=true;0 }
unsafe fn fib6_rule_delete(rule:*mut fib_rule){let net=(*rule).fr_net;if (*net).ipv6.fib6_rules_require_fldissect!=0&&fib_rule_requires_fldissect(rule){(*net).ipv6.fib6_rules_require_fldissect-=1;}}
unsafe fn fib6_rule_nlmsg_payload(_rule:*mut fib_rule)->usize{nla_total_size(16)+nla_total_size(16)+nla_total_size(1)+nla_total_size(1)+nla_total_size(4)+nla_total_size(4)}
unsafe fn fib6_rule_flush_cache(ops:*mut fib_rules_ops){rt_genid_bump_ipv6((*ops).fro_net);}
pub unsafe fn fib6_rules_init() -> c_int { register_pernet_subsys(&mut fib6_rules_net_ops) }
pub unsafe fn fib6_rules_cleanup() { unregister_pernet_subsys(&mut fib6_rules_net_ops); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
