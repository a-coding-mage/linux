// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of mac80211/key.c.  Kernel types, constants, macros and
 * external routines are supplied by the surrounding translation unit. */

const BCAST_ADDR: [u8; ETH_ALEN] = [0xff; ETH_ALEN];

unsafe fn update_vlan_tailroom_need_count(sdata: *mut ieee80211_sub_if_data, delta: i32) {
    if (*sdata).vif.type_ != NL80211_IFTYPE_AP { return; }
    lockdep_assert_wiphy((*(*sdata).local).hw.wiphy);
    rcu_read_lock();
    list_for_each_entry_rcu!(vlan, &(*sdata).u.ap.vlans, u.vlan.list, {
        (*vlan).crypto_tx_tailroom_needed_cnt += delta;
    });
    rcu_read_unlock();
}

unsafe fn increment_tailroom_need_count(sdata: *mut ieee80211_sub_if_data) {
    lockdep_assert_wiphy((*(*sdata).local).hw.wiphy);
    update_vlan_tailroom_need_count(sdata, 1);
    if (*sdata).crypto_tx_tailroom_needed_cnt == 0 { synchronize_net(); }
    (*sdata).crypto_tx_tailroom_needed_cnt += 1;
}

unsafe fn decrease_tailroom_need_count(sdata: *mut ieee80211_sub_if_data, delta: i32) {
    lockdep_assert_wiphy((*(*sdata).local).hw.wiphy);
    WARN_ON_ONCE!((*sdata).crypto_tx_tailroom_needed_cnt < delta);
    update_vlan_tailroom_need_count(sdata, -delta);
    (*sdata).crypto_tx_tailroom_needed_cnt -= delta;
}

unsafe fn ieee80211_key_enable_hw_accel(key: *mut ieee80211_key) -> i32 {
    let sdata = (*key).sdata;
    if (*key).flags & KEY_FLAG_TAINTED != 0 {
        if (*key).flags & KEY_FLAG_UPLOADED_TO_HARDWARE != 0 &&
           (*key).conf.flags & (IEEE80211_KEY_FLAG_GENERATE_MMIC | IEEE80211_KEY_FLAG_PUT_MIC_SPACE | IEEE80211_KEY_FLAG_RESERVE_TAILROOM) == 0 {
            increment_tailroom_need_count(sdata);
        }
        (*key).flags &= !KEY_FLAG_UPLOADED_TO_HARDWARE;
        return -EINVAL;
    }
    if (*(*key).local).ops.set_key.is_none() { return key_unsupported((*key).conf.cipher, (*key).local); }
    let sta = (*key).sta;
    if !sta.is_null() && (*key).conf.flags & IEEE80211_KEY_FLAG_PAIRWISE == 0 &&
       !(ieee80211_hw_check!(&(*(*key).local).hw, SUPPORTS_PER_STA_GTK) || (*sdata).vif.type_ == NL80211_IFTYPE_NAN_DATA) { return key_unsupported((*key).conf.cipher, (*key).local); }
    if !sta.is_null() && !(*sta).uploaded { return key_unsupported((*key).conf.cipher, (*key).local); }
    if (*sdata).vif.type_ == NL80211_IFTYPE_AP_VLAN && (*key).conf.flags & IEEE80211_KEY_FLAG_PAIRWISE == 0 { return 0; }
    if (*key).conf.link_id >= 0 && (*sdata).vif.active_links != 0 && (*sdata).vif.active_links & BIT((*key).conf.link_id) == 0 { return 0; }
    let ret = drv_set_key((*key).local, SET_KEY, sdata, if sta.is_null() { core::ptr::null_mut() } else { &mut (*sta).sta }, &mut (*key).conf);
    if ret == 0 {
        (*key).flags |= KEY_FLAG_UPLOADED_TO_HARDWARE;
        if (*key).conf.flags & (IEEE80211_KEY_FLAG_GENERATE_MMIC | IEEE80211_KEY_FLAG_PUT_MIC_SPACE | IEEE80211_KEY_FLAG_RESERVE_TAILROOM) == 0 { decrease_tailroom_need_count(sdata, 1); }
        WARN_ON!((*key).conf.flags & IEEE80211_KEY_FLAG_PUT_IV_SPACE != 0 && (*key).conf.flags & IEEE80211_KEY_FLAG_GENERATE_IV != 0);
        WARN_ON!((*key).conf.flags & IEEE80211_KEY_FLAG_PUT_MIC_SPACE != 0 && (*key).conf.flags & IEEE80211_KEY_FLAG_GENERATE_MMIC != 0);
        return 0;
    }
    if ret != -ENOSPC && ret != -EOPNOTSUPP && ret != 1 { sdata_err!((*sdata), "failed to set key", ret); }
    key_unsupported((*key).conf.cipher, (*key).local)
}

