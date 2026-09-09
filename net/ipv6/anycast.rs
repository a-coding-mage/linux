// SPDX-License-Identifier: GPL-2.0-or-later
/* Anycast support for IPv6; Linux INET6 implementation. */
// Kernel dependencies are supplied by other translation units.

pub const IN6_ADDR_HSIZE_SHIFT: u32 = 8;
pub const IN6_ADDR_HSIZE: usize = 1usize << IN6_ADDR_HSIZE_SHIFT;

static mut inet6_acaddr_lst: [hlist_head; IN6_ADDR_HSIZE] = [hlist_head::default(); IN6_ADDR_HSIZE];
static mut acaddr_hash_lock: spinlock_t = spinlock_t::default();

unsafe extern "C" {
    fn ipv6_dev_ac_dec(dev: *mut net_device, addr: *const in6_addr) -> c_int;
}

unsafe fn inet6_acaddr_hash(net: *const net, addr: *const in6_addr) -> u32 {
    let val = __ipv6_addr_jhash(addr, net_hash_mix(net));
    hash_32(val, IN6_ADDR_HSIZE_SHIFT)
}

pub unsafe fn ipv6_sock_ac_join(sk: *mut sock, ifindex: c_int, addr: *const in6_addr) -> c_int {
    let np = inet6_sk(sk);
    let mut pac: *mut ipv6_ac_socklist = core::ptr::null_mut();
    let net = sock_net(sk);
    let mut dev_tracker = netdevice_tracker::default();
    let mut dev: *mut net_device = core::ptr::null_mut();
    let mut err = 0;
    let mut ishost: bool;
    if !ns_capable((*net).user_ns, CAP_NET_ADMIN) { return -EPERM; }
    if ipv6_addr_is_multicast(addr) { return -EINVAL; }
    if ifindex != 0 { dev = netdev_get_by_index(net, ifindex, &mut dev_tracker, GFP_KERNEL); }
    if ipv6_chk_addr_and_flags(net, addr, dev, true, 0, IFA_F_TENTATIVE) { err = -EINVAL; goto_error!(); }
    pac = sock_kmalloc(sk, core::mem::size_of::<ipv6_ac_socklist>(), GFP_KERNEL);
    if pac.is_null() { err = -ENOMEM; goto_error!(); }
    (*pac).acl_next = core::ptr::null_mut(); (*pac).acl_addr = *addr;
    ishost = !READ_ONCE((*(*net).ipv6).devconf_all.forwarding);
    if ifindex == 0 {
        let mut rt: *mut rt6_info;
        rcu_read_lock(); rt = rt6_lookup(net, addr, core::ptr::null(), 0, core::ptr::null(), 0);
        if !rt.is_null() { dev = dst_dev_rcu(&mut (*rt).dst); netdev_hold(dev, &mut dev_tracker, GFP_ATOMIC); ip6_rt_put(rt); }
        else if ishost { rcu_read_unlock(); err = -EADDRNOTAVAIL; goto_error!(); }
        else { dev = netdev_get_by_flags_rcu(net, &mut dev_tracker, IFF_UP, IFF_UP | IFF_LOOPBACK); }
        rcu_read_unlock();
    }
    if dev.is_null() { err = -ENODEV; goto_error!(); }
    let idev = in6_dev_get(dev); if idev.is_null() { err = if ifindex != 0 { -ENODEV } else { -EADDRNOTAVAIL }; goto_error!(); }
    ishost = !READ_ONCE((*idev).cnf.forwarding); (*pac).acl_ifindex = (*dev).ifindex;
    if !ipv6_chk_prefix(addr, dev) && ishost { err = -EADDRNOTAVAIL; goto_error_idev!(); }
    err = __ipv6_dev_ac_inc(idev, addr);
    if err == 0 { (*pac).acl_next = (*np).ipv6_ac_list; (*np).ipv6_ac_list = pac; pac = core::ptr::null_mut(); }
    in6_dev_put(idev);
    netdev_put(dev, &mut dev_tracker);
    if !pac.is_null() { sock_kfree_s(sk, pac, core::mem::size_of::<ipv6_ac_socklist>()); }
    err
}

pub unsafe fn ipv6_sock_ac_drop(sk: *mut sock, ifindex: c_int, addr: *const in6_addr) -> c_int {
    let np = inet6_sk(sk); let net = sock_net(sk); let mut prev = core::ptr::null_mut(); let mut pac = (*np).ipv6_ac_list;
    while !pac.is_null() { if (ifindex == 0 || (*pac).acl_ifindex == ifindex) && ipv6_addr_equal(&(*pac).acl_addr, addr) { break; } prev = pac; pac = (*pac).acl_next; }
    if pac.is_null() { return -ENOENT; }
    if !prev.is_null() { (*prev).acl_next = (*pac).acl_next; } else { (*np).ipv6_ac_list = (*pac).acl_next; }
    let dev = dev_get_by_index(net, (*pac).acl_ifindex); if !dev.is_null() { ipv6_dev_ac_dec(dev, &(*pac).acl_addr); dev_put(dev); }
    sock_kfree_s(sk, pac, core::mem::size_of::<ipv6_ac_socklist>()); 0
}

