// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding ethtool translation.

#[repr(C)]
pub struct linkmodes_req_info {
    pub base: ethnl_req_info,
}

#[repr(C)]
pub struct linkmodes_reply_data {
    pub base: ethnl_reply_data,
    pub ksettings: ethtool_link_ksettings,
    pub lsettings: *mut ethtool_link_settings,
    pub peer_empty: bool,
}

pub const ethnl_linkmodes_get_policy: [nla_policy; ETHTOOL_A_LINKMODES_HEADER as usize + 1] = [
    /* [ETHTOOL_A_LINKMODES_HEADER] = NLA_POLICY_NESTED(ethnl_header_policy) */
];

unsafe fn linkmodes_prepare_data(
    _req_base: *const ethnl_req_info,
    reply_base: *mut ethnl_reply_data,
    info: *const genl_info,
) -> i32 {
    let data = reply_base as *mut linkmodes_reply_data;
    let dev = (*reply_base).dev;
    (*data).lsettings = &mut (*data).ksettings.base;

    let mut ret = ethnl_ops_begin(dev);
    if ret < 0 {
        return ret;
    }

    ret = netif_get_link_ksettings(dev, &mut (*data).ksettings);
    if ret < 0 {
        GENL_SET_ERR_MSG(info, "failed to retrieve link settings");
    } else {
        if !(*dev).ethtool_ops.as_ref().unwrap().cap_link_lanes_supported {
            (*data).ksettings.lanes = 0;
        }
        (*data).peer_empty = bitmap_empty(
            (*data).ksettings.link_modes.lp_advertising,
            __ETHTOOL_LINK_MODE_MASK_NBITS,
        );
    }

    ethnl_ops_complete(dev);
    ret
}

unsafe fn linkmodes_reply_size(
    req_base: *const ethnl_req_info,
    reply_base: *const ethnl_reply_data,
) -> i32 {
    let data = reply_base as *const linkmodes_reply_data;
    let ksettings = &(*data).ksettings;
    let lsettings = &ksettings.base;
    let compact = (*req_base).flags & ETHTOOL_FLAG_COMPACT_BITSETS != 0;
    let mut len = nla_total_size(core::mem::size_of::<u8>() as i32)
        + nla_total_size(core::mem::size_of::<u32>() as i32)
        + nla_total_size(core::mem::size_of::<u32>() as i32)
        + nla_total_size(core::mem::size_of::<u8>() as i32)
        + nla_total_size(core::mem::size_of::<u8>() as i32);
    let mut ret = ethnl_bitset_size(
        ksettings.link_modes.advertising,
        ksettings.link_modes.supported,
        __ETHTOOL_LINK_MODE_MASK_NBITS,
        link_mode_names,
        compact,
    );
    if ret < 0 { return ret; }
    len += ret;
    if !(*data).peer_empty {
        ret = ethnl_bitset_size(ksettings.link_modes.lp_advertising, core::ptr::null(), __ETHTOOL_LINK_MODE_MASK_NBITS, link_mode_names, compact);
        if ret < 0 { return ret; }
        len += ret;
    }
    if lsettings.master_slave_cfg != MASTER_SLAVE_CFG_UNSUPPORTED { len += nla_total_size(core::mem::size_of::<u8>() as i32); }
    if lsettings.master_slave_state != MASTER_SLAVE_STATE_UNSUPPORTED { len += nla_total_size(core::mem::size_of::<u8>() as i32); }
    len
}

unsafe fn linkmodes_fill_reply(
    skb: *mut sk_buff,
    req_base: *const ethnl_req_info,
    reply_base: *const ethnl_reply_data,
) -> i32 {
    let data = reply_base as *const linkmodes_reply_data;
    let k = &(*data).ksettings;
    let l = &k.base;
    let compact = (*req_base).flags & ETHTOOL_FLAG_COMPACT_BITSETS != 0;
    if nla_put_u8(skb, ETHTOOL_A_LINKMODES_AUTONEG, l.autoneg) != 0 { return -EMSGSIZE; }
    let mut ret = ethnl_put_bitset(skb, ETHTOOL_A_LINKMODES_OURS, k.link_modes.advertising, k.link_modes.supported, __ETHTOOL_LINK_MODE_MASK_NBITS, link_mode_names, compact);
    if ret < 0 { return -EMSGSIZE; }
    if !(*data).peer_empty {
        ret = ethnl_put_bitset(skb, ETHTOOL_A_LINKMODES_PEER, k.link_modes.lp_advertising, core::ptr::null(), __ETHTOOL_LINK_MODE_MASK_NBITS, link_mode_names, compact);
        if ret < 0 { return -EMSGSIZE; }
    }
    if nla_put_u32(skb, ETHTOOL_A_LINKMODES_SPEED, l.speed) != 0 || nla_put_u8(skb, ETHTOOL_A_LINKMODES_DUPLEX, l.duplex) != 0 { return -EMSGSIZE; }
    if k.lanes != 0 && nla_put_u32(skb, ETHTOOL_A_LINKMODES_LANES, k.lanes) != 0 { return -EMSGSIZE; }
    if l.master_slave_cfg != MASTER_SLAVE_CFG_UNSUPPORTED && nla_put_u8(skb, ETHTOOL_A_LINKMODES_MASTER_SLAVE_CFG, l.master_slave_cfg) != 0 { return -EMSGSIZE; }
    if l.master_slave_state != MASTER_SLAVE_STATE_UNSUPPORTED && nla_put_u8(skb, ETHTOOL_A_LINKMODES_MASTER_SLAVE_STATE, l.master_slave_state) != 0 { return -EMSGSIZE; }
    if nla_put_u8(skb, ETHTOOL_A_LINKMODES_RATE_MATCHING, l.rate_matching) != 0 { return -EMSGSIZE; }
    0
}

