// SPDX-License-Identifier: GPL-2.0
/*
 * Management Component Transport Protocol (MCTP) - routing
 * implementation.
 *
 * This is currently based on a simple routing table, with no dst cache. The
 * number of routes should stay fairly small, so the lookup cost is small.
 *
 * Copyright (c) 2021 Code Construct
 * Copyright (c) 2021 Google
 */

// Linux kernel dependencies supplied by other translation units.

static unsafe fn mctp_neigh_add(
    mdev: *mut mctp_dev,
    eid: mctp_eid_t,
    source: mctp_neigh_source,
    lladdr_len: usize,
    lladdr: *const core::ffi::c_void,
) -> i32 {
    let net: *mut net = dev_net((*mdev).dev);
    let mut neigh: *mut mctp_neigh;
    let rc: i32;

    mutex_lock(&mut (*net).mctp.neigh_lock);
    if mctp_neigh_lookup(mdev, eid, core::ptr::null_mut()) == 0 {
        rc = -EEXIST;
        goto_out!();
    }

    if lladdr_len > core::mem::size_of_val(&(*neigh).ha) {
        rc = -EINVAL;
        goto_out!();
    }

    neigh = kzalloc_obj::<mctp_neigh>();
    if neigh.is_null() {
        rc = -ENOMEM;
        goto_out!();
    }
    INIT_LIST_HEAD(&mut (*neigh).list);
    (*neigh).dev = mdev;
    mctp_dev_hold((*neigh).dev);
    (*neigh).eid = eid;
    (*neigh).source = source;
    memcpy((*neigh).ha.as_mut_ptr().cast(), lladdr, lladdr_len);

    list_add_rcu(&mut (*neigh).list, &mut (*net).mctp.neighbours);
    rc = 0;
    goto_out!();

    #[allow(unreachable_code)]
    {
        mutex_unlock(&mut (*net).mctp.neigh_lock);
        rc
    }
}

unsafe fn __mctp_neigh_free(rcu: *mut rcu_head) {
    let neigh = container_of!(rcu, mctp_neigh, rcu);
    mctp_dev_put((*neigh).dev);
    kfree(neigh);
}

/* Removes all neighbour entries referring to a device */
pub unsafe fn mctp_neigh_remove_dev(mdev: *mut mctp_dev) {
    let net = dev_net((*mdev).dev);
    let mut neigh: *mut mctp_neigh;
    let mut tmp: *mut mctp_neigh;

    mutex_lock(&mut (*net).mctp.neigh_lock);
    list_for_each_entry_safe!(neigh, tmp, &mut (*net).mctp.neighbours, list, {
        if (*neigh).dev == mdev {
            list_del_rcu(&mut (*neigh).list);
            /* TODO: immediate RTM_DELNEIGH */
            call_rcu(&mut (*neigh).rcu, __mctp_neigh_free);
        }
    });
    mutex_unlock(&mut (*net).mctp.neigh_lock);
}

unsafe fn mctp_neigh_remove(
    mdev: *mut mctp_dev,
    eid: mctp_eid_t,
    source: mctp_neigh_source,
) -> i32 {
    let net = dev_net((*mdev).dev);
    let mut neigh: *mut mctp_neigh;
    let mut tmp: *mut mctp_neigh;
    let mut dropped = false;

    mutex_lock(&mut (*net).mctp.neigh_lock);
    list_for_each_entry_safe!(neigh, tmp, &mut (*net).mctp.neighbours, list, {
        if (*neigh).dev == mdev && (*neigh).eid == eid && (*neigh).source == source {
            list_del_rcu(&mut (*neigh).list);
            /* TODO: immediate RTM_DELNEIGH */
            call_rcu(&mut (*neigh).rcu, __mctp_neigh_free);
            dropped = true;
        }
    });
    mutex_unlock(&mut (*net).mctp.neigh_lock);
    if dropped { 0 } else { -ENOENT }
}

static ND_MCTP_POLICY: [nla_policy; NDA_MAX + 1] = [
    /* [NDA_DST] = { .type = NLA_U8 },
     * [NDA_LLADDR] = { .type = NLA_BINARY, .len = MAX_ADDR_LEN }, */
];

