// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct channels_req_info {
    pub base: ethnl_req_info,
}

#[repr(C)]
pub struct channels_reply_data {
    pub base: ethnl_reply_data,
    pub channels: ethtool_channels,
}

#[inline]
unsafe fn channels_repdata(reply_base: *mut ethnl_reply_data) -> *mut channels_reply_data {
    reply_base as *mut channels_reply_data
}

pub static ethnl_channels_get_policy: [nla_policy; ETHTOOL_A_CHANNELS_HEADER as usize + 1] =
    [NLA_POLICY_NESTED(ethnl_header_policy); ETHTOOL_A_CHANNELS_HEADER as usize + 1];

unsafe fn channels_prepare_data(
    _req_base: *const ethnl_req_info,
    reply_base: *mut ethnl_reply_data,
    _info: *const genl_info,
) -> i32 {
    let data = &mut *channels_repdata(reply_base);
    let dev = (*reply_base).dev;
    if (*(*dev).ethtool_ops).get_channels.is_none() {
        return -EOPNOTSUPP;
    }
    let ret = ethnl_ops_begin(dev);
    if ret < 0 {
        return ret;
    }
    ((*(*dev).ethtool_ops).get_channels.unwrap())(dev, &mut data.channels);
    ethnl_ops_complete(dev);
    0
}

unsafe fn channels_reply_size(
    _req_base: *const ethnl_req_info,
    _reply_base: *const ethnl_reply_data,
) -> i32 {
    nla_total_size(core::mem::size_of::<u32>() as i32)
        + nla_total_size(core::mem::size_of::<u32>() as i32)
        + nla_total_size(core::mem::size_of::<u32>() as i32)
        + nla_total_size(core::mem::size_of::<u32>() as i32)
        + nla_total_size(core::mem::size_of::<u32>() as i32)
        + nla_total_size(core::mem::size_of::<u32>() as i32)
        + nla_total_size(core::mem::size_of::<u32>() as i32)
        + nla_total_size(core::mem::size_of::<u32>() as i32)
}

unsafe fn channels_fill_reply(
    skb: *mut sk_buff,
    _req_base: *const ethnl_req_info,
    reply_base: *const ethnl_reply_data,
) -> i32 {
    let data = &*(reply_base as *const channels_reply_data);
    let channels = &data.channels;
    if (channels.max_rx != 0
        && (nla_put_u32(skb, ETHTOOL_A_CHANNELS_RX_MAX, channels.max_rx) != 0
            || nla_put_u32(skb, ETHTOOL_A_CHANNELS_RX_COUNT, channels.rx_count) != 0))
        || (channels.max_tx != 0
            && (nla_put_u32(skb, ETHTOOL_A_CHANNELS_TX_MAX, channels.max_tx) != 0
                || nla_put_u32(skb, ETHTOOL_A_CHANNELS_TX_COUNT, channels.tx_count) != 0))
        || (channels.max_other != 0
            && (nla_put_u32(skb, ETHTOOL_A_CHANNELS_OTHER_MAX, channels.max_other) != 0
                || nla_put_u32(skb, ETHTOOL_A_CHANNELS_OTHER_COUNT, channels.other_count) != 0))
        || (channels.max_combined != 0
            && (nla_put_u32(skb, ETHTOOL_A_CHANNELS_COMBINED_MAX, channels.max_combined) != 0
                || nla_put_u32(skb, ETHTOOL_A_CHANNELS_COMBINED_COUNT, channels.combined_count) != 0))
    {
        return -EMSGSIZE;
    }
    0
}

/* CHANNELS_SET */
pub static ethnl_channels_set_policy: [nla_policy; ETHTOOL_A_CHANNELS_COMBINED_COUNT as usize + 1] =
    [NLA_POLICY_NESTED(ethnl_header_policy); ETHTOOL_A_CHANNELS_COMBINED_COUNT as usize + 1];

unsafe fn ethnl_set_channels_validate(req_info: *mut ethnl_req_info, _info: *mut genl_info) -> i32 {
    let ops = (*(*req_info).dev).ethtool_ops;
    if (*ops).get_channels.is_some() && (*ops).set_channels.is_some() { 1 } else { -EOPNOTSUPP }
}

