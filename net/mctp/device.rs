// SPDX-License-Identifier: GPL-2.0
/*
 * Management Component Transport Protocol (MCTP) - device implementation.
 *
 * Copyright (c) 2021 Code Construct
 * Copyright (c) 2021 Google
 */

// Kernel headers and symbols are supplied by the surrounding translation.

#[repr(C)]
pub struct mctp_dump_cb {
    pub ifindex: ::core::ffi::c_ulong,
    pub a_idx: usize,
}

/* unlocked: caller must hold rcu_read_lock.
 * Returned mctp_dev has its refcount incremented, or NULL if unset.
 */
pub unsafe fn __mctp_dev_get(dev: *const net_device) -> *mut mctp_dev {
    let mdev = rcu_dereference((*dev).mctp_ptr);
    if !mdev.is_null() && !refcount_inc_not_zero(&mut (*mdev).refs) {
        return core::ptr::null_mut();
    }
    mdev
}

/* Returned mctp_dev does not have refcount incremented. The returned pointer
 * remains live while rtnl_lock is held, as that prevents mctp_unregister()
 */
pub unsafe fn mctp_dev_get_rtnl(dev: *const net_device) -> *mut mctp_dev {
    rtnl_dereference((*dev).mctp_ptr)
}

unsafe fn mctp_addrinfo_size() -> i32 {
    (NLMSG_ALIGN(core::mem::size_of::<ifaddrmsg>())
        + nla_total_size(1)
        + nla_total_size(1)) as i32
}

/* flag should be NLM_F_MULTI for dump calls */
unsafe fn mctp_fill_addrinfo(skb: *mut sk_buff, mdev: *mut mctp_dev, eid: mctp_eid_t,
                             msg_type: i32, portid: u32, seq: u32, flag: i32) -> i32 {
    let nlh = nlmsg_put(skb, portid, seq, msg_type, core::mem::size_of::<ifaddrmsg>() as u32, flag);
    if nlh.is_null() { return -EMSGSIZE; }
    let hdr = nlmsg_data(nlh) as *mut ifaddrmsg;
    core::ptr::write_bytes(hdr, 0, 1);
    (*hdr).ifa_family = AF_MCTP as u8;
    (*hdr).ifa_prefixlen = 0;
    (*hdr).ifa_flags = 0;
    (*hdr).ifa_scope = 0;
    (*hdr).ifa_index = (*(*mdev).dev).ifindex;
    if nla_put_u8(skb, IFA_LOCAL, eid) != 0 || nla_put_u8(skb, IFA_ADDRESS, eid) != 0 {
        nlmsg_cancel(skb, nlh);
        return -EMSGSIZE;
    }
    nlmsg_end(skb, nlh);
    0
}

unsafe fn mctp_dump_dev_addrinfo(mdev: *mut mctp_dev, skb: *mut sk_buff,
                                 cb: *mut netlink_callback) -> i32 {
    let mcb = (*cb).ctx as *mut mctp_dump_cb;
    let portid = NETLINK_CB((*cb).skb).portid;
    let seq = (*(*cb).nlh).nlmsg_seq;
    let mut rc = 0;
    while (*mcb).a_idx < (*mdev).num_addrs {
        rc = mctp_fill_addrinfo(skb, mdev, *(*mdev).addrs.add((*mcb).a_idx), RTM_NEWADDR,
                                portid, seq, NLM_F_MULTI);
        if rc < 0 { break; }
        (*mcb).a_idx += 1;
    }
    rc
}

