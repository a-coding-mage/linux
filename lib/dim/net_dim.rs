// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
/*
 * Copyright (c) 2018, Mellanox Technologies inc.  All rights reserved.
 */

// Dependencies supplied by linux/dim.h and linux/rtnetlink.h are external.

const RX_PROFILE: [[dim_cq_moder; NET_DIM_PARAMS_NUM_PROFILES]; DIM_CQ_PERIOD_NUM_MODES] = [
    [
        dim_cq_moder { usec: 1, pkts: NET_DIM_DEFAULT_RX_CQ_PKTS_FROM_EQE, ..unsafe { core::mem::zeroed() } },
        dim_cq_moder { usec: 8, pkts: NET_DIM_DEFAULT_RX_CQ_PKTS_FROM_EQE, ..unsafe { core::mem::zeroed() } },
        dim_cq_moder { usec: 64, pkts: NET_DIM_DEFAULT_RX_CQ_PKTS_FROM_EQE, ..unsafe { core::mem::zeroed() } },
        dim_cq_moder { usec: 128, pkts: NET_DIM_DEFAULT_RX_CQ_PKTS_FROM_EQE, ..unsafe { core::mem::zeroed() } },
        dim_cq_moder { usec: 256, pkts: NET_DIM_DEFAULT_RX_CQ_PKTS_FROM_EQE, ..unsafe { core::mem::zeroed() } },
    ],
    [
        dim_cq_moder { usec: 2, pkts: 256, ..unsafe { core::mem::zeroed() } },
        dim_cq_moder { usec: 8, pkts: 128, ..unsafe { core::mem::zeroed() } },
        dim_cq_moder { usec: 16, pkts: 64, ..unsafe { core::mem::zeroed() } },
        dim_cq_moder { usec: 32, pkts: 64, ..unsafe { core::mem::zeroed() } },
        dim_cq_moder { usec: 64, pkts: 64, ..unsafe { core::mem::zeroed() } },
    ],
];

const TX_PROFILE: [[dim_cq_moder; NET_DIM_PARAMS_NUM_PROFILES]; DIM_CQ_PERIOD_NUM_MODES] = [
    [
        dim_cq_moder { usec: 1, pkts: NET_DIM_DEFAULT_TX_CQ_PKTS_FROM_EQE, ..unsafe { core::mem::zeroed() } },
        dim_cq_moder { usec: 8, pkts: NET_DIM_DEFAULT_TX_CQ_PKTS_FROM_EQE, ..unsafe { core::mem::zeroed() } },
        dim_cq_moder { usec: 32, pkts: NET_DIM_DEFAULT_TX_CQ_PKTS_FROM_EQE, ..unsafe { core::mem::zeroed() } },
        dim_cq_moder { usec: 64, pkts: NET_DIM_DEFAULT_TX_CQ_PKTS_FROM_EQE, ..unsafe { core::mem::zeroed() } },
        dim_cq_moder { usec: 128, pkts: NET_DIM_DEFAULT_TX_CQ_PKTS_FROM_EQE, ..unsafe { core::mem::zeroed() } },
    ],
    [
        dim_cq_moder { usec: 5, pkts: 128, ..unsafe { core::mem::zeroed() } },
        dim_cq_moder { usec: 8, pkts: 64, ..unsafe { core::mem::zeroed() } },
        dim_cq_moder { usec: 16, pkts: 32, ..unsafe { core::mem::zeroed() } },
        dim_cq_moder { usec: 32, pkts: 32, ..unsafe { core::mem::zeroed() } },
        dim_cq_moder { usec: 64, pkts: 32, ..unsafe { core::mem::zeroed() } },
    ],
];

pub unsafe fn net_dim_get_rx_moderation(cq_period_mode: u8, ix: usize) -> dim_cq_moder {
    let mut cq_moder = RX_PROFILE[cq_period_mode as usize][ix];
    cq_moder.cq_period_mode = cq_period_mode;
    cq_moder
}

pub unsafe fn net_dim_get_def_rx_moderation(cq_period_mode: u8) -> dim_cq_moder {
    let profile_ix = if cq_period_mode == DIM_CQ_PERIOD_MODE_START_FROM_CQE { NET_DIM_DEF_PROFILE_CQE } else { NET_DIM_DEF_PROFILE_EQE };
    net_dim_get_rx_moderation(cq_period_mode, profile_ix as usize)
}

pub unsafe fn net_dim_get_tx_moderation(cq_period_mode: u8, ix: usize) -> dim_cq_moder {
    let mut cq_moder = TX_PROFILE[cq_period_mode as usize][ix];
    cq_moder.cq_period_mode = cq_period_mode;
    cq_moder
}

