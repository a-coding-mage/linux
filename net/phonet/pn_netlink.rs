// SPDX-License-Identifier: GPL-2.0-only
/*
 * File: pn_netlink.c
 *
 * Phonet netlink interface
 *
 * Copyright (C) 2008 Nokia Corporation.
 *
 * Authors: Sakari Ailus <sakari.ailus@nokia.com>
 *          Remi Denis-Courmont
 */

/* Device address handling */

unsafe extern "C" {
    fn fill_addr(skb: *mut sk_buff, ifindex: u32, addr: u8,
                 portid: u32, seq: u32, event: i32) -> i32;
}

pub unsafe extern "C" fn phonet_address_notify(
    net: *mut net,
    event: i32,
    ifindex: u32,
    addr: u8,
) {
    let mut skb: *mut sk_buff;
    let mut err: i32 = -ENOBUFS;

    skb = nlmsg_new(
        NLMSG_ALIGN(core::mem::size_of::<ifaddrmsg>()) + nla_total_size(1),
        GFP_KERNEL,
    );
    if skb.is_null() {
        return rtnl_set_sk_err(net, RTNLGRP_PHONET_IFADDR, err);
    }

    err = fill_addr(skb, ifindex, addr, 0, 0, event);
    if err < 0 {
        WARN_ON(err == -EMSGSIZE);
        kfree_skb(skb);
        return rtnl_set_sk_err(net, RTNLGRP_PHONET_IFADDR, err);
    }

    rtnl_notify(skb, net, 0, RTNLGRP_PHONET_IFADDR, core::ptr::null_mut(), GFP_KERNEL);
}

static const ifa_phonet_policy_entry: nla_policy = nla_policy { type_: NLA_U8 };

unsafe extern "C" fn addr_doit(
    skb: *mut sk_buff,
    nlh: *mut nlmsghdr,
    extack: *mut netlink_ext_ack,
) -> i32 {
    let net = sock_net((*skb).sk);
    let mut tb: [*mut nlattr; IFA_MAX as usize + 1] = [core::ptr::null_mut(); IFA_MAX as usize + 1];
    let mut dev: *mut net_device;
    let ifm: *mut ifaddrmsg;
    let mut err: i32;
    let pnaddr: u8;

    if !netlink_capable(skb, CAP_NET_ADMIN) || !netlink_capable(skb, CAP_SYS_ADMIN) {
        return -EPERM;
    }
    err = nlmsg_parse_deprecated(nlh, core::mem::size_of::<ifaddrmsg>(), tb.as_mut_ptr(), IFA_MAX,
                                 &ifa_phonet_policy, extack);
    if err < 0 { return err; }
    ifm = nlmsg_data(nlh);
    if tb[IFA_LOCAL as usize].is_null() { return -EINVAL; }
    pnaddr = nla_get_u8(tb[IFA_LOCAL as usize]);
    if pnaddr & 3 != 0 { return -EINVAL; }

    rcu_read_lock();
    dev = dev_get_by_index_rcu(net, (*ifm).ifa_index);
    if dev.is_null() {
        rcu_read_unlock();
        return -ENODEV;
    }
    if (*nlh).nlmsg_type == RTM_NEWADDR { err = phonet_address_add(dev, pnaddr); }
    else { err = phonet_address_del(dev, pnaddr); }
    rcu_read_unlock();
    if err == 0 { phonet_address_notify(net, (*nlh).nlmsg_type, (*ifm).ifa_index, pnaddr); }
    err
}

unsafe extern "C" fn fill_addr(skb: *mut sk_buff, ifindex: u32, addr: u8,
                                portid: u32, seq: u32, event: i32) -> i32 {
    let nlh = nlmsg_put(skb, portid, seq, event, core::mem::size_of::<ifaddrmsg>(), 0);
    if nlh.is_null() { return -EMSGSIZE; }
    let ifm: *mut ifaddrmsg = nlmsg_data(nlh);
    (*ifm).ifa_family = AF_PHONET;
    (*ifm).ifa_prefixlen = 0;
    (*ifm).ifa_flags = IFA_F_PERMANENT;
    (*ifm).ifa_scope = RT_SCOPE_LINK;
    (*ifm).ifa_index = ifindex;
    if nla_put_u8(skb, IFA_LOCAL, addr) != 0 {
        nlmsg_cancel(skb, nlh);
        return -EMSGSIZE;
    }
    nlmsg_end(skb, nlh);
    0
}

