/* SPDX-License-Identifier: GPL-2.0 */

// Declarations supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined.

#[inline]
unsafe fn drv_xmit_async(local: *mut ieee802154_local, skb: *mut sk_buff) -> i32 {
    (*(*local).ops).xmit_async(&mut (*local).hw, skb)
}

#[inline]
unsafe fn drv_xmit_sync(local: *mut ieee802154_local, skb: *mut sk_buff) -> i32 {
    might_sleep();
    (*(*local).ops).xmit_sync(&mut (*local).hw, skb)
}

#[inline]
unsafe fn drv_set_pan_id(local: *mut ieee802154_local, pan_id: __le16) -> i32 {
    might_sleep();
    if (*(*local).ops).set_hw_addr_filt.is_none() {
        WARN_ON(1);
        return -EOPNOTSUPP;
    }
    let mut filt: ieee802154_hw_addr_filt = core::mem::zeroed();
    filt.pan_id = pan_id;
    trace_802154_drv_set_pan_id(local, pan_id);
    let ret = (*(*local).ops).set_hw_addr_filt.unwrap()(&mut (*local).hw, &mut filt, IEEE802154_AFILT_PANID_CHANGED);
    trace_802154_drv_return_int(local, ret);
    ret
}

#[inline]
unsafe fn drv_set_extended_addr(local: *mut ieee802154_local, extended_addr: __le64) -> i32 {
    might_sleep();
    if (*(*local).ops).set_hw_addr_filt.is_none() { WARN_ON(1); return -EOPNOTSUPP; }
    let mut filt: ieee802154_hw_addr_filt = core::mem::zeroed();
    filt.ieee_addr = extended_addr;
    trace_802154_drv_set_extended_addr(local, extended_addr);
    let ret = (*(*local).ops).set_hw_addr_filt.unwrap()(&mut (*local).hw, &mut filt, IEEE802154_AFILT_IEEEADDR_CHANGED);
    trace_802154_drv_return_int(local, ret); ret
}

#[inline]
unsafe fn drv_set_short_addr(local: *mut ieee802154_local, short_addr: __le16) -> i32 {
    might_sleep();
    if (*(*local).ops).set_hw_addr_filt.is_none() { WARN_ON(1); return -EOPNOTSUPP; }
    let mut filt: ieee802154_hw_addr_filt = core::mem::zeroed();
    filt.short_addr = short_addr;
    trace_802154_drv_set_short_addr(local, short_addr);
    let ret = (*(*local).ops).set_hw_addr_filt.unwrap()(&mut (*local).hw, &mut filt, IEEE802154_AFILT_SADDR_CHANGED);
    trace_802154_drv_return_int(local, ret); ret
}

#[inline]
unsafe fn drv_set_pan_coord(local: *mut ieee802154_local, is_coord: bool) -> i32 {
    might_sleep();
    if (*(*local).ops).set_hw_addr_filt.is_none() { WARN_ON(1); return -EOPNOTSUPP; }
    let mut filt: ieee802154_hw_addr_filt = core::mem::zeroed();
    filt.pan_coord = is_coord;
    trace_802154_drv_set_pan_coord(local, is_coord);
    let ret = (*(*local).ops).set_hw_addr_filt.unwrap()(&mut (*local).hw, &mut filt, IEEE802154_AFILT_PANC_CHANGED);
    trace_802154_drv_return_int(local, ret); ret
}

#[inline]
unsafe fn drv_set_promiscuous_mode(local: *mut ieee802154_local, on: bool) -> i32 {
    might_sleep();
    if (*(*local).ops).set_promiscuous_mode.is_none() { WARN_ON(1); return -EOPNOTSUPP; }
    trace_802154_drv_set_promiscuous_mode(local, on);
    let ret = (*(*local).ops).set_promiscuous_mode.unwrap()(&mut (*local).hw, on);
    trace_802154_drv_return_int(local, ret); ret
}

