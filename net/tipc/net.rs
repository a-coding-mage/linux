/*
 * net/tipc/net.c: TIPC network routing code
 *
 * Copyright (c) 1995-2006, 2014, Ericsson AB
 * Copyright (c) 2005, 2010-2011, Wind River Systems
 * All rights reserved.
 *
 * Translated from the C implementation. External declarations and constants
 * are supplied by the surrounding TIPC implementation.
 */

/* The TIPC locking policy and locking-domain documentation from the source
 * is retained conceptually here; locking is provided by the external TIPC
 * implementation. */

unsafe fn tipc_net_finalize(net: *mut net, addr: u32) {
    let tn: *mut tipc_net = tipc_net(net);
    let mut sk: tipc_socket_addr = core::mem::zeroed();
    sk.node = addr;
    let mut ua: tipc_uaddr = core::mem::zeroed();

    tipc_uaddr(&mut ua, TIPC_SERVICE_RANGE, TIPC_CLUSTER_SCOPE,
               TIPC_NODE_STATE, addr, addr);

    if cmpxchg(&mut (*tn).node_addr, 0, addr) != 0 {
        return;
    }
    tipc_set_node_addr(net, addr);
    tipc_named_reinit(net);
    tipc_sk_reinit(net);
    tipc_mon_reinit_self(net);
    tipc_nametbl_publish(net, &mut ua, &mut sk, addr);
}

pub unsafe fn tipc_net_init(net: *mut net, node_id: *mut u8, addr: u32) -> i32 {
    if tipc_own_id(net) {
        pr_info("Cannot configure node identity twice\n");
        return -1;
    }
    pr_info("Started in network mode\n");

    if !node_id.is_null() {
        tipc_set_node_id(net, node_id);
    }
    if addr != 0 {
        tipc_net_finalize(net, addr);
    }
    0
}

pub unsafe fn tipc_net_finalize_work(work: *mut work_struct) {
    let tn: *mut tipc_net = container_of(work, tipc_net::work);
    rtnl_lock();
    tipc_net_finalize(tipc_link_net((*tn).bcl), (*tn).trial_addr);
    rtnl_unlock();
}

pub unsafe fn tipc_net_stop(net: *mut net) {
    if !tipc_own_id(net) {
        return;
    }
    rtnl_lock();
    tipc_bearer_stop(net);
    tipc_node_stop(net);
    rtnl_unlock();
    pr_info("Left network mode\n");
}

unsafe fn __tipc_nl_add_net(net: *mut net, msg: *mut tipc_nl_msg) -> i32 {
    let tn: *mut tipc_net = net_generic(net, tipc_net_id);
    let w0: u64 = core::ptr::read((*tn).node_id.as_ptr() as *const u64);
    let w1: u64 = core::ptr::read((*tn).node_id.as_ptr().add(8) as *const u64);
    let hdr = genlmsg_put((*msg).skb, (*msg).portid, (*msg).seq,
                          &tipc_genl_family, NLM_F_MULTI, TIPC_NL_NET_GET);
    if hdr.is_null() { return -EMSGSIZE; }
    let attrs = nla_nest_start_noflag((*msg).skb, TIPC_NLA_NET);
    if attrs.is_null() { genlmsg_cancel((*msg).skb, hdr); return -EMSGSIZE; }
    if nla_put_u32((*msg).skb, TIPC_NLA_NET_ID, (*tn).net_id) != 0 ||
       nla_put_u64_64bit((*msg).skb, TIPC_NLA_NET_NODEID, w0, 0) != 0 ||
       nla_put_u64_64bit((*msg).skb, TIPC_NLA_NET_NODEID_W1, w1, 0) != 0 {
        nla_nest_cancel((*msg).skb, attrs);
        genlmsg_cancel((*msg).skb, hdr);
        return -EMSGSIZE;
    }
    nla_nest_end((*msg).skb, attrs);
    genlmsg_end((*msg).skb, hdr);
    0
}

pub unsafe fn tipc_nl_net_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32 {
    let net = sock_net((*skb).sk);
    let mut done = (*cb).args[0];
    if done != 0 { return 0; }
    let mut msg: tipc_nl_msg = core::mem::zeroed();
    msg.skb = skb;
    msg.portid = NETLINK_CB((*cb).skb).portid;
    msg.seq = (*cb).nlh.nlmsg_seq;
    if __tipc_nl_add_net(net, &mut msg) == 0 { done = 1; }
    (*cb).args[0] = done;
    (*skb).len as i32
}