pub unsafe fn __ipv6_sock_ac_close(sk: *mut sock) {
    let np = inet6_sk(sk); let net = sock_net(sk); let mut dev = core::ptr::null_mut(); let mut prev_index = 0; let mut pac = (*np).ipv6_ac_list; (*np).ipv6_ac_list = core::ptr::null_mut();
    while !pac.is_null() { let next = (*pac).acl_next; if (*pac).acl_ifindex != prev_index { dev_put(dev); dev = dev_get_by_index(net, (*pac).acl_ifindex); prev_index = (*pac).acl_ifindex; } if !dev.is_null() { ipv6_dev_ac_dec(dev, &(*pac).acl_addr); } sock_kfree_s(sk, pac, core::mem::size_of::<ipv6_ac_socklist>()); pac = next; } dev_put(dev);
}
pub unsafe fn ipv6_sock_ac_close(sk: *mut sock) { if !(*inet6_sk(sk)).ipv6_ac_list.is_null() { __ipv6_sock_ac_close(sk); } }

unsafe fn ipv6_add_acaddr_hash(net: *mut net, aca: *mut ifacaddr6) { let hash = inet6_acaddr_hash(net, &(*aca).aca_addr) as usize; spin_lock_bh(&mut acaddr_hash_lock); hlist_add_head_rcu(&mut (*aca).aca_addr_lst, &mut inet6_acaddr_lst[hash]); spin_unlock_bh(&mut acaddr_hash_lock); }
unsafe fn ipv6_del_acaddr_hash(aca: *mut ifacaddr6) { spin_lock_bh(&mut acaddr_hash_lock); hlist_del_init_rcu(&mut (*aca).aca_addr_lst); spin_unlock_bh(&mut acaddr_hash_lock); }
unsafe fn aca_get(aca: *mut ifacaddr6) { refcount_inc(&mut (*aca).aca_refcnt); }
unsafe fn aca_free_rcu(h: *mut rcu_head) { let aca = container_of!(h, ifacaddr6, rcu); fib6_info_release((*aca).aca_rt); kfree(aca); }
unsafe fn aca_put(ac: *mut ifacaddr6) { if refcount_dec_and_test(&mut (*ac).aca_refcnt) { call_rcu_hurry(&mut (*ac).rcu, aca_free_rcu); } }
unsafe fn aca_alloc(f6i: *mut fib6_info, addr: *const in6_addr) -> *mut ifacaddr6 { let aca = kzalloc_obj::<ifacaddr6>(GFP_ATOMIC); if aca.is_null() { return core::ptr::null_mut(); } (*aca).aca_addr = *addr; fib6_info_hold(f6i); (*aca).aca_rt = f6i; INIT_HLIST_NODE(&mut (*aca).aca_addr_lst); (*aca).aca_users = 1; (*aca).aca_cstamp = jiffies; (*aca).aca_tstamp = jiffies; refcount_set(&mut (*aca).aca_refcnt, 1); aca }

pub unsafe fn __ipv6_dev_ac_inc(idev: *mut inet6_dev, addr: *const in6_addr) -> c_int { write_lock_bh(&mut (*idev).lock); if (*idev).dead { write_unlock_bh(&mut (*idev).lock); return -ENODEV; } let mut aca = (*idev).ac_list; while !aca.is_null() { if ipv6_addr_equal(&(*aca).aca_addr, addr) { (*aca).aca_users += 1; write_unlock_bh(&mut (*idev).lock); return 0; } aca = (*aca).aca_next; } let net = dev_net((*idev).dev); let f6i = addrconf_f6i_alloc(net, idev, addr, true, GFP_ATOMIC, core::ptr::null_mut()); if IS_ERR(f6i) { let e = PTR_ERR(f6i); write_unlock_bh(&mut (*idev).lock); return e; } aca = aca_alloc(f6i, addr); if aca.is_null() { fib6_info_release(f6i); write_unlock_bh(&mut (*idev).lock); return -ENOMEM; } aca_get(aca); (*aca).aca_next = (*idev).ac_list; rcu_assign_pointer(&mut (*idev).ac_list, aca); ipv6_add_acaddr_hash(net, aca); write_unlock_bh(&mut (*idev).lock); ip6_ins_rt(net, f6i); addrconf_join_solict((*idev).dev, &(*aca).aca_addr); inet6_ifacaddr_notify((*idev).dev, aca, RTM_NEWANYCAST); aca_put(aca); 0 }