pub const ethnl_linkmodes_set_policy: [nla_policy; ETHTOOL_A_LINKMODES_LANES as usize + 1] = [
    /* Header: NLA_POLICY_NESTED(ethnl_header_policy), AUTONEG/OURS/SPEED/DUPLEX/MASTER_SLAVE_CFG/LANES as declared in C. */
];

/* Set advertised link modes to all supported modes matching requested speed,
 * lanes and duplex values. Called when autonegotiation is on, speed, lanes or
 * duplex is requested but no link mode change. This is done in userspace with
 * ioctl() interface, move it into kernel for netlink.
 * Returns true if advertised modes bitmap was modified.
 */
unsafe fn ethnl_auto_linkmodes(k: *mut ethtool_link_ksettings, req_speed: bool, req_lanes: bool, req_duplex: bool) -> bool {
    let advertising = (*k).link_modes.advertising;
    let supported = (*k).link_modes.supported;
    let mut old_adv = [0usize; __ETHTOOL_LINK_MODE_MASK_NBITS as usize];
    bitmap_copy(old_adv.as_mut_ptr(), advertising, __ETHTOOL_LINK_MODE_MASK_NBITS);
    for i in 0..__ETHTOOL_LINK_MODE_MASK_NBITS {
        let info = &link_mode_params[i as usize];
        if info.speed == SPEED_UNKNOWN { continue; }
        if test_bit(i, supported) && (!req_speed || info.speed == (*k).base.speed) && (!req_lanes || info.lanes == (*k).lanes) && (!req_duplex || info.duplex == (*k).base.duplex) { set_bit(i, advertising); } else { clear_bit(i, advertising); }
    }
    !bitmap_equal(old_adv.as_ptr(), advertising, __ETHTOOL_LINK_MODE_MASK_NBITS)
}

unsafe fn ethnl_validate_master_slave_cfg(cfg: u8) -> bool {
    matches!(cfg, MASTER_SLAVE_CFG_MASTER_PREFERRED | MASTER_SLAVE_CFG_SLAVE_PREFERRED | MASTER_SLAVE_CFG_MASTER_FORCE | MASTER_SLAVE_CFG_SLAVE_FORCE)
}

unsafe fn ethnl_check_linkmodes(info: *mut genl_info, tb: *mut *mut nlattr) -> i32 {
    let master_slave_cfg = *tb.add(ETHTOOL_A_LINKMODES_MASTER_SLAVE_CFG as usize);
    if !master_slave_cfg.is_null() && !ethnl_validate_master_slave_cfg(nla_get_u8(master_slave_cfg)) {
        NL_SET_ERR_MSG_ATTR((*info).extack, master_slave_cfg, "master/slave value is invalid");
        return -EOPNOTSUPP;
    }
    let lanes_cfg = *tb.add(ETHTOOL_A_LINKMODES_LANES as usize);
    if !lanes_cfg.is_null() && !is_power_of_2(nla_get_u32(lanes_cfg)) {
        NL_SET_ERR_MSG_ATTR((*info).extack, lanes_cfg, "lanes value is invalid");
        return -EINVAL;
    }
    0
}

