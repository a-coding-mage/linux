// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of rate.c; kernel/project dependencies are supplied externally. */

#[repr(C)]
struct RateControlAlg {
    list: list_head,
    ops: *const rate_control_ops,
}

static mut rate_ctrl_algs: list_head = list_head { __dummy: 0 };
static mut rate_ctrl_mutex: mutex = mutex { __dummy: 0 };
static mut ieee80211_default_rc_algo: *mut i8 = CONFIG_MAC80211_RC_DEFAULT as *mut i8;

pub unsafe fn rate_control_rate_init(link_sta: *mut link_sta_info) {
    let sta = (*link_sta).sta;
    let local = (*(*sta).sdata).local;
    let ref_ = (*sta).rate_ctrl;
    let ista = &mut (*sta).sta as *mut ieee80211_sta;
    let priv_sta = (*sta).rate_ctrl_priv;
    let mut sband: *mut ieee80211_supported_band;
    let chanctx_conf: *mut ieee80211_chanctx_conf;
    if ref_.is_null() { return; }
    if WARN_ON(ieee80211_vif_is_mld(&(*(*sta).sdata).vif)) { return; }
    rcu_read_lock();
    chanctx_conf = rcu_dereference((*(*sta).sdata).vif.bss_conf.chanctx_conf);
    if WARN_ON(chanctx_conf.is_null()) { rcu_read_unlock(); return; }
    sband = (*(*(*local).hw.wiphy).bands)[(*chanctx_conf).def.chan.band as usize];
    if (*sband).band == NL80211_BAND_S1GHZ { ieee80211_s1g_sta_rate_init(sta); rcu_read_unlock(); return; }
    spin_lock_bh(&mut (*sta).rate_ctrl_lock);
    ((*(*ref_).ops).rate_init)(*ref_ .priv, sband, &(*chanctx_conf).def, ista, priv_sta);
    spin_unlock_bh(&mut (*sta).rate_ctrl_lock);
    rcu_read_unlock();
    set_sta_flag(sta, WLAN_STA_RATE_CONTROL);
}

pub unsafe fn rate_control_rate_init_all_links(sta: *mut sta_info) {
    for link_id in 0..ARRAY_SIZE((*sta).link) {
        let link_sta = sdata_dereference((*sta).link[link_id], (*sta).sdata);
        if !link_sta.is_null() { rate_control_rate_init(link_sta); }
    }
}

pub unsafe fn rate_control_tx_status(local: *mut ieee80211_local, st: *mut ieee80211_tx_status) {
    let ref_ = (*local).rate_ctrl;
    let sta = container_of((*st).sta, sta_info, sta);
    let priv_sta = (*sta).rate_ctrl_priv;
    if ref_.is_null() || !test_sta_flag(sta, WLAN_STA_RATE_CONTROL) || (*st).info.band >= NUM_NL80211_BANDS { return; }
    let sband = (*(*(*local).hw.wiphy).bands)[(*st).info.band as usize];
    spin_lock_bh(&mut (*sta).rate_ctrl_lock);
    if !(*(*ref_).ops).tx_status_ext.is_none() { ((*(*ref_).ops).tx_status_ext.unwrap())((*ref_).priv, sband, priv_sta, st); }
    else if !(*st).skb.is_null() { ((*(*ref_).ops).tx_status)((*ref_).priv, sband, (*st).sta, priv_sta, (*st).skb); }
    else { WARN_ON_ONCE(true); }
    spin_unlock_bh(&mut (*sta).rate_ctrl_lock);
}

pub unsafe fn rate_control_rate_update(local: *mut ieee80211_local, sband: *mut ieee80211_supported_band, link_sta: *mut link_sta_info, changed: u32) {
    let ref_ = (*local).rate_ctrl; let sta = (*link_sta).sta; let ista = &mut (*sta).sta as *mut ieee80211_sta; let priv_sta = (*sta).rate_ctrl_priv;
    if !ref_.is_null() && !(*(*ref_).ops).rate_update.is_none() {
        rcu_read_lock(); let c = rcu_dereference((*(*sta).sdata).vif.bss_conf.chanctx_conf);
        if WARN_ON(c.is_null()) { rcu_read_unlock(); return; }
        spin_lock_bh(&mut (*sta).rate_ctrl_lock); ((*(*ref_).ops).rate_update.unwrap())((*ref_).priv, sband, &(*c).def, ista, priv_sta, changed); spin_unlock_bh(&mut (*sta).rate_ctrl_lock); rcu_read_unlock();
    }
    if (*sta).uploaded { drv_link_sta_rc_update(local, (*sta).sdata, (*link_sta).pub_, changed); }
}

pub unsafe fn ieee80211_rate_control_register(ops: *const rate_control_ops) -> i32 {
    if (*ops).name.is_null() { return -EINVAL; }
    mutex_lock(&mut rate_ctrl_mutex);
    let mut alg = list_first_entry_or_null(&mut rate_ctrl_algs, rate_control_alg, list);
    while !alg.is_null() { if !strcmp((*(*alg).ops).name, (*ops).name) { WARN_ON(true); mutex_unlock(&mut rate_ctrl_mutex); return -EALREADY; } alg = list_next_entry_or_null(alg, &rate_ctrl_algs, list); }
    let alg = kzalloc_obj::<rate_control_alg>(); if alg.is_null() { mutex_unlock(&mut rate_ctrl_mutex); return -ENOMEM; }
    (*alg).ops = ops; list_add_tail(&mut (*alg).list, &mut rate_ctrl_algs); mutex_unlock(&mut rate_ctrl_mutex); 0
}

