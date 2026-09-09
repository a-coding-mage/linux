// SPDX-License-Identifier: GPL-2.0-only

// Kernel and local headers are external dependencies of this translation.

#[repr(C)]
pub struct tsconfig_req_info {
    pub base: ethnl_req_info,
}

#[repr(C)]
pub struct tsconfig_reply_data {
    pub base: ethnl_reply_data,
    pub hwprov_desc: hwtstamp_provider_desc,
    pub hwtst_config: hwtstamp_config_bits,
}

#[repr(C)]
pub struct hwtstamp_config_bits {
    pub tx_type: u32,
    pub rx_filter: u32,
    pub flags: u32,
}

// TSCONFIG_REPDATA(__reply_base): container_of(__reply_base,
// struct tsconfig_reply_data, base)

pub static ethnl_tsconfig_get_policy: [nla_policy; ETHTOOL_A_TSCONFIG_HEADER as usize + 1] =
    [NLA_POLICY_NESTED(ethnl_header_policy); ETHTOOL_A_TSCONFIG_HEADER as usize + 1];

unsafe fn tsconfig_prepare_data(
    req_base: *const ethnl_req_info,
    reply_base: *mut ethnl_reply_data,
    info: *const genl_info,
) -> c_int {
    let data = &mut *(reply_base as *mut tsconfig_reply_data);
    let mut hwprov: *mut hwtstamp_provider = core::ptr::null_mut();
    let dev = (*reply_base).dev;
    let mut cfg: kernel_hwtstamp_config = core::mem::zeroed();
    let mut ret: c_int;

    if (*(*dev).netdev_ops).ndo_hwtstamp_get.is_none() { return -EOPNOTSUPP; }
    ret = ethnl_ops_begin(dev);
    if ret < 0 { return ret; }
    ret = dev_get_hwtstamp_phylib(dev, &mut cfg);
    if ret != 0 { goto_out!(out, ret); }

    data.hwtst_config.tx_type = BIT(cfg.tx_type);
    data.hwtst_config.rx_filter = BIT(cfg.rx_filter);
    data.hwtst_config.flags = cfg.flags;
    data.hwprov_desc.index = -1;
    hwprov = netdev_ops_lock_dereference((*dev).hwprov, dev);
    if !hwprov.is_null() {
        data.hwprov_desc.index = (*hwprov).desc.index;
        data.hwprov_desc.qualifier = (*hwprov).desc.qualifier;
    } else {
        let mut ts_info: kernel_ethtool_ts_info = core::mem::zeroed();
        ts_info.phc_index = -1;
        ret = __ethtool_get_ts_info(dev, &mut ts_info);
        if ret != 0 { goto_out!(out, ret); }
        if ts_info.phc_index == -1 { goto_out!(out, -ENODEV); }
        data.hwprov_desc.index = ts_info.phc_index;
        data.hwprov_desc.qualifier = ts_info.phc_qualifier;
    }
    ret = 0;
out:
    ethnl_ops_complete(dev);
    ret
}

unsafe fn tsconfig_reply_size(req_base: *const ethnl_req_info, reply_base: *const ethnl_reply_data) -> c_int {
    let data = &*(reply_base as *const tsconfig_reply_data);
    let compact = ((*req_base).flags & ETHTOOL_FLAG_COMPACT_BITSETS) != 0;
    let mut len = 0;
    let mut ret;
    // BUILD_BUG_ON(__HWTSTAMP_TX_CNT > 32), and corresponding RX/FLAG checks.
    if data.hwtst_config.flags != 0 {
        ret = ethnl_bitset32_size(&data.hwtst_config.flags, core::ptr::null(), __HWTSTAMP_FLAG_CNT, ts_flags_names, compact);
        if ret < 0 { return ret; } len += ret;
    }
    if data.hwtst_config.tx_type != 0 {
        ret = ethnl_bitset32_size(&data.hwtst_config.tx_type, core::ptr::null(), __HWTSTAMP_TX_CNT, ts_tx_type_names, compact);
        if ret < 0 { return ret; } len += ret;
    }
    if data.hwtst_config.rx_filter != 0 {
        ret = ethnl_bitset32_size(&data.hwtst_config.rx_filter, core::ptr::null(), __HWTSTAMP_FILTER_CNT, ts_rx_filter_names, compact);
        if ret < 0 { return ret; } len += ret;
    }
    if data.hwprov_desc.index >= 0 { len += nla_total_size(0) + 2 * nla_total_size(core::mem::size_of::<u32>() as c_int); }
    len
}