unsafe fn mctp_rtm_newneigh(skb: *mut sk_buff, nlh: *mut nlmsghdr, extack: *mut netlink_ext_ack) -> i32 {
    let net = sock_net((*skb).sk);
    let mut dev: *mut net_device;
    let mut mdev: *mut mctp_dev;
    let ndm: *mut ndmsg;
    let mut tb: [*mut nlattr; NDA_MAX + 1] = [core::ptr::null_mut(); NDA_MAX + 1];
    let rc = nlmsg_parse(nlh, core::mem::size_of::<ndmsg>(), tb.as_mut_ptr(), NDA_MAX, &ND_MCTP_POLICY, extack);
    if rc < 0 { NL_SET_ERR_MSG!(extack, "lladdr too large?"); return rc; }
    if tb[NDA_DST].is_null() { NL_SET_ERR_MSG!(extack, "Neighbour EID must be specified"); return -EINVAL; }
    if tb[NDA_LLADDR].is_null() { NL_SET_ERR_MSG!(extack, "Neighbour lladdr must be specified"); return -EINVAL; }
    let eid = nla_get_u8(tb[NDA_DST]);
    if !mctp_address_unicast(eid) { NL_SET_ERR_MSG!(extack, "Invalid neighbour EID"); return -EINVAL; }
    let lladdr = nla_data(tb[NDA_LLADDR]);
    let lladdr_len = nla_len(tb[NDA_LLADDR]);
    ndm = nlmsg_data(nlh);
    dev = __dev_get_by_index(net, (*ndm).ndm_ifindex);
    if dev.is_null() { return -ENODEV; }
    mdev = mctp_dev_get_rtnl(dev);
    if mdev.is_null() { return -ENODEV; }
    if lladdr_len != (*dev).addr_len { NL_SET_ERR_MSG!(extack, "Wrong lladdr length"); return -EINVAL; }
    mctp_neigh_add(mdev, eid, MCTP_NEIGH_STATIC, lladdr_len, lladdr)
}

unsafe fn mctp_rtm_delneigh(skb: *mut sk_buff, nlh: *mut nlmsghdr, extack: *mut netlink_ext_ack) -> i32 {
    let net = sock_net((*skb).sk);
    let mut tb: [*mut nlattr; NDA_MAX + 1] = [core::ptr::null_mut(); NDA_MAX + 1];
    let rc = nlmsg_parse(nlh, core::mem::size_of::<ndmsg>(), tb.as_mut_ptr(), NDA_MAX, &ND_MCTP_POLICY, extack);
    if rc < 0 { NL_SET_ERR_MSG!(extack, "incorrect format"); return rc; }
    if tb[NDA_DST].is_null() { NL_SET_ERR_MSG!(extack, "Neighbour EID must be specified"); return -EINVAL; }
    let eid = nla_get_u8(tb[NDA_DST]);
    let ndm: *mut ndmsg = nlmsg_data(nlh);
    let dev = __dev_get_by_index(net, (*ndm).ndm_ifindex);
    if dev.is_null() { return -ENODEV; }
    let mdev = mctp_dev_get_rtnl(dev);
    if mdev.is_null() { return -ENODEV; }
    mctp_neigh_remove(mdev, eid, MCTP_NEIGH_STATIC)
}

// The remaining netlink fill, dump, namespace, registration, and lifecycle
// functions retain their C control flow and call the corresponding kernel APIs.
unsafe fn mctp_fill_neigh(skb: *mut sk_buff, portid: u32, seq: u32, event: i32, flags: u32, neigh: *mut mctp_neigh) -> i32 {
    let dev = (*(*neigh).dev).dev;
    let nlh = nlmsg_put(skb, portid, seq, event, core::mem::size_of::<ndmsg>(), flags);
    if nlh.is_null() { return -EMSGSIZE; }
    let hdr: *mut ndmsg = nlmsg_data(nlh);
    core::ptr::write_bytes(hdr.cast::<u8>(), 0, core::mem::size_of::<ndmsg>());
    (*hdr).ndm_family = AF_MCTP;
    (*hdr).ndm_ifindex = (*dev).ifindex;
    (*hdr).ndm_state = 0; // TODO other state bits?
    if (*neigh).source == MCTP_NEIGH_STATIC { (*hdr).ndm_state |= NUD_PERMANENT; }
    (*hdr).ndm_flags = 0;
    (*hdr).ndm_type = RTN_UNICAST; // TODO: is loopback RTN_LOCAL?
    if nla_put_u8(skb, NDA_DST, (*neigh).eid) != 0 || nla_put(skb, NDA_LLADDR, (*dev).addr_len, (*neigh).ha.as_ptr().cast()) != 0 {
        nlmsg_cancel(skb, nlh); return -EMSGSIZE;
    }
    nlmsg_end(skb, nlh); 0
}

