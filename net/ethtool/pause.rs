// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by common.h and netlink.h in the surrounding translation.

#[repr(C)]
struct pause_req_info {
    base: ethnl_req_info,
    src: ethtool_mac_stats_src,
}

#[repr(C)]
struct pause_reply_data {
    base: ethnl_reply_data,
    pauseparam: ethtool_pauseparam,
    pausestat: ethtool_pause_stats,
}

const ethnl_pause_get_policy: [nla_policy; _] = [
    [ETHTOOL_A_PAUSE_HEADER] = NLA_POLICY_NESTED(ethnl_header_policy_stats),
    [ETHTOOL_A_PAUSE_STATS_SRC] = NLA_POLICY_MAX(NLA_U32, ETHTOOL_MAC_STATS_SRC_PMAC),
];

unsafe fn pause_parse_request(
    req_base: *mut ethnl_req_info,
    info: *const genl_info,
    tb: *mut *mut nlattr,
    extack: *mut netlink_ext_ack,
) -> i32 {
    let mut src: ethtool_mac_stats_src = ETHTOOL_MAC_STATS_SRC_AGGREGATE;
    let req_info = &mut *((req_base as *mut u8).sub(offsetof_pause_req_info_base()) as *mut pause_req_info);

    if !(*tb.add(ETHTOOL_A_PAUSE_STATS_SRC)).is_null() {
        if (*req_base).flags & ETHTOOL_FLAG_STATS == 0 {
            NL_SET_ERR_MSG_MOD(extack, "ETHTOOL_FLAG_STATS must be set when requesting a source of stats");
            return -EINVAL;
        }
        src = nla_get_u32(*tb.add(ETHTOOL_A_PAUSE_STATS_SRC));
    }

    req_info.src = src;
    0
}

unsafe fn pause_prepare_data(
    req_base: *const ethnl_req_info,
    reply_base: *mut ethnl_reply_data,
    info: *const genl_info,
) -> i32 {
    let req_info = &*((req_base as *const u8).sub(offsetof_pause_req_info_base()) as *const pause_req_info);
    let data = &mut *((reply_base as *mut u8).sub(offsetof_pause_reply_data_base()) as *mut pause_reply_data);
    let src = req_info.src;
    let dev = (*reply_base).dev;

    if (*(*dev).ethtool_ops).get_pauseparam.is_none() { return -EOPNOTSUPP; }
    ethtool_stats_init(&mut data.pausestat as *mut _ as *mut u64, core::mem::size_of::<ethtool_pause_stats>() / 8);
    data.pausestat.src = src;
    let ret = ethnl_ops_begin(dev);
    if ret < 0 { return ret; }
    if (src == ETHTOOL_MAC_STATS_SRC_EMAC || src == ETHTOOL_MAC_STATS_SRC_PMAC) && !__ethtool_dev_mm_supported(dev) {
        NL_SET_ERR_MSG_MOD((*info).extack, "Device does not support MAC merge layer");
        ethnl_ops_complete(dev);
        return -EOPNOTSUPP;
    }
    ((*(*dev).ethtool_ops).get_pauseparam.unwrap())(dev, &mut data.pauseparam);
    if (*req_base).flags & ETHTOOL_FLAG_STATS != 0 {
        if let Some(get_pause_stats) = (*(*dev).ethtool_ops).get_pause_stats {
            get_pause_stats(dev, &mut data.pausestat);
        }
    }
    ethnl_ops_complete(dev);
    0
}

unsafe fn pause_reply_size(req_base: *const ethnl_req_info, _reply_base: *const ethnl_reply_data) -> i32 {
    let mut n = nla_total_size(core::mem::size_of::<u8>()) * 3;
    if (*req_base).flags & ETHTOOL_FLAG_STATS != 0 {
        n += nla_total_size(0) + nla_total_size(core::mem::size_of::<u32>())
            + nla_total_size_64bit(core::mem::size_of::<u64>()) * ETHTOOL_PAUSE_STAT_CNT;
    }
    n
}

unsafe fn ethtool_put_stat(skb: *mut sk_buff, val: u64, attrtype: u16, padtype: u16) -> i32 {
    if val == ETHTOOL_STAT_NOT_SET { return 0; }
    if nla_put_u64_64bit(skb, attrtype, val, padtype) != 0 { return -EMSGSIZE; }
    0
}

