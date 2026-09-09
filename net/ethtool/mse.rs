// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel/Rust bindings.

/* Channels A-D only; WORST and LINK are exclusive alternatives */
const PHY_MSE_CHANNEL_COUNT: usize = 4;

#[repr(C)]
struct MseReqInfo {
    base: EthnlReqInfo,
}

#[repr(C)]
struct MseSnapshotEntry {
    snapshot: PhyMseSnapshot,
    channel: i32,
}

#[repr(C)]
struct MseReplyData {
    base: EthnlReplyData,
    capability: PhyMseCapability,
    snapshots: *mut MseSnapshotEntry,
    num_snapshots: u32,
}

unsafe fn mse_repdata(reply_base: *const EthnlReplyData) -> *mut MseReplyData {
    reply_base as *mut MseReplyData
}

#[no_mangle]
static ethnl_mse_get_policy: [NlaPolicy; ETHTOOL_A_MSE_HEADER as usize + 1] = [
    /* ETHTOOL_A_MSE_HEADER = NLA_POLICY_NESTED(ethnl_header_policy_phy) */
    NlaPolicy::default(),
];

unsafe fn get_snapshot_if_supported(
    phydev: *mut PhyDevice,
    data: *mut MseReplyData,
    idx: *mut u32,
    cap_bit: u32,
    channel: i32,
) -> i32 {
    let mut ret: i32;

    if (*data).capability.supported_caps & cap_bit != 0 {
        ret = (*(*phydev).drv).get_mse_snapshot(
            phydev,
            channel,
            &mut (*(*data).snapshots.add(*idx as usize)).snapshot,
        );
        if ret != 0 {
            return ret;
        }
        (*(*data).snapshots.add(*idx as usize)).channel = channel;
        *idx += 1;
    }

    0
}

unsafe fn mse_get_channels(phydev: *mut PhyDevice, data: *mut MseReplyData) -> i32 {
    let mut i: u32 = 0;
    let mut ret: i32;

    if (*data).capability.supported_caps == 0 {
        return 0;
    }

    (*data).snapshots = kzalloc_objs((*data).snapshots, PHY_MSE_CHANNEL_COUNT);
    if (*data).snapshots.is_null() {
        return -ENOMEM;
    }

    /* Priority 1: Individual channels */
    ret = get_snapshot_if_supported(phydev, data, &mut i, PHY_MSE_CAP_CHANNEL_A, PHY_MSE_CHANNEL_A);
    if ret != 0 { return ret; }
    ret = get_snapshot_if_supported(phydev, data, &mut i, PHY_MSE_CAP_CHANNEL_B, PHY_MSE_CHANNEL_B);
    if ret != 0 { return ret; }
    ret = get_snapshot_if_supported(phydev, data, &mut i, PHY_MSE_CAP_CHANNEL_C, PHY_MSE_CHANNEL_C);
    if ret != 0 { return ret; }
    ret = get_snapshot_if_supported(phydev, data, &mut i, PHY_MSE_CAP_CHANNEL_D, PHY_MSE_CHANNEL_D);
    if ret != 0 { return ret; }

    /* If any individual channels were found, we are done. */
    if i > 0 {
        (*data).num_snapshots = i;
        return 0;
    }

    /* Priority 2: Worst channel, if no individual channels supported. */
    ret = get_snapshot_if_supported(phydev, data, &mut i, PHY_MSE_CAP_WORST_CHANNEL, PHY_MSE_CHANNEL_WORST);
    if ret != 0 { return ret; }

    /* If worst channel was found, we are done. */
    if i > 0 {
        (*data).num_snapshots = i;
        return 0;
    }

    /* Priority 3: Link-wide, if nothing else is supported. */
    ret = get_snapshot_if_supported(phydev, data, &mut i, PHY_MSE_CAP_LINK, PHY_MSE_CHANNEL_LINK);
    if ret != 0 { return ret; }

    (*data).num_snapshots = i;
    0
}