pub unsafe fn ieee80211_rate_control_unregister(ops: *const rate_control_ops) {
    mutex_lock(&mut rate_ctrl_mutex); let mut alg = list_first_entry_or_null(&mut rate_ctrl_algs, rate_control_alg, list);
    while !alg.is_null() { let next = list_next_entry_or_null(alg, &rate_ctrl_algs, list); if (*alg).ops == ops { list_del(&mut (*alg).list); kfree(alg as *mut _); break; } alg = next; } mutex_unlock(&mut rate_ctrl_mutex);
}

unsafe fn ieee80211_try_rate_control_ops_get(name: *const i8) -> *const rate_control_ops {
    if name.is_null() { return core::ptr::null(); } mutex_lock(&mut rate_ctrl_mutex); let mut alg = list_first_entry_or_null(&mut rate_ctrl_algs, rate_control_alg, list); let mut ops = core::ptr::null(); while !alg.is_null() { if !strcmp((*(*alg).ops).name, name) { ops = (*alg).ops; break; } alg = list_next_entry_or_null(alg, &rate_ctrl_algs, list); } mutex_unlock(&mut rate_ctrl_mutex); ops
}

unsafe fn ieee80211_rate_control_ops_get(name: *const i8) -> *const rate_control_ops {
    kernel_param_lock(THIS_MODULE); let alg_name = if name.is_null() { ieee80211_default_rc_algo } else { name }; let mut ops = ieee80211_try_rate_control_ops_get(alg_name); if ops.is_null() && !name.is_null() { ops = ieee80211_try_rate_control_ops_get(ieee80211_default_rc_algo); } if ops.is_null() && strlen(CONFIG_MAC80211_RC_DEFAULT) > 0 { ops = ieee80211_try_rate_control_ops_get(CONFIG_MAC80211_RC_DEFAULT); } kernel_param_unlock(THIS_MODULE); ops
}

unsafe fn rate_control_alloc(name: *const i8, local: *mut ieee80211_local) -> *mut rate_control_ref {
    let r = kmalloc_obj::<rate_control_ref>(); if r.is_null() { return core::ptr::null_mut(); } (*r).ops = ieee80211_rate_control_ops_get(name); if (*r).ops.is_null() { kfree(r); return core::ptr::null_mut(); } (*r).priv_ = ((*(*r).ops).alloc)(&mut (*local).hw); if (*r).priv_.is_null() { kfree(r); return core::ptr::null_mut(); } r
}

unsafe fn rate_control_free(local: *mut ieee80211_local, ctrl_ref: *mut rate_control_ref) { ((*(*ctrl_ref).ops).free)((*ctrl_ref).priv_); #[cfg(CONFIG_MAC80211_DEBUGFS)] { debugfs_remove_recursive((*local).debugfs.rcdir); (*local).debugfs.rcdir = core::ptr::null_mut(); } kfree(ctrl_ref); }

// The following declarations retain the remaining file-local interfaces; their
// implementations are supplied by the corresponding translated rate-control unit.
unsafe extern "C" {
    fn ieee80211_check_rate_mask(link: *mut ieee80211_link_data);
    fn ieee80211_get_tx_rates(vif: *mut ieee80211_vif, sta: *mut ieee80211_sta,
                              skb: *mut sk_buff, dest: *mut ieee80211_tx_rate,
                              max_rates: i32);
    fn rate_control_get_rate(sdata: *mut ieee80211_sub_if_data,
                             sta: *mut sta_info,
                             txrc: *mut ieee80211_tx_rate_control);
    fn rate_control_set_rates(hw: *mut ieee80211_hw, pubsta: *mut ieee80211_sta,
                              rates: *mut ieee80211_sta_rates) -> i32;
}

pub unsafe fn ieee80211_init_rate_ctrl_alg(local: *mut ieee80211_local, name: *const i8) -> i32 {
    ASSERT_RTNL();
    if (*local).open_count != 0 { return -EBUSY; }
    if ieee80211_hw_check(&mut (*local).hw, HAS_RATE_CONTROL) {
        if WARN_ON((*local).ops.set_rts_threshold.is_none()) { return -EINVAL; }
        return 0;
    }
    let r = rate_control_alloc(name, local);
    if r.is_null() { wiphy_warn((*local).hw.wiphy, "Failed to select rate control algorithm\n"); return -ENOENT; }
    WARN_ON(!(*local).rate_ctrl.is_null()); (*local).rate_ctrl = r;
    wiphy_debug((*local).hw.wiphy, "Selected rate control algorithm '%s'\n", (*(*r).ops).name);
    0
}

pub unsafe fn rate_control_deinitialize(local: *mut ieee80211_local) {
    let r = (*local).rate_ctrl; if r.is_null() { return; } (*local).rate_ctrl = core::ptr::null_mut(); rate_control_free(local, r);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
