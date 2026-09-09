// SPDX-License-Identifier: GPL-2.0-only
/*
 * net/psample/psample.c - Netlink channel for packet sampling
 * Copyright (c) 2017 Yotam Gigi <yotamg@mellanox.com>
 */

// Translated from C. Kernel-provided types, constants, functions, and macros
// are intentionally referenced as external dependencies.

const PSAMPLE_MAX_PACKET_SIZE: usize = 0xffff;

static mut psample_groups_list: list_head = LIST_HEAD_INIT;
static mut psample_groups_lock: spinlock_t = DEFINE_SPINLOCK_INIT;

#[repr(C)]
enum psample_nl_multicast_groups {
    PSAMPLE_NL_MCGRP_CONFIG,
    PSAMPLE_NL_MCGRP_SAMPLE,
}

static psample_nl_mcgrps: [genl_multicast_group; 2] = [
    genl_multicast_group { name: PSAMPLE_NL_MCGRP_CONFIG_NAME, flags: 0 },
    genl_multicast_group { name: PSAMPLE_NL_MCGRP_SAMPLE_NAME, flags: GENL_MCAST_CAP_NET_ADMIN },
];

static mut psample_nl_family: genl_family = genl_family {
    name: PSAMPLE_GENL_NAME,
    version: PSAMPLE_GENL_VERSION,
    maxattr: PSAMPLE_ATTR_MAX,
    netnsok: true,
    module: THIS_MODULE,
    mcgrps: psample_nl_mcgrps.as_ptr(),
    small_ops: psample_nl_ops.as_ptr(),
    n_small_ops: psample_nl_ops.len(),
    resv_start_op: PSAMPLE_CMD_GET_GROUP + 1,
    n_mcgrps: psample_nl_mcgrps.len(),
};

unsafe fn psample_group_nl_fill(msg: *mut sk_buff, group: *mut psample_group,
                                cmd: psample_command, portid: u32, seq: u32,
                                flags: i32) -> i32 {
    let hdr = genlmsg_put(msg, portid, seq, &mut psample_nl_family, flags, cmd);
    if hdr.is_null() { return -EMSGSIZE; }
    if nla_put_u32(msg, PSAMPLE_ATTR_SAMPLE_GROUP, (*group).group_num) < 0 ||
       nla_put_u32(msg, PSAMPLE_ATTR_GROUP_REFCOUNT, (*group).refcount) < 0 ||
       nla_put_u32(msg, PSAMPLE_ATTR_GROUP_SEQ, (*group).seq) < 0 {
        genlmsg_cancel(msg, hdr);
        return -EMSGSIZE;
    }
    genlmsg_end(msg, hdr);
    0
}

unsafe extern "C" fn psample_nl_cmd_get_group_dumpit(msg: *mut sk_buff,
                                                       cb: *mut netlink_callback) -> i32 {
    let start = (*cb).args[0] as i32;
    let mut idx = 0;
    let mut err = 0;
    spin_lock_bh(&mut psample_groups_lock);
    list_for_each_entry!(group, psample_groups_list, list, psample_group, {
        if !net_eq((*group).net, sock_net((*msg).sk)) { continue; }
        if idx < start { idx += 1; continue; }
        err = psample_group_nl_fill(msg, group, PSAMPLE_CMD_NEW_GROUP,
            NETLINK_CB((*cb).skb).portid, (*(*cb).nlh).nlmsg_seq, NLM_F_MULTI);
        if err != 0 { break; }
        idx += 1;
    });
    spin_unlock_bh(&mut psample_groups_lock);
    (*cb).args[0] = idx as u64;
    (*msg).len as i32
}

static psample_nl_ops: [genl_small_ops; 1] = [genl_small_ops {
    cmd: PSAMPLE_CMD_GET_GROUP,
    validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP,
    dumpit: Some(psample_nl_cmd_get_group_dumpit),
}];

unsafe fn psample_group_notify(group: *mut psample_group, cmd: psample_command) {
    let msg = nlmsg_new(NLMSG_DEFAULT_SIZE, GFP_ATOMIC);
    if msg.is_null() { return; }
    let err = psample_group_nl_fill(msg, group, cmd, 0, 0, NLM_F_MULTI);
    if err == 0 {
        genlmsg_multicast_netns(&mut psample_nl_family, (*group).net, msg, 0,
                                 PSAMPLE_NL_MCGRP_CONFIG, GFP_ATOMIC);
    } else { nlmsg_free(msg); }
}