pub unsafe fn net_dim_get_def_tx_moderation(cq_period_mode: u8) -> dim_cq_moder {
    let profile_ix = if cq_period_mode == DIM_CQ_PERIOD_MODE_START_FROM_CQE { NET_DIM_DEF_PROFILE_CQE } else { NET_DIM_DEF_PROFILE_EQE };
    net_dim_get_tx_moderation(cq_period_mode, profile_ix as usize)
}

pub unsafe fn net_dim_init_irq_moder(dev: *mut net_device, profile_flags: u8, coal_flags: u8, rx_mode: u8, tx_mode: u8, rx_dim_work: Option<unsafe extern "C" fn(*mut work_struct)>, tx_dim_work: Option<unsafe extern "C" fn(*mut work_struct)>) -> i32 {
    let mut rxp: *mut dim_cq_moder = core::ptr::null_mut();
    let moder = kzalloc_obj::<dim_irq_moder>();
    (*dev).irq_moder = moder;
    if moder.is_null() { return -ENOMEM; }
    (*moder).coal_flags = coal_flags;
    (*moder).profile_flags = profile_flags;
    if profile_flags & DIM_PROFILE_RX != 0 {
        (*moder).rx_dim_work = rx_dim_work;
        (*moder).dim_rx_mode = rx_mode;
        rxp = kmemdup(RX_PROFILE[rx_mode as usize].as_ptr(), NET_DIM_PARAMS_NUM_PROFILES * core::mem::size_of::<dim_cq_moder>(), GFP_KERNEL);
        if rxp.is_null() { kfree(moder); return -ENOMEM; }
        rcu_assign_pointer((*moder).rx_profile, rxp);
    }
    if profile_flags & DIM_PROFILE_TX != 0 {
        (*moder).tx_dim_work = tx_dim_work;
        (*moder).dim_tx_mode = tx_mode;
        let txp = kmemdup(TX_PROFILE[tx_mode as usize].as_ptr(), NET_DIM_PARAMS_NUM_PROFILES * core::mem::size_of::<dim_cq_moder>(), GFP_KERNEL);
        if txp.is_null() { kfree(rxp); kfree(moder); return -ENOMEM; }
        rcu_assign_pointer((*moder).tx_profile, txp);
    }
    0
}

pub unsafe fn net_dim_free_irq_moder(dev: *mut net_device) {
    if (*dev).irq_moder.is_null() { return; }
    let rxp = rtnl_dereference((*(*dev).irq_moder).rx_profile);
    let txp = rtnl_dereference((*(*dev).irq_moder).tx_profile);
    rcu_assign_pointer((*(*dev).irq_moder).rx_profile, core::ptr::null_mut());
    rcu_assign_pointer((*(*dev).irq_moder).tx_profile, core::ptr::null_mut());
    kfree_rcu(rxp); kfree_rcu(txp); kfree((*dev).irq_moder);
}

pub unsafe fn net_dim_setting(dev: *mut net_device, dim: *mut dim, is_tx: bool) {
    let irq_moder = (*dev).irq_moder;
    if irq_moder.is_null() { return; }
    if is_tx { INIT_WORK(&mut (*dim).work, (*irq_moder).tx_dim_work); (*dim).mode = READ_ONCE((*irq_moder).dim_tx_mode); return; }
    INIT_WORK(&mut (*dim).work, (*irq_moder).rx_dim_work);
    (*dim).mode = READ_ONCE((*irq_moder).dim_rx_mode);
}

pub unsafe fn net_dim_work_cancel(dim: *mut dim) { cancel_work_sync(&mut (*dim).work); }

pub unsafe fn net_dim_get_rx_irq_moder(dev: *mut net_device, dim: *mut dim) -> dim_cq_moder {
    rcu_read_lock(); let profile = rcu_dereference((*(*dev).irq_moder).rx_profile); let mut res = *profile.add((*dim).profile_ix as usize); rcu_read_unlock(); res.cq_period_mode = (*dim).mode; res
}

pub unsafe fn net_dim_get_tx_irq_moder(dev: *mut net_device, dim: *mut dim) -> dim_cq_moder {
    rcu_read_lock(); let profile = rcu_dereference((*(*dev).irq_moder).tx_profile); let mut res = *profile.add((*dim).profile_ix as usize); rcu_read_unlock(); res.cq_period_mode = (*dim).mode; res
}

pub unsafe fn net_dim_set_rx_mode(dev: *mut net_device, rx_mode: u8) { WRITE_ONCE((*(*dev).irq_moder).dim_rx_mode, rx_mode); }
pub unsafe fn net_dim_set_tx_mode(dev: *mut net_device, tx_mode: u8) { WRITE_ONCE((*(*dev).irq_moder).dim_tx_mode, tx_mode); }