unsafe fn ethnl_set_channels(req_info: *mut ethnl_req_info, info: *mut genl_info) -> i32 {
    let dev = (*req_info).dev;
    let mut old_combined: u32;
    let mut old_rx: u32;
    let mut old_tx: u32;
    let mut i: u32;
    let mut mod_ = false;
    let mut mod_combined = false;
    let mut channels: ethtool_channels = core::mem::zeroed();
    let tb = (*info).attrs;
    let mut err_attr: u32;
    let mut ret: i32;

    ((*(*dev).ethtool_ops).get_channels.unwrap())(dev, &mut channels);
    old_combined = channels.combined_count;
    old_rx = channels.rx_count;
    old_tx = channels.tx_count;
    ethnl_update_u32(&mut channels.rx_count, *tb.add(ETHTOOL_A_CHANNELS_RX_COUNT as usize), &mut mod_);
    ethnl_update_u32(&mut channels.tx_count, *tb.add(ETHTOOL_A_CHANNELS_TX_COUNT as usize), &mut mod_);
    ethnl_update_u32(&mut channels.other_count, *tb.add(ETHTOOL_A_CHANNELS_OTHER_COUNT as usize), &mut mod_);
    ethnl_update_u32(&mut channels.combined_count, *tb.add(ETHTOOL_A_CHANNELS_COMBINED_COUNT as usize), &mut mod_combined);
    mod_ |= mod_combined;
    if !mod_ { return 0; }
    if channels.rx_count > channels.max_rx { err_attr = ETHTOOL_A_CHANNELS_RX_COUNT; }
    else if channels.tx_count > channels.max_tx { err_attr = ETHTOOL_A_CHANNELS_TX_COUNT; }
    else if channels.other_count > channels.max_other { err_attr = ETHTOOL_A_CHANNELS_OTHER_COUNT; }
    else if channels.combined_count > channels.max_combined { err_attr = ETHTOOL_A_CHANNELS_COMBINED_COUNT; }
    else { err_attr = 0; }
    if err_attr != 0 {
        NL_SET_ERR_MSG_ATTR((*info).extack, *tb.add(err_attr as usize), "requested channel count exceeds maximum");
        return -EINVAL;
    }
    if channels.combined_count == 0 && channels.rx_count == 0 { err_attr = ETHTOOL_A_CHANNELS_RX_COUNT; }
    else if channels.combined_count == 0 && channels.tx_count == 0 { err_attr = ETHTOOL_A_CHANNELS_TX_COUNT; }
    else { err_attr = 0; }
    if err_attr != 0 {
        if mod_combined { err_attr = ETHTOOL_A_CHANNELS_COMBINED_COUNT; }
        NL_SET_ERR_MSG_ATTR((*info).extack, *tb.add(err_attr as usize), "requested channel counts would result in no RX or TX channel being configured");
        return -EINVAL;
    }
    ret = ethtool_check_max_channel(dev, channels, info);
    if ret != 0 { return ret; }
    i = channels.combined_count + channels.rx_count;
    while i < old_combined + old_rx { if netdev_queue_busy(dev, i, NETDEV_QUEUE_TYPE_RX, (*info).extack) { return -EINVAL; } i += 1; }
    i = channels.combined_count + channels.tx_count;
    while i < old_combined + old_tx { if netdev_queue_busy(dev, i, NETDEV_QUEUE_TYPE_TX, (*info).extack) { return -EINVAL; } i += 1; }
    ret = ((*(*dev).ethtool_ops).set_channels.unwrap())(dev, &mut channels);
    if ret < 0 { ret } else { 1 }
}

pub static ethnl_channels_request_ops: ethnl_request_ops = ethnl_request_ops {
    request_cmd: ETHTOOL_MSG_CHANNELS_GET,
    reply_cmd: ETHTOOL_MSG_CHANNELS_GET_REPLY,
    hdr_attr: ETHTOOL_A_CHANNELS_HEADER,
    req_info_size: core::mem::size_of::<channels_req_info>(),
    reply_data_size: core::mem::size_of::<channels_reply_data>(),
    prepare_data: Some(channels_prepare_data),
    reply_size: Some(channels_reply_size),
    fill_reply: Some(channels_fill_reply),
    set_validate: Some(ethnl_set_channels_validate),
    set: Some(ethnl_set_channels),
    set_ntf_cmd: ETHTOOL_MSG_CHANNELS_NTF,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
