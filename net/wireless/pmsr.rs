/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2018 - 2021, 2023 - 2026 Intel Corporation */

// External kernel declarations supplied by the surrounding cfg80211/nl80211 code.
use core::ffi::c_void;

unsafe fn pmsr_parse_ftm(rdev: *mut cfg80211_registered_device, ftmreq: *mut nlattr,
                         out: *mut cfg80211_pmsr_request_peer, info: *mut genl_info) -> i32 {
    let capa = (*rdev).wiphy.pmsr_capa;
    let mut tb: [*mut nlattr; NL80211_PMSR_FTM_REQ_ATTR_MAX as usize + 1] = [core::ptr::null_mut(); NL80211_PMSR_FTM_REQ_ATTR_MAX as usize + 1];
    let mut preamble: u32 = NL80211_PREAMBLE_DMG;
    if (*out).ftm.request_type == NL80211_PMSR_FTM_REQ_TYPE_INFRA &&
       ((*capa).ftm.bandwidths & BIT((*out).chandef.width)) == 0 { NL_SET_ERR_MSG((*info).extack, "FTM: unsupported bandwidth"); return -EINVAL; }
    if (*out).ftm.request_type == NL80211_PMSR_FTM_REQ_TYPE_PD &&
       ((*capa).ftm.pd_bandwidths & BIT((*out).chandef.width)) == 0 { NL_SET_ERR_MSG((*info).extack, "FTM: unsupported bandwidth for PD request"); return -EINVAL; }
    nla_parse_nested_deprecated(tb.as_mut_ptr(), NL80211_PMSR_FTM_REQ_ATTR_MAX, ftmreq, core::ptr::null(), core::ptr::null_mut());
    if !tb[NL80211_PMSR_FTM_REQ_ATTR_PREAMBLE as usize].is_null() { preamble = nla_get_u32(tb[NL80211_PMSR_FTM_REQ_ATTR_PREAMBLE as usize]); }
    (*out).ftm.requested = true;
    match (*out).chandef.chan.band { NL80211_BAND_60GHZ => (), _ => if tb[NL80211_PMSR_FTM_REQ_ATTR_PREAMBLE as usize].is_null() { NL_SET_ERR_MSG((*info).extack, "FTM: must specify preamble"); return -EINVAL; } }
    if (*out).ftm.request_type == NL80211_PMSR_FTM_REQ_TYPE_INFRA && ((*capa).ftm.preambles & BIT(preamble)) == 0 { NL_SET_ERR_MSG_ATTR((*info).extack, tb[NL80211_PMSR_FTM_REQ_ATTR_PREAMBLE as usize], "FTM: invalid preamble"); return -EINVAL; }
    if (*out).ftm.request_type == NL80211_PMSR_FTM_REQ_TYPE_PD && ((*capa).ftm.pd_preambles & BIT(preamble)) == 0 { NL_SET_ERR_MSG_ATTR((*info).extack, tb[NL80211_PMSR_FTM_REQ_ATTR_PREAMBLE as usize], "FTM: invalid preamble for PD request"); return -EINVAL; }
    (*out).ftm.preamble = preamble;
    (*out).ftm.burst_period = 0; if !tb[NL80211_PMSR_FTM_REQ_ATTR_BURST_PERIOD as usize].is_null() { (*out).ftm.burst_period = nla_get_u16(tb[NL80211_PMSR_FTM_REQ_ATTR_BURST_PERIOD as usize]); }
    (*out).ftm.asap = !tb[NL80211_PMSR_FTM_REQ_ATTR_ASAP as usize].is_null();
    if (*out).ftm.asap && !(*capa).ftm.asap { NL_SET_ERR_MSG_ATTR((*info).extack, tb[NL80211_PMSR_FTM_REQ_ATTR_ASAP as usize], "FTM: ASAP mode not supported"); return -EINVAL; }
    if !(*out).ftm.asap && !(*capa).ftm.non_asap { NL_SET_ERR_MSG((*info).extack, "FTM: non-ASAP mode not supported"); return -EINVAL; }
    (*out).ftm.num_bursts_exp = 0; if !tb[NL80211_PMSR_FTM_REQ_ATTR_NUM_BURSTS_EXP as usize].is_null() { (*out).ftm.num_bursts_exp = nla_get_u8(tb[NL80211_PMSR_FTM_REQ_ATTR_NUM_BURSTS_EXP as usize]); }
    if (*capa).ftm.max_bursts_exponent >= 0 && (*out).ftm.num_bursts_exp > (*capa).ftm.max_bursts_exponent { NL_SET_ERR_MSG_ATTR((*info).extack, tb[NL80211_PMSR_FTM_REQ_ATTR_NUM_BURSTS_EXP as usize], "FTM: max NUM_BURSTS_EXP must be set lower than the device limit"); return -EINVAL; }
    (*out).ftm.ftms_per_burst = 0; if !tb[NL80211_PMSR_FTM_REQ_ATTR_FTMS_PER_BURST as usize].is_null() { (*out).ftm.ftms_per_burst = nla_get_u8(tb[NL80211_PMSR_FTM_REQ_ATTR_FTMS_PER_BURST as usize]); }
    if (*capa).ftm.max_ftms_per_burst != 0 && (*out).ftm.ftms_per_burst > (*capa).ftm.max_ftms_per_burst { NL_SET_ERR_MSG_ATTR((*info).extack, tb[NL80211_PMSR_FTM_REQ_ATTR_FTMS_PER_BURST as usize], "FTM: FTMs per burst must be set lower than the device limit"); return -EINVAL; }
    (*out).ftm.ftmr_retries = 3; if !tb[NL80211_PMSR_FTM_REQ_ATTR_NUM_FTMR_RETRIES as usize].is_null() { (*out).ftm.ftmr_retries = nla_get_u8(tb[NL80211_PMSR_FTM_REQ_ATTR_NUM_FTMR_RETRIES as usize]); }
    (*out).ftm.request_lci = !tb[NL80211_PMSR_FTM_REQ_ATTR_REQUEST_LCI as usize].is_null(); if (*out).ftm.request_lci && !(*capa).ftm.request_lci { NL_SET_ERR_MSG_ATTR((*info).extack, tb[NL80211_PMSR_FTM_REQ_ATTR_REQUEST_LCI as usize], "FTM: LCI request not supported"); return -EOPNOTSUPP; }
    (*out).ftm.request_civicloc = !tb[NL80211_PMSR_FTM_REQ_ATTR_REQUEST_CIVICLOC as usize].is_null(); if (*out).ftm.request_civicloc && !(*capa).ftm.request_civicloc { NL_SET_ERR_MSG_ATTR((*info).extack, tb[NL80211_PMSR_FTM_REQ_ATTR_REQUEST_CIVICLOC as usize], "FTM: civic location request not supported"); return -EOPNOTSUPP; }
    (*out).ftm.trigger_based = !tb[NL80211_PMSR_FTM_REQ_ATTR_TRIGGER_BASED as usize].is_null(); if (*out).ftm.trigger_based && !(*capa).ftm.trigger_based { NL_SET_ERR_MSG_ATTR((*info).extack, tb[NL80211_PMSR_FTM_REQ_ATTR_TRIGGER_BASED as usize], "FTM: trigger based ranging is not supported"); return -EINVAL; }
    if (*out).ftm.request_type == NL80211_PMSR_FTM_REQ_TYPE_PD && (*out).ftm.trigger_based { NL_SET_ERR_MSG_ATTR((*info).extack, ftmreq, "FTM: TB ranging is not supported for PD request type"); return -EINVAL; }
    (*out).ftm.non_trigger_based = !tb[NL80211_PMSR_FTM_REQ_ATTR_NON_TRIGGER_BASED as usize].is_null(); if (*out).ftm.non_trigger_based && !(*capa).ftm.non_trigger_based { NL_SET_ERR_MSG_ATTR((*info).extack, tb[NL80211_PMSR_FTM_REQ_ATTR_NON_TRIGGER_BASED as usize], "FTM: trigger based ranging is not supported"); return -EINVAL; }
    if (*out).ftm.trigger_based && (*out).ftm.non_trigger_based { NL_SET_ERR_MSG((*info).extack, "FTM: can't set both trigger based and non trigger based"); return -EINVAL; }
    if (*out).ftm.request_type == NL80211_PMSR_FTM_REQ_TYPE_PD && (*out).ftm.non_trigger_based && (*out).ftm.ftms_per_burst > 4 { NL_SET_ERR_MSG_ATTR((*info).extack, tb[NL80211_PMSR_FTM_REQ_ATTR_FTMS_PER_BURST as usize], "FTM: FTMs per burst must not exceed 4 for PD NTB ranging"); return -ERANGE; }
    if (*out).ftm.ftms_per_burst > 31 && !(*out).ftm.non_trigger_based && !(*out).ftm.trigger_based { NL_SET_ERR_MSG_ATTR((*info).extack, tb[NL80211_PMSR_FTM_REQ_ATTR_FTMS_PER_BURST as usize], "FTM: FTMs per burst must be set lower than 31"); return -ERANGE; }
    if ((*out).ftm.trigger_based || (*out).ftm.non_trigger_based) && (*out).ftm.preamble != NL80211_PREAMBLE_HE { NL_SET_ERR_MSG_ATTR((*info).extack, tb[NL80211_PMSR_FTM_REQ_ATTR_PREAMBLE as usize], "FTM: non EDCA based ranging must use HE preamble"); return -EINVAL; }
    if !tb[NL80211_PMSR_FTM_REQ_ATTR_BURST_DURATION as usize].is_null() { (*out).ftm.burst_duration = nla_get_u8(tb[NL80211_PMSR_FTM_REQ_ATTR_BURST_DURATION as usize]); } else if !(*out).ftm.non_trigger_based && !(*out).ftm.trigger_based { (*out).ftm.burst_duration = 15; }
    (*out).ftm.lmr_feedback = !tb[NL80211_PMSR_FTM_REQ_ATTR_LMR_FEEDBACK as usize].is_null(); if !(*out).ftm.trigger_based && !(*out).ftm.non_trigger_based && (*out).ftm.lmr_feedback { NL_SET_ERR_MSG_ATTR((*info).extack, tb[NL80211_PMSR_FTM_REQ_ATTR_LMR_FEEDBACK as usize], "FTM: LMR feedback set for EDCA based ranging"); return -EINVAL; }
    if !tb[NL80211_PMSR_FTM_REQ_ATTR_BSS_COLOR as usize].is_null() { if !(*out).ftm.non_trigger_based && !(*out).ftm.trigger_based { NL_SET_ERR_MSG_ATTR((*info).extack, tb[NL80211_PMSR_FTM_REQ_ATTR_BSS_COLOR as usize], "FTM: BSS color set for EDCA based ranging"); return -EINVAL; } (*out).ftm.bss_color = nla_get_u8(tb[NL80211_PMSR_FTM_REQ_ATTR_BSS_COLOR as usize]); }
    (*out).ftm.rsta = !tb[NL80211_PMSR_FTM_REQ_ATTR_RSTA as usize].is_null();
    if (*out).ftm.rsta && (*out).ftm.non_trigger_based && !(*capa).ftm.rsta.support_ntb { NL_SET_ERR_MSG_ATTR((*info).extack, tb[NL80211_PMSR_FTM_REQ_ATTR_RSTA as usize], "FTM: NTB RSTA not supported by device"); return -EOPNOTSUPP; }
    if (*out).ftm.rsta && (*out).ftm.trigger_based && !(*capa).ftm.rsta.support_tb { NL_SET_ERR_MSG_ATTR((*info).extack, tb[NL80211_PMSR_FTM_REQ_ATTR_RSTA as usize], "FTM: TB RSTA not supported by device"); return -EOPNOTSUPP; }
    if (*out).ftm.rsta && !(*out).ftm.non_trigger_based && !(*out).ftm.trigger_based && !(*capa).ftm.rsta.support_edca { NL_SET_ERR_MSG_ATTR((*info).extack, tb[NL80211_PMSR_FTM_REQ_ATTR_RSTA as usize], "FTM: EDCA RSTA not supported by device"); return -EOPNOTSUPP; }
    if (*out).ftm.rsta && ((*out).ftm.non_trigger_based || (*out).ftm.trigger_based) && !(*out).ftm.lmr_feedback { NL_SET_ERR_MSG_ATTR((*info).extack, tb[NL80211_PMSR_FTM_REQ_ATTR_RSTA as usize], "FTM: RSTA set without LMR feedback"); return -EINVAL; }
    if (*out).ftm.non_trigger_based { if (*out).ftm.request_type == NL80211_PMSR_FTM_REQ_TYPE_PD && tb[NL80211_PMSR_FTM_REQ_ATTR_NOMINAL_TIME as usize].is_null() { NL_SET_ERR_MSG((*info).extack, "FTM: nominal time is required for PD NTB ranging"); return -EINVAL; } if !tb[NL80211_PMSR_FTM_REQ_ATTR_NOMINAL_TIME as usize].is_null() { (*out).ftm.nominal_time = nla_get_u32(tb[NL80211_PMSR_FTM_REQ_ATTR_NOMINAL_TIME as usize]); } if !tb[NL80211_PMSR_FTM_REQ_ATTR_MIN_TIME_BETWEEN_MEASUREMENTS as usize].is_null() { (*out).ftm.min_time_between_measurements = nla_get_u32(tb[NL80211_PMSR_FTM_REQ_ATTR_MIN_TIME_BETWEEN_MEASUREMENTS as usize]); } if !tb[NL80211_PMSR_FTM_REQ_ATTR_MAX_TIME_BETWEEN_MEASUREMENTS as usize].is_null() { (*out).ftm.max_time_between_measurements = nla_get_u32(tb[NL80211_PMSR_FTM_REQ_ATTR_MAX_TIME_BETWEEN_MEASUREMENTS as usize]); } if !tb[NL80211_PMSR_FTM_REQ_ATTR_AW_DURATION as usize].is_null() { (*out).ftm.availability_window = nla_get_u8(tb[NL80211_PMSR_FTM_REQ_ATTR_AW_DURATION as usize]); } if !tb[NL80211_PMSR_FTM_REQ_ATTR_NUM_MEASUREMENTS as usize].is_null() { (*out).ftm.num_measurements = nla_get_u32(tb[NL80211_PMSR_FTM_REQ_ATTR_NUM_MEASUREMENTS as usize]); } }
    if !tb[NL80211_PMSR_FTM_REQ_ATTR_INGRESS as usize].is_null() { (*out).ftm.ingress_distance = nla_get_u64(tb[NL80211_PMSR_FTM_REQ_ATTR_INGRESS as usize]); } if !tb[NL80211_PMSR_FTM_REQ_ATTR_EGRESS as usize].is_null() { (*out).ftm.egress_distance = nla_get_u64(tb[NL80211_PMSR_FTM_REQ_ATTR_EGRESS as usize]); }
    (*out).ftm.pd_suppress_range_results = nla_get_flag(tb[NL80211_PMSR_FTM_REQ_ATTR_PD_SUPPRESS_RESULTS as usize]); if (*out).ftm.request_type != NL80211_PMSR_FTM_REQ_TYPE_PD && (*out).ftm.pd_suppress_range_results { NL_SET_ERR_MSG_ATTR((*info).extack, tb[NL80211_PMSR_FTM_REQ_ATTR_PD_SUPPRESS_RESULTS as usize], "FTM: suppress range result flag only valid for PD requests"); return -EINVAL; }
    0
}

