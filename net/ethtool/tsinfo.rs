// SPDX-License-Identifier: GPL-2.0-only

// External kernel and ethtool declarations are supplied by the surrounding translation unit.

#[repr(C)]
pub struct tsinfo_req_info {
    pub base: ethnl_req_info,
    pub hwprov_desc: hwtstamp_provider_desc,
}

#[repr(C)]
pub struct tsinfo_reply_data {
    pub base: ethnl_reply_data,
    pub ts_info: kernel_ethtool_ts_info,
    pub stats: ethtool_ts_stats,
}

pub const ETHTOOL_TS_STAT_CNT: usize =
    (__ETHTOOL_A_TS_STAT_CNT - (ETHTOOL_A_TS_STAT_UNSPEC + 1)) as usize;

pub static ethnl_tsinfo_get_policy: [nla_policy; (ETHTOOL_A_TSINFO_MAX + 1) as usize] = [
    nla_policy::default(); (ETHTOOL_A_TSINFO_MAX + 1) as usize
];

pub unsafe fn ts_parse_hwtst_provider(
    nest: *const nlattr,
    hwprov_desc: *mut hwtstamp_provider_desc,
    extack: *mut netlink_ext_ack,
    mod_: *mut bool,
) -> c_int {
    let mut tb: [*mut nlattr; ARRAY_SIZE_ethnl_ts_hwtst_prov_policy] = [core::ptr::null_mut(); ARRAY_SIZE_ethnl_ts_hwtst_prov_policy];
    let ret = nla_parse_nested(tb.as_mut_ptr(), ARRAY_SIZE_ethnl_ts_hwtst_prov_policy - 1, nest,
                               ethnl_ts_hwtst_prov_policy.as_ptr(), extack);
    if ret < 0 { return ret; }
    if NL_REQ_ATTR_CHECK(extack, nest, tb.as_mut_ptr(), ETHTOOL_A_TS_HWTSTAMP_PROVIDER_INDEX) != 0
        || NL_REQ_ATTR_CHECK(extack, nest, tb.as_mut_ptr(), ETHTOOL_A_TS_HWTSTAMP_PROVIDER_QUALIFIER) != 0 {
        return -EINVAL;
    }
    ethnl_update_u32(&mut (*hwprov_desc).index, tb[ETHTOOL_A_TS_HWTSTAMP_PROVIDER_INDEX as usize], mod_);
    ethnl_update_u32(&mut (*hwprov_desc).qualifier, tb[ETHTOOL_A_TS_HWTSTAMP_PROVIDER_QUALIFIER as usize], mod_);
    0
}

unsafe fn tsinfo_parse_request(req_base: *mut ethnl_req_info, _info: *const genl_info,
                               tb: *mut *mut nlattr, extack: *mut netlink_ext_ack) -> c_int {
    let req = req_base as *mut tsinfo_req_info;
    let mut mod_ = false;
    (*req).hwprov_desc.index = -1;
    if (*tb.add(ETHTOOL_A_TSINFO_HWTSTAMP_PROVIDER as usize)).is_null() { return 0; }
    if (*req_base).flags & ETHTOOL_FLAG_STATS != 0 {
        NL_SET_ERR_MSG(extack, "can't query statistics for a provider");
        return -EOPNOTSUPP;
    }
    ts_parse_hwtst_provider(*tb.add(ETHTOOL_A_TSINFO_HWTSTAMP_PROVIDER as usize),
                            &mut (*req).hwprov_desc, extack, &mut mod_)
}

unsafe fn tsinfo_prepare_data(req_base: *const ethnl_req_info, reply_base: *mut ethnl_reply_data,
                              _info: *const genl_info) -> c_int {
    let data = reply_base as *mut tsinfo_reply_data;
    let req = req_base as *mut tsinfo_req_info;
    let dev = (*reply_base).dev;
    let mut ret = ethnl_ops_begin(dev);
    if ret < 0 { return ret; }
    if (*req).hwprov_desc.index != -1 {
        ret = ethtool_get_ts_info_by_phc(dev, &mut (*data).ts_info, &(*req).hwprov_desc);
        ethnl_ops_complete(dev); return ret;
    }
    if (*req_base).flags & ETHTOOL_FLAG_STATS != 0 {
        ethtool_stats_init(&mut (*data).stats as *mut _ as *mut u64,
                           core::mem::size_of::<ethtool_ts_stats>() / core::mem::size_of::<u64>());
        if !(*(*dev).ethtool_ops).get_ts_stats.is_none() {
            ((*(*dev).ethtool_ops).get_ts_stats.unwrap())(dev, &mut (*data).stats);
        }
    }
    ret = __ethtool_get_ts_info(dev, &mut (*data).ts_info);
    ethnl_ops_complete(dev); ret
}

unsafe fn tsinfo_reply_size(req_base: *const ethnl_req_info, reply_base: *const ethnl_reply_data) -> c_int {
    let data = reply_base as *const tsinfo_reply_data;
    let compact = (*req_base).flags & ETHTOOL_FLAG_COMPACT_BITSETS != 0;
    let info = &(*data).ts_info;
    let mut len = 0;
    if info.so_timestamping != 0 { len += ethnl_bitset32_size(&info.so_timestamping, core::ptr::null(), __SOF_TIMESTAMPING_CNT, sof_timestamping_names, compact); }
    if info.tx_types != 0 { len += ethnl_bitset32_size(&info.tx_types, core::ptr::null(), __HWTSTAMP_TX_CNT, ts_tx_type_names, compact); }
    if info.rx_filters != 0 { len += ethnl_bitset32_size(&info.rx_filters, core::ptr::null(), __HWTSTAMP_FILTER_CNT, ts_rx_filter_names, compact); }
    if info.phc_index >= 0 { len += nla_total_size(core::mem::size_of::<u32>() as c_int); len += nla_total_size(0) + 2 * nla_total_size(core::mem::size_of::<u32>() as c_int); }
    if info.phc_source != 0 { len += nla_total_size(core::mem::size_of::<u32>() as c_int); if info.phc_phyindex != 0 { len += nla_total_size(core::mem::size_of::<u32>() as c_int); } }
    if (*req_base).flags & ETHTOOL_FLAG_STATS != 0 { len += nla_total_size(0) + nla_total_size_64bit(core::mem::size_of::<u64>() as c_int) * ETHTOOL_TS_STAT_CNT as c_int; }
    len
}