unsafe fn mse_prepare_data(
    req_base: *const EthnlReqInfo,
    reply_base: *mut EthnlReplyData,
    info: *const GenlInfo,
) -> i32 {
    let data = mse_repdata(reply_base);
    let dev = (*reply_base).dev;
    let phydev;
    let mut ret: i32;

    phydev = ethnl_req_get_phydev(req_base, (*info).attrs, ETHTOOL_A_MSE_HEADER, (*info).extack);
    if IS_ERR(phydev) { return PTR_ERR(phydev); }
    if phydev.is_null() { return -EOPNOTSUPP; }

    ret = ethnl_ops_begin(dev);
    if ret != 0 { return ret; }

    mutex_lock(&mut (*phydev).lock);

    if (*phydev).drv.is_null()
        || (*(*phydev).drv).get_mse_capability.is_none()
        || (*(*phydev).drv).get_mse_snapshot.is_none()
    {
        ret = -EOPNOTSUPP;
        goto_out_unlock!(ret);
    }
    if !(*phydev).link {
        ret = -ENETDOWN;
        goto_out_unlock!(ret);
    }

    ret = (*(*phydev).drv).get_mse_capability(phydev, &mut (*data).capability);
    if ret != 0 { goto_out_unlock!(ret); }

    ret = mse_get_channels(phydev, data);

    mutex_unlock(&mut (*phydev).lock);
    ethnl_ops_complete(dev);
    if ret != 0 { kfree((*data).snapshots); }
    ret
}

unsafe fn mse_cleanup_data(reply_base: *mut EthnlReplyData) {
    let data = mse_repdata(reply_base);
    kfree((*data).snapshots);
}

unsafe fn mse_reply_size(_req_base: *const EthnlReqInfo, reply_base: *const EthnlReplyData) -> usize {
    let data = mse_repdata(reply_base);
    let mut len: usize = 0;

    /* ETHTOOL_A_MSE_CAPABILITIES */
    len += nla_total_size(0);
    if (*data).capability.supported_caps & PHY_MSE_CAP_AVG != 0 {
        /* ETHTOOL_A_MSE_CAPABILITIES_MAX_AVERAGE_MSE */
        len += nla_total_size(core::mem::size_of::<u64>());
    }
    if (*data).capability.supported_caps & (PHY_MSE_CAP_PEAK | PHY_MSE_CAP_WORST_PEAK) != 0 {
        /* ETHTOOL_A_MSE_CAPABILITIES_MAX_PEAK_MSE */
        len += nla_total_size(core::mem::size_of::<u64>());
    }
    /* ETHTOOL_A_MSE_CAPABILITIES_REFRESH_RATE_PS */
    len += nla_total_size(core::mem::size_of::<u64>());
    /* ETHTOOL_A_MSE_CAPABILITIES_NUM_SYMBOLS */
    len += nla_total_size(core::mem::size_of::<u64>());

    for i in 0..(*data).num_snapshots {
        let mut snapshot_len: usize = 0;
        /* Per-channel nest (e.g., ETHTOOL_A_MSE_CHANNEL_A / _B / _C /
         * _D / _WORST_CHANNEL / _LINK)
         */
        snapshot_len += nla_total_size(0);
        if (*data).capability.supported_caps & PHY_MSE_CAP_AVG != 0 { snapshot_len += nla_total_size(core::mem::size_of::<u64>()); }
        if (*data).capability.supported_caps & PHY_MSE_CAP_PEAK != 0 { snapshot_len += nla_total_size(core::mem::size_of::<u64>()); }
        if (*data).capability.supported_caps & PHY_MSE_CAP_WORST_PEAK != 0 { snapshot_len += nla_total_size(core::mem::size_of::<u64>()); }
        let _ = i;
        len += snapshot_len;
    }
    len
}

unsafe fn mse_channel_to_attr(ch: i32) -> i32 {
    match ch {
        PHY_MSE_CHANNEL_A => ETHTOOL_A_MSE_CHANNEL_A,
        PHY_MSE_CHANNEL_B => ETHTOOL_A_MSE_CHANNEL_B,
        PHY_MSE_CHANNEL_C => ETHTOOL_A_MSE_CHANNEL_C,
        PHY_MSE_CHANNEL_D => ETHTOOL_A_MSE_CHANNEL_D,
        PHY_MSE_CHANNEL_WORST => ETHTOOL_A_MSE_WORST_CHANNEL,
        PHY_MSE_CHANNEL_LINK => ETHTOOL_A_MSE_LINK,
        _ => -EINVAL,
    }
}