unsafe fn net_dim_step(dim: *mut dim) -> i32 {
    if (*dim).tired == NET_DIM_PARAMS_NUM_PROFILES * 2 { return DIM_TOO_TIRED; }
    match (*dim).tune_state {
        DIM_PARKING_ON_TOP | DIM_PARKING_TIRED => {},
        DIM_GOING_RIGHT => { if (*dim).profile_ix == NET_DIM_PARAMS_NUM_PROFILES - 1 { return DIM_ON_EDGE; } (*dim).profile_ix += 1; (*dim).steps_right += 1; },
        DIM_GOING_LEFT => { if (*dim).profile_ix == 0 { return DIM_ON_EDGE; } (*dim).profile_ix -= 1; (*dim).steps_left += 1; },
        _ => {},
    }
    (*dim).tired += 1; DIM_STEPPED
}

unsafe fn net_dim_exit_parking(dim: *mut dim) { (*dim).tune_state = if (*dim).profile_ix != 0 { DIM_GOING_LEFT } else { DIM_GOING_RIGHT }; net_dim_step(dim); }

unsafe fn net_dim_stats_compare(curr: *mut dim_stats, prev: *mut dim_stats) -> i32 {
    if (*prev).bpms == 0 { return if (*curr).bpms != 0 { DIM_STATS_BETTER } else { DIM_STATS_SAME }; }
    if IS_SIGNIFICANT_DIFF((*curr).bpms, (*prev).bpms) { return if (*curr).bpms > (*prev).bpms { DIM_STATS_BETTER } else { DIM_STATS_WORSE }; }
    if (*prev).ppms == 0 { return if (*curr).ppms != 0 { DIM_STATS_BETTER } else { DIM_STATS_SAME }; }
    if IS_SIGNIFICANT_DIFF((*curr).ppms, (*prev).ppms) { return if (*curr).ppms > (*prev).ppms { DIM_STATS_BETTER } else { DIM_STATS_WORSE }; }
    if (*prev).epms == 0 { return DIM_STATS_SAME; }
    if IS_SIGNIFICANT_DIFF((*curr).epms, (*prev).epms) { return if (*curr).epms < (*prev).epms { DIM_STATS_BETTER } else { DIM_STATS_WORSE }; }
    DIM_STATS_SAME
}

unsafe fn net_dim_decision(curr_stats: *mut dim_stats, dim: *mut dim) -> bool {
    let prev_state = (*dim).tune_state; let prev_ix = (*dim).profile_ix;
    match (*dim).tune_state {
        DIM_PARKING_ON_TOP => { if net_dim_stats_compare(curr_stats, &mut (*dim).prev_stats) != DIM_STATS_SAME { net_dim_exit_parking(dim); } },
        DIM_PARKING_TIRED => { (*dim).tired -= 1; if (*dim).tired == 0 { net_dim_exit_parking(dim); } },
        DIM_GOING_RIGHT | DIM_GOING_LEFT => { if net_dim_stats_compare(curr_stats, &mut (*dim).prev_stats) != DIM_STATS_BETTER { dim_turn(dim); } if dim_on_top(dim) { dim_park_on_top(dim); } else { match net_dim_step(dim) { DIM_ON_EDGE => dim_park_on_top(dim), DIM_TOO_TIRED => dim_park_tired(dim), _ => {} } } },
        _ => {},
    }
    if prev_state != DIM_PARKING_ON_TOP || (*dim).tune_state != DIM_PARKING_ON_TOP { (*dim).prev_stats = *curr_stats; }
    (*dim).profile_ix != prev_ix
}

pub unsafe fn net_dim(dim: *mut dim, end_sample: *const dim_sample) {
    let mut curr_stats: dim_stats = core::mem::zeroed();
    match (*dim).state {
        DIM_MEASURE_IN_PROGRESS => { let nevents = BIT_GAP(BITS_PER_TYPE::<u16>(), (*end_sample).event_ctr, (*dim).start_sample.event_ctr); if nevents < DIM_NEVENTS { return; } if !dim_calc_stats(&(*dim).start_sample, end_sample, &mut curr_stats) { return; } if net_dim_decision(&mut curr_stats, dim) { (*dim).state = DIM_APPLY_NEW_PROFILE; schedule_work(&mut (*dim).work); return; } },
        DIM_START_MEASURE => {},
        DIM_APPLY_NEW_PROFILE => return,
        _ => return,
    }
    dim_update_sample((*end_sample).event_ctr, (*end_sample).pkt_ctr, (*end_sample).byte_ctr, &mut (*dim).start_sample);
    (*dim).state = DIM_MEASURE_IN_PROGRESS;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