pub unsafe fn __ipv6_dev_ac_dec(idev: *mut inet6_dev, addr: *const in6_addr) -> c_int { write_lock_bh(&mut (*idev).lock); let mut prev = core::ptr::null_mut(); let mut aca = (*idev).ac_list; while !aca.is_null() { if ipv6_addr_equal(&(*aca).aca_addr, addr) { break; } prev = aca; aca = (*aca).aca_next; } if aca.is_null() { write_unlock_bh(&mut (*idev).lock); return -ENOENT; } (*aca).aca_users -= 1; if (*aca).aca_users > 0 { write_unlock_bh(&mut (*idev).lock); return 0; } if !prev.is_null() { rcu_assign_pointer(&mut (*prev).aca_next, (*aca).aca_next); } else { rcu_assign_pointer(&mut (*idev).ac_list, (*aca).aca_next); } write_unlock_bh(&mut (*idev).lock); ipv6_del_acaddr_hash(aca); addrconf_leave_solict(idev, &(*aca).aca_addr); ip6_del_rt(dev_net((*idev).dev), (*aca).aca_rt, false); inet6_ifacaddr_notify((*idev).dev, aca, RTM_DELANYCAST); aca_put(aca); 0 }

unsafe fn inet6_ifacaddr_notify(dev: *mut net_device, ifaca: *const ifacaddr6, event: c_int) { let net = dev_net(dev); let mut args = inet6_fill_args { event, netnsid: -1 }; let skb = nlmsg_new(NLMSG_ALIGN(core::mem::size_of::<ifaddrmsg>()) + nla_total_size(core::mem::size_of::<in6_addr>()) + nla_total_size(core::mem::size_of::<ifa_cacheinfo>()), GFP_KERNEL); if skb.is_null() { rtnl_set_sk_err(net, RTNLGRP_IPV6_ACADDR, -ENOMEM); return; } let err = inet6_fill_ifacaddr(skb, ifaca, &mut args); if err < 0 { nlmsg_free(skb); rtnl_set_sk_err(net, RTNLGRP_IPV6_ACADDR, err); return; } rtnl_notify(skb, net, 0, RTNLGRP_IPV6_ACADDR, core::ptr::null_mut(), GFP_KERNEL); }
unsafe fn ipv6_dev_ac_dec(dev: *mut net_device, addr: *const in6_addr) -> c_int { let idev = in6_dev_get(dev); if idev.is_null() { return -ENODEV; } let e = __ipv6_dev_ac_dec(idev, addr); in6_dev_put(idev); e }
pub unsafe fn ipv6_ac_destroy_dev(idev: *mut inet6_dev) { write_lock_bh(&mut (*idev).lock); loop { let aca = (*idev).ac_list; if aca.is_null() { break; } rcu_assign_pointer(&mut (*idev).ac_list, (*aca).aca_next); write_unlock_bh(&mut (*idev).lock); ipv6_del_acaddr_hash(aca); addrconf_leave_solict(idev, &(*aca).aca_addr); ip6_del_rt(dev_net((*idev).dev), (*aca).aca_rt, false); aca_put(aca); write_lock_bh(&mut (*idev).lock); } write_unlock_bh(&mut (*idev).lock); }

pub unsafe fn ipv6_chk_acast_addr(net: *mut net, dev: *mut net_device, addr: *const in6_addr) -> bool { rcu_read_lock(); let found = if !dev.is_null() { ipv6_chk_acast_dev(dev, addr) } else { let hash = inet6_acaddr_hash(net, addr) as usize; let mut aca = inet6_acaddr_lst[hash].first; let mut yes = false; while !aca.is_null() { let nh = fib6_info_nh_dev((*aca).aca_rt); if !nh.is_null() && net_eq(dev_net(nh), net) && ipv6_addr_equal(&(*aca).aca_addr, addr) { yes = true; break; } aca = (*aca).aca_addr_lst.next; } yes }; rcu_read_unlock(); found }
unsafe fn ipv6_chk_acast_dev(dev: *mut net_device, addr: *const in6_addr) -> bool { let idev = __in6_dev_get(dev); let mut aca = if !idev.is_null() { (*idev).ac_list } else { core::ptr::null_mut() }; while !aca.is_null() { if ipv6_addr_equal(&(*aca).aca_addr, addr) { return true; } aca = (*aca).aca_next; } false }
pub unsafe fn ipv6_chk_acast_addr_src(net: *mut net, dev: *mut net_device, addr: *const in6_addr) -> bool { ipv6_chk_acast_addr(net, if ipv6_addr_type(addr) & IPV6_ADDR_LINKLOCAL != 0 { dev } else { core::ptr::null_mut() }, addr) }

pub unsafe fn ipv6_anycast_init() -> c_int { for i in 0..IN6_ADDR_HSIZE { INIT_HLIST_HEAD(&mut inet6_acaddr_lst[i]); } 0 }
pub unsafe fn ipv6_anycast_cleanup() { spin_lock_bh(&mut acaddr_hash_lock); for i in 0..IN6_ADDR_HSIZE { WARN_ON(!hlist_empty(&inet6_acaddr_lst[i])); } spin_unlock_bh(&mut acaddr_hash_lock); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