unsafe extern "C" fn getaddr_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32 {
    let mut addr_idx = 0;
    let mut addr_start_idx = (*cb).args[1];
    let mut dev_idx = 0;
    let dev_start_idx = (*cb).args[0];
    let pndevs = phonet_device_list(sock_net((*skb).sk));
    let mut err = 0;
    rcu_read_lock();
    let mut pnd = (*pndevs).list.next as *mut phonet_device;
    while pnd != &mut (*pndevs).list as *mut _ as *mut phonet_device {
        if dev_idx > dev_start_idx { addr_start_idx = 0; }
        dev_idx += 1;
        if dev_idx - 1 >= dev_start_idx {
            addr_idx = 0;
            let addrs = (*pnd).addrs;
            for addr in 0..64u8 {
                if (addrs[addr as usize / 64] & (1u64 << (addr % 64))) == 0 { continue; }
                addr_idx += 1;
                if addr_idx - 1 < addr_start_idx { continue; }
                err = fill_addr(skb, READ_ONCE((*(*pnd).netdev).ifindex), addr << 2,
                                NETLINK_CB((*cb).skb).portid, (*(*cb).nlh).nlmsg_seq, RTM_NEWADDR);
                if err < 0 { break; }
            }
        }
        if err < 0 { break; }
        pnd = (*pnd).list.next as *mut phonet_device;
    }
    rcu_read_unlock();
    (*cb).args[0] = dev_idx;
    (*cb).args[1] = addr_idx;
    err
}

/* Routes handling */

unsafe extern "C" fn fill_route(skb: *mut sk_buff, ifindex: u32, dst: u8,
                                 portid: u32, seq: u32, event: i32) -> i32 {
    let nlh = nlmsg_put(skb, portid, seq, event, core::mem::size_of::<rtmsg>(), 0);
    if nlh.is_null() { return -EMSGSIZE; }
    let rtm: *mut rtmsg = nlmsg_data(nlh);
    (*rtm).rtm_family = AF_PHONET;
    (*rtm).rtm_dst_len = 6;
    (*rtm).rtm_src_len = 0;
    (*rtm).rtm_tos = 0;
    (*rtm).rtm_table = RT_TABLE_MAIN;
    (*rtm).rtm_protocol = RTPROT_STATIC;
    (*rtm).rtm_scope = RT_SCOPE_UNIVERSE;
    (*rtm).rtm_type = RTN_UNICAST;
    (*rtm).rtm_flags = 0;
    if nla_put_u8(skb, RTA_DST, dst) != 0 || nla_put_u32(skb, RTA_OIF, ifindex) != 0 {
        nlmsg_cancel(skb, nlh);
        return -EMSGSIZE;
    }
    nlmsg_end(skb, nlh);
    0
}

pub unsafe extern "C" fn rtm_phonet_notify(net: *mut net, event: i32, ifindex: u32, dst: u8) {
    let skb = nlmsg_new(NLMSG_ALIGN(core::mem::size_of::<rtmsg>()) + nla_total_size(1) + nla_total_size(4), GFP_KERNEL);
    if skb.is_null() { return rtnl_set_sk_err(net, RTNLGRP_PHONET_ROUTE, -ENOBUFS); }
    let err = fill_route(skb, ifindex, dst, 0, 0, event);
    if err < 0 {
        WARN_ON(err == -EMSGSIZE);
        kfree_skb(skb);
        return rtnl_set_sk_err(net, RTNLGRP_PHONET_ROUTE, err);
    }
    rtnl_notify(skb, net, 0, RTNLGRP_PHONET_ROUTE, core::ptr::null_mut(), GFP_KERNEL);
}

