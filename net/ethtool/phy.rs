// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2023 Bootlin
 *
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

#[repr(C)]
pub struct phy_req_info {
    pub base: ethnl_req_info,
}

#[repr(C)]
pub struct phy_reply_data {
    pub base: ethnl_reply_data,
    pub phyindex: u32,
    pub drvname: *mut ::core::ffi::c_char,
    pub name: *mut ::core::ffi::c_char,
    pub upstream_type: ::core::ffi::c_uint,
    pub upstream_sfp_name: *mut ::core::ffi::c_char,
    pub upstream_index: ::core::ffi::c_uint,
    pub downstream_sfp_name: *mut ::core::ffi::c_char,
}

// PHY_REPDATA(__reply_base) == container_of(__reply_base, struct phy_reply_data, base)

pub static ethnl_phy_get_policy: [nla_policy; (ETHTOOL_A_PHY_HEADER + 1) as usize] = [
    // ETHTOOL_A_PHY_HEADER = NLA_POLICY_NESTED(ethnl_header_policy)
];

unsafe fn phy_reply_size(
    _req_info: *const ethnl_req_info,
    reply_data: *const ethnl_reply_data,
) -> isize {
    let rep_data = &*(reply_data as *const phy_reply_data);
    let mut size: usize = 0;

    // ETHTOOL_A_PHY_INDEX
    size += nla_total_size(core::mem::size_of::<u32>());

    // ETHTOOL_A_DRVNAME
    if !rep_data.drvname.is_null() {
        size += nla_total_size(strlen(rep_data.drvname) + 1);
    }

    // ETHTOOL_A_NAME
    size += nla_total_size(strlen(rep_data.name) + 1);

    // ETHTOOL_A_PHY_UPSTREAM_TYPE
    size += nla_total_size(core::mem::size_of::<u32>());

    // ETHTOOL_A_PHY_UPSTREAM_SFP_NAME
    if !rep_data.upstream_sfp_name.is_null() {
        size += nla_total_size(strlen(rep_data.upstream_sfp_name) + 1);
    }

    // ETHTOOL_A_PHY_UPSTREAM_INDEX
    if rep_data.upstream_index != 0 {
        size += nla_total_size(core::mem::size_of::<u32>());
    }

    // ETHTOOL_A_PHY_DOWNSTREAM_SFP_NAME
    if !rep_data.downstream_sfp_name.is_null() {
        size += nla_total_size(strlen(rep_data.downstream_sfp_name) + 1);
    }

    size as isize
}

unsafe fn phy_prepare_data(
    req_info: *const ethnl_req_info,
    reply_data: *mut ethnl_reply_data,
    info: *const genl_info,
) -> i32 {
    let topo = (*reply_data).dev.link_topo;
    let rep_data = &mut *(reply_data as *mut phy_reply_data);
    let tb = (*info).attrs;
    let phydev = ethnl_req_get_phydev(req_info, tb, ETHTOOL_A_PHY_HEADER, (*info).extack);
    if phydev.is_err_or_null() {
        return -EOPNOTSUPP;
    }

    let pdn = xa_load(&(*topo).phys, (*phydev).phyindex);
    if pdn.is_null() {
        return -EOPNOTSUPP;
    }

    rep_data.phyindex = (*phydev).phyindex;
    rep_data.name = kstrdup(dev_name(&(*phydev).mdio.dev), GFP_KERNEL);
    if rep_data.name.is_null() {
        return -ENOMEM;
    }

    if !(*phydev).drv.is_null() {
        rep_data.drvname = kstrdup((*(*phydev).drv).name, GFP_KERNEL);
        if rep_data.drvname.is_null() {
            let ret = -ENOMEM;
            kfree(rep_data.name);
            return ret;
        }
    }

    rep_data.upstream_type = (*pdn).upstream_type;

    if (*pdn).upstream_type == PHY_UPSTREAM_PHY {
        let upstream = (*pdn).upstream.phydev;
        rep_data.upstream_index = (*upstream).phyindex;
    }

    if !(*pdn).parent_sfp_bus.is_null() {
        rep_data.upstream_sfp_name = kstrdup(sfp_get_name((*pdn).parent_sfp_bus), GFP_KERNEL);
        if rep_data.upstream_sfp_name.is_null() {
            let ret = -ENOMEM;
            kfree(rep_data.drvname);
            kfree(rep_data.name);
            return ret;
        }
    }

    if !(*phydev).sfp_bus.is_null() {
        rep_data.downstream_sfp_name = kstrdup(sfp_get_name((*phydev).sfp_bus), GFP_KERNEL);
        if rep_data.downstream_sfp_name.is_null() {
            let ret = -ENOMEM;
            kfree(rep_data.upstream_sfp_name);
            kfree(rep_data.drvname);
            kfree(rep_data.name);
            return ret;
        }
    }

    0
}

unsafe fn phy_fill_reply(
    skb: *mut sk_buff,
    _req_info: *const ethnl_req_info,
    reply_data: *const ethnl_reply_data,
) -> i32 {
    let rep_data = &*(reply_data as *const phy_reply_data);

    if nla_put_u32(skb, ETHTOOL_A_PHY_INDEX, rep_data.phyindex) != 0
        || nla_put_string(skb, ETHTOOL_A_PHY_NAME, rep_data.name) != 0
        || nla_put_u32(skb, ETHTOOL_A_PHY_UPSTREAM_TYPE, rep_data.upstream_type) != 0
    {
        return -EMSGSIZE;
    }

    if !rep_data.drvname.is_null() && nla_put_string(skb, ETHTOOL_A_PHY_DRVNAME, rep_data.drvname) != 0 {
        return -EMSGSIZE;
    }
    if rep_data.upstream_index != 0
        && nla_put_u32(skb, ETHTOOL_A_PHY_UPSTREAM_INDEX, rep_data.upstream_index) != 0
    {
        return -EMSGSIZE;
    }
    if !rep_data.upstream_sfp_name.is_null()
        && nla_put_string(skb, ETHTOOL_A_PHY_UPSTREAM_SFP_NAME, rep_data.upstream_sfp_name) != 0
    {
        return -EMSGSIZE;
    }
    if !rep_data.downstream_sfp_name.is_null()
        && nla_put_string(skb, ETHTOOL_A_PHY_DOWNSTREAM_SFP_NAME, rep_data.downstream_sfp_name) != 0
    {
        return -EMSGSIZE;
    }
    0
}

unsafe fn phy_cleanup_data(reply_data: *mut ethnl_reply_data) {
    let rep_data = &mut *(reply_data as *mut phy_reply_data);
    kfree(rep_data.drvname);
    kfree(rep_data.name);
    kfree(rep_data.upstream_sfp_name);
    kfree(rep_data.downstream_sfp_name);
}

pub static ethnl_phy_request_ops: ethnl_request_ops = ethnl_request_ops {
    request_cmd: ETHTOOL_MSG_PHY_GET,
    reply_cmd: ETHTOOL_MSG_PHY_GET_REPLY,
    hdr_attr: ETHTOOL_A_PHY_HEADER,
    req_info_size: core::mem::size_of::<phy_req_info>(),
    reply_data_size: core::mem::size_of::<phy_reply_data>(),
    prepare_data: Some(phy_prepare_data),
    reply_size: Some(phy_reply_size),
    fill_reply: Some(phy_fill_reply),
    cleanup_data: Some(phy_cleanup_data),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