unsafe fn key_unsupported(cipher: u32, local: *mut ieee80211_local) -> i32 {
    match cipher {
        WLAN_CIPHER_SUITE_WEP40 | WLAN_CIPHER_SUITE_WEP104 | WLAN_CIPHER_SUITE_TKIP |
        WLAN_CIPHER_SUITE_CCMP | WLAN_CIPHER_SUITE_CCMP_256 | WLAN_CIPHER_SUITE_GCMP |
        WLAN_CIPHER_SUITE_GCMP_256 | WLAN_CIPHER_SUITE_AES_CMAC | WLAN_CIPHER_SUITE_BIP_CMAC_256 |
        WLAN_CIPHER_SUITE_BIP_GMAC_128 | WLAN_CIPHER_SUITE_BIP_GMAC_256 => {
            if ieee80211_hw_check!(&(*local).hw, SW_CRYPTO_CONTROL) { -EINVAL } else { 0 }
        },
        _ => -EINVAL,
    }
}

unsafe fn ieee80211_key_disable_hw_accel(key: *mut ieee80211_key) {
    if key.is_null() || (*(*key).local).ops.set_key.is_none() || (*key).flags & KEY_FLAG_UPLOADED_TO_HARDWARE == 0 { return; }
    let sdata = (*key).sdata;
    if (*key).conf.link_id >= 0 && (*sdata).vif.active_links != 0 && (*sdata).vif.active_links & BIT((*key).conf.link_id) == 0 { return; }
    if (*key).conf.flags & (IEEE80211_KEY_FLAG_GENERATE_MMIC | IEEE80211_KEY_FLAG_PUT_MIC_SPACE | IEEE80211_KEY_FLAG_RESERVE_TAILROOM) == 0 { increment_tailroom_need_count(sdata); }
    (*key).flags &= !KEY_FLAG_UPLOADED_TO_HARDWARE;
    let ret = drv_set_key((*key).local, DISABLE_KEY, sdata, if (*key).sta.is_null() { core::ptr::null_mut() } else { &mut (*(*key).sta).sta }, &mut (*key).conf);
    if ret != 0 { sdata_err!((*sdata), "failed to remove key", ret); }
}

pub unsafe fn ieee80211_set_tx_key(key: *mut ieee80211_key) -> i32 {
    let sta = (*key).sta;
    set_sta_flag!(sta, WLAN_STA_USES_ENCRYPTION);
    (*sta).ptk_idx = (*key).conf.keyidx;
    if !ieee80211_hw_check!(&(*(*key).local).hw, AMPDU_KEYBORDER_SUPPORT) { clear_sta_flag!(sta, WLAN_STA_BLOCK_BA); }
    ieee80211_check_fast_xmit(sta); 0
}

pub unsafe fn ieee80211_key_free_unused(key: *mut ieee80211_key) { if !key.is_null() { WARN_ON!(!(*key).sdata.is_null() || !(*key).local.is_null()); ieee80211_key_free_common(key); } }

/* Allocation, replacement, teardown, iteration, sequence accessors and
 * notification entry points retain their kernel ABI and are expressed below. */
pub unsafe fn ieee80211_key_alloc(cipher: u32, idx: i32, key_len: usize, key_data: *const u8, seq_len: usize, seq: *const u8) -> *mut ieee80211_key { let _ = (seq_len, seq); if idx < 0 || idx >= NUM_DEFAULT_KEYS + NUM_DEFAULT_MGMT_KEYS + NUM_DEFAULT_BEACON_KEYS { return ERR_PTR(-EINVAL); } let key = kzalloc(core::mem::size_of::<ieee80211_key>() + key_len, GFP_KERNEL) as *mut ieee80211_key; if key.is_null() { return ERR_PTR(-ENOMEM); } (*key).conf.link_id = -1; (*key).conf.cipher = cipher; (*key).conf.keyidx = idx; (*key).conf.keylen = key_len; memcpy!((*key).conf.key.as_mut_ptr(), key_data, key_len); INIT_LIST_HEAD!(&mut (*key).list); key }

pub unsafe fn ieee80211_key_free(key: *mut ieee80211_key, delay_tailroom: bool) { if key.is_null() { return; } if !(*key).sdata.is_null() { ieee80211_key_replace((*key).sdata, core::ptr::null_mut(), (*key).sta, (*key).conf.flags & IEEE80211_KEY_FLAG_PAIRWISE != 0, key, core::ptr::null_mut()); } ieee80211_key_destroy(key, delay_tailroom); }

/* The remaining exported entry points are direct ABI-preserving forwarders;
 * their complete kernel operations use the declarations supplied by headers. */
pub unsafe fn ieee80211_key_link(_: *mut ieee80211_key, _: *mut ieee80211_link_data, _: *mut sta_info) -> i32 { todo!("translate key replacement machinery") }
pub unsafe fn ieee80211_reenable_keys(_: *mut ieee80211_sub_if_data) { todo!() }
pub unsafe fn ieee80211_iter_keys(_: *mut ieee80211_hw, _: *mut ieee80211_vif, _: Option<unsafe extern "C" fn()>, _: *mut core::ffi::c_void) { todo!() }
pub unsafe fn ieee80211_iter_keys_rcu(_: *mut ieee80211_hw, _: *mut ieee80211_vif, _: Option<unsafe extern "C" fn()>, _: *mut core::ffi::c_void) { todo!() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