unsafe extern "C" fn route_doit(skb: *mut sk_buff, nlh: *mut nlmsghdr,
                                 extack: *mut netlink_ext_ack) -> i32 {
    let net = sock_net((*skb).sk);
    let mut tb: [*mut nlattr; RTA_MAX as usize + 1] = [core::ptr::null_mut(); RTA_MAX as usize + 1];
    let mut sync_needed = false;
    let mut err = nlmsg_parse_deprecated(nlh, core::mem::size_of::<rtmsg>(), tb.as_mut_ptr(), RTA_MAX, &rtm_phonet_policy, extack);
    if !netlink_capable(skb, CAP_NET_ADMIN) || !netlink_capable(skb, CAP_SYS_ADMIN) { return -EPERM; }
    if err < 0 { return err; }
    let rtm: *mut rtmsg = nlmsg_data(nlh);
    if (*rtm).rtm_table != RT_TABLE_MAIN || (*rtm).rtm_type != RTN_UNICAST || tb[RTA_DST as usize].is_null() || tb[RTA_OIF as usize].is_null() { return -EINVAL; }
    let dst = nla_get_u8(tb[RTA_DST as usize]);
    if dst & 3 != 0 { return -EINVAL; }
    let ifindex = nla_get_u32(tb[RTA_OIF as usize]);
    rcu_read_lock();
    let dev = dev_get_by_index_rcu(net, ifindex);
    if dev.is_null() { rcu_read_unlock(); return -ENODEV; }
    if (*nlh).nlmsg_type == RTM_NEWROUTE { err = phonet_route_add(dev, dst); }
    else { err = phonet_route_del(dev, dst); if err == 0 { sync_needed = true; } }
    rcu_read_unlock();
    if sync_needed { synchronize_rcu(); dev_put(dev); }
    if err == 0 { rtm_phonet_notify(net, (*nlh).nlmsg_type, ifindex, dst); }
    err
}

unsafe extern "C" fn route_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32 {
    let net = sock_net((*skb).sk);
    let mut err = 0;
    let mut addr = (*cb).args[0] as u8;
    rcu_read_lock();
    while addr < 64 {
        let dev = phonet_route_get_rcu(net, addr << 2);
        if !dev.is_null() {
            err = fill_route(skb, READ_ONCE((*dev).ifindex), addr << 2, NETLINK_CB((*cb).skb).portid, (*(*cb).nlh).nlmsg_seq, RTM_NEWROUTE);
            if err < 0 { break; }
        }
        addr += 1;
    }
    rcu_read_unlock();
    (*cb).args[0] = addr as i32;
    err
}

static phonet_rtnl_msg_handlers: [rtnl_msg_handler; 6] = [
    rtnl_msg_handler { owner: THIS_MODULE, protocol: PF_PHONET, msgtype: RTM_NEWADDR, doit: Some(addr_doit), dumpit: None, flags: RTNL_FLAG_DOIT_UNLOCKED },
    rtnl_msg_handler { owner: THIS_MODULE, protocol: PF_PHONET, msgtype: RTM_DELADDR, doit: Some(addr_doit), dumpit: None, flags: RTNL_FLAG_DOIT_UNLOCKED },
    rtnl_msg_handler { owner: THIS_MODULE, protocol: PF_PHONET, msgtype: RTM_GETADDR, doit: None, dumpit: Some(getaddr_dumpit), flags: RTNL_FLAG_DUMP_UNLOCKED },
    rtnl_msg_handler { owner: THIS_MODULE, protocol: PF_PHONET, msgtype: RTM_NEWROUTE, doit: Some(route_doit), dumpit: None, flags: RTNL_FLAG_DOIT_UNLOCKED },
    rtnl_msg_handler { owner: THIS_MODULE, protocol: PF_PHONET, msgtype: RTM_DELROUTE, doit: Some(route_doit), dumpit: None, flags: RTNL_FLAG_DOIT_UNLOCKED },
    rtnl_msg_handler { owner: THIS_MODULE, protocol: PF_PHONET, msgtype: RTM_GETROUTE, doit: None, dumpit: Some(route_dumpit), flags: RTNL_FLAG_DUMP_UNLOCKED },
];

pub unsafe extern "C" fn phonet_netlink_register() -> i32 {
    rtnl_register_many(&phonet_rtnl_msg_handlers)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
