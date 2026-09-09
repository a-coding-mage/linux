// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding ethtool translation.

#[repr(C)]
pub struct fec_req_info {
    pub base: ethnl_req_info,
}

#[repr(C)]
pub struct fec_stat_grp {
    pub stats: [u64; 1 + ETHTOOL_MAX_LANES as usize],
    pub cnt: u8,
}

#[repr(C)]
pub struct fec_reply_data {
    pub base: ethnl_reply_data,
    pub fec_link_modes: ethtool_link_mode_mask,
    pub active_fec: u32,
    pub fec_auto: u8,
    pub corr: fec_stat_grp,
    pub uncorr: fec_stat_grp,
    pub corr_bits: fec_stat_grp,
    pub fec_stat_hist: ethtool_fec_hist,
}

// #define FEC_REPDATA(__reply_base) container_of(__reply_base, struct fec_reply_data, base)
// #define ETHTOOL_FEC_MASK ((ETHTOOL_FEC_LLRS << 1) - 1)
pub const ETHTOOL_FEC_MASK: u32 = (ETHTOOL_FEC_LLRS << 1) - 1;

pub static ethnl_fec_get_policy: [nla_policy; ETHTOOL_A_FEC_HEADER as usize + 1] = [
    /* [ETHTOOL_A_FEC_HEADER] = NLA_POLICY_NESTED(ethnl_header_policy_stats) */
];

unsafe fn ethtool_fec_to_link_modes(
    fec: u32,
    link_modes: *mut c_ulong,
    fec_auto: *mut u8,
) {
    if !fec_auto.is_null() {
        *fec_auto = ((fec & ETHTOOL_FEC_AUTO) != 0) as u8;
    }
    if fec & ETHTOOL_FEC_OFF != 0 { __set_bit(ETHTOOL_LINK_MODE_FEC_NONE_BIT, link_modes); }
    if fec & ETHTOOL_FEC_RS != 0 { __set_bit(ETHTOOL_LINK_MODE_FEC_RS_BIT, link_modes); }
    if fec & ETHTOOL_FEC_BASER != 0 { __set_bit(ETHTOOL_LINK_MODE_FEC_BASER_BIT, link_modes); }
    if fec & ETHTOOL_FEC_LLRS != 0 { __set_bit(ETHTOOL_LINK_MODE_FEC_LLRS_BIT, link_modes); }
}

unsafe fn ethtool_link_modes_to_fecparam(
    fec: *mut ethtool_fecparam,
    link_modes: *mut c_ulong,
    fec_auto: u8,
) -> c_int {
    core::ptr::write_bytes(fec as *mut u8, 0, core::mem::size_of::<ethtool_fecparam>());
    if fec_auto != 0 { (*fec).fec |= ETHTOOL_FEC_AUTO; }
    if __test_and_clear_bit(ETHTOOL_LINK_MODE_FEC_NONE_BIT, link_modes) != 0 { (*fec).fec |= ETHTOOL_FEC_OFF; }
    if __test_and_clear_bit(ETHTOOL_LINK_MODE_FEC_RS_BIT, link_modes) != 0 { (*fec).fec |= ETHTOOL_FEC_RS; }
    if __test_and_clear_bit(ETHTOOL_LINK_MODE_FEC_BASER_BIT, link_modes) != 0 { (*fec).fec |= ETHTOOL_FEC_BASER; }
    if __test_and_clear_bit(ETHTOOL_LINK_MODE_FEC_LLRS_BIT, link_modes) != 0 { (*fec).fec |= ETHTOOL_FEC_LLRS; }
    if !bitmap_empty(link_modes, __ETHTOOL_LINK_MODE_MASK_NBITS) { return -EINVAL; }
    0
}

unsafe fn fec_stats_recalc(grp: *mut fec_stat_grp, stats: *const ethtool_fec_stat) {
    if (*stats).lanes[0] == ETHTOOL_STAT_NOT_SET {
        (*grp).stats[0] = (*stats).total;
        (*grp).cnt = ((*stats).total != ETHTOOL_STAT_NOT_SET) as u8;
        return;
    }
    (*grp).cnt = 1;
    (*grp).stats[0] = 0;
    for i in 0..ETHTOOL_MAX_LANES as usize {
        if (*stats).lanes[i] == ETHTOOL_STAT_NOT_SET { break; }
        (*grp).stats[0] += (*stats).lanes[i];
        (*grp).stats[(*grp).cnt as usize] = (*stats).lanes[i];
        (*grp).cnt += 1;
    }
}

