// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding ethtool/kernel bindings.

#[repr(C)]
pub struct linkstate_req_info {
    pub base: ethnl_req_info,
}

#[repr(C)]
pub struct linkstate_reply_data {
    pub base: ethnl_reply_data,
    pub link: i32,
    pub sqi: i32,
    pub sqi_max: i32,
    pub link_stats: ethtool_link_ext_stats,
    pub link_ext_state_provided: bool,
    pub ethtool_link_ext_state_info: ethtool_link_ext_state_info,
}

#[inline]
unsafe fn linkstate_repdata(reply_base: *mut ethnl_reply_data) -> *mut linkstate_reply_data {
    reply_base as *mut linkstate_reply_data
}

pub static ethnl_linkstate_get_policy: [nla_policy; ETHTOOL_A_LINKSTATE_HEADER as usize + 1] =
    [nla_policy::nested(ethnl_header_policy_stats); ETHTOOL_A_LINKSTATE_HEADER as usize + 1];

unsafe fn linkstate_get_sqi(phydev: *mut phy_device) -> i32 {
    let mut ret: i32;
    if phydev.is_null() {
        return -EOPNOTSUPP;
    }
    mutex_lock(&mut (*phydev).lock);
    if (*phydev).drv.is_null() || (*(*phydev).drv).get_sqi.is_none() {
        ret = -EOPNOTSUPP;
    } else if !(*phydev).link {
        ret = -ENETDOWN;
    } else {
        ret = ((*(*phydev).drv).get_sqi.unwrap())(phydev);
    }
    mutex_unlock(&mut (*phydev).lock);
    ret
}

unsafe fn linkstate_get_sqi_max(phydev: *mut phy_device) -> i32 {
    let mut ret: i32;
    if phydev.is_null() {
        return -EOPNOTSUPP;
    }
    mutex_lock(&mut (*phydev).lock);
    if (*phydev).drv.is_null() || (*(*phydev).drv).get_sqi_max.is_none() {
        ret = -EOPNOTSUPP;
    } else if !(*phydev).link {
        ret = -ENETDOWN;
    } else {
        ret = ((*(*phydev).drv).get_sqi_max.unwrap())(phydev);
    }
    mutex_unlock(&mut (*phydev).lock);
    ret
}

fn linkstate_sqi_critical_error(sqi: i32) -> bool {
    sqi < 0 && sqi != -EOPNOTSUPP && sqi != -ENETDOWN
}

unsafe fn linkstate_sqi_valid(data: *const linkstate_reply_data) -> bool {
    (*data).sqi >= 0 && (*data).sqi_max >= 0 && (*data).sqi <= (*data).sqi_max
}

unsafe fn linkstate_get_link_ext_state(
    dev: *mut net_device,
    data: *mut linkstate_reply_data,
) -> i32 {
    if (*(*dev).ethtool_ops).get_link_ext_state.is_none() {
        return -EOPNOTSUPP;
    }
    let err = ((*(*dev).ethtool_ops).get_link_ext_state.unwrap())(
        dev,
        &mut (*data).ethtool_link_ext_state_info,
    );
    if err != 0 {
        return err;
    }
    (*data).link_ext_state_provided = true;
    0
}