unsafe fn psample_group_create(net: *mut net, group_num: u32) -> *mut psample_group {
    let group = kzalloc_obj!(psample_group, GFP_ATOMIC);
    if group.is_null() { return core::ptr::null_mut(); }
    (*group).net = net;
    (*group).group_num = group_num;
    list_add_tail(&mut (*group).list, &mut psample_groups_list);
    psample_group_notify(group, PSAMPLE_CMD_NEW_GROUP);
    group
}

unsafe fn psample_group_destroy(group: *mut psample_group) {
    psample_group_notify(group, PSAMPLE_CMD_DEL_GROUP);
    list_del(&mut (*group).list);
    kfree_rcu!(group, rcu);
}

unsafe fn psample_group_lookup(net: *mut net, group_num: u32) -> *mut psample_group {
    let mut result = core::ptr::null_mut();
    list_for_each_entry!(group, psample_groups_list, list, psample_group, {
        if (*group).group_num == group_num && (*group).net == net { result = group; break; }
    });
    result
}

#[no_mangle]
pub unsafe extern "C" fn psample_group_get(net: *mut net, group_num: u32) -> *mut psample_group {
    spin_lock_bh(&mut psample_groups_lock);
    let mut group = psample_group_lookup(net, group_num);
    if group.is_null() {
        group = psample_group_create(net, group_num);
        if group.is_null() { spin_unlock_bh(&mut psample_groups_lock); return group; }
    }
    (*group).refcount += 1;
    spin_unlock_bh(&mut psample_groups_lock);
    group
}

#[no_mangle]
pub unsafe extern "C" fn psample_group_take(group: *mut psample_group) {
    spin_lock_bh(&mut psample_groups_lock); (*group).refcount += 1; spin_unlock_bh(&mut psample_groups_lock);
}

#[no_mangle]
pub unsafe extern "C" fn psample_group_put(group: *mut psample_group) {
    spin_lock_bh(&mut psample_groups_lock);
    (*group).refcount -= 1;
    if (*group).refcount == 0 { psample_group_destroy(group); }
    spin_unlock_bh(&mut psample_groups_lock);
}

// CONFIG_INET-dependent tunnel attribute helpers are preserved below as
// declarations because their kernel ABI types and helpers are external.
#[cfg(feature = "CONFIG_INET")]
unsafe fn __psample_ip_tun_to_nlattr(skb: *mut sk_buff, tun_info: *mut ip_tunnel_info) -> i32 {
    let tun_proto = ip_tunnel_info_af(tun_info);
    let tun_opts = ip_tunnel_info_opts(tun_info);
    let tun_key = &(*tun_info).key;
    let tun_opts_len = (*tun_info).options_len;
    if test_bit(IP_TUNNEL_KEY_BIT, tun_key.tun_flags) && nla_put_be64(skb, PSAMPLE_TUNNEL_KEY_ATTR_ID, tun_key.tun_id, PSAMPLE_TUNNEL_KEY_ATTR_PAD) != 0 { return -EMSGSIZE; }
    if (*tun_info).mode & IP_TUNNEL_INFO_BRIDGE != 0 && nla_put_flag(skb, PSAMPLE_TUNNEL_KEY_ATTR_IPV4_INFO_BRIDGE) != 0 { return -EMSGSIZE; }
    match tun_proto {
        AF_INET => { if tun_key.u.ipv4.src != 0 && nla_put_in_addr(skb, PSAMPLE_TUNNEL_KEY_ATTR_IPV4_SRC, tun_key.u.ipv4.src) != 0 { return -EMSGSIZE; } if tun_key.u.ipv4.dst != 0 && nla_put_in_addr(skb, PSAMPLE_TUNNEL_KEY_ATTR_IPV4_DST, tun_key.u.ipv4.dst) != 0 { return -EMSGSIZE; } }
        AF_INET6 => { if !ipv6_addr_any(&tun_key.u.ipv6.src) && nla_put_in6_addr(skb, PSAMPLE_TUNNEL_KEY_ATTR_IPV6_SRC, &tun_key.u.ipv6.src) != 0 { return -EMSGSIZE; } if !ipv6_addr_any(&tun_key.u.ipv6.dst) && nla_put_in6_addr(skb, PSAMPLE_TUNNEL_KEY_ATTR_IPV6_DST, &tun_key.u.ipv6.dst) != 0 { return -EMSGSIZE; } }
        _ => {}
    }
    if tun_key.tos != 0 && nla_put_u8(skb, PSAMPLE_TUNNEL_KEY_ATTR_TOS, tun_key.tos) != 0 { return -EMSGSIZE; }
    if nla_put_u8(skb, PSAMPLE_TUNNEL_KEY_ATTR_TTL, tun_key.ttl) != 0 { return -EMSGSIZE; }
    if test_bit(IP_TUNNEL_DONT_FRAGMENT_BIT, tun_key.tun_flags) && nla_put_flag(skb, PSAMPLE_TUNNEL_KEY_ATTR_DONT_FRAGMENT) != 0 { return -EMSGSIZE; }
    if test_bit(IP_TUNNEL_CSUM_BIT, tun_key.tun_flags) && nla_put_flag(skb, PSAMPLE_TUNNEL_KEY_ATTR_CSUM) != 0 { return -EMSGSIZE; }
    if tun_key.tp_src != 0 && nla_put_be16(skb, PSAMPLE_TUNNEL_KEY_ATTR_TP_SRC, tun_key.tp_src) != 0 { return -EMSGSIZE; }
    if tun_key.tp_dst != 0 && nla_put_be16(skb, PSAMPLE_TUNNEL_KEY_ATTR_TP_DST, tun_key.tp_dst) != 0 { return -EMSGSIZE; }
    if test_bit(IP_TUNNEL_OAM_BIT, tun_key.tun_flags) && nla_put_flag(skb, PSAMPLE_TUNNEL_KEY_ATTR_OAM) != 0 { return -EMSGSIZE; }
    if tun_opts_len != 0 { if test_bit(IP_TUNNEL_GENEVE_OPT_BIT, tun_key.tun_flags) && nla_put(skb, PSAMPLE_TUNNEL_KEY_ATTR_GENEVE_OPTS, tun_opts_len, tun_opts) != 0 { return -EMSGSIZE; } else if test_bit(IP_TUNNEL_ERSPAN_OPT_BIT, tun_key.tun_flags) && nla_put(skb, PSAMPLE_TUNNEL_KEY_ATTR_ERSPAN_OPTS, tun_opts_len, tun_opts) != 0 { return -EMSGSIZE; } }
    0
}

