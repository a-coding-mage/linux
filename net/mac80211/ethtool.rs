// SPDX-License-Identifier: GPL-2.0-only
/*
 * mac80211 ethtool hooks for cfg80211
 *
 * Copied from cfg.c - originally
 * Copyright 2006-2010 Johannes Berg <johannes@sipsolutions.net>
 * Copyright 2014 Intel Corporation (Author: Johannes Berg)
 * Copyright (C) 2018, 2022-2023 Intel Corporation
 */
// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

unsafe fn ieee80211_set_ringparam(
    dev: *mut net_device,
    rp: *mut ethtool_ringparam,
    _kernel_rp: *mut kernel_ethtool_ringparam,
    _extack: *mut netlink_ext_ack,
) -> i32 {
    let local = wiphy_priv((*(*dev).ieee80211_ptr).wiphy);

    if (*rp).rx_mini_pending != 0 || (*rp).rx_jumbo_pending != 0 {
        return -EINVAL;
    }

    // guard(wiphy)(local->hw.wiphy);
    let _wiphy_guard = WiphyGuard::new((*local).hw.wiphy);

    drv_set_ringparam(local, (*rp).tx_pending, (*rp).rx_pending)
}

unsafe fn ieee80211_get_ringparam(
    dev: *mut net_device,
    rp: *mut ethtool_ringparam,
    _kernel_rp: *mut kernel_ethtool_ringparam,
    _extack: *mut netlink_ext_ack,
) {
    let local = wiphy_priv((*(*dev).ieee80211_ptr).wiphy);

    memset(rp as *mut u8, 0, core::mem::size_of::<ethtool_ringparam>());

    // guard(wiphy)(local->hw.wiphy);
    let _wiphy_guard = WiphyGuard::new((*local).hw.wiphy);

    drv_get_ringparam(
        local,
        &mut (*rp).tx_pending,
        &mut (*rp).tx_max_pending,
        &mut (*rp).rx_pending,
        &mut (*rp).rx_max_pending,
    );
}

static IEEE80211_GSTRINGS_STA_STATS: [[u8; ETH_GSTRING_LEN]; 22] = [
    *b"rx_packets", *b"rx_bytes", *b"rx_duplicates", *b"rx_fragments",
    *b"rx_dropped", *b"tx_packets", *b"tx_bytes", *b"tx_filtered",
    *b"tx_retry_failed", *b"tx_retries", *b"tx_handlers_drop", *b"sta_state",
    *b"txrate", *b"rxrate", *b"signal", *b"channel", *b"noise", *b"ch_time",
    *b"ch_time_busy", *b"ch_time_ext_busy", *b"ch_time_rx", *b"ch_time_tx",
];
const STA_STATS_LEN: usize = IEEE80211_GSTRINGS_STA_STATS.len();

unsafe fn ieee80211_get_sset_count(dev: *mut net_device, sset: i32) -> i32 {
    let sdata = IEEE80211_DEV_TO_SUB_IF(dev);
    let mut rv = 0;

    if sset == ETH_SS_STATS {
        rv += STA_STATS_LEN as i32;
    }
    rv += drv_get_et_sset_count(sdata, sset);
    if rv == 0 { -EOPNOTSUPP } else { rv }
}

