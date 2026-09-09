// SPDX-License-Identifier: GPL-2.0-only

// Kernel and project headers from the C translation unit are supplied by the
// surrounding Rust build.

#[no_mangle]
pub static ethnl_tunnel_info_get_policy: [nla_policy; ETHTOOL_A_TUNNEL_INFO_HEADER as usize + 1] = {
    let mut policy = [nla_policy::default(); ETHTOOL_A_TUNNEL_INFO_HEADER as usize + 1];
    policy[ETHTOOL_A_TUNNEL_INFO_HEADER as usize] = NLA_POLICY_NESTED(ethnl_header_policy);
    policy
};

// static_assert(ETHTOOL_UDP_TUNNEL_TYPE_VXLAN == ilog2(UDP_TUNNEL_TYPE_VXLAN));
// static_assert(ETHTOOL_UDP_TUNNEL_TYPE_GENEVE == ilog2(UDP_TUNNEL_TYPE_GENEVE));
// static_assert(ETHTOOL_UDP_TUNNEL_TYPE_VXLAN_GPE == ilog2(UDP_TUNNEL_TYPE_VXLAN_GPE));

unsafe fn ethnl_udp_table_reply_size(types: c_uint, compact: bool) -> ssize_t {
    let size = ethnl_bitset32_size(
        &types,
        core::ptr::null(),
        __ETHTOOL_UDP_TUNNEL_TYPE_CNT,
        udp_tunnel_type_names,
        compact,
    );
    if size < 0 {
        return size;
    }
    size + nla_total_size(0) + nla_total_size(core::mem::size_of::<u32>())
}

unsafe fn ethnl_tunnel_info_reply_size(
    req_base: *const ethnl_req_info,
    extack: *mut netlink_ext_ack,
) -> ssize_t {
    let compact = (*req_base).flags & ETHTOOL_FLAG_COMPACT_BITSETS != 0;
    let info = (*(*req_base).dev).udp_tunnel_nic_info;
    if info.is_null() {
        NL_SET_ERR_MSG(extack, c"device does not report tunnel offload info");
        return -EOPNOTSUPP;
    }

    let mut size = nla_total_size(0) as size_t;
    for i in 0..UDP_TUNNEL_NIC_MAX_TABLES {
        if (*info).tables[i].n_entries == 0 {
            break;
        }
        let ret = ethnl_udp_table_reply_size((*info).tables[i].tunnel_types, compact);
        if ret < 0 {
            return ret;
        }
        size += ret as size_t;
        size += udp_tunnel_nic_dump_size((*req_base).dev, i) as size_t;
    }
    if (*info).flags & UDP_TUNNEL_NIC_INFO_STATIC_IANA_VXLAN != 0 {
        let ret = ethnl_udp_table_reply_size(0, compact);
        if ret < 0 {
            return ret;
        }
        size += ret as size_t;
        size += (nla_total_size(0) + nla_total_size(core::mem::size_of::<__be16>())
            + nla_total_size(core::mem::size_of::<u32>())) as size_t;
    }
    size as ssize_t
}

unsafe fn ethnl_tunnel_info_fill_reply(req_base: *const ethnl_req_info, skb: *mut sk_buff) -> c_int {
    let compact = (*req_base).flags & ETHTOOL_FLAG_COMPACT_BITSETS != 0;
    let info = (*(*req_base).dev).udp_tunnel_nic_info;
    if info.is_null() { return -EOPNOTSUPP; }

    let ports = nla_nest_start(skb, ETHTOOL_A_TUNNEL_INFO_UDP_PORTS);
    if ports.is_null() { return -EMSGSIZE; }
    for i in 0..UDP_TUNNEL_NIC_MAX_TABLES {
        if (*info).tables[i].n_entries == 0 { break; }
        let table = nla_nest_start(skb, ETHTOOL_A_TUNNEL_UDP_TABLE);
        if table.is_null() { nla_nest_cancel(skb, ports); return -EMSGSIZE; }
        if nla_put_u32(skb, ETHTOOL_A_TUNNEL_UDP_TABLE_SIZE, (*info).tables[i].n_entries) != 0
            || ethnl_put_bitset32(skb, ETHTOOL_A_TUNNEL_UDP_TABLE_TYPES,
                &(*info).tables[i].tunnel_types, core::ptr::null(),
                __ETHTOOL_UDP_TUNNEL_TYPE_CNT, udp_tunnel_type_names, compact) != 0
            || udp_tunnel_nic_dump_write((*req_base).dev, i, skb) != 0 {
            nla_nest_cancel(skb, table); nla_nest_cancel(skb, ports); return -EMSGSIZE;
        }
        nla_nest_end(skb, table);
    }
    if (*info).flags & UDP_TUNNEL_NIC_INFO_STATIC_IANA_VXLAN != 0 {
        let zero: u32 = 0;
        let table = nla_nest_start(skb, ETHTOOL_A_TUNNEL_UDP_TABLE);
        if table.is_null() { nla_nest_cancel(skb, ports); return -EMSGSIZE; }
        if nla_put_u32(skb, ETHTOOL_A_TUNNEL_UDP_TABLE_SIZE, 1) != 0
            || ethnl_put_bitset32(skb, ETHTOOL_A_TUNNEL_UDP_TABLE_TYPES, &zero,
                core::ptr::null(), __ETHTOOL_UDP_TUNNEL_TYPE_CNT,
                udp_tunnel_type_names, compact) != 0 {
            nla_nest_cancel(skb, table); nla_nest_cancel(skb, ports); return -EMSGSIZE;
        }
        let entry = nla_nest_start(skb, ETHTOOL_A_TUNNEL_UDP_TABLE_ENTRY);
        if entry.is_null() { nla_nest_cancel(skb, table); nla_nest_cancel(skb, ports); return -EMSGSIZE; }
        if nla_put_be16(skb, ETHTOOL_A_TUNNEL_UDP_ENTRY_PORT, htons(IANA_VXLAN_UDP_PORT)) != 0
            || nla_put_u32(skb, ETHTOOL_A_TUNNEL_UDP_ENTRY_TYPE, ilog2(UDP_TUNNEL_TYPE_VXLAN)) != 0 {
            nla_nest_cancel(skb, entry); nla_nest_cancel(skb, table); nla_nest_cancel(skb, ports); return -EMSGSIZE;
        }
        nla_nest_end(skb, entry); nla_nest_end(skb, table);
    }
    nla_nest_end(skb, ports);
    0
}

