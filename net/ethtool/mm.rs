// SPDX-License-Identifier: GPL-2.0-only
/* Copyright 2022-2025 NXP; Copyright 2024 Furong Xu <0x1207@gmail.com> */

// Dependencies supplied by the surrounding kernel/netlink translation.

#[repr(C)]
pub struct mm_req_info { pub base: ethnl_req_info }

#[repr(C)]
pub struct mm_reply_data {
    pub base: ethnl_reply_data,
    pub state: ethtool_mm_state,
    pub stats: ethtool_mm_stats,
}

// ETHTOOL_MM_STAT_CNT = __ETHTOOL_A_MM_STAT_CNT - (ETHTOOL_A_MM_STAT_PAD + 1)

pub static mut ethnl_mm_get_policy: [nla_policy; ETHTOOL_A_MM_HEADER as usize + 1] = [nla_policy::default(); ETHTOOL_A_MM_HEADER as usize + 1];

unsafe fn mm_prepare_data(req_base: *const ethnl_req_info, reply_base: *mut ethnl_reply_data, _info: *const genl_info) -> i32 {
    let data = reply_base as *mut mm_reply_data;
    let dev = (*reply_base).dev;
    let ops = (*dev).ethtool_ops;
    if (*ops).get_mm.is_none() { return -EOPNOTSUPP; }
    ethtool_stats_init((&mut (*data).stats as *mut _ as *mut u64), core::mem::size_of::<ethtool_mm_stats>() / core::mem::size_of::<u64>());
    let mut ret = ethnl_ops_begin(dev);
    if ret < 0 { return ret; }
    ret = ((*ops).get_mm.unwrap())(dev, &mut (*data).state);
    if ret != 0 { ethnl_ops_complete(dev); return ret; }
    if (*ops).get_mm_stats.is_some() && ((*req_base).flags & ETHTOOL_FLAG_STATS) != 0 {
        ((*ops).get_mm_stats.unwrap())(dev, &mut (*data).stats);
    }
    ethnl_ops_complete(dev); ret
}

unsafe fn mm_reply_size(req_base: *const ethnl_req_info, _reply_base: *const ethnl_reply_data) -> i32 {
    let mut len = 0;
    len += nla_total_size(core::mem::size_of::<u8>() as i32); // PMAC enabled
    len += nla_total_size(core::mem::size_of::<u8>() as i32); // TX enabled
    len += nla_total_size(core::mem::size_of::<u8>() as i32); // TX active
    len += nla_total_size(core::mem::size_of::<u8>() as i32); // verify enabled
    len += nla_total_size(core::mem::size_of::<u8>() as i32); // verify status
    len += nla_total_size(core::mem::size_of::<u32>() as i32) * 4;
    if ((*req_base).flags & ETHTOOL_FLAG_STATS) != 0 {
        len += nla_total_size(0) + nla_total_size_64bit(core::mem::size_of::<u64>() as i32) * (ETHTOOL_MM_STAT_CNT as i32);
    }
    len
}

unsafe fn mm_put_stat(skb: *mut sk_buff, val: u64, attrtype: u16) -> i32 {
    if val == ETHTOOL_STAT_NOT_SET { return 0; }
    if nla_put_u64_64bit(skb, attrtype, val, ETHTOOL_A_MM_STAT_PAD) != 0 { return -EMSGSIZE; }
    0
}