#[inline]
unsafe fn drv_start(local: *mut ieee802154_local, level: ieee802154_filtering_level, addr_filt: *const ieee802154_hw_addr_filt) -> i32 {
    might_sleep();
    if (*local).hw.flags & IEEE802154_HW_AFILT != 0 {
        let mut ret = drv_set_pan_id(local, (*addr_filt).pan_id); if ret < 0 { return ret; }
        ret = drv_set_short_addr(local, (*addr_filt).short_addr); if ret < 0 { return ret; }
        ret = drv_set_extended_addr(local, (*addr_filt).ieee_addr); if ret < 0 { return ret; }
    }
    match level {
        IEEE802154_FILTERING_NONE | IEEE802154_FILTERING_1_FCS |
        IEEE802154_FILTERING_2_PROMISCUOUS | IEEE802154_FILTERING_3_SCAN => {
            if (*local).hw.flags & IEEE802154_HW_PROMISCUOUS != 0 {
                let ret = drv_set_promiscuous_mode(local, true); if ret < 0 { return ret; }
            } else { return -EOPNOTSUPP; }
            (*(*local).phy).filtering = IEEE802154_FILTERING_NONE;
        }
        IEEE802154_FILTERING_4_FRAME_FIELDS => {
            if (*local).hw.flags & IEEE802154_HW_PROMISCUOUS != 0 {
                let ret = drv_set_promiscuous_mode(local, false); if ret < 0 { return ret; }
            }
            (*(*local).phy).filtering = IEEE802154_FILTERING_4_FRAME_FIELDS;
        }
        _ => { WARN_ON(1); return -EINVAL; }
    }
    trace_802154_drv_start(local);
    (*local).started = true;
    smp_mb();
    let ret = (*(*local).ops).start(&mut (*local).hw);
    trace_802154_drv_return_int(local, ret); ret
}

#[inline]
unsafe fn drv_stop(local: *mut ieee802154_local) {
    might_sleep();
    trace_802154_drv_stop(local);
    (*(*local).ops).stop(&mut (*local).hw);
    trace_802154_drv_return_void(local);
    tasklet_disable(&mut (*local).tasklet);
    tasklet_enable(&mut (*local).tasklet);
    barrier();
    (*local).started = false;
}

#[inline]
unsafe fn drv_set_channel(local: *mut ieee802154_local, page: u8, channel: u8) -> i32 {
    might_sleep();
    trace_802154_drv_set_channel(local, page, channel);
    let ret = (*(*local).ops).set_channel(&mut (*local).hw, page, channel);
    trace_802154_drv_return_int(local, ret); ret
}

#[inline]
unsafe fn drv_set_tx_power(local: *mut ieee802154_local, mbm: i32) -> i32 {
    might_sleep();
    if (*(*local).ops).set_txpower.is_none() { WARN_ON(1); return -EOPNOTSUPP; }
    trace_802154_drv_set_tx_power(local, mbm);
    let ret = (*(*local).ops).set_txpower.unwrap()(&mut (*local).hw, mbm);
    trace_802154_drv_return_int(local, ret); ret
}

#[inline]
unsafe fn drv_set_cca_mode(local: *mut ieee802154_local, cca: *const wpan_phy_cca) -> i32 {
    might_sleep();
    if (*(*local).ops).set_cca_mode.is_none() { WARN_ON(1); return -EOPNOTSUPP; }
    trace_802154_drv_set_cca_mode(local, cca);
    let ret = (*(*local).ops).set_cca_mode.unwrap()(&mut (*local).hw, cca);
    trace_802154_drv_return_int(local, ret); ret
}

#[inline]
unsafe fn drv_set_lbt_mode(local: *mut ieee802154_local, mode: bool) -> i32 {
    might_sleep();
    if (*(*local).ops).set_lbt.is_none() { WARN_ON(1); return -EOPNOTSUPP; }
    trace_802154_drv_set_lbt_mode(local, mode);
    let ret = (*(*local).ops).set_lbt.unwrap()(&mut (*local).hw, mode);
    trace_802154_drv_return_int(local, ret); ret
}

#[inline]
unsafe fn drv_set_cca_ed_level(local: *mut ieee802154_local, mbm: i32) -> i32 {
    might_sleep();
    if (*(*local).ops).set_cca_ed_level.is_none() { WARN_ON(1); return -EOPNOTSUPP; }
    trace_802154_drv_set_cca_ed_level(local, mbm);
    let ret = (*(*local).ops).set_cca_ed_level.unwrap()(&mut (*local).hw, mbm);
    trace_802154_drv_return_int(local, ret); ret
}

#[inline]
unsafe fn drv_set_csma_params(local: *mut ieee802154_local, min_be: u8, max_be: u8, max_csma_backoffs: u8) -> i32 {
    might_sleep();
    if (*(*local).ops).set_csma_params.is_none() { WARN_ON(1); return -EOPNOTSUPP; }
    trace_802154_drv_set_csma_params(local, min_be, max_be, max_csma_backoffs);
    let ret = (*(*local).ops).set_csma_params.unwrap()(&mut (*local).hw, min_be, max_be, max_csma_backoffs);
    trace_802154_drv_return_int(local, ret); ret
}

#[inline]
unsafe fn drv_set_max_frame_retries(local: *mut ieee802154_local, max_frame_retries: i8) -> i32 {
    might_sleep();
    if (*(*local).ops).set_frame_retries.is_none() { WARN_ON(1); return -EOPNOTSUPP; }
    trace_802154_drv_set_max_frame_retries(local, max_frame_retries);
    let ret = (*(*local).ops).set_frame_retries.unwrap()(&mut (*local).hw, max_frame_retries);
    trace_802154_drv_return_int(local, ret); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