unsafe fn pause_put_stats(skb: *mut sk_buff, pause_stats: *const ethtool_pause_stats) -> i32 {
    let pad = ETHTOOL_A_PAUSE_STAT_PAD;
    if nla_put_u32(skb, ETHTOOL_A_PAUSE_STATS_SRC, (*pause_stats).src) != 0 { return -EMSGSIZE; }
    let nest = nla_nest_start(skb, ETHTOOL_A_PAUSE_STATS);
    if nest.is_null() { return -EMSGSIZE; }
    if ethtool_put_stat(skb, (*pause_stats).tx_pause_frames, ETHTOOL_A_PAUSE_STAT_TX_FRAMES, pad) != 0
        || ethtool_put_stat(skb, (*pause_stats).rx_pause_frames, ETHTOOL_A_PAUSE_STAT_RX_FRAMES, pad) != 0
        || ethtool_put_stat(skb, (*pause_stats).tx_pause_storm_events, ETHTOOL_A_PAUSE_STAT_TX_PAUSE_STORM_EVENTS, pad) != 0 {
        nla_nest_cancel(skb, nest);
        return -EMSGSIZE;
    }
    nla_nest_end(skb, nest);
    0
}

unsafe fn pause_fill_reply(skb: *mut sk_buff, req_base: *const ethnl_req_info, reply_base: *const ethnl_reply_data) -> i32 {
    let data = &*((reply_base as *const u8).sub(offsetof_pause_reply_data_base()) as *const pause_reply_data);
    let pauseparam = &data.pauseparam;
    if nla_put_u8(skb, ETHTOOL_A_PAUSE_AUTONEG, (pauseparam.autoneg != 0) as u8) != 0
        || nla_put_u8(skb, ETHTOOL_A_PAUSE_RX, (pauseparam.rx_pause != 0) as u8) != 0
        || nla_put_u8(skb, ETHTOOL_A_PAUSE_TX, (pauseparam.tx_pause != 0) as u8) != 0 { return -EMSGSIZE; }
    if (*req_base).flags & ETHTOOL_FLAG_STATS != 0 && pause_put_stats(skb, &data.pausestat) != 0 { return -EMSGSIZE; }
    0
}

// PAUSE_SET
const ethnl_pause_set_policy: [nla_policy; _] = [
    [ETHTOOL_A_PAUSE_HEADER] = NLA_POLICY_NESTED(ethnl_header_policy),
    [ETHTOOL_A_PAUSE_AUTONEG] = nla_policy { type_: NLA_U8 },
    [ETHTOOL_A_PAUSE_RX] = nla_policy { type_: NLA_U8 },
    [ETHTOOL_A_PAUSE_TX] = nla_policy { type_: NLA_U8 },
    [ETHTOOL_A_PAUSE_STATS_SRC] = nla_policy { type_: NLA_REJECT },
];

unsafe fn ethnl_set_pause_validate(req_info: *mut ethnl_req_info, _info: *mut genl_info) -> i32 {
    let ops = (*(*req_info).dev).ethtool_ops;
    if (*ops).get_pauseparam.is_some() && (*ops).set_pauseparam.is_some() { 1 } else { -EOPNOTSUPP }
}

unsafe fn ethnl_set_pause(req_info: *mut ethnl_req_info, info: *mut genl_info) -> i32 {
    let dev = (*req_info).dev;
    let mut params: ethtool_pauseparam = core::mem::zeroed();
    let mut modified = false;
    ((*(*dev).ethtool_ops).get_pauseparam.unwrap())(dev, &mut params);
    ethnl_update_bool32(&mut params.autoneg, (*info).attrs.add(ETHTOOL_A_PAUSE_AUTONEG), &mut modified);
    ethnl_update_bool32(&mut params.rx_pause, (*info).attrs.add(ETHTOOL_A_PAUSE_RX), &mut modified);
    ethnl_update_bool32(&mut params.tx_pause, (*info).attrs.add(ETHTOOL_A_PAUSE_TX), &mut modified);
    if !modified { return 0; }
    let ret = ((*(*dev).ethtool_ops).set_pauseparam.unwrap())(dev, &mut params);
    if ret < 0 { ret } else { 1 }
}

const ethnl_pause_request_ops: ethnl_request_ops = ethnl_request_ops {
    request_cmd: ETHTOOL_MSG_PAUSE_GET,
    reply_cmd: ETHTOOL_MSG_PAUSE_GET_REPLY,
    hdr_attr: ETHTOOL_A_PAUSE_HEADER,
    req_info_size: core::mem::size_of::<pause_req_info>(),
    reply_data_size: core::mem::size_of::<pause_reply_data>(),
    parse_request: Some(pause_parse_request),
    prepare_data: Some(pause_prepare_data),
    reply_size: Some(pause_reply_size),
    fill_reply: Some(pause_fill_reply),
    set_validate: Some(ethnl_set_pause_validate),
    set: Some(ethnl_set_pause),
    set_ntf_cmd: ETHTOOL_MSG_PAUSE_NTF,
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
