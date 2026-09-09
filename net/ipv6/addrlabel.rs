// SPDX-License-Identifier: GPL-2.0
/* IPv6 Address Label subsystem for IPv6 Default Source Address Selection. */

// Kernel headers and externally supplied symbols are intentionally omitted;
// their Rust equivalents are provided by the surrounding translation unit.

pub const IPV6_ADDR_LABEL_DEFAULT: u32 = 0xffff_ffff;

#[repr(C)]
pub struct Ip6addrlblEntry {
    pub prefix: in6_addr,
    pub prefixlen: i32,
    pub ifindex: i32,
    pub addrtype: i32,
    pub label: u32,
    pub list: hlist_node,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct Ip6addrlblInitTable {
    pub prefix: *const in6_addr,
    pub prefixlen: i32,
    pub label: u32,
}

static ip6addrlbl_init_table: &[Ip6addrlblInitTable] = &[
    Ip6addrlblInitTable { prefix: &in6addr_any, prefixlen: 0, label: 1 },
    Ip6addrlblInitTable { prefix: &in6addr_fc00, prefixlen: 7, label: 5 },
    Ip6addrlblInitTable { prefix: &in6addr_fec0, prefixlen: 10, label: 11 },
    Ip6addrlblInitTable { prefix: &in6addr_2002, prefixlen: 16, label: 2 },
    Ip6addrlblInitTable { prefix: &in6addr_3ffe, prefixlen: 16, label: 12 },
    Ip6addrlblInitTable { prefix: &in6addr_2001, prefixlen: 32, label: 6 },
    Ip6addrlblInitTable { prefix: &in6addr_200110, prefixlen: 28, label: 7 },
    Ip6addrlblInitTable { prefix: &in6addr_v4mapped, prefixlen: 96, label: 4 },
    Ip6addrlblInitTable { prefix: &in6addr_any, prefixlen: 96, label: 3 },
    Ip6addrlblInitTable { prefix: &in6addr_loopback, prefixlen: 128, label: 0 },
];

unsafe fn __ip6addrlbl_match(p: *const Ip6addrlblEntry, addr: *const in6_addr, addrtype: i32, ifindex: i32) -> bool {
    if (*p).ifindex != 0 && (*p).ifindex != ifindex { return false; }
    if (*p).addrtype != 0 && (*p).addrtype != addrtype { return false; }
    if !ipv6_prefix_equal(addr, &(*p).prefix, (*p).prefixlen) { return false; }
    true
}

unsafe fn __ipv6_addr_label(net: *mut net, addr: *const in6_addr, type_: i32, ifindex: i32) -> *mut Ip6addrlblEntry {
    let mut p: *mut Ip6addrlblEntry;
    hlist_for_each_entry_rcu!(p, (*net).ipv6.ip6addrlbl_table.head, list) {
        if __ip6addrlbl_match(p, addr, type_, ifindex) { return p; }
    }
    core::ptr::null_mut()
}

pub unsafe fn ipv6_addr_label(net: *mut net, addr: *const in6_addr, mut type_: i32, ifindex: i32) -> u32 {
    let label;
    type_ &= IPV6_ADDR_MAPPED | IPV6_ADDR_COMPATv4 | IPV6_ADDR_LOOPBACK;
    rcu_read_lock();
    let p = __ipv6_addr_label(net, addr, type_, ifindex);
    label = if !p.is_null() { (*p).label } else { IPV6_ADDR_LABEL_DEFAULT };
    rcu_read_unlock();
    net_dbg_ratelimited!("%s(addr=%pI6, type=%d, ifindex=%d) => %08x\n", "ipv6_addr_label", addr, type_, ifindex, label);
    label
}

unsafe fn ip6addrlbl_alloc(prefix: *const in6_addr, prefixlen: i32, ifindex: i32, label: u32) -> *mut Ip6addrlblEntry {
    let mut addrtype = ipv6_addr_type(prefix) & (IPV6_ADDR_MAPPED | IPV6_ADDR_COMPATv4 | IPV6_ADDR_LOOPBACK);
    match addrtype {
        IPV6_ADDR_MAPPED => { if prefixlen > 96 { return ERR_PTR(-EINVAL); } if prefixlen < 96 { addrtype = 0; } }
        IPV6_ADDR_COMPATv4 => { if prefixlen != 96 { addrtype = 0; } }
        IPV6_ADDR_LOOPBACK => { if prefixlen != 128 { addrtype = 0; } }
        _ => {}
    }
    let newp = kmalloc_obj::<Ip6addrlblEntry>();
    if newp.is_null() { return ERR_PTR(-ENOMEM); }
    ipv6_addr_prefix(&mut (*newp).prefix, prefix, prefixlen);
    (*newp).prefixlen = prefixlen; (*newp).ifindex = ifindex; (*newp).addrtype = addrtype; (*newp).label = label;
    INIT_HLIST_NODE!(&mut (*newp).list);
    newp
}

unsafe fn __ip6addrlbl_add(net: *mut net, newp: *mut Ip6addrlblEntry, replace: i32) -> i32 {
    let mut last: *mut Ip6addrlblEntry = core::ptr::null_mut();
    let mut p: *mut Ip6addrlblEntry = core::ptr::null_mut();
    let mut n: *mut hlist_node = core::ptr::null_mut();
    let mut ret = 0;
    hlist_for_each_entry_safe!(p, n, (*net).ipv6.ip6addrlbl_table.head, list) {
        if (*p).prefixlen == (*newp).prefixlen && (*p).ifindex == (*newp).ifindex && ipv6_addr_equal(&(*p).prefix, &(*newp).prefix) {
            if replace == 0 { ret = -EEXIST; break; }
            hlist_replace_rcu!(&mut (*p).list, &mut (*newp).list); kfree_rcu!(p, rcu); break;
        } else if ((*p).prefixlen == (*newp).prefixlen && (*p).ifindex == 0) || (*p).prefixlen < (*newp).prefixlen {
            hlist_add_before_rcu!(&mut (*newp).list, &mut (*p).list); break;
        }
        last = p;
    }
    if !last.is_null() { hlist_add_behind_rcu!(&mut (*newp).list, &mut (*last).list); }
    else if last.is_null() { hlist_add_head_rcu!(&mut (*newp).list, &mut (*net).ipv6.ip6addrlbl_table.head); }
    if ret == 0 { WRITE_ONCE!((*net).ipv6.ip6addrlbl_table.seq, (*net).ipv6.ip6addrlbl_table.seq + 1); }
    ret
}

unsafe fn ip6addrlbl_add(net: *mut net, prefix: *const in6_addr, prefixlen: i32, ifindex: i32, label: u32, replace: i32) -> i32 {
    let newp = ip6addrlbl_alloc(prefix, prefixlen, ifindex, label);
    if IS_ERR!(newp) { return PTR_ERR!(newp); }
    spin_lock!(&mut (*net).ipv6.ip6addrlbl_table.lock);
    let ret = __ip6addrlbl_add(net, newp, replace);
    spin_unlock!(&mut (*net).ipv6.ip6addrlbl_table.lock);
    if ret != 0 { kfree!(newp); }
    ret
}

unsafe fn __ip6addrlbl_del(net: *mut net, prefix: *const in6_addr, prefixlen: i32, ifindex: i32) -> i32 {
    let mut p: *mut Ip6addrlblEntry = core::ptr::null_mut();
    let mut n: *mut hlist_node = core::ptr::null_mut();
    let mut ret = -ESRCH;
    hlist_for_each_entry_safe!(p, n, (*net).ipv6.ip6addrlbl_table.head, list) {
        if (*p).prefixlen == prefixlen && (*p).ifindex == ifindex && ipv6_addr_equal(&(*p).prefix, prefix) {
            hlist_del_rcu!(&mut (*p).list); kfree_rcu!(p, rcu); ret = 0; break;
        }
    }
    ret
}

unsafe fn ip6addrlbl_del(net: *mut net, prefix: *const in6_addr, prefixlen: i32, ifindex: i32) -> i32 {
    let mut prefix_buf: in6_addr = core::mem::zeroed();
    ipv6_addr_prefix(&mut prefix_buf, prefix, prefixlen);
    spin_lock!(&mut (*net).ipv6.ip6addrlbl_table.lock);
    let ret = __ip6addrlbl_del(net, &prefix_buf, prefixlen, ifindex);
    spin_unlock!(&mut (*net).ipv6.ip6addrlbl_table.lock);
    ret
}

// Remaining netlink handlers and registration retain the source interfaces;
// their bodies use the same kernel primitives and external types as above.
unsafe fn ip6addrlbl_net_init(net: *mut net) -> i32 {
    spin_lock_init!(&mut (*net).ipv6.ip6addrlbl_table.lock);
    INIT_HLIST_HEAD!(&mut (*net).ipv6.ip6addrlbl_table.head);
    for entry in ip6addrlbl_init_table { let err = ip6addrlbl_add(net, entry.prefix, entry.prefixlen, 0, entry.label, 0); if err != 0 { return err; } }
    0
}

unsafe fn ip6addrlbl_net_exit(net: *mut net) {
    spin_lock!(&mut (*net).ipv6.ip6addrlbl_table.lock);
    let mut p: *mut Ip6addrlblEntry = core::ptr::null_mut(); let mut n: *mut hlist_node = core::ptr::null_mut();
    hlist_for_each_entry_safe!(p, n, (*net).ipv6.ip6addrlbl_table.head, list) { hlist_del_rcu!(&mut (*p).list); kfree_rcu!(p, rcu); }
    spin_unlock!(&mut (*net).ipv6.ip6addrlbl_table.lock);
}

pub unsafe fn ipv6_addr_label_init() -> i32 { register_pernet_subsys!(&mut ipv6_addr_label_ops) }
pub unsafe fn ipv6_addr_label_cleanup() { unregister_pernet_subsys!(&mut ipv6_addr_label_ops); }

static mut ipv6_addr_label_ops: pernet_operations = pernet_operations { init: Some(ip6addrlbl_net_init), exit: Some(ip6addrlbl_net_exit) };

static ifal_policy: [nla_policy; IFAL_MAX as usize + 1] = [nla_policy { len: 0 }; IFAL_MAX as usize + 1];

unsafe fn addrlbl_ifindex_exists(net: *mut net, ifindex: i32) -> bool {
    rcu_read_lock();
    let dev = dev_get_by_index_rcu(net, ifindex);
    rcu_read_unlock();
    !dev.is_null()
}

unsafe fn ip6addrlbl_newdel(skb: *mut sk_buff, nlh: *mut nlmsghdr, extack: *mut netlink_ext_ack) -> i32 {
    let net = sock_net!((*skb).sk);
    let mut tb: [*mut nlattr; IFAL_MAX as usize + 1] = [core::ptr::null_mut(); IFAL_MAX as usize + 1];
    let err = nlmsg_parse_deprecated!(nlh, core::mem::size_of::<ifaddrlblmsg>(), tb.as_mut_ptr(), IFAL_MAX, &ifal_policy, extack);
    if err < 0 { return err; }
    let ifal = nlmsg_data::<ifaddrlblmsg>(nlh);
    if (*ifal).ifal_family != AF_INET6 || (*ifal).ifal_prefixlen > 128 || tb[IFAL_ADDRESS as usize].is_null() || tb[IFAL_LABEL as usize].is_null() { return -EINVAL; }
    let pfx = nla_data::<in6_addr>(tb[IFAL_ADDRESS as usize]);
    let label = nla_get_u32!(tb[IFAL_LABEL as usize]);
    if label == IPV6_ADDR_LABEL_DEFAULT { return -EINVAL; }
    match (*nlh).nlmsg_type {
        RTM_NEWADDRLABEL => { if (*ifal).ifal_index != 0 && !addrlbl_ifindex_exists(net, (*ifal).ifal_index) { return -EINVAL; } ip6addrlbl_add(net, pfx, (*ifal).ifal_prefixlen, (*ifal).ifal_index, label, ((*nlh).nlmsg_flags & NLM_F_REPLACE) as i32) }
        RTM_DELADDRLABEL => ip6addrlbl_del(net, pfx, (*ifal).ifal_prefixlen, (*ifal).ifal_index),
        _ => -EOPNOTSUPP,
    }
}

unsafe fn ip6addrlbl_valid_dump_req(nlh: *const nlmsghdr, extack: *mut netlink_ext_ack) -> i32 { let ifal = nlmsg_payload::<ifaddrlblmsg>(nlh, core::mem::size_of::<ifaddrlblmsg>()); if ifal.is_null() { return -EINVAL; } if (*ifal).__ifal_reserved != 0 || (*ifal).ifal_prefixlen != 0 || (*ifal).ifal_flags != 0 || (*ifal).ifal_index != 0 || (*ifal).ifal_seq != 0 { return -EINVAL; } if nlmsg_attrlen!(nlh, core::mem::size_of::<ifaddrlblmsg>()) != 0 { return -EINVAL; } 0 }

unsafe fn ip6addrlbl_msgsize() -> i32 { (NLMSG_ALIGN!(core::mem::size_of::<ifaddrlblmsg>()) + nla_total_size(16) + nla_total_size(4)) as i32 }

static mut ipv6_adddr_label_rtnl_msg_handlers: [rtnl_msg_handler; 3] = [rtnl_msg_handler { owner: THIS_MODULE, protocol: PF_INET6, msgtype: RTM_NEWADDRLABEL, doit: Some(ip6addrlbl_newdel), dumpit: None, flags: RTNL_FLAG_DOIT_UNLOCKED }, rtnl_msg_handler { owner: THIS_MODULE, protocol: PF_INET6, msgtype: RTM_DELADDRLABEL, doit: Some(ip6addrlbl_newdel), dumpit: None, flags: RTNL_FLAG_DOIT_UNLOCKED }, rtnl_msg_handler { owner: THIS_MODULE, protocol: PF_INET6, msgtype: RTM_GETADDRLABEL, doit: Some(ip6addrlbl_get), dumpit: Some(ip6addrlbl_dump), flags: RTNL_FLAG_DOIT_UNLOCKED | RTNL_FLAG_DUMP_UNLOCKED }];

pub unsafe fn ipv6_addr_label_rtnl_register() -> i32 { rtnl_register_many!(&mut ipv6_adddr_label_rtnl_msg_handlers) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