unsafe fn tsinfo_put_stat(skb: *mut sk_buff, val: u64, attrtype: u16) -> c_int {
    if val == ETHTOOL_STAT_NOT_SET { return 0; }
    if nla_put_uint(skb, attrtype, val) != 0 { return -EMSGSIZE; }
    0
}

unsafe fn tsinfo_put_stats(skb: *mut sk_buff, stats: *const ethtool_ts_stats) -> c_int {
    let nest = nla_nest_start(skb, ETHTOOL_A_TSINFO_STATS);
    if nest.is_null() { return -EMSGSIZE; }
    if tsinfo_put_stat(skb, (*stats).tx_stats.pkts, ETHTOOL_A_TS_STAT_TX_PKTS) != 0
        || tsinfo_put_stat(skb, (*stats).tx_stats.onestep_pkts_unconfirmed, ETHTOOL_A_TS_STAT_TX_ONESTEP_PKTS_UNCONFIRMED) != 0
        || tsinfo_put_stat(skb, (*stats).tx_stats.lost, ETHTOOL_A_TS_STAT_TX_LOST) != 0
        || tsinfo_put_stat(skb, (*stats).tx_stats.err, ETHTOOL_A_TS_STAT_TX_ERR) != 0 {
        nla_nest_cancel(skb, nest); return -EMSGSIZE;
    }
    nla_nest_end(skb, nest); 0
}

unsafe fn tsinfo_fill_reply(skb: *mut sk_buff, req_base: *const ethnl_req_info, reply_base: *const ethnl_reply_data) -> c_int {
    let data = reply_base as *const tsinfo_reply_data;
    let compact = (*req_base).flags & ETHTOOL_FLAG_COMPACT_BITSETS != 0;
    let info = &(*data).ts_info;
    if info.so_timestamping != 0 { let r = ethnl_put_bitset32(skb, ETHTOOL_A_TSINFO_TIMESTAMPING, &info.so_timestamping, core::ptr::null(), __SOF_TIMESTAMPING_CNT, sof_timestamping_names, compact); if r < 0 { return r; } }
    if info.tx_types != 0 { let r = ethnl_put_bitset32(skb, ETHTOOL_A_TSINFO_TX_TYPES, &info.tx_types, core::ptr::null(), __HWTSTAMP_TX_CNT, ts_tx_type_names, compact); if r < 0 { return r; } }
    if info.rx_filters != 0 { let r = ethnl_put_bitset32(skb, ETHTOOL_A_TSINFO_RX_FILTERS, &info.rx_filters, core::ptr::null(), __HWTSTAMP_FILTER_CNT, ts_rx_filter_names, compact); if r < 0 { return r; } }
    if info.phc_index >= 0 {
        if nla_put_u32(skb, ETHTOOL_A_TSINFO_PHC_INDEX, info.phc_index) != 0 { return -EMSGSIZE; }
        let nest = nla_nest_start(skb, ETHTOOL_A_TSINFO_HWTSTAMP_PROVIDER); if nest.is_null() { return -EMSGSIZE; }
        if nla_put_u32(skb, ETHTOOL_A_TS_HWTSTAMP_PROVIDER_INDEX, info.phc_index) != 0 || nla_put_u32(skb, ETHTOOL_A_TS_HWTSTAMP_PROVIDER_QUALIFIER, info.phc_qualifier) != 0 { nla_nest_cancel(skb, nest); return -EMSGSIZE; }
        nla_nest_end(skb, nest);
    }
    if info.phc_source != 0 { if nla_put_u32(skb, ETHTOOL_A_TSINFO_HWTSTAMP_SOURCE, info.phc_source) != 0 { return -EMSGSIZE; } if info.phc_phyindex != 0 && nla_put_u32(skb, ETHTOOL_A_TSINFO_HWTSTAMP_PHYINDEX, info.phc_phyindex) != 0 { return -EMSGSIZE; } }
    if (*req_base).flags & ETHTOOL_FLAG_STATS != 0 && tsinfo_put_stats(skb, &(*data).stats) != 0 { return -EMSGSIZE; }
    0
}

#[repr(C)]
pub struct ethnl_tsinfo_dump_ctx {
    pub req_info: *mut tsinfo_req_info,
    pub reply_data: *mut tsinfo_reply_data,
    pub pos_ifindex: c_ulong,
    pub netdev_dump_done: bool,
    pub pos_phyindex: c_ulong,
    pub pos_phcqualifier: hwtstamp_provider_qualifier,
}

// The remaining dump callbacks preserve the C callback ABI and delegate to the external
// kernel helpers and declarations supplied by the surrounding translation unit.
extern "C" {
    pub fn ethnl_tsinfo_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
    pub fn ethnl_tsinfo_start(cb: *mut netlink_callback) -> c_int;
    pub fn ethnl_tsinfo_done(cb: *mut netlink_callback) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