// The packet sampling entry point and module registration retain the C ABI;
// kernel structure fields and netlink helpers are supplied by dependencies.
#[no_mangle]
pub unsafe extern "C" fn psample_sample_packet(group: *mut psample_group, skb: *const sk_buff, sample_rate: u32, md: *const psample_metadata) {
    let tstamp = ktime_get_real();
    if genl_has_listeners(&mut psample_nl_family, (*group).net, PSAMPLE_NL_MCGRP_SAMPLE) == 0 { return; }
    let trunc_size = (*md).trunc_size;
    let data_len = core::cmp::min((*skb).len, trunc_size);
    let nl_skb = genlmsg_new(nla_total_size(data_len), GFP_ATOMIC);
    if nl_skb.is_null() { return; }
    let data = genlmsg_put(nl_skb, 0, 0, &mut psample_nl_family, 0, PSAMPLE_CMD_SAMPLE);
    if data.is_null() { nlmsg_free(nl_skb); return; }
    if (*md).in_ifindex != 0 && nla_put_u16(nl_skb, PSAMPLE_ATTR_IIFINDEX, (*md).in_ifindex) < 0 { nlmsg_free(nl_skb); return; }
    if (*md).out_ifindex != 0 && nla_put_u16(nl_skb, PSAMPLE_ATTR_OIFINDEX, (*md).out_ifindex) < 0 { nlmsg_free(nl_skb); return; }
    if nla_put_u32(nl_skb, PSAMPLE_ATTR_SAMPLE_RATE, sample_rate) < 0 || nla_put_u32(nl_skb, PSAMPLE_ATTR_ORIGSIZE, (*skb).len) < 0 || nla_put_u32(nl_skb, PSAMPLE_ATTR_SAMPLE_GROUP, (*group).group_num) < 0 || nla_put_u32(nl_skb, PSAMPLE_ATTR_GROUP_SEQ, (*group).seq) < 0 { nlmsg_free(nl_skb); return; }
    (*group).seq += 1;
    if nla_put_u64_64bit(nl_skb, PSAMPLE_ATTR_TIMESTAMP, ktime_to_ns(tstamp), PSAMPLE_ATTR_PAD) < 0 { nlmsg_free(nl_skb); return; }
    if nla_put_u16(nl_skb, PSAMPLE_ATTR_PROTO, be16_to_cpu((*skb).protocol)) < 0 { nlmsg_free(nl_skb); return; }
    genlmsg_end(nl_skb, data);
    genlmsg_multicast_netns(&mut psample_nl_family, (*group).net, nl_skb, 0, PSAMPLE_NL_MCGRP_SAMPLE, GFP_ATOMIC);
}

unsafe fn psample_module_init() -> i32 { genl_register_family(&mut psample_nl_family) }
unsafe fn psample_module_exit() { genl_unregister_family(&mut psample_nl_family); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