unsafe fn ethnl_update_linkmodes(info: *mut genl_info, tb: *mut *mut nlattr, k: *mut ethtool_link_ksettings, modified: *mut bool, dev: *const net_device) -> i32 {
    let l = &mut (*k).base;
    let master_slave_cfg = *tb.add(ETHTOOL_A_LINKMODES_MASTER_SLAVE_CFG as usize);
    if !master_slave_cfg.is_null() && l.master_slave_cfg == MASTER_SLAVE_CFG_UNSUPPORTED {
        NL_SET_ERR_MSG_ATTR((*info).extack, master_slave_cfg, "master/slave configuration not supported by device");
        return -EOPNOTSUPP;
    }
    *modified = false;
    let req_speed = !(*tb.add(ETHTOOL_A_LINKMODES_SPEED as usize)).is_null();
    let req_lanes = !(*tb.add(ETHTOOL_A_LINKMODES_LANES as usize)).is_null();
    let req_duplex = !(*tb.add(ETHTOOL_A_LINKMODES_DUPLEX as usize)).is_null();
    ethnl_update_u8(&mut l.autoneg, *tb.add(ETHTOOL_A_LINKMODES_AUTONEG as usize), modified);
    let lanes_cfg = *tb.add(ETHTOOL_A_LINKMODES_LANES as usize);
    if !lanes_cfg.is_null() {
        if l.autoneg == 0 && !(*(*dev).ethtool_ops).cap_link_lanes_supported {
            NL_SET_ERR_MSG_ATTR((*info).extack, lanes_cfg, "lanes configuration not supported by device");
            return -EOPNOTSUPP;
        }
    } else if l.autoneg == 0 && (*k).lanes != 0 {
        (*k).lanes = 0;
        *modified = true;
    }
    let ret = ethnl_update_bitset((*k).link_modes.advertising, __ETHTOOL_LINK_MODE_MASK_NBITS, *tb.add(ETHTOOL_A_LINKMODES_OURS as usize), link_mode_names, (*info).extack, modified);
    if ret < 0 { return ret; }
    ethnl_update_u32(&mut l.speed, *tb.add(ETHTOOL_A_LINKMODES_SPEED as usize), modified);
    ethnl_update_u32(&mut (*k).lanes, lanes_cfg, modified);
    ethnl_update_u8(&mut l.duplex, *tb.add(ETHTOOL_A_LINKMODES_DUPLEX as usize), modified);
    ethnl_update_u8(&mut l.master_slave_cfg, master_slave_cfg, modified);
    if (*tb.add(ETHTOOL_A_LINKMODES_OURS as usize)).is_null() && l.autoneg != 0 && (req_speed || req_lanes || req_duplex) && ethnl_auto_linkmodes(k, req_speed, req_lanes, req_duplex) { *modified = true; }
    0
}

unsafe fn ethnl_set_linkmodes_validate(req_info: *mut ethnl_req_info, info: *mut genl_info) -> i32 {
    let ops = (*req_info).dev.ethtool_ops;
    let ret = ethnl_check_linkmodes(info, (*info).attrs);
    if ret < 0 { return ret; }
    if (*ops).get_link_ksettings.is_none() || (*ops).set_link_ksettings.is_none() { return -EOPNOTSUPP; }
    1
}

unsafe fn ethnl_set_linkmodes(req_info: *mut ethnl_req_info, info: *mut genl_info) -> i32 {
    let mut ksettings: ethtool_link_ksettings = core::mem::zeroed();
    let dev = (*req_info).dev;
    let mut modified = false;
    let mut ret = netif_get_link_ksettings(dev, &mut ksettings);
    if ret < 0 { GENL_SET_ERR_MSG(info, "failed to retrieve link settings"); return ret; }
    ret = ethnl_update_linkmodes(info, (*info).attrs, &mut ksettings, &mut modified, dev);
    if ret < 0 { return ret; }
    if !modified { return 0; }
    ret = ((*(*dev).ethtool_ops).set_link_ksettings.unwrap())(dev, &mut ksettings);
    if ret < 0 { GENL_SET_ERR_MSG(info, "link settings update failed"); return ret; }
    1
}

pub static ethnl_linkmodes_request_ops: ethnl_request_ops = ethnl_request_ops {
    request_cmd: ETHTOOL_MSG_LINKMODES_GET,
    reply_cmd: ETHTOOL_MSG_LINKMODES_GET_REPLY,
    hdr_attr: ETHTOOL_A_LINKMODES_HEADER,
    req_info_size: core::mem::size_of::<linkmodes_req_info>(),
    reply_data_size: core::mem::size_of::<linkmodes_reply_data>(),
    prepare_data: Some(linkmodes_prepare_data),
    reply_size: Some(linkmodes_reply_size),
    fill_reply: Some(linkmodes_fill_reply),
    set_validate: Some(ethnl_set_linkmodes_validate),
    set: Some(ethnl_set_linkmodes),
    set_ntf_cmd: ETHTOOL_MSG_LINKMODES_NTF,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