unsafe fn linkstate_prepare_data(
    req_base: *const ethnl_req_info,
    reply_base: *mut ethnl_reply_data,
    info: *const genl_info,
) -> i32 {
    let data = linkstate_repdata(reply_base);
    let dev = (*reply_base).dev;
    let tb = (*info).attrs;
    let phydev = ethnl_req_get_phydev(req_base, tb, ETHTOOL_A_LINKSTATE_HEADER, (*info).extack);
    if IS_ERR(phydev) {
        return PTR_ERR(phydev);
    }
    let mut ret = ethnl_ops_begin(dev);
    if ret < 0 { return ret; }
    (*data).link = __ethtool_get_link(dev);
    ret = linkstate_get_sqi(phydev);
    if linkstate_sqi_critical_error(ret) { ethnl_ops_complete(dev); return ret; }
    (*data).sqi = ret;
    ret = linkstate_get_sqi_max(phydev);
    if linkstate_sqi_critical_error(ret) { ethnl_ops_complete(dev); return ret; }
    (*data).sqi_max = ret;
    if (*dev).flags & IFF_UP != 0 {
        ret = linkstate_get_link_ext_state(dev, data);
        if ret < 0 && ret != -EOPNOTSUPP && ret != -ENODATA {
            ethnl_ops_complete(dev); return ret;
        }
    }
    ethtool_stats_init(&mut (*data).link_stats as *mut _ as *mut u64,
                       core::mem::size_of::<ethtool_link_ext_stats>() / 8);
    if (*req_base).flags & ETHTOOL_FLAG_STATS != 0 {
        if !phydev.is_null() { phy_ethtool_get_link_ext_stats(phydev, &mut (*data).link_stats); }
        if (*(*dev).ethtool_ops).get_link_ext_stats.is_some() {
            ((*(*dev).ethtool_ops).get_link_ext_stats.unwrap())(dev, &mut (*data).link_stats);
        }
    }
    ethnl_ops_complete(dev);
    0
}

unsafe fn linkstate_reply_size(_req_base: *const ethnl_req_info, reply_base: *const ethnl_reply_data) -> i32 {
    let data = linkstate_repdata(reply_base as *mut _);
    let mut len = nla_total_size(core::mem::size_of::<u8>());
    if linkstate_sqi_valid(data) { len += nla_total_size(core::mem::size_of::<u32>()) * 2; }
    if (*data).link_ext_state_provided { len += nla_total_size(core::mem::size_of::<u8>()); }
    if (*data).ethtool_link_ext_state_info.__link_ext_substate != 0 { len += nla_total_size(core::mem::size_of::<u8>()); }
    if (*data).link_stats.link_down_events != ETHTOOL_STAT_NOT_SET { len += nla_total_size(core::mem::size_of::<u32>()); }
    len
}

unsafe fn linkstate_fill_reply(skb: *mut sk_buff, _req_base: *const ethnl_req_info, reply_base: *const ethnl_reply_data) -> i32 {
    let data = linkstate_repdata(reply_base as *mut _);
    if (*data).link >= 0 && nla_put_u8(skb, ETHTOOL_A_LINKSTATE_LINK, ((*data).link != 0) as u8) != 0 { return -EMSGSIZE; }
    if linkstate_sqi_valid(data) {
        if nla_put_u32(skb, ETHTOOL_A_LINKSTATE_SQI, (*data).sqi as u32) != 0 { return -EMSGSIZE; }
        if nla_put_u32(skb, ETHTOOL_A_LINKSTATE_SQI_MAX, (*data).sqi_max as u32) != 0 { return -EMSGSIZE; }
    }
    if (*data).link_ext_state_provided {
        if nla_put_u8(skb, ETHTOOL_A_LINKSTATE_EXT_STATE, (*data).ethtool_link_ext_state_info.link_ext_state) != 0 { return -EMSGSIZE; }
        if (*data).ethtool_link_ext_state_info.__link_ext_substate != 0 && nla_put_u8(skb, ETHTOOL_A_LINKSTATE_EXT_SUBSTATE, (*data).ethtool_link_ext_state_info.__link_ext_substate) != 0 { return -EMSGSIZE; }
    }
    if (*data).link_stats.link_down_events != ETHTOOL_STAT_NOT_SET && nla_put_u32(skb, ETHTOOL_A_LINKSTATE_EXT_DOWN_CNT, (*data).link_stats.link_down_events) != 0 { return -EMSGSIZE; }
    0
}

pub static ethnl_linkstate_request_ops: ethnl_request_ops = ethnl_request_ops {
    request_cmd: ETHTOOL_MSG_LINKSTATE_GET,
    reply_cmd: ETHTOOL_MSG_LINKSTATE_GET_REPLY,
    hdr_attr: ETHTOOL_A_LINKSTATE_HEADER,
    req_info_size: core::mem::size_of::<linkstate_req_info>(),
    reply_data_size: core::mem::size_of::<linkstate_reply_data>(),
    prepare_data: Some(linkstate_prepare_data),
    reply_size: Some(linkstate_reply_size),
    fill_reply: Some(linkstate_fill_reply),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