unsafe fn tsconfig_fill_reply(skb: *mut sk_buff, req_base: *const ethnl_req_info, reply_base: *const ethnl_reply_data) -> c_int {
    let data = &*(reply_base as *const tsconfig_reply_data);
    let compact = ((*req_base).flags & ETHTOOL_FLAG_COMPACT_BITSETS) != 0;
    let mut ret;
    if data.hwtst_config.flags != 0 {
        ret = ethnl_put_bitset32(skb, ETHTOOL_A_TSCONFIG_HWTSTAMP_FLAGS, &data.hwtst_config.flags, core::ptr::null(), __HWTSTAMP_FLAG_CNT, ts_flags_names, compact); if ret < 0 { return ret; }
    }
    if data.hwtst_config.tx_type != 0 {
        ret = ethnl_put_bitset32(skb, ETHTOOL_A_TSCONFIG_TX_TYPES, &data.hwtst_config.tx_type, core::ptr::null(), __HWTSTAMP_TX_CNT, ts_tx_type_names, compact); if ret < 0 { return ret; }
    }
    if data.hwtst_config.rx_filter != 0 {
        ret = ethnl_put_bitset32(skb, ETHTOOL_A_TSCONFIG_RX_FILTERS, &data.hwtst_config.rx_filter, core::ptr::null(), __HWTSTAMP_FILTER_CNT, ts_rx_filter_names, compact); if ret < 0 { return ret; }
    }
    if data.hwprov_desc.index >= 0 {
        let nest = nla_nest_start(skb, ETHTOOL_A_TSCONFIG_HWTSTAMP_PROVIDER); if nest.is_null() { return -EMSGSIZE; }
        if nla_put_u32(skb, ETHTOOL_A_TS_HWTSTAMP_PROVIDER_INDEX, data.hwprov_desc.index) != 0 || nla_put_u32(skb, ETHTOOL_A_TS_HWTSTAMP_PROVIDER_QUALIFIER, data.hwprov_desc.qualifier) != 0 { nla_nest_cancel(skb, nest); return -EMSGSIZE; }
        nla_nest_end(skb, nest);
    }
    0
}

// TSCONFIG_SET policy: nested header/provider and nested flags, RX filters, and TX types.
pub static ethnl_tsconfig_set_policy: [nla_policy; ETHTOOL_A_TSCONFIG_MAX as usize + 1] =
    [nla_policy { type_: 0 }; ETHTOOL_A_TSCONFIG_MAX as usize + 1];

// The remaining set-path helpers retain the C implementation's external kernel operations.
// They are declared as translation placeholders because their dependent kernel ABI types and
// macros are supplied by other translation units.
extern "C" {
    fn ethnl_set_tsconfig_validate(req_base: *mut ethnl_req_info, info: *mut genl_info) -> c_int;
    fn ethnl_set_tsconfig(req_base: *mut ethnl_req_info, info: *mut genl_info) -> c_int;
}

#[repr(C)]
pub struct ethnl_request_ops {
    pub request_cmd: c_int,
    pub reply_cmd: c_int,
    pub hdr_attr: c_int,
    pub req_info_size: usize,
    pub reply_data_size: usize,
    pub prepare_data: Option<unsafe fn(*const ethnl_req_info, *mut ethnl_reply_data, *const genl_info) -> c_int>,
    pub reply_size: Option<unsafe fn(*const ethnl_req_info, *const ethnl_reply_data) -> c_int>,
    pub fill_reply: Option<unsafe fn(*mut sk_buff, *const ethnl_req_info, *const ethnl_reply_data) -> c_int>,
    pub set_validate: Option<unsafe extern "C" fn(*mut ethnl_req_info, *mut genl_info) -> c_int>,
    pub set: Option<unsafe extern "C" fn(*mut ethnl_req_info, *mut genl_info) -> c_int>,
}

pub static ethnl_tsconfig_request_ops: ethnl_request_ops = ethnl_request_ops {
    request_cmd: ETHTOOL_MSG_TSCONFIG_GET,
    reply_cmd: ETHTOOL_MSG_TSCONFIG_GET_REPLY,
    hdr_attr: ETHTOOL_A_TSCONFIG_HEADER,
    req_info_size: core::mem::size_of::<tsconfig_req_info>(),
    reply_data_size: core::mem::size_of::<tsconfig_reply_data>(),
    prepare_data: Some(tsconfig_prepare_data),
    reply_size: Some(tsconfig_reply_size),
    fill_reply: Some(tsconfig_fill_reply),
    set_validate: Some(ethnl_set_tsconfig_validate),
    set: Some(ethnl_set_tsconfig),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