pub unsafe fn ethnl_tunnel_info_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    let mut req_info: ethnl_req_info = core::mem::zeroed();
    let tb = (*info).attrs;
    let mut ret = ethnl_parse_header_dev_get(&mut req_info, *tb.add(ETHTOOL_A_TUNNEL_INFO_HEADER as usize),
        genl_info_net(info), (*info).extack, true);
    if ret < 0 { return ret; }
    rtnl_lock();
    ret = ethnl_tunnel_info_reply_size(&req_info, (*info).extack);
    if ret < 0 { rtnl_unlock(); ethnl_parse_header_dev_put(&mut req_info); return ret; }
    let reply_len = ret + ethnl_reply_header_size();
    let mut reply_payload: *mut c_void = core::ptr::null_mut();
    let rskb = ethnl_reply_init(reply_len, req_info.dev, ETHTOOL_MSG_TUNNEL_INFO_GET_REPLY,
        ETHTOOL_A_TUNNEL_INFO_HEADER, info, &mut reply_payload);
    if rskb.is_null() { rtnl_unlock(); ethnl_parse_header_dev_put(&mut req_info); return -ENOMEM; }
    ret = ethnl_tunnel_info_fill_reply(&req_info, rskb);
    if ret != 0 { nlmsg_free(rskb); rtnl_unlock(); ethnl_parse_header_dev_put(&mut req_info); return ret; }
    rtnl_unlock(); ethnl_parse_header_dev_put(&mut req_info); genlmsg_end(rskb, reply_payload);
    genlmsg_reply(rskb, info)
}

#[repr(C)]
pub struct ethnl_tunnel_info_dump_ctx { pub req_info: ethnl_req_info, pub ifindex: c_ulong }

pub unsafe fn ethnl_tunnel_info_start(cb: *mut netlink_callback) -> c_int {
    let info = genl_dumpit_info(cb); let ctx = (*cb).ctx.as_mut_ptr() as *mut ethnl_tunnel_info_dump_ctx;
    core::ptr::write_bytes(ctx, 0, 1);
    let tb = (*info).info.attrs;
    let ret = ethnl_parse_header_dev_get(&mut (*ctx).req_info, *tb.add(ETHTOOL_A_TUNNEL_INFO_HEADER as usize),
        sock_net((*cb).skb.sk), (*cb).extack, false);
    if !(*ctx).req_info.dev.is_null() { ethnl_parse_header_dev_put(&mut (*ctx).req_info); (*ctx).req_info.dev = core::ptr::null_mut(); }
    ret
}

pub unsafe fn ethnl_tunnel_info_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    let ctx = (*cb).ctx.as_mut_ptr() as *mut ethnl_tunnel_info_dump_ctx;
    let net = sock_net((*skb).sk); let mut dev: *mut net_device = core::ptr::null_mut();
    let mut ret = 0; let mut ehdr: *mut c_void;
    rtnl_lock();
    for_each_netdev_dump(net, dev, (*ctx).ifindex) {
        ehdr = ethnl_dump_put(skb, cb, ETHTOOL_MSG_TUNNEL_INFO_GET_REPLY);
        if ehdr.is_null() { ret = -EMSGSIZE; break; }
        ret = ethnl_fill_reply_header(skb, dev, ETHTOOL_A_TUNNEL_INFO_HEADER);
        if ret < 0 { genlmsg_cancel(skb, ehdr); break; }
        (*ctx).req_info.dev = dev; ret = ethnl_tunnel_info_fill_reply(&(*ctx).req_info, skb); (*ctx).req_info.dev = core::ptr::null_mut();
        if ret < 0 { genlmsg_cancel(skb, ehdr); if ret == -EOPNOTSUPP { continue; } break; }
        genlmsg_end(skb, ehdr);
    }
    rtnl_unlock();
    if ret == -EMSGSIZE && (*skb).len != 0 { return (*skb).len as c_int; }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