unsafe fn mm_put_stats(skb: *mut sk_buff, stats: *const ethtool_mm_stats) -> i32 {
    let nest = nla_nest_start(skb, ETHTOOL_A_MM_STATS);
    if nest.is_null() { return -EMSGSIZE; }
    let bad = mm_put_stat(skb, (*stats).MACMergeFrameAssErrorCount, ETHTOOL_A_MM_STAT_REASSEMBLY_ERRORS) != 0
        || mm_put_stat(skb, (*stats).MACMergeFrameSmdErrorCount, ETHTOOL_A_MM_STAT_SMD_ERRORS) != 0
        || mm_put_stat(skb, (*stats).MACMergeFrameAssOkCount, ETHTOOL_A_MM_STAT_REASSEMBLY_OK) != 0
        || mm_put_stat(skb, (*stats).MACMergeFragCountRx, ETHTOOL_A_MM_STAT_RX_FRAG_COUNT) != 0
        || mm_put_stat(skb, (*stats).MACMergeFragCountTx, ETHTOOL_A_MM_STAT_TX_FRAG_COUNT) != 0
        || mm_put_stat(skb, (*stats).MACMergeHoldCount, ETHTOOL_A_MM_STAT_HOLD_COUNT) != 0;
    if bad { nla_nest_cancel(skb, nest); return -EMSGSIZE; }
    nla_nest_end(skb, nest); 0
}

unsafe fn mm_fill_reply(skb: *mut sk_buff, req_base: *const ethnl_req_info, reply_base: *const ethnl_reply_data) -> i32 {
    let data = reply_base as *const mm_reply_data;
    let s = &(*data).state;
    if nla_put_u8(skb, ETHTOOL_A_MM_TX_ENABLED, s.tx_enabled) != 0 || nla_put_u8(skb, ETHTOOL_A_MM_TX_ACTIVE, s.tx_active) != 0
        || nla_put_u8(skb, ETHTOOL_A_MM_PMAC_ENABLED, s.pmac_enabled) != 0 || nla_put_u8(skb, ETHTOOL_A_MM_VERIFY_ENABLED, s.verify_enabled) != 0
        || nla_put_u8(skb, ETHTOOL_A_MM_VERIFY_STATUS, s.verify_status) != 0 || nla_put_u32(skb, ETHTOOL_A_MM_VERIFY_TIME, s.verify_time) != 0
        || nla_put_u32(skb, ETHTOOL_A_MM_MAX_VERIFY_TIME, s.max_verify_time) != 0 || nla_put_u32(skb, ETHTOOL_A_MM_TX_MIN_FRAG_SIZE, s.tx_min_frag_size) != 0
        || nla_put_u32(skb, ETHTOOL_A_MM_RX_MIN_FRAG_SIZE, s.rx_min_frag_size) != 0 { return -EMSGSIZE; }
    if ((*req_base).flags & ETHTOOL_FLAG_STATS) != 0 && mm_put_stats(skb, &(*data).stats) != 0 { return -EMSGSIZE; }
    0
}

pub static mut ethnl_mm_set_policy: [nla_policy; ETHTOOL_A_MM_MAX as usize + 1] = [nla_policy::default(); ETHTOOL_A_MM_MAX as usize + 1];

unsafe fn mm_state_to_cfg(state: *const ethtool_mm_state, cfg: *mut ethtool_mm_cfg) {
    (*cfg).verify_enabled = (*state).verify_enabled; (*cfg).verify_time = (*state).verify_time;
    (*cfg).tx_enabled = (*state).tx_enabled; (*cfg).pmac_enabled = (*state).pmac_enabled;
    (*cfg).tx_min_frag_size = (*state).tx_min_frag_size;
}

unsafe fn ethnl_set_mm_validate(req: *mut ethnl_req_info, _info: *mut genl_info) -> i32 {
    let ops = (*(*req).dev).ethtool_ops;
    if (*ops).get_mm.is_some() && (*ops).set_mm.is_some() { 1 } else { -EOPNOTSUPP }
}

