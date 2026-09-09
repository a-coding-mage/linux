// SPDX-License-Identifier: GPL-2.0-only
/*
 * VLAN netlink control interface
 *
 * Copyright (c) 2007 Patrick McHardy <kaber@trash.net>
 */

// Linux kernel dependencies supplied by other translation units.

static VLAN_POLICY: [struct_nla_policy; IFLA_VLAN_MAX + 1] = [
    struct_nla_policy { type_: NLA_U16, ..Default::default() },
    struct_nla_policy { ..Default::default() },
    struct_nla_policy { len: core::mem::size_of::<struct_ifla_vlan_flags>(), ..Default::default() },
    struct_nla_policy { type_: NLA_NESTED, ..Default::default() },
    struct_nla_policy { type_: NLA_NESTED, ..Default::default() },
    struct_nla_policy { type_: NLA_U16, ..Default::default() },
];

static VLAN_MAP_POLICY: [struct_nla_policy; IFLA_VLAN_QOS_MAX + 1] = [
    struct_nla_policy { ..Default::default() },
    struct_nla_policy { len: core::mem::size_of::<struct_ifla_vlan_qos_mapping>(), ..Default::default() },
];

#[inline]
unsafe fn vlan_validate_qos_map(attr: *mut struct_nlattr) -> i32 {
    if attr.is_null() {
        return 0;
    }
    nla_validate_nested_deprecated(attr, IFLA_VLAN_QOS_MAX, VLAN_MAP_POLICY.as_ptr(), core::ptr::null_mut())
}

unsafe fn vlan_validate(
    tb: *mut *mut struct_nlattr,
    data: *mut *mut struct_nlattr,
    extack: *mut struct_netlink_ext_ack,
) -> i32 {
    let flags: *mut struct_ifla_vlan_flags;
    let id: u16;
    let mut err: i32;

    if !(*tb.add(IFLA_ADDRESS)).is_null() {
        if nla_len(*tb.add(IFLA_ADDRESS)) != ETH_ALEN {
            NL_SET_ERR_MSG_MOD(extack, "Invalid link address");
            return -EINVAL;
        }
        if !is_valid_ether_addr(nla_data(*tb.add(IFLA_ADDRESS))) {
            NL_SET_ERR_MSG_MOD(extack, "Invalid link address");
            return -EADDRNOTAVAIL;
        }
    }

    if data.is_null() {
        NL_SET_ERR_MSG_MOD(extack, "VLAN properties not specified");
        return -EINVAL;
    }

    if !(*data.add(IFLA_VLAN_PROTOCOL)).is_null() {
        match nla_get_be16(*data.add(IFLA_VLAN_PROTOCOL)) {
            x if x == htons(ETH_P_8021Q) || x == htons(ETH_P_8021AD) => {}
            _ => {
                NL_SET_ERR_MSG_MOD(extack, "Invalid VLAN protocol");
                return -EPROTONOSUPPORT;
            }
        }
    }

    if !(*data.add(IFLA_VLAN_ID)).is_null() {
        id = nla_get_u16(*data.add(IFLA_VLAN_ID));
        if id >= VLAN_VID_MASK {
            NL_SET_ERR_MSG_MOD(extack, "Invalid VLAN id");
            return -ERANGE;
        }
    }
    if !(*data.add(IFLA_VLAN_FLAGS)).is_null() {
        flags = nla_data(*data.add(IFLA_VLAN_FLAGS)) as *mut struct_ifla_vlan_flags;
        if ((*flags).flags & (*flags).mask)
            & !(VLAN_FLAG_REORDER_HDR | VLAN_FLAG_GVRP | VLAN_FLAG_LOOSE_BINDING |
                VLAN_FLAG_MVRP | VLAN_FLAG_BRIDGE_BINDING) != 0
        {
            NL_SET_ERR_MSG_MOD(extack, "Invalid VLAN flags");
            return -EINVAL;
        }
    }

    err = vlan_validate_qos_map(*data.add(IFLA_VLAN_INGRESS_QOS));
    if err < 0 {
        NL_SET_ERR_MSG_MOD(extack, "Invalid ingress QOS map");
        return err;
    }
    err = vlan_validate_qos_map(*data.add(IFLA_VLAN_EGRESS_QOS));
    if err < 0 {
        NL_SET_ERR_MSG_MOD(extack, "Invalid egress QOS map");
        return err;
    }
    0
}

