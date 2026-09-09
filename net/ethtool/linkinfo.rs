// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by common.h and netlink.h remain external to this translation unit.

#[repr(C)]
pub struct linkinfo_req_info {
    pub base: ethnl_req_info,
}

#[repr(C)]
pub struct linkinfo_reply_data {
    pub base: ethnl_reply_data,
    pub ksettings: ethtool_link_ksettings,
    pub lsettings: *mut ethtool_link_settings,
}

// LINKINFO_REPDATA(__reply_base): container_of(__reply_base, struct linkinfo_reply_data, base)

pub static ethnl_linkinfo_get_policy: [nla_policy; 1] = [
    nla_policy { type_: NLA_POLICY_NESTED(ethnl_header_policy) },
];

unsafe fn linkinfo_prepare_data(
    _req_base: *const ethnl_req_info,
    reply_base: *mut ethnl_reply_data,
    info: *const genl_info,
) -> c_int {
    let data = &mut *(reply_base as *mut linkinfo_reply_data);
    let dev = (*reply_base).dev;
    let mut ret: c_int;

    data.lsettings = &mut data.ksettings.base;

    ret = ethnl_ops_begin(dev);
    if ret < 0 {
        return ret;
    }
    ret = netif_get_link_ksettings(dev, &mut data.ksettings);
    if ret < 0 {
        GENL_SET_ERR_MSG(info, "failed to retrieve link settings");
    }
    ethnl_ops_complete(dev);

    ret
}

unsafe fn linkinfo_reply_size(
    _req_base: *const ethnl_req_info,
    _reply_base: *const ethnl_reply_data,
) -> c_int {
    nla_total_size(core::mem::size_of::<u8>() as c_int) // LINKINFO_PORT
        + nla_total_size(core::mem::size_of::<u8>() as c_int) // LINKINFO_PHYADDR
        + nla_total_size(core::mem::size_of::<u8>() as c_int) // LINKINFO_TP_MDIX
        + nla_total_size(core::mem::size_of::<u8>() as c_int) // LINKINFO_TP_MDIX_CTRL
        + nla_total_size(core::mem::size_of::<u8>() as c_int) // LINKINFO_TRANSCEIVER
        + 0
}

unsafe fn linkinfo_fill_reply(
    skb: *mut sk_buff,
    _req_base: *const ethnl_req_info,
    reply_base: *const ethnl_reply_data,
) -> c_int {
    let data = &*(reply_base as *const linkinfo_reply_data);
    let lsettings = &*data.lsettings;

    if nla_put_u8(skb, ETHTOOL_A_LINKINFO_PORT, lsettings.port) != 0
        || nla_put_u8(skb, ETHTOOL_A_LINKINFO_PHYADDR, lsettings.phy_address) != 0
        || nla_put_u8(skb, ETHTOOL_A_LINKINFO_TP_MDIX, lsettings.eth_tp_mdix) != 0
        || nla_put_u8(skb, ETHTOOL_A_LINKINFO_TP_MDIX_CTRL, lsettings.eth_tp_mdix_ctrl) != 0
        || nla_put_u8(skb, ETHTOOL_A_LINKINFO_TRANSCEIVER, lsettings.transceiver) != 0
    {
        return -EMSGSIZE;
    }

    0
}

/* LINKINFO_SET */

pub static ethnl_linkinfo_set_policy: [nla_policy; 4] = [
    nla_policy { type_: NLA_POLICY_NESTED(ethnl_header_policy) },
    nla_policy { type_: NLA_U8 },
    nla_policy { type_: NLA_U8 },
    nla_policy { type_: NLA_U8 },
];

unsafe fn ethnl_set_linkinfo_validate(
    req_info: *mut ethnl_req_info,
    _info: *const genl_info,
) -> c_int {
    let ops = (*(*req_info).dev).ethtool_ops;

    if (*ops).get_link_ksettings.is_none() || (*ops).set_link_ksettings.is_none() {
        return -EOPNOTSUPP;
    }
    1
}

unsafe fn ethnl_set_linkinfo(
    req_info: *mut ethnl_req_info,
    info: *mut genl_info,
) -> c_int {
    let mut ksettings: ethtool_link_ksettings = core::mem::zeroed();
    let lsettings: *mut ethtool_link_settings;
    let dev = (*req_info).dev;
    let tb = (*info).attrs;
    let mut mod_: bool = false;
    let mut ret: c_int;

    ret = netif_get_link_ksettings(dev, &mut ksettings);
    if ret < 0 {
        GENL_SET_ERR_MSG(info, "failed to retrieve link settings");
        return ret;
    }
    lsettings = &mut ksettings.base;

    ethnl_update_u8(&mut (*lsettings).port, *tb.add(ETHTOOL_A_LINKINFO_PORT), &mut mod_);
    ethnl_update_u8(&mut (*lsettings).phy_address, *tb.add(ETHTOOL_A_LINKINFO_PHYADDR), &mut mod_);
    ethnl_update_u8(&mut (*lsettings).eth_tp_mdix_ctrl, *tb.add(ETHTOOL_A_LINKINFO_TP_MDIX_CTRL), &mut mod_);
    if !mod_ {
        return 0;
    }

    ret = ((*(*dev).ethtool_ops).set_link_ksettings.unwrap())(dev, &mut ksettings);
    if ret < 0 {
        GENL_SET_ERR_MSG(info, "link settings update failed");
        return ret;
    }

    1
}

pub static ethnl_linkinfo_request_ops: ethnl_request_ops = ethnl_request_ops {
    request_cmd: ETHTOOL_MSG_LINKINFO_GET,
    reply_cmd: ETHTOOL_MSG_LINKINFO_GET_REPLY,
    hdr_attr: ETHTOOL_A_LINKINFO_HEADER,
    req_info_size: core::mem::size_of::<linkinfo_req_info>(),
    reply_data_size: core::mem::size_of::<linkinfo_reply_data>(),
    prepare_data: Some(linkinfo_prepare_data),
    reply_size: Some(linkinfo_reply_size),
    fill_reply: Some(linkinfo_fill_reply),
    set_validate: Some(ethnl_set_linkinfo_validate),
    set: Some(ethnl_set_linkinfo),
    set_ntf_cmd: ETHTOOL_MSG_LINKINFO_NTF,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