unsafe fn ethnl_set_mm(req: *mut ethnl_req_info, info: *mut genl_info) -> i32 {
    let dev = (*req).dev; let mut state = core::mem::zeroed::<ethtool_mm_state>(); let mut cfg = core::mem::zeroed::<ethtool_mm_cfg>();
    let mut modified = false; let ret = ((*(*dev).ethtool_ops).get_mm.unwrap())(dev, &mut state); if ret != 0 { return ret; }
    mm_state_to_cfg(&state, &mut cfg);
    ethnl_update_bool(&mut cfg.verify_enabled, (*info).attrs[ETHTOOL_A_MM_VERIFY_ENABLED as usize], &mut modified);
    ethnl_update_u32(&mut cfg.verify_time, (*info).attrs[ETHTOOL_A_MM_VERIFY_TIME as usize], &mut modified);
    ethnl_update_bool(&mut cfg.tx_enabled, (*info).attrs[ETHTOOL_A_MM_TX_ENABLED as usize], &mut modified);
    ethnl_update_bool(&mut cfg.pmac_enabled, (*info).attrs[ETHTOOL_A_MM_PMAC_ENABLED as usize], &mut modified);
    ethnl_update_u32(&mut cfg.tx_min_frag_size, (*info).attrs[ETHTOOL_A_MM_TX_MIN_FRAG_SIZE as usize], &mut modified);
    if !modified { return 0; }
    if cfg.verify_time > state.max_verify_time { NL_SET_ERR_MSG_ATTR((*info).extack, (*info).attrs[ETHTOOL_A_MM_VERIFY_TIME as usize], "verifyTime exceeds device maximum"); return -ERANGE; }
    if cfg.verify_enabled && !cfg.tx_enabled { NL_SET_ERR_MSG((*info).extack, "Verification requires TX enabled"); return -EINVAL; }
    if cfg.tx_enabled && !cfg.pmac_enabled { NL_SET_ERR_MSG((*info).extack, "TX enabled requires pMAC enabled"); return -EINVAL; }
    let ret = ((*(*dev).ethtool_ops).set_mm.unwrap())(dev, &cfg, (*info).extack); if ret < 0 { ret } else { 1 }
}

pub unsafe fn __ethtool_dev_mm_supported(dev: *mut net_device) -> bool {
    let ops = (*dev).ethtool_ops; if ops.is_null() { return false; }
    let mut state = core::mem::zeroed::<ethtool_mm_state>(); (*ops).get_mm.is_some() && ((*ops).get_mm.unwrap())(dev, &mut state) == 0
}

pub unsafe fn ethtool_dev_mm_supported(dev: *mut net_device) -> bool {
    ASSERT_RTNL!(); let ops = (*dev).ethtool_ops; if ops.is_null() { return false; }
    let ret = ethnl_ops_begin(dev); if ret < 0 { return false; }
    let supported = __ethtool_dev_mm_supported(dev); ethnl_ops_complete(dev); supported
}

unsafe fn ethtool_mmsv_configure_tx(m: *mut ethtool_mmsv, active: bool) { if (*(*m).ops).configure_tx.is_some() { ((*(*m).ops).configure_tx.unwrap())(m, active); } }
unsafe fn ethtool_mmsv_configure_pmac(m: *mut ethtool_mmsv, enabled: bool) { if (*(*m).ops).configure_pmac.is_some() { ((*(*m).ops).configure_pmac.unwrap())(m, enabled); } }
unsafe fn ethtool_mmsv_send_mpacket(m: *mut ethtool_mmsv, p: ethtool_mpacket) { if (*(*m).ops).send_mpacket.is_some() { ((*(*m).ops).send_mpacket.unwrap())(m, p); } }

// Timer callback and the remaining exported software-verification helpers retain
// the C control flow and synchronization semantics.
unsafe fn ethtool_mmsv_verify_timer(t: *mut timer_list) {
    let m = timer_container_of!(t, ethtool_mmsv, verify_timer); let mut flags = 0; let mut rearm = false;
    spin_lock_irqsave!(&mut (*m).lock, flags);
    match (*m).status {
        ETHTOOL_MM_VERIFY_STATUS_INITIAL | ETHTOOL_MM_VERIFY_STATUS_VERIFYING => { if (*m).verify_retries != 0 { ethtool_mmsv_send_mpacket(m, ETHTOOL_MPACKET_VERIFY); rearm = true; } else { (*m).status = ETHTOOL_MM_VERIFY_STATUS_FAILED; } (*m).verify_retries -= 1; }
        ETHTOOL_MM_VERIFY_STATUS_SUCCEEDED => ethtool_mmsv_configure_tx(m, true), _ => {}
    }
    if rearm { mod_timer!(&mut (*m).verify_timer, jiffies + msecs_to_jiffies((*m).verify_time)); }
    spin_unlock_irqrestore!(&mut (*m).lock, flags);
}