unsafe fn ieee80211_get_stats(
    dev: *mut net_device,
    stats: *mut ethtool_stats,
    data: *mut u64,
) {
    let sdata = IEEE80211_DEV_TO_SUB_IF(dev);
    let mut chanctx_conf: *mut ieee80211_chanctx_conf;
    let mut channel: *mut ieee80211_channel;
    let mut sta: *mut sta_info;
    let local = (*sdata).local;
    let mut sinfo: station_info = core::mem::zeroed();
    let mut survey: survey_info = core::mem::zeroed();
    let mut i: usize;
    let mut q: i32;
    const STA_STATS_SURVEY_LEN: usize = 7;

    memset(data as *mut u8, 0, core::mem::size_of::<u64>() * STA_STATS_LEN);

    // ADD_STA_STATS(sta)
    macro_rules! add_sta_stats {
        ($station:expr) => {{
            *data.add(i) += sinfo.rx_packets; i += 1;
            *data.add(i) += sinfo.rx_bytes; i += 1;
            *data.add(i) += (*$station).rx_stats.num_duplicates; i += 1;
            *data.add(i) += (*$station).rx_stats.fragments; i += 1;
            *data.add(i) += sinfo.rx_dropped_misc; i += 1;
            *data.add(i) += sinfo.tx_packets; i += 1;
            *data.add(i) += sinfo.tx_bytes; i += 1;
            *data.add(i) += (*$station).status_stats.filtered; i += 1;
            *data.add(i) += sinfo.tx_failed; i += 1;
            *data.add(i) += sinfo.tx_retries; i += 1;
        }};
    }

    /* For Managed stations, find the single station based on BSSID
     * and use that. For interface types, iterate through all available
     * stations and add stats for any station that is assigned to this
     * network device.
     */
    let _wiphy_guard = WiphyGuard::new((*local).hw.wiphy);

    if (*sdata).vif.r#type == NL80211_IFTYPE_STATION {
        sta = sta_info_get_bss(sdata, (*sdata).deflink.u.mgd.bssid);
        if sta.is_null() || WARN_ON((*sta).sdata.dev != dev) { goto_do_survey!(); }
        sinfo = core::mem::zeroed();
        sta_set_sinfo(sta, &mut sinfo, false);
        i = 0;
        add_sta_stats!(&mut (*sta).deflink);
        *data.add(i) = (*sdata).tx_handlers_drop; i += 1;
        *data.add(i) = (*sta).sta_state; i += 1;
        if sinfo.filled & BIT_ULL(NL80211_STA_INFO_TX_BITRATE) != 0 {
            *data.add(i) = 100000 * cfg80211_calculate_bitrate(&sinfo.txrate);
        } i += 1;
        if sinfo.filled & BIT_ULL(NL80211_STA_INFO_RX_BITRATE) != 0 {
            *data.add(i) = 100000 * cfg80211_calculate_bitrate(&sinfo.rxrate);
        } i += 1;
        if sinfo.filled & BIT_ULL(NL80211_STA_INFO_SIGNAL_AVG) != 0 {
            *data.add(i) = sinfo.signal_avg as u8 as u64;
        } i += 1;
    } else {
        list_for_each_entry!(sta, (*local).sta_list, list, {
            if (*sta).sdata.dev != dev { continue; }
            sinfo = core::mem::zeroed();
            sta_set_sinfo(sta, &mut sinfo, false);
            i = 0;
            add_sta_stats!(&mut (*sta).deflink);
            *data.add(i) = (*sdata).tx_handlers_drop;
        });
    }

    'do_survey: {
        i = STA_STATS_LEN - STA_STATS_SURVEY_LEN;
        survey.filled = 0;
        rcu_read_lock();
        chanctx_conf = rcu_dereference((*sdata).vif.bss_conf.chanctx_conf);
        if !chanctx_conf.is_null() { channel = (*chanctx_conf).def_.chan; }
        else if (*local).open_count > 0 && (*local).open_count == (*local).virt_monitors && (*sdata).vif.r#type == NL80211_IFTYPE_MONITOR { channel = (*local).monitor_chanreq.oper.chan; }
        else { channel = core::ptr::null_mut(); }
        rcu_read_unlock();
        if !channel.is_null() {
            q = 0;
            loop {
                survey.filled = 0;
                if drv_get_survey(local, q, &mut survey) != 0 { survey.filled = 0; break; }
                q += 1;
                if channel == survey.channel { break; }
            }
        }
        *data.add(i) = if survey.filled != 0 { (*survey.channel).center_freq as u64 } else { 0 }; i += 1;
        *data.add(i) = if survey.filled & SURVEY_INFO_NOISE_DBM != 0 { survey.noise as u8 as u64 } else { (-1i64) as u64 }; i += 1;
        *data.add(i) = if survey.filled & SURVEY_INFO_TIME != 0 { survey.time } else { (-1i64) as u64 }; i += 1;
        *data.add(i) = if survey.filled & SURVEY_INFO_TIME_BUSY != 0 { survey.time_busy } else { (-1i64) as u64 }; i += 1;
        *data.add(i) = if survey.filled & SURVEY_INFO_TIME_EXT_BUSY != 0 { survey.time_ext_busy } else { (-1i64) as u64 }; i += 1;
        *data.add(i) = if survey.filled & SURVEY_INFO_TIME_RX != 0 { survey.time_rx } else { (-1i64) as u64 }; i += 1;
        *data.add(i) = if survey.filled & SURVEY_INFO_TIME_TX != 0 { survey.time_tx } else { (-1i64) as u64 }; i += 1;
        if WARN_ON(i != STA_STATS_LEN) { return; }
        drv_get_et_stats(sdata, stats, data.add(STA_STATS_LEN));
    }
}

unsafe fn ieee80211_get_strings(dev: *mut net_device, sset: u32, data: *mut u8) {
    let sdata = IEEE80211_DEV_TO_SUB_IF(dev);
    let mut sz_sta_stats = 0;
    if sset == ETH_SS_STATS {
        sz_sta_stats = core::mem::size_of_val(&IEEE80211_GSTRINGS_STA_STATS);
        memcpy(data, IEEE80211_GSTRINGS_STA_STATS.as_ptr() as *const u8, sz_sta_stats);
    }
    drv_get_et_strings(sdata, sset, data.add(sz_sta_stats));
}

unsafe fn ieee80211_get_regs_len(_dev: *mut net_device) -> i32 { 0 }

unsafe fn ieee80211_get_regs(dev: *mut net_device, regs: *mut ethtool_regs, _data: *mut core::ffi::c_void) {
    let wdev = (*dev).ieee80211_ptr;
    (*regs).version = (*(*wdev).wiphy).hw_version;
    (*regs).len = 0;
}

const ieee80211_ethtool_ops: ethtool_ops = ethtool_ops {
    get_drvinfo: Some(cfg80211_get_drvinfo),
    get_regs_len: Some(ieee80211_get_regs_len),
    get_regs: Some(ieee80211_get_regs),
    get_link: Some(ethtool_op_get_link),
    get_ringparam: Some(ieee80211_get_ringparam),
    set_ringparam: Some(ieee80211_set_ringparam),
    get_strings: Some(ieee80211_get_strings),
    get_ethtool_stats: Some(ieee80211_get_stats),
    get_sset_count: Some(ieee80211_get_sset_count),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