unsafe fn mctp_dump_addrinfo(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32 {
    let mcb = (*cb).ctx as *mut mctp_dump_cb;
    let net = sock_net((*skb).sk);
    let mut dev: *mut net_device = core::ptr::null_mut();
    let hdr = nlmsg_payload((*cb).nlh, core::mem::size_of::<ifaddrmsg>());
    let mut ifindex = 0;
    if !hdr.is_null() { ifindex = (*hdr as *mut ifaddrmsg).read().ifa_index; }
    else if (*cb).strict_check { NL_SET_ERR_MSG((*cb).extack, "mctp: Invalid header for addr dump request"); return -EINVAL; }
    rcu_read_lock();
    for_each_netdev_dump(net, dev, (*mcb).ifindex) {
        if ifindex != 0 && ifindex != (*dev).ifindex { continue; }
        let mdev = __mctp_dev_get(dev);
        if mdev.is_null() { continue; }
        let rc = mctp_dump_dev_addrinfo(mdev, skb, cb);
        mctp_dev_put(mdev);
        if rc < 0 { break; }
        (*mcb).a_idx = 0;
    }
    rcu_read_unlock();
    (*skb).len as i32
}

unsafe fn mctp_addr_notify(mdev: *mut mctp_dev, eid: mctp_eid_t, msg_type: i32,
                           req_skb: *mut sk_buff, req_nlh: *mut nlmsghdr) {
    let portid = NETLINK_CB(req_skb).portid;
    let net = dev_net((*mdev).dev);
    let skb = nlmsg_new(mctp_addrinfo_size(), GFP_KERNEL);
    if skb.is_null() { rtnl_set_sk_err(net, RTNLGRP_MCTP_IFADDR, -ENOBUFS); return; }
    let rc = mctp_fill_addrinfo(skb, mdev, eid, msg_type, portid, (*req_nlh).nlmsg_seq, 0);
    if rc < 0 { WARN_ON_ONCE(rc == -EMSGSIZE); kfree_skb(skb); rtnl_set_sk_err(net, RTNLGRP_MCTP_IFADDR, rc); return; }
    rtnl_notify(skb, net, portid, RTNLGRP_MCTP_IFADDR, req_nlh, GFP_KERNEL);
}

static ifa_mctp_policy: [nla_policy; IFA_MAX + 1] = [nla_policy { type_: NLA_U8 }; IFA_MAX + 1];
static ifla_af_mctp_policy: [nla_policy; IFLA_MCTP_MAX + 1] = [nla_policy { type_: NLA_U32 }; IFLA_MCTP_MAX + 1];

static mut mctp_af_ops: rtnl_af_ops = rtnl_af_ops {
    family: AF_MCTP,
    fill_link_af: Some(mctp_fill_link_af),
    get_link_af_size: Some(mctp_get_link_af_size),
    set_link_af: Some(mctp_set_link_af),
};
static mut mctp_dev_nb: notifier_block = notifier_block { notifier_call: Some(mctp_dev_notify), priority: ADDRCONF_NOTIFY_PRIORITY };
static mctp_device_rtnl_msg_handlers: [rtnl_msg_handler; 3] = [
    rtnl_msg_handler { owner: THIS_MODULE, protocol: PF_MCTP, msgtype: RTM_NEWADDR, doit: Some(mctp_rtm_newaddr), dumpit: None },
    rtnl_msg_handler { owner: THIS_MODULE, protocol: PF_MCTP, msgtype: RTM_DELADDR, doit: Some(mctp_rtm_deladdr), dumpit: None },
    rtnl_msg_handler { owner: THIS_MODULE, protocol: PF_MCTP, msgtype: RTM_GETADDR, doit: None, dumpit: Some(mctp_dump_addrinfo) },
];

unsafe fn mctp_rtm_newaddr(skb: *mut sk_buff, nlh: *mut nlmsghdr, extack: *mut netlink_ext_ack) -> i32 {
    let net = sock_net((*skb).sk); let mut tb = [core::ptr::null_mut(); IFA_MAX + 1];
    let rc = nlmsg_parse(nlh, core::mem::size_of::<ifaddrmsg>() as u32, tb.as_mut_ptr(), IFA_MAX, &ifa_mctp_policy, extack);
    if rc < 0 { return rc; }
    let ifm = nlmsg_data(nlh) as *mut ifaddrmsg;
    let attr = if !tb[IFA_LOCAL].is_null() { tb[IFA_LOCAL] } else if !tb[IFA_ADDRESS].is_null() { tb[IFA_ADDRESS] } else { return -EINVAL };
    let dev = __dev_get_by_index(net, (*ifm).ifa_index); if dev.is_null() { return -ENODEV; }
    let mdev = mctp_dev_get_rtnl(dev); if mdev.is_null() { return -ENODEV; }
    let addr = nla_data(attr) as *mut mctp_addr;
    if !mctp_address_unicast((*addr).s_addr) { return -EINVAL; }
    if !memchr((*mdev).addrs, (*addr).s_addr as i32, (*mdev).num_addrs).is_null() { return -EEXIST; }
    let tmp = kmalloc((*mdev).num_addrs + 1, GFP_KERNEL) as *mut u8; if tmp.is_null() { return -ENOMEM; }
    memcpy(tmp, (*mdev).addrs, (*mdev).num_addrs); *tmp.add((*mdev).num_addrs) = (*addr).s_addr;
    let mut flags = 0; spin_lock_irqsave(&mut (*mdev).addrs_lock, &mut flags); (*mdev).num_addrs += 1; swap(&mut (*mdev).addrs, &mut (tmp as *mut u8)); spin_unlock_irqrestore(&mut (*mdev).addrs_lock, flags); kfree(tmp as *mut _);
    mctp_addr_notify(mdev, (*addr).s_addr, RTM_NEWADDR, skb, nlh); mctp_route_add_local(mdev, (*addr).s_addr); 0
}

unsafe fn mctp_rtm_deladdr(skb: *mut sk_buff, nlh: *mut nlmsghdr, extack: *mut netlink_ext_ack) -> i32 {
    let net = sock_net((*skb).sk); let mut tb = [core::ptr::null_mut(); IFA_MAX + 1];
    let rc = nlmsg_parse(nlh, core::mem::size_of::<ifaddrmsg>() as u32, tb.as_mut_ptr(), IFA_MAX, &ifa_mctp_policy, extack); if rc < 0 { return rc; }
    let ifm = nlmsg_data(nlh) as *mut ifaddrmsg; let attr = if !tb[IFA_LOCAL].is_null() { tb[IFA_LOCAL] } else if !tb[IFA_ADDRESS].is_null() { tb[IFA_ADDRESS] } else { return -EINVAL };
    let dev = __dev_get_by_index(net, (*ifm).ifa_index); if dev.is_null() { return -ENODEV; } let mdev = mctp_dev_get_rtnl(dev); if mdev.is_null() { return -ENODEV; }
    let addr = nla_data(attr) as *mut mctp_addr; let pos = memchr((*mdev).addrs, (*addr).s_addr as i32, (*mdev).num_addrs); if pos.is_null() { return -ENOENT; }
    let rc = mctp_route_remove_local(mdev, (*addr).s_addr); if rc < 0 && rc != -ENOENT { return rc; }
    let mut flags = 0; spin_lock_irqsave(&mut (*mdev).addrs_lock, &mut flags); memmove(pos, pos.add(1), (*mdev).num_addrs - 1 - pos.offset_from((*mdev).addrs) as usize); (*mdev).num_addrs -= 1; spin_unlock_irqrestore(&mut (*mdev).addrs_lock, flags);
    mctp_addr_notify(mdev, (*addr).s_addr, RTM_DELADDR, skb, nlh); 0
}

pub unsafe fn mctp_dev_hold(mdev: *mut mctp_dev) { refcount_inc(&mut (*mdev).refs); }
pub unsafe fn mctp_dev_put(mdev: *mut mctp_dev) { if !mdev.is_null() && refcount_dec_and_test(&mut (*mdev).refs) { kfree((*mdev).addrs as *mut _); dev_put((*mdev).dev); kfree_rcu(mdev, rcu); } }

pub unsafe fn mctp_dev_release_key(dev: *mut mctp_dev, key: *mut mctp_sk_key) { if dev.is_null() { return; } if !(*dev).ops.is_null() && !(*(*dev).ops).release_flow.is_none() { ((*(*dev).ops).release_flow.unwrap())(dev, key); } (*key).dev = core::ptr::null_mut(); mctp_dev_put(dev); }
pub unsafe fn mctp_dev_set_key(dev: *mut mctp_dev, key: *mut mctp_sk_key) { mctp_dev_hold(dev); (*key).dev = dev; }

// Remaining registration and netdevice plumbing mirrors the C implementation.
pub unsafe fn mctp_register_netdev(dev: *mut net_device, ops: *const mctp_netdev_ops, binding: mctp_phys_binding) -> i32 { rtnl_lock(); let mdev = mctp_add_dev(dev); if IS_ERR(mdev) { let rc = PTR_ERR(mdev); rtnl_unlock(); return rc; } (*mdev).ops = ops; (*mdev).binding = binding; let rc = register_netdevice(dev); rtnl_unlock(); rc }
pub unsafe fn mctp_unregister_netdev(dev: *mut net_device) { unregister_netdev(dev); }

unsafe fn mctp_add_dev(dev: *mut net_device) -> *mut mctp_dev {
    ASSERT_RTNL();
    let mdev = kzalloc_obj::<mctp_dev>(); if mdev.is_null() { return ERR_PTR(-ENOMEM); }
    spin_lock_init(&mut (*mdev).addrs_lock); (*mdev).net = mctp_default_net(dev_net(dev)); refcount_set(&mut (*mdev).refs, 1); rcu_assign_pointer((*dev).mctp_ptr, mdev); dev_hold(dev); (*mdev).dev = dev; mdev
}

unsafe fn mctp_fill_link_af(skb: *mut sk_buff, dev: *const net_device, _ext_filter_mask: u32) -> i32 {
    let mdev = mctp_dev_get_rtnl(dev); if mdev.is_null() { return -ENODATA; }
    if nla_put_u32(skb, IFLA_MCTP_NET, (*mdev).net) != 0 || nla_put_u8(skb, IFLA_MCTP_PHYS_BINDING, (*mdev).binding) != 0 { return -EMSGSIZE; } 0
}

unsafe fn mctp_get_link_af_size(dev: *const net_device, _ext_filter_mask: u32) -> usize {
    let mdev = __mctp_dev_get(dev); if mdev.is_null() { return 0; } let ret = nla_total_size(4) + nla_total_size(1); mctp_dev_put(mdev); ret
}

unsafe fn mctp_set_link_af(dev: *mut net_device, attr: *const nlattr, _extack: *mut netlink_ext_ack) -> i32 {
    let mut tb = [core::ptr::null_mut(); IFLA_MCTP_MAX + 1]; let rc = nla_parse_nested(tb.as_mut_ptr(), IFLA_MCTP_MAX, attr, &ifla_af_mctp_policy, core::ptr::null_mut()); if rc != 0 { return rc; }
    let mdev = mctp_dev_get_rtnl(dev); if mdev.is_null() { return 0; } if !tb[IFLA_MCTP_NET].is_null() { WRITE_ONCE((*mdev).net, nla_get_u32(tb[IFLA_MCTP_NET])); } 0
}

unsafe fn mctp_known(dev: *mut net_device) -> bool { (*dev).type_ == ARPHRD_MCTP || (*dev).type_ == ARPHRD_LOOPBACK || (*dev).type_ == ARPHRD_NONE }
unsafe fn mctp_unregister(dev: *mut net_device) { let mdev = mctp_dev_get_rtnl(dev); if mdev.is_null() { return; } RCU_INIT_POINTER((*mdev).dev, core::ptr::null_mut()); mctp_route_remove_dev(mdev); mctp_neigh_remove_dev(mdev); mctp_dev_put(mdev); }
unsafe fn mctp_register(dev: *mut net_device) -> i32 { if !rtnl_dereference((*dev).mctp_ptr).is_null() || !mctp_known(dev) { return 0; } let mdev = mctp_add_dev(dev); if IS_ERR(mdev) { return PTR_ERR(mdev); } 0 }
unsafe fn mctp_register_netdevice(dev: *mut net_device, ops: *const mctp_netdev_ops, binding: mctp_phys_binding) -> i32 { let mdev = mctp_add_dev(dev); if IS_ERR(mdev) { return PTR_ERR(mdev); } (*mdev).ops = ops; (*mdev).binding = binding; register_netdevice(dev) }

unsafe extern "C" fn mctp_dev_notify(_this: *mut notifier_block, event: ::core::ffi::c_ulong, ptr: *mut ::core::ffi::c_void) -> i32 {
    let dev = netdev_notifier_info_to_dev(ptr); match event { NETDEV_REGISTER => { let rc = mctp_register(dev); if rc != 0 { return notifier_from_errno(rc); } }, NETDEV_UNREGISTER => mctp_unregister(dev), _ => {} } NOTIFY_OK
}

pub unsafe extern "C" fn mctp_device_init() -> i32 {
    let mut err = register_netdevice_notifier(&mut mctp_dev_nb); if err != 0 { return err; }
    err = rtnl_af_register(&mut mctp_af_ops); if err != 0 { unregister_netdevice_notifier(&mut mctp_dev_nb); return err; }
    err = rtnl_register_many(mctp_device_rtnl_msg_handlers.as_ptr()); if err != 0 { rtnl_af_unregister(&mut mctp_af_ops); unregister_netdevice_notifier(&mut mctp_dev_nb); return err; } 0
}
pub unsafe extern "C" fn mctp_device_exit() { rtnl_unregister_many(mctp_device_rtnl_msg_handlers.as_ptr()); rtnl_af_unregister(&mut mctp_af_ops); unregister_netdevice_notifier(&mut mctp_dev_nb); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