pub unsafe fn __tipc_nl_net_set(skb: *mut sk_buff, info: *mut genl_info) -> i32 {
    let mut attrs: [*mut nlattr; TIPC_NLA_NET_MAX as usize + 1] = [core::ptr::null_mut(); TIPC_NLA_NET_MAX as usize + 1];
    let net = sock_net((*skb).sk);
    let tn = tipc_net(net);
    if (*info).attrs[TIPC_NLA_NET as usize].is_null() { return -EINVAL; }
    let err = nla_parse_nested_deprecated(attrs.as_mut_ptr(), TIPC_NLA_NET_MAX,
        (*info).attrs[TIPC_NLA_NET as usize], tipc_nl_net_policy, (*info).extack);
    if err != 0 { return err; }
    if tipc_own_addr(net) { return -EPERM; }
    if !attrs[TIPC_NLA_NET_ID as usize].is_null() {
        let val = nla_get_u32(attrs[TIPC_NLA_NET_ID as usize]);
        if val < 1 || val > 9999 { return -EINVAL; }
        (*tn).net_id = val;
    }
    if !attrs[TIPC_NLA_NET_ADDR as usize].is_null() {
        let addr = nla_get_u32(attrs[TIPC_NLA_NET_ADDR as usize]);
        if addr == 0 { return -EINVAL; }
        (*tn).legacy_addr_format = true;
        tipc_net_init(net, core::ptr::null_mut(), addr);
    }
    if !attrs[TIPC_NLA_NET_NODEID as usize].is_null() {
        let mut node_id = [0u8; NODE_ID_LEN as usize];
        if attrs[TIPC_NLA_NET_NODEID_W1 as usize].is_null() { return -EINVAL; }
        core::ptr::write(node_id.as_mut_ptr() as *mut u64, nla_get_u64(attrs[TIPC_NLA_NET_NODEID as usize]));
        core::ptr::write(node_id.as_mut_ptr().add(8) as *mut u64, nla_get_u64(attrs[TIPC_NLA_NET_NODEID_W1 as usize]));
        tipc_net_init(net, node_id.as_mut_ptr(), 0);
    }
    0
}

pub unsafe fn tipc_nl_net_set(skb: *mut sk_buff, info: *mut genl_info) -> i32 {
    rtnl_lock();
    let err = __tipc_nl_net_set(skb, info);
    rtnl_unlock();
    err
}

unsafe fn __tipc_nl_addr_legacy_get(net: *mut net, msg: *mut tipc_nl_msg) -> i32 {
    let tn = tipc_net(net);
    let hdr = genlmsg_put((*msg).skb, (*msg).portid, (*msg).seq, &tipc_genl_family, 0, TIPC_NL_ADDR_LEGACY_GET);
    if hdr.is_null() { return -EMSGSIZE; }
    let attrs = nla_nest_start((*msg).skb, TIPC_NLA_NET);
    if attrs.is_null() { genlmsg_cancel((*msg).skb, hdr); return -EMSGSIZE; }
    if (*tn).legacy_addr_format && nla_put_flag((*msg).skb, TIPC_NLA_NET_ADDR_LEGACY) != 0 {
        nla_nest_cancel((*msg).skb, attrs); genlmsg_cancel((*msg).skb, hdr); return -EMSGSIZE;
    }
    nla_nest_end((*msg).skb, attrs); genlmsg_end((*msg).skb, hdr); 0
}

pub unsafe fn tipc_nl_net_addr_legacy_get(skb: *mut sk_buff, info: *mut genl_info) -> i32 {
    let net = sock_net((*skb).sk);
    let rep = nlmsg_new(NLMSG_GOODSIZE, GFP_KERNEL);
    if rep.is_null() { return -ENOMEM; }
    let mut msg: tipc_nl_msg = core::mem::zeroed();
    msg.skb = rep; msg.portid = (*info).snd_portid; msg.seq = (*info).snd_seq;
    let err = __tipc_nl_addr_legacy_get(net, &mut msg);
    if err != 0 { nlmsg_free(msg.skb); return err; }
    genlmsg_reply(msg.skb, info)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