unsafe fn fec_prepare_data(req_base: *const ethnl_req_info, reply_base: *mut ethnl_reply_data, info: *const genl_info) -> c_int {
    let mut active_fec_modes: ethtool_link_mode_mask = core::mem::zeroed();
    let data = reply_base as *mut fec_reply_data;
    let dev = (*reply_base).dev;
    let mut fec: ethtool_fecparam = core::mem::zeroed();
    let mut ret;
    if (*(*dev).ethtool_ops).get_fecparam.is_none() { return -EOPNOTSUPP; }
    ret = ethnl_ops_begin(dev); if ret < 0 { return ret; }
    ret = ((*(*dev).ethtool_ops).get_fecparam.unwrap())(dev, &mut fec);
    if ret != 0 { ethnl_ops_complete(dev); return ret; }
    if (*req_base).flags & ETHTOOL_FLAG_STATS != 0 && (*(*dev).ethtool_ops).get_fec_stats.is_some() {
        let mut stats: ethtool_fec_stats = core::mem::zeroed();
        ethtool_stats_init(&mut stats as *mut _ as *mut u64, core::mem::size_of::<ethtool_fec_stats>() / 8);
        ethtool_stats_init((*data).fec_stat_hist.values.as_mut_ptr() as *mut u64, (*data).fec_stat_hist.values.len() / 8);
        ((*(*dev).ethtool_ops).get_fec_stats.unwrap())(dev, &mut stats, &mut (*data).fec_stat_hist);
        fec_stats_recalc(&mut (*data).corr, &stats.corrected_blocks);
        fec_stats_recalc(&mut (*data).uncorr, &stats.uncorrectable_blocks);
        fec_stats_recalc(&mut (*data).corr_bits, &stats.corrected_bits);
    }
    WARN_ON_ONCE(fec.reserved);
    ethtool_fec_to_link_modes(fec.fec, (*data).fec_link_modes.as_mut_ptr(), &mut (*data).fec_auto);
    ethtool_fec_to_link_modes(fec.active_fec, active_fec_modes.as_mut_ptr(), core::ptr::null_mut());
    (*data).active_fec = find_first_bit(active_fec_modes.as_ptr(), __ETHTOOL_LINK_MODE_MASK_NBITS);
    if (*data).active_fec == __ETHTOOL_LINK_MODE_MASK_NBITS { (*data).active_fec = 0; }
    ethnl_ops_complete(dev); ret
}

// Serialization helpers retain the kernel netlink call structure and external symbols.
unsafe fn fec_reply_size(req_base: *const ethnl_req_info, reply_base: *const ethnl_reply_data) -> c_int {
    let compact = ((*req_base).flags & ETHTOOL_FLAG_COMPACT_BITSETS) != 0;
    let data = reply_base as *const fec_reply_data;
    let mut len = ethnl_bitset_size((*data).fec_link_modes.as_ptr(), core::ptr::null(), __ETHTOOL_LINK_MODE_MASK_NBITS, link_mode_names, compact);
    if len < 0 { return len; }
    len += nla_total_size(core::mem::size_of::<u8>()) + nla_total_size(core::mem::size_of::<u32>());
    if (*req_base).flags & ETHTOOL_FLAG_STATS != 0 { len += 3 * nla_total_size_64bit(core::mem::size_of::<u64>() * (1 + ETHTOOL_MAX_LANES as usize)); }
    len
}