unsafe fn vlan_changelink(
    dev: *mut struct_net_device,
    _tb: *mut *mut struct_nlattr,
    data: *mut *mut struct_nlattr,
    _extack: *mut struct_netlink_ext_ack,
) -> i32 {
    let mut err: i32;
    if !(*data.add(IFLA_VLAN_FLAGS)).is_null() {
        let flags = nla_data(*data.add(IFLA_VLAN_FLAGS)) as *mut struct_ifla_vlan_flags;
        err = vlan_dev_change_flags(dev, (*flags).flags, (*flags).mask);
        if err != 0 { return err; }
    }
    if !(*data.add(IFLA_VLAN_INGRESS_QOS)).is_null() {
        let mut attr: *mut struct_nlattr = core::ptr::null_mut();
        let mut rem: i32 = 0;
        nla_for_each_nested_type!(attr, IFLA_VLAN_QOS_MAPPING, *data.add(IFLA_VLAN_INGRESS_QOS), rem) {
            let m = nla_data(attr) as *mut struct_ifla_vlan_qos_mapping;
            vlan_dev_set_ingress_priority(dev, (*m).to, (*m).from);
        }
    }
    if !(*data.add(IFLA_VLAN_EGRESS_QOS)).is_null() {
        let mut attr: *mut struct_nlattr = core::ptr::null_mut();
        let mut rem: i32 = 0;
        nla_for_each_nested_type!(attr, IFLA_VLAN_QOS_MAPPING, *data.add(IFLA_VLAN_EGRESS_QOS), rem) {
            let m = nla_data(attr) as *mut struct_ifla_vlan_qos_mapping;
            err = vlan_dev_set_egress_priority(dev, (*m).from, (*m).to);
            if err != 0 { return err; }
        }
    }
    0
}

unsafe fn vlan_newlink(
    dev: *mut struct_net_device,
    params: *mut struct_rtnl_newlink_params,
    extack: *mut struct_netlink_ext_ack,
) -> i32 {
    let link_net = rtnl_newlink_link_net(params);
    let vlan = vlan_dev_priv(dev);
    let data = (*params).data;
    let tb = (*params).tb;
    let real_dev: *mut struct_net_device;
    let max_mtu: u32;
    let proto: __be16;
    let mut err: i32;

    if (*data.add(IFLA_VLAN_ID)).is_null() {
        NL_SET_ERR_MSG_MOD(extack, "VLAN id not specified"); return -EINVAL;
    }
    if (*tb.add(IFLA_LINK)).is_null() {
        NL_SET_ERR_MSG_MOD(extack, "link not specified"); return -EINVAL;
    }
    real_dev = __dev_get_by_index(link_net, nla_get_u32(*tb.add(IFLA_LINK)));
    if real_dev.is_null() {
        NL_SET_ERR_MSG_MOD(extack, "link does not exist"); return -ENODEV;
    }
    proto = nla_get_be16_default(*data.add(IFLA_VLAN_PROTOCOL), htons(ETH_P_8021Q));
    (*vlan).vlan_proto = proto;
    (*vlan).vlan_id = nla_get_u16(*data.add(IFLA_VLAN_ID));
    (*vlan).real_dev = real_dev;
    (*dev).priv_flags |= (*real_dev).priv_flags & IFF_XMIT_DST_RELEASE;
    (*vlan).flags = VLAN_FLAG_REORDER_HDR;
    err = vlan_check_real_dev(real_dev, (*vlan).vlan_proto, (*vlan).vlan_id, extack);
    if err < 0 { return err; }
    max_mtu = if netif_reduces_vlan_mtu(real_dev) { (*real_dev).mtu - VLAN_HLEN } else { (*real_dev).mtu };
    if (*tb.add(IFLA_MTU)).is_null() { (*dev).mtu = max_mtu; }
    else if (*dev).mtu > max_mtu { return -EINVAL; }
    // If this initial vlan_changelink() fails, free egress priority memory.
    err = vlan_changelink(dev, tb, data, extack);
    if err == 0 { err = register_vlan_dev(dev, extack); }
    if err != 0 { vlan_dev_free_egress_priority(dev); }
    err
}

#[inline]
unsafe fn vlan_qos_map_size(n: u32) -> usize {
    if n == 0 { return 0; }
    nla_total_size(core::mem::size_of::<struct_nlattr>()) +
        nla_total_size(core::mem::size_of::<struct_ifla_vlan_qos_mapping>()) * n as usize
}

