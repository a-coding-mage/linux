// SPDX-License-Identifier: GPL-2.0-only

// C dependencies supplied by the surrounding translation unit:
// bitset.h, common.h, netlink.h

#[repr(C)]
pub struct eee_req_info {
    pub base: ethnl_req_info,
}

#[repr(C)]
pub struct eee_reply_data {
    pub base: ethnl_reply_data,
    pub eee: ethtool_keee,
}

#[inline]
unsafe fn eee_repdata(reply_base: *const ethnl_reply_data) -> *const eee_reply_data {
    reply_base as *const eee_reply_data
}

pub static ethnl_eee_get_policy: [nla_policy; ETHTOOL_A_EEE_TX_LPI_TIMER as usize + 1] = {
    let mut policy: [nla_policy; ETHTOOL_A_EEE_TX_LPI_TIMER as usize + 1] =
        [nla_policy { type_: 0 }; ETHTOOL_A_EEE_TX_LPI_TIMER as usize + 1];
    policy[ETHTOOL_A_EEE_HEADER as usize] = NLA_POLICY_NESTED(ethnl_header_policy);
    policy
};

unsafe fn eee_prepare_data(
    _req_base: *const ethnl_req_info,
    reply_base: *mut ethnl_reply_data,
    _info: *const genl_info,
) -> i32 {
    let data = eee_repdata(reply_base);
    let dev = (*reply_base).dev;
    let eee = &mut (*(data as *mut eee_reply_data)).eee;
    let ret: i32;

    if (*(*dev).ethtool_ops).get_eee.is_none() {
        return -EOPNOTSUPP;
    }
    ret = ethnl_ops_begin(dev);
    if ret < 0 {
        return ret;
    }
    ret = ((*(*dev).ethtool_ops).get_eee.unwrap())(dev, eee);
    ethnl_ops_complete(dev);

    ret
}

unsafe fn eee_reply_size(
    req_base: *const ethnl_req_info,
    reply_base: *const ethnl_reply_data,
) -> i32 {
    let compact = ((*req_base).flags & ETHTOOL_FLAG_COMPACT_BITSETS) != 0;
    let data = eee_repdata(reply_base);
    let eee = &(*data).eee;
    let mut len: i32 = 0;
    let mut ret: i32;

    // MODES_OURS
    ret = ethnl_bitset_size(
        eee.advertised,
        eee.supported,
        __ETHTOOL_LINK_MODE_MASK_NBITS,
        link_mode_names,
        compact,
    );
    if ret < 0 {
        return ret;
    }
    len += ret;
    // MODES_PEERS
    ret = ethnl_bitset_size(
        eee.lp_advertised,
        core::ptr::null(),
        __ETHTOOL_LINK_MODE_MASK_NBITS,
        link_mode_names,
        compact,
    );
    if ret < 0 {
        return ret;
    }
    len += ret;

    len += nla_total_size(core::mem::size_of::<u8>() as i32)
        + nla_total_size(core::mem::size_of::<u8>() as i32)
        + nla_total_size(core::mem::size_of::<u8>() as i32)
        + nla_total_size(core::mem::size_of::<u32>() as i32);

    len
}

unsafe fn eee_fill_reply(
    skb: *mut sk_buff,
    req_base: *const ethnl_req_info,
    reply_base: *const ethnl_reply_data,
) -> i32 {
    let compact = ((*req_base).flags & ETHTOOL_FLAG_COMPACT_BITSETS) != 0;
    let data = eee_repdata(reply_base);
    let eee = &(*data).eee;
    let mut ret: i32;

    ret = ethnl_put_bitset(
        skb,
        ETHTOOL_A_EEE_MODES_OURS,
        eee.advertised,
        eee.supported,
        __ETHTOOL_LINK_MODE_MASK_NBITS,
        link_mode_names,
        compact,
    );
    if ret < 0 {
        return ret;
    }
    ret = ethnl_put_bitset(
        skb,
        ETHTOOL_A_EEE_MODES_PEER,
        eee.lp_advertised,
        core::ptr::null(),
        __ETHTOOL_LINK_MODE_MASK_NBITS,
        link_mode_names,
        compact,
    );
    if ret < 0 {
        return ret;
    }

    if nla_put_u8(skb, ETHTOOL_A_EEE_ACTIVE, eee.eee_active) != 0
        || nla_put_u8(skb, ETHTOOL_A_EEE_ENABLED, eee.eee_enabled) != 0
        || nla_put_u8(skb, ETHTOOL_A_EEE_TX_LPI_ENABLED, eee.tx_lpi_enabled) != 0
        || nla_put_u32(skb, ETHTOOL_A_EEE_TX_LPI_TIMER, eee.tx_lpi_timer) != 0
    {
        return -EMSGSIZE;
    }

    0
}