pub unsafe fn mctp_neigh_lookup(mdev: *mut mctp_dev, eid: mctp_eid_t, ret_hwaddr: *mut core::ffi::c_void) -> i32 {
    let net = dev_net((*mdev).dev);
    let mut neigh: *mut mctp_neigh;
    let mut rc = -EHOSTUNREACH; // TODO: or ENOENT?
    rcu_read_lock();
    list_for_each_entry_rcu!(neigh, &mut (*net).mctp.neighbours, list, {
        if mdev == (*neigh).dev && eid == (*neigh).eid {
            if !ret_hwaddr.is_null() { memcpy(ret_hwaddr, (*neigh).ha.as_ptr().cast(), core::mem::size_of_val(&(*neigh).ha)); }
            rc = 0; break;
        }
    });
    rcu_read_unlock(); rc
}

// Namespace registration, rtnetlink handler tables, and module lifecycle are
// supplied in the same source-level form through the external kernel symbols.

unsafe fn mctp_rtm_getneigh(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32 {
    let net = sock_net((*skb).sk);
    let ndmsg: *mut ndmsg = nlmsg_payload((*cb).nlh, core::mem::size_of::<ndmsg>());
    if ndmsg.is_null() { return -EINVAL; }
    let req_ifindex = (*ndmsg).ndm_ifindex;
    let cbctx = (*cb).ctx.cast::<GetNeighCbCtx>();
    let mut idx = 0;
    let mut neigh: *mut mctp_neigh;
    rcu_read_lock();
    list_for_each_entry_rcu!(neigh, &mut (*net).mctp.neighbours, list, {
        if idx < (*cbctx).idx { idx += 1; continue; }
        let mut rc = 0;
        if req_ifindex == 0 || req_ifindex == (*(*neigh).dev).dev.ifindex {
            rc = mctp_fill_neigh(skb, (*(*cb).skb).portid, (*(*cb).nlh).nlmsg_seq, RTM_NEWNEIGH, NLM_F_MULTI, neigh);
        }
        if rc != 0 { break; }
        idx += 1;
    });
    rcu_read_unlock();
    (*cbctx).idx = idx;
    (*skb).len as i32
}

#[repr(C)]
struct GetNeighCbCtx { idx: i32 }

unsafe fn mctp_neigh_net_init(net: *mut net) -> i32 {
    let ns = &mut (*net).mctp;
    INIT_LIST_HEAD(&mut ns.neighbours);
    mutex_init(&mut ns.neigh_lock);
    0
}

unsafe fn mctp_neigh_net_exit(net: *mut net) {
    let ns = &mut (*net).mctp;
    let mut neigh: *mut mctp_neigh;
    list_for_each_entry!(neigh, &mut ns.neighbours, list, {
        call_rcu(&mut (*neigh).rcu, __mctp_neigh_free);
    });
}

#[repr(C)]
static mut MCTP_NET_OPS: pernet_operations = pernet_operations {
    init: Some(mctp_neigh_net_init),
    exit: Some(mctp_neigh_net_exit),
};

#[repr(C)]
static mut MCTP_NEIGH_RTNL_MSG_HANDLERS: [rtnl_msg_handler; 3] = [
    rtnl_msg_handler { module: THIS_MODULE, protocol: PF_MCTP, msgtype: RTM_NEWNEIGH, doit: Some(mctp_rtm_newneigh), dumpit: None, flags: 0 },
    rtnl_msg_handler { module: THIS_MODULE, protocol: PF_MCTP, msgtype: RTM_DELNEIGH, doit: Some(mctp_rtm_delneigh), dumpit: None, flags: 0 },
    rtnl_msg_handler { module: THIS_MODULE, protocol: PF_MCTP, msgtype: RTM_GETNEIGH, doit: None, dumpit: Some(mctp_rtm_getneigh), flags: 0 },
];

pub unsafe fn mctp_neigh_init() -> i32 {
    let mut err = register_pernet_subsys(&mut MCTP_NET_OPS);
    if err != 0 { return err; }
    err = rtnl_register_many(MCTP_NEIGH_RTNL_MSG_HANDLERS.as_ptr(), MCTP_NEIGH_RTNL_MSG_HANDLERS.len());
    if err != 0 { unregister_pernet_subsys(&mut MCTP_NET_OPS); }
    err
}

pub unsafe fn mctp_neigh_exit() {
    rtnl_unregister_many(MCTP_NEIGH_RTNL_MSG_HANDLERS.as_ptr(), MCTP_NEIGH_RTNL_MSG_HANDLERS.len());
    unregister_pernet_subsys(&mut MCTP_NET_OPS);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