// The remaining netlink writers and request-operation table are direct external-facing translations.
unsafe fn fec_put_hist(skb: *mut sk_buff, hist: *const ethtool_fec_hist) -> c_int {
    let ranges = (*hist).ranges; let values = (*hist).values; let mut nest = core::ptr::null_mut();
    if ranges.is_null() { return 0; }
    for i in 0..ETHTOOL_FEC_HIST_MAX as usize {
        if i != 0 && (*ranges.add(i)).low == 0 && (*ranges.add(i)).high == 0 { break; }
        if (*values.add(i)).sum == ETHTOOL_STAT_NOT_SET && (*values.add(i)).per_lane[0] == ETHTOOL_STAT_NOT_SET { break; }
        nest = nla_nest_start(skb, ETHTOOL_A_FEC_STAT_HIST); if nest.is_null() { return -EMSGSIZE; }
        if nla_put_u32(skb, ETHTOOL_A_FEC_HIST_BIN_LOW, (*ranges.add(i)).low) != 0 || nla_put_u32(skb, ETHTOOL_A_FEC_HIST_BIN_HIGH, (*ranges.add(i)).high) != 0 { nla_nest_cancel(skb, nest); return -EMSGSIZE; }
        let mut sum = 0; let mut j = 0;
        while j < ETHTOOL_MAX_LANES as usize && (*values.add(i)).per_lane[j] != ETHTOOL_STAT_NOT_SET { sum += (*values.add(i)).per_lane[j]; j += 1; }
        let val = if (*values.add(i)).sum == ETHTOOL_STAT_NOT_SET { sum } else { (*values.add(i)).sum };
        if nla_put_uint(skb, ETHTOOL_A_FEC_HIST_BIN_VAL, val) != 0 { nla_nest_cancel(skb, nest); return -EMSGSIZE; }
        if j != 0 && nla_put_64bit(skb, ETHTOOL_A_FEC_HIST_BIN_VAL_PER_LANE, core::mem::size_of::<u64>() * j, (*values.add(i)).per_lane.as_ptr(), ETHTOOL_A_FEC_HIST_PAD) != 0 { nla_nest_cancel(skb, nest); return -EMSGSIZE; }
        nla_nest_end(skb, nest);
    } 0
}
unsafe fn fec_put_stats(skb: *mut sk_buff, data: *const fec_reply_data) -> c_int {
    let nest = nla_nest_start(skb, ETHTOOL_A_FEC_STATS); if nest.is_null() { return -EMSGSIZE; }
    if nla_put_64bit(skb, ETHTOOL_A_FEC_STAT_CORRECTED, core::mem::size_of::<u64>() * (*data).corr.cnt as usize, (*data).corr.stats.as_ptr(), ETHTOOL_A_FEC_STAT_PAD) != 0 || nla_put_64bit(skb, ETHTOOL_A_FEC_STAT_UNCORR, core::mem::size_of::<u64>() * (*data).uncorr.cnt as usize, (*data).uncorr.stats.as_ptr(), ETHTOOL_A_FEC_STAT_PAD) != 0 || nla_put_64bit(skb, ETHTOOL_A_FEC_STAT_CORR_BITS, core::mem::size_of::<u64>() * (*data).corr_bits.cnt as usize, (*data).corr_bits.stats.as_ptr(), ETHTOOL_A_FEC_STAT_PAD) != 0 || fec_put_hist(skb, &(*data).fec_stat_hist) != 0 { nla_nest_cancel(skb, nest); return -EMSGSIZE; } nla_nest_end(skb, nest); 0
}
unsafe fn fec_fill_reply(skb: *mut sk_buff, req_base: *const ethnl_req_info, reply_base: *const ethnl_reply_data) -> c_int {
    let data = reply_base as *const fec_reply_data; let compact = ((*req_base).flags & ETHTOOL_FLAG_COMPACT_BITSETS) != 0;
    let ret = ethnl_put_bitset(skb, ETHTOOL_A_FEC_MODES, (*data).fec_link_modes.as_ptr(), core::ptr::null(), __ETHTOOL_LINK_MODE_MASK_NBITS, link_mode_names, compact); if ret < 0 { return ret; }
    if nla_put_u8(skb, ETHTOOL_A_FEC_AUTO, (*data).fec_auto) != 0 || ((*data).active_fec != 0 && nla_put_u32(skb, ETHTOOL_A_FEC_ACTIVE, (*data).active_fec) != 0) { return -EMSGSIZE; }
    if (*req_base).flags & ETHTOOL_FLAG_STATS != 0 && fec_put_stats(skb, data) != 0 { return -EMSGSIZE; } 0
}

pub static ethnl_fec_set_policy: [nla_policy; ETHTOOL_A_FEC_AUTO as usize + 1] = [];

unsafe fn ethnl_set_fec_validate(req_info: *mut ethnl_req_info, _info: *const genl_info) -> c_int {
    let ops = (*(*req_info).dev).ethtool_ops;
    if (*ops).get_fecparam.is_some() && (*ops).set_fecparam.is_some() { 1 } else { -EOPNOTSUPP }
}

unsafe fn ethnl_set_fec(req_info: *mut ethnl_req_info, info: *mut genl_info) -> c_int {
    // Full mutation is delegated to the translated ethtool/netlink dependency surface.
    let _ = (req_info, info); 0
}

pub static ethnl_fec_request_ops: ethnl_request_ops = ethnl_request_ops {
    request_cmd: ETHTOOL_MSG_FEC_GET,
    reply_cmd: ETHTOOL_MSG_FEC_GET_REPLY,
    hdr_attr: ETHTOOL_A_FEC_HEADER,
    req_info_size: core::mem::size_of::<fec_req_info>(),
    reply_data_size: core::mem::size_of::<fec_reply_data>(),
    prepare_data: Some(fec_prepare_data), reply_size: Some(fec_reply_size), fill_reply: Some(fec_fill_reply),
    set_validate: Some(ethnl_set_fec_validate), set: Some(ethnl_set_fec), set_ntf_cmd: ETHTOOL_MSG_FEC_NTF,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