unsafe fn vlan_get_size(dev: *const struct_net_device) -> usize {
    let vlan = vlan_dev_priv(dev as *mut struct_net_device);
    nla_total_size(2) + nla_total_size(2) +
        nla_total_size(core::mem::size_of::<struct_ifla_vlan_flags>()) +
        vlan_qos_map_size((*vlan).nr_ingress_mappings) + vlan_qos_map_size((*vlan).nr_egress_mappings)
}

unsafe fn vlan_fill_info(skb: *mut struct_sk_buff, dev: *const struct_net_device) -> i32 {
    let vlan = vlan_dev_priv(dev as *mut struct_net_device);
    let mut f: struct_ifla_vlan_flags;
    let mut m: struct_ifla_vlan_qos_mapping;
    let mut nest: *mut struct_nlattr;
    let mut i: u32;
    if nla_put_be16(skb, IFLA_VLAN_PROTOCOL, (*vlan).vlan_proto) != 0 ||
       nla_put_u16(skb, IFLA_VLAN_ID, (*vlan).vlan_id) != 0 { return -EMSGSIZE; }
    if (*vlan).flags != 0 {
        f.flags = (*vlan).flags; f.mask = !0;
        if nla_put(skb, IFLA_VLAN_FLAGS, core::mem::size_of_val(&f), &f as *const _ as *const core::ffi::c_void) != 0 { return -EMSGSIZE; }
    }
    if (*vlan).nr_ingress_mappings != 0 {
        nest = nla_nest_start_noflag(skb, IFLA_VLAN_INGRESS_QOS);
        if nest.is_null() { return -EMSGSIZE; }
        i = 0;
        while i < core::mem::size_of_val(&(*vlan).ingress_priority_map) as u32 / core::mem::size_of::<u8>() as u32 {
            if (*vlan).ingress_priority_map[i as usize] != 0 {
                m.from = i; m.to = (*vlan).ingress_priority_map[i as usize];
                if nla_put(skb, IFLA_VLAN_QOS_MAPPING, core::mem::size_of_val(&m), &m as *const _ as *const core::ffi::c_void) != 0 { return -EMSGSIZE; }
            }
            i += 1;
        }
        nla_nest_end(skb, nest);
    }
    if (*vlan).nr_egress_mappings != 0 {
        nest = nla_nest_start_noflag(skb, IFLA_VLAN_EGRESS_QOS);
        if nest.is_null() { return -EMSGSIZE; }
        i = 0;
        while i < core::mem::size_of_val(&(*vlan).egress_priority_map) as u32 / core::mem::size_of::<u8>() as u32 {
            let mut pm = rcu_dereference_rtnl((*vlan).egress_priority_map[i as usize]);
            while !pm.is_null() {
                let vlan_qos = READ_ONCE((*pm).vlan_qos);
                m.from = (*pm).priority; m.to = (vlan_qos >> 13) & 0x7;
                if nla_put(skb, IFLA_VLAN_QOS_MAPPING, core::mem::size_of_val(&m), &m as *const _ as *const core::ffi::c_void) != 0 { return -EMSGSIZE; }
                pm = rcu_dereference_rtnl((*pm).next);
            }
            i += 1;
        }
        nla_nest_end(skb, nest);
    }
    0
}

unsafe fn vlan_get_link_net(dev: *const struct_net_device) -> *mut struct_net {
    dev_net(vlan_dev_priv(dev as *mut struct_net_device).as_ref().unwrap().real_dev)
}

#[no_mangle]
pub static mut vlan_link_ops: struct_rtnl_link_ops = struct_rtnl_link_ops {
    kind: "vlan", maxtype: IFLA_VLAN_MAX, policy: VLAN_POLICY.as_ptr(),
    priv_size: core::mem::size_of::<struct_vlan_dev_priv>(), setup: Some(vlan_setup),
    validate: Some(vlan_validate), newlink: Some(vlan_newlink), changelink: Some(vlan_changelink),
    dellink: Some(unregister_vlan_dev), get_size: Some(vlan_get_size),
    fill_info: Some(vlan_fill_info), get_link_net: Some(vlan_get_link_net),
};

#[no_mangle]
pub unsafe extern "C" fn vlan_netlink_init() -> i32 { rtnl_link_register(&raw mut vlan_link_ops) }

#[no_mangle]
pub unsafe extern "C" fn vlan_netlink_fini() { rtnl_link_unregister(&raw mut vlan_link_ops); }

// MODULE_ALIAS_RTNL_LINK("vlan");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