// The remaining netlink serialization is kept as a direct unsafe translation;
// all referenced types and helpers are supplied by the surrounding bindings.
unsafe fn mse_fill_reply(skb: *mut SkBuff, _req_base: *const EthnlReqInfo, reply_base: *const EthnlReplyData) -> i32 {
    let data = mse_repdata(reply_base);
    let mut nest = nla_nest_start(skb, ETHTOOL_A_MSE_CAPABILITIES);
    if nest.is_null() { return -EMSGSIZE; }
    if (*data).capability.supported_caps & PHY_MSE_CAP_AVG != 0 && nla_put_uint(skb, ETHTOOL_A_MSE_CAPABILITIES_MAX_AVERAGE_MSE, (*data).capability.max_average_mse) < 0 { nla_nest_cancel(skb, nest); return -EMSGSIZE; }
    if (*data).capability.supported_caps & (PHY_MSE_CAP_PEAK | PHY_MSE_CAP_WORST_PEAK) != 0 && nla_put_uint(skb, ETHTOOL_A_MSE_CAPABILITIES_MAX_PEAK_MSE, (*data).capability.max_peak_mse) < 0 { nla_nest_cancel(skb, nest); return -EMSGSIZE; }
    if nla_put_uint(skb, ETHTOOL_A_MSE_CAPABILITIES_REFRESH_RATE_PS, (*data).capability.refresh_rate_ps) < 0 { nla_nest_cancel(skb, nest); return -EMSGSIZE; }
    if nla_put_uint(skb, ETHTOOL_A_MSE_CAPABILITIES_NUM_SYMBOLS, (*data).capability.num_symbols) < 0 { nla_nest_cancel(skb, nest); return -EMSGSIZE; }
    nla_nest_end(skb, nest);
    for i in 0..(*data).num_snapshots {
        let s = &*(*data).snapshots.add(i as usize);
        let chan_attr = mse_channel_to_attr(s.channel);
        if chan_attr < 0 { return chan_attr; }
        nest = nla_nest_start(skb, chan_attr);
        if nest.is_null() { return -EMSGSIZE; }
        if (*data).capability.supported_caps & PHY_MSE_CAP_AVG != 0 && nla_put_uint(skb, ETHTOOL_A_MSE_SNAPSHOT_AVERAGE_MSE, s.snapshot.average_mse) != 0 { nla_nest_cancel(skb, nest); return -EMSGSIZE; }
        if (*data).capability.supported_caps & PHY_MSE_CAP_PEAK != 0 && nla_put_uint(skb, ETHTOOL_A_MSE_SNAPSHOT_PEAK_MSE, s.snapshot.peak_mse) != 0 { nla_nest_cancel(skb, nest); return -EMSGSIZE; }
        if (*data).capability.supported_caps & PHY_MSE_CAP_WORST_PEAK != 0 && nla_put_uint(skb, ETHTOOL_A_MSE_SNAPSHOT_WORST_PEAK_MSE, s.snapshot.worst_peak_mse) != 0 { nla_nest_cancel(skb, nest); return -EMSGSIZE; }
        nla_nest_end(skb, nest);
    }
    0
}

#[no_mangle]
static ethnl_mse_request_ops: EthnlRequestOps = EthnlRequestOps {
    request_cmd: ETHTOOL_MSG_MSE_GET,
    reply_cmd: ETHTOOL_MSG_MSE_GET_REPLY,
    hdr_attr: ETHTOOL_A_MSE_HEADER,
    req_info_size: core::mem::size_of::<MseReqInfo>(),
    reply_data_size: core::mem::size_of::<MseReplyData>(),
    prepare_data: Some(mse_prepare_data),
    cleanup_data: Some(mse_cleanup_data),
    reply_size: Some(mse_reply_size),
    fill_reply: Some(mse_fill_reply),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