unsafe fn ethtool_mmsv_verify_timer_arm(m: *mut ethtool_mmsv) { if (*m).pmac_enabled && (*m).tx_enabled && (*m).verify_enabled && (*m).status != ETHTOOL_MM_VERIFY_STATUS_FAILED && (*m).status != ETHTOOL_MM_VERIFY_STATUS_SUCCEEDED { timer_setup!(&mut (*m).verify_timer, ethtool_mmsv_verify_timer, 0); mod_timer!(&mut (*m).verify_timer, jiffies); } }
unsafe fn ethtool_mmsv_apply(m: *mut ethtool_mmsv) { if !(*m).verify_enabled { ethtool_mmsv_configure_pmac(m, (*m).pmac_enabled); ethtool_mmsv_configure_tx(m, (*m).tx_enabled); } else { (*m).status = ETHTOOL_MM_VERIFY_STATUS_INITIAL; (*m).verify_retries = ETHTOOL_MM_MAX_VERIFY_RETRIES; if netif_running((*m).dev) { ethtool_mmsv_verify_timer_arm(m); } } }

pub unsafe fn ethtool_mmsv_stop(m: *mut ethtool_mmsv) { timer_shutdown_sync(&mut (*m).verify_timer); }
pub unsafe fn ethtool_mmsv_link_state_handle(m: *mut ethtool_mmsv, up: bool) { let mut flags = 0; ethtool_mmsv_stop(m); spin_lock_irqsave!(&mut (*m).lock, flags); if up && (*m).pmac_enabled { ethtool_mmsv_configure_pmac(m, true); ethtool_mmsv_apply(m); } else { if (*m).verify_enabled { (*m).status = ETHTOOL_MM_VERIFY_STATUS_INITIAL; } ethtool_mmsv_configure_pmac(m, false); ethtool_mmsv_configure_tx(m, false); } spin_unlock_irqrestore!(&mut (*m).lock, flags); }
pub unsafe fn ethtool_mmsv_event_handle(m: *mut ethtool_mmsv, event: ethtool_mmsv_event) { spin_lock!(&mut (*m).lock); if !(*m).pmac_enabled { spin_unlock!(&mut (*m).lock); return; } match event { ETHTOOL_MMSV_LP_SENT_VERIFY_MPACKET => ethtool_mmsv_send_mpacket(m, ETHTOOL_MPACKET_RESPONSE), ETHTOOL_MMSV_LD_SENT_VERIFY_MPACKET => if (*m).status != ETHTOOL_MM_VERIFY_STATUS_SUCCEEDED { (*m).status = ETHTOOL_MM_VERIFY_STATUS_VERIFYING; }, ETHTOOL_MMSV_LP_SENT_RESPONSE_MPACKET => if (*m).status == ETHTOOL_MM_VERIFY_STATUS_VERIFYING { (*m).status = ETHTOOL_MM_VERIFY_STATUS_SUCCEEDED; }, } spin_unlock!(&mut (*m).lock); }
unsafe fn ethtool_mmsv_is_tx_active(m: *mut ethtool_mmsv) -> bool { (*m).tx_enabled && ((*m).status == ETHTOOL_MM_VERIFY_STATUS_SUCCEEDED || (*m).status == ETHTOOL_MM_VERIFY_STATUS_DISABLED) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