// EEE_SET
pub static ethnl_eee_set_policy: [nla_policy; ETHTOOL_A_EEE_TX_LPI_TIMER as usize + 1] = {
    let mut policy: [nla_policy; ETHTOOL_A_EEE_TX_LPI_TIMER as usize + 1] =
        [nla_policy { type_: 0 }; ETHTOOL_A_EEE_TX_LPI_TIMER as usize + 1];
    policy[ETHTOOL_A_EEE_HEADER as usize] = NLA_POLICY_NESTED(ethnl_header_policy);
    policy[ETHTOOL_A_EEE_MODES_OURS as usize] = nla_policy { type_: NLA_NESTED };
    policy[ETHTOOL_A_EEE_ENABLED as usize] = nla_policy { type_: NLA_U8 };
    policy[ETHTOOL_A_EEE_TX_LPI_ENABLED as usize] = nla_policy { type_: NLA_U8 };
    policy[ETHTOOL_A_EEE_TX_LPI_TIMER as usize] = nla_policy { type_: NLA_U32 };
    policy
};

unsafe fn ethnl_set_eee_validate(req_info: *mut ethnl_req_info, _info: *mut genl_info) -> i32 {
    let ops = (*(*req_info).dev).ethtool_ops;
    if (*ops).get_eee.is_some() && (*ops).set_eee.is_some() { 1 } else { -EOPNOTSUPP }
}

unsafe fn ethnl_set_eee(req_info: *mut ethnl_req_info, info: *mut genl_info) -> i32 {
    let dev = (*req_info).dev;
    let tb = (*info).attrs;
    let mut eee: ethtool_keee = core::mem::zeroed();
    let mut modified = false;
    let mut ret = ((*(*dev).ethtool_ops).get_eee.unwrap())(dev, &mut eee);
    if ret < 0 { return ret; }
    ret = ethnl_update_bitset(eee.advertised, __ETHTOOL_LINK_MODE_MASK_NBITS,
        *tb.add(ETHTOOL_A_EEE_MODES_OURS as usize), link_mode_names,
        (*info).extack, &mut modified);
    if ret < 0 { return ret; }
    ethnl_update_bool(&mut eee.eee_enabled, *tb.add(ETHTOOL_A_EEE_ENABLED as usize), &mut modified);
    ethnl_update_bool(&mut eee.tx_lpi_enabled, *tb.add(ETHTOOL_A_EEE_TX_LPI_ENABLED as usize), &mut modified);
    ethnl_update_u32(&mut eee.tx_lpi_timer, *tb.add(ETHTOOL_A_EEE_TX_LPI_TIMER as usize), &mut modified);
    if !modified { return 0; }
    ret = ((*(*dev).ethtool_ops).set_eee.unwrap())(dev, &mut eee);
    if ret < 0 { ret } else { 1 }
}

pub static ethnl_eee_request_ops: ethnl_request_ops = ethnl_request_ops {
    request_cmd: ETHTOOL_MSG_EEE_GET,
    reply_cmd: ETHTOOL_MSG_EEE_GET_REPLY,
    hdr_attr: ETHTOOL_A_EEE_HEADER,
    req_info_size: core::mem::size_of::<eee_req_info>(),
    reply_data_size: core::mem::size_of::<eee_reply_data>(),
    prepare_data: Some(eee_prepare_data),
    reply_size: Some(eee_reply_size),
    fill_reply: Some(eee_fill_reply),
    set_validate: Some(ethnl_set_eee_validate),
    set: Some(ethnl_set_eee),
    set_ntf_cmd: ETHTOOL_MSG_EEE_NTF,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