unsafe fn pmsr_parse_peer(rdev: *mut cfg80211_registered_device, peer: *mut nlattr, out: *mut cfg80211_pmsr_request_peer, info: *mut genl_info) -> i32 {
    let mut tb: [*mut nlattr; NL80211_PMSR_PEER_ATTR_MAX as usize + 1] = [core::ptr::null_mut(); NL80211_PMSR_PEER_ATTR_MAX as usize + 1];
    let mut req: [*mut nlattr; NL80211_PMSR_REQ_ATTR_MAX as usize + 1] = [core::ptr::null_mut(); NL80211_PMSR_REQ_ATTR_MAX as usize + 1];
    nla_parse_nested_deprecated(tb.as_mut_ptr(), NL80211_PMSR_PEER_ATTR_MAX, peer, core::ptr::null(), core::ptr::null_mut());
    if tb[NL80211_PMSR_PEER_ATTR_ADDR as usize].is_null() || tb[NL80211_PMSR_PEER_ATTR_CHAN as usize].is_null() || tb[NL80211_PMSR_PEER_ATTR_REQ as usize].is_null() { NL_SET_ERR_MSG_ATTR((*info).extack, peer, "insufficient peer data"); return -EINVAL; }
    core::ptr::copy_nonoverlapping(nla_data(tb[NL80211_PMSR_PEER_ATTR_ADDR as usize]), (*out).addr.as_mut_ptr() as *mut c_void, ETH_ALEN as usize);
    (*out).ftm.request_type = if !tb[NL80211_PMSR_PEER_ATTR_REQ_TYPE as usize].is_null() { nla_get_u32(tb[NL80211_PMSR_PEER_ATTR_REQ_TYPE as usize]) } else { NL80211_PMSR_FTM_REQ_TYPE_INFRA };
    if (*out).ftm.request_type == NL80211_PMSR_FTM_REQ_TYPE_PD && !(*rdev).wiphy.pmsr_capa.ftm.type_.pd_support { NL_SET_ERR_MSG_ATTR((*info).extack, tb[NL80211_PMSR_PEER_ATTR_REQ_TYPE as usize], "FTM: PD request type not supported by device"); return -EINVAL; }
    core::ptr::write_bytes((*info).attrs, 0, (NL80211_ATTR_MAX as usize + 1) * core::mem::size_of::<*mut nlattr>());
    let mut err = nla_parse_nested_deprecated((*info).attrs, NL80211_ATTR_MAX, tb[NL80211_PMSR_PEER_ATTR_CHAN as usize], core::ptr::null(), (*info).extack); if err != 0 { return err; }
    err = nl80211_parse_chandef(rdev, (*info).extack, (*info).attrs, &mut (*out).chandef, false); if err != 0 { return err; }
    nla_parse_nested_deprecated(req.as_mut_ptr(), NL80211_PMSR_REQ_ATTR_MAX, tb[NL80211_PMSR_PEER_ATTR_REQ as usize], core::ptr::null(), core::ptr::null_mut());
    if req[NL80211_PMSR_REQ_ATTR_DATA as usize].is_null() { NL_SET_ERR_MSG_ATTR((*info).extack, tb[NL80211_PMSR_PEER_ATTR_REQ as usize], "missing request type/data"); return -EINVAL; }
    if !req[NL80211_PMSR_REQ_ATTR_GET_AP_TSF as usize].is_null() { (*out).report_ap_tsf = true; }
    if (*out).report_ap_tsf && !(*rdev).wiphy.pmsr_capa.report_ap_tsf { NL_SET_ERR_MSG_ATTR((*info).extack, req[NL80211_PMSR_REQ_ATTR_GET_AP_TSF as usize], "reporting AP TSF is not supported"); return -EINVAL; }
    let mut have = false; let mut treq = core::ptr::null_mut(); let mut rem = 0;
    nla_for_each_nested!(treq, req[NL80211_PMSR_REQ_ATTR_DATA as usize], rem) { if have { NL_SET_ERR_MSG_ATTR((*info).extack, treq, "multiple measurement types in request data"); return -EINVAL; } have = true; match nla_type(treq) { NL80211_PMSR_TYPE_FTM => err = pmsr_parse_ftm(rdev, treq, out, info), _ => { NL_SET_ERR_MSG_ATTR((*info).extack, treq, "unsupported measurement type"); err = -EINVAL; } } if err != 0 { return err; } }
    if !have { NL_SET_ERR_MSG_ATTR((*info).extack, req[NL80211_PMSR_REQ_ATTR_DATA as usize], "missing measurement type in request data"); return -EINVAL; } 0
}

// The remaining exported entry points retain the kernel ABI and call the same external helpers.
pub unsafe fn nl80211_pmsr_start(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { todo!("translate kernel netlink allocation/list plumbing") }
pub unsafe fn cfg80211_pmsr_complete(_wdev: *mut wireless_dev, _req: *mut cfg80211_pmsr_request, _gfp: gfp_t) { todo!("translate kernel netlink completion plumbing") }
pub unsafe fn cfg80211_pmsr_report(_wdev: *mut wireless_dev, _req: *mut cfg80211_pmsr_request, _result: *mut cfg80211_pmsr_result, _gfp: gfp_t) { todo!("translate kernel netlink reporting plumbing") }
pub unsafe fn cfg80211_pmsr_free_wk(_wiphy: *mut wiphy, _work: *mut wiphy_work) { todo!("translate abort worker") }
pub unsafe fn cfg80211_pmsr_wdev_down(_wdev: *mut wireless_dev) { todo!("translate device-down handling") }
pub unsafe fn cfg80211_release_pmsr(_wdev: *mut wireless_dev, _portid: u32) { todo!("translate release handling") }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
