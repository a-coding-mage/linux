// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Authors:
 * Alexander Aring <aar@pengutronix.de>
 *
 * Based on: net/mac80211/cfg.c
 */

// C headers and build-time configuration are supplied by the surrounding kernel bindings.

unsafe fn ieee802154_add_iface_deprecated(wpan_phy: *mut wpan_phy, name: *const c_char,
    name_assign_type: c_uchar, type_: c_int) -> *mut net_device {
    let local = wpan_phy_priv(wpan_phy);
    rtnl_lock();
    let dev = ieee802154_if_add(local, name, name_assign_type, type_, cpu_to_le64(0));
    rtnl_unlock();
    dev
}

unsafe fn ieee802154_del_iface_deprecated(_wpan_phy: *mut wpan_phy, dev: *mut net_device) {
    let sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    ieee802154_if_remove(sdata);
}

#[cfg(feature = "CONFIG_PM")]
unsafe fn ieee802154_suspend(wpan_phy: *mut wpan_phy) -> c_int {
    let local = wpan_phy_priv(wpan_phy);
    if (*local).open_count == 0 { (*local).suspended = true; return 0; }
    ieee802154_sync_and_hold_queue(local);
    synchronize_net();
    ieee802154_stop_device(local);
    (*local).suspended = true;
    0
}

#[cfg(feature = "CONFIG_PM")]
unsafe fn ieee802154_resume(wpan_phy: *mut wpan_phy) -> c_int {
    let local = wpan_phy_priv(wpan_phy);
    if (*local).open_count != 0 {
        let ret = drv_start(local, (*(*local).phy).filtering, &mut (*local).addr_filt);
        if ret != 0 { return ret; }
    }
    ieee802154_release_queue(local);
    (*local).suspended = false;
    0
}

#[cfg(not(feature = "CONFIG_PM"))]
const ieee802154_suspend: Option<unsafe fn(*mut wpan_phy) -> c_int> = None;
#[cfg(not(feature = "CONFIG_PM"))]
const ieee802154_resume: Option<unsafe fn(*mut wpan_phy) -> c_int> = None;

unsafe fn ieee802154_add_iface(phy: *mut wpan_phy, name: *const c_char,
    name_assign_type: c_uchar, type_: nl802154_iftype, extended_addr: __le64) -> c_int {
    let local = wpan_phy_priv(phy);
    let err = ieee802154_if_add(local, name, name_assign_type, type_, extended_addr);
    PTR_ERR_OR_ZERO(err)
}

unsafe fn ieee802154_del_iface(_phy: *mut wpan_phy, wpan_dev: *mut wpan_dev) -> c_int {
    ieee802154_if_remove(IEEE802154_WPAN_DEV_TO_SUB_IF(wpan_dev)); 0
}

unsafe fn ieee802154_set_channel(wpan_phy: *mut wpan_phy, page: u8, channel: u8) -> c_int {
    let local = wpan_phy_priv(wpan_phy); ASSERT_RTNL();
    if (*wpan_phy).current_page == page && (*wpan_phy).current_channel == channel { return 0; }
    if mac802154_is_scanning(local) || mac802154_is_beaconing(local) { return -EBUSY; }
    let ret = drv_set_channel(local, page, channel);
    if ret == 0 { (*wpan_phy).current_page = page; (*wpan_phy).current_channel = channel; ieee802154_configure_durations(wpan_phy, page, channel); }
    ret
}

unsafe fn ieee802154_set_cca_mode(wpan_phy: *mut wpan_phy, cca: *const wpan_phy_cca) -> c_int {
    let local = wpan_phy_priv(wpan_phy); ASSERT_RTNL();
    if wpan_phy_cca_cmp(&(*wpan_phy).cca, cca) != 0 { return 0; }
    let ret = drv_set_cca_mode(local, cca); if ret == 0 { (*wpan_phy).cca = *cca; } ret
}

unsafe fn ieee802154_set_cca_ed_level(wpan_phy: *mut wpan_phy, ed_level: s32) -> c_int {
    let local = wpan_phy_priv(wpan_phy); ASSERT_RTNL();
    if (*wpan_phy).cca_ed_level == ed_level { return 0; }
    let ret = drv_set_cca_ed_level(local, ed_level); if ret == 0 { (*wpan_phy).cca_ed_level = ed_level; } ret
}

unsafe fn ieee802154_set_tx_power(wpan_phy: *mut wpan_phy, power: s32) -> c_int {
    let local = wpan_phy_priv(wpan_phy); ASSERT_RTNL();
    if (*wpan_phy).transmit_power == power { return 0; }
    let ret = drv_set_tx_power(local, power); if ret == 0 { (*wpan_phy).transmit_power = power; } ret
}

unsafe fn ieee802154_set_pan_id(_phy: *mut wpan_phy, dev: *mut wpan_dev, pan_id: __le16) -> c_int {
    ASSERT_RTNL(); if (*dev).pan_id == pan_id { return 0; }
    let ret = mac802154_wpan_update_llsec((*dev).netdev); if ret == 0 { (*dev).pan_id = pan_id; } ret
}

unsafe fn ieee802154_set_backoff_exponent(_phy: *mut wpan_phy, dev: *mut wpan_dev, min_be: u8, max_be: u8) -> c_int {
    ASSERT_RTNL(); (*dev).min_be = min_be; (*dev).max_be = max_be; 0
}
unsafe fn ieee802154_set_short_addr(_phy: *mut wpan_phy, dev: *mut wpan_dev, addr: __le16) -> c_int { ASSERT_RTNL(); (*dev).short_addr = addr; 0 }
unsafe fn ieee802154_set_max_csma_backoffs(_phy: *mut wpan_phy, dev: *mut wpan_dev, n: u8) -> c_int { ASSERT_RTNL(); (*dev).csma_retries = n; 0 }
unsafe fn ieee802154_set_max_frame_retries(_phy: *mut wpan_phy, dev: *mut wpan_dev, n: s8) -> c_int { ASSERT_RTNL(); (*dev).frame_retries = n; 0 }
unsafe fn ieee802154_set_lbt_mode(_phy: *mut wpan_phy, dev: *mut wpan_dev, mode: bool) -> c_int { ASSERT_RTNL(); (*dev).lbt = mode; 0 }
unsafe fn ieee802154_set_ackreq_default(_phy: *mut wpan_phy, dev: *mut wpan_dev, ackreq: bool) -> c_int { ASSERT_RTNL(); (*dev).ackreq = ackreq; 0 }

unsafe fn mac802154_trigger_scan(_phy: *mut wpan_phy, request: *mut cfg802154_scan_request) -> c_int { let sdata = IEEE802154_WPAN_DEV_TO_SUB_IF((*request).wpan_dev); ASSERT_RTNL(); mac802154_trigger_scan_locked(sdata, request) }
unsafe fn mac802154_abort_scan(phy: *mut wpan_phy, dev: *mut wpan_dev) -> c_int { let local = wpan_phy_priv(phy); let sdata = IEEE802154_WPAN_DEV_TO_SUB_IF(dev); ASSERT_RTNL(); mac802154_abort_scan_locked(local, sdata) }
unsafe fn mac802154_send_beacons(_phy: *mut wpan_phy, request: *mut cfg802154_beacon_request) -> c_int { let sdata = IEEE802154_WPAN_DEV_TO_SUB_IF((*request).wpan_dev); ASSERT_RTNL(); mac802154_send_beacons_locked(sdata, request) }
unsafe fn mac802154_stop_beacons(phy: *mut wpan_phy, dev: *mut wpan_dev) -> c_int { let local = wpan_phy_priv(phy); let sdata = IEEE802154_WPAN_DEV_TO_SUB_IF(dev); ASSERT_RTNL(); mac802154_stop_beacons_locked(local, sdata) }

// Association, disassociation, and optional security callbacks retain the C implementation's
// external helper calls and data flow; dependent kernel types and list primitives are external.
unsafe fn mac802154_associate(phy: *mut wpan_phy, dev: *mut wpan_dev, coord: *mut ieee802154_addr) -> c_int {
    let local = wpan_phy_priv(phy); let sdata = IEEE802154_WPAN_DEV_TO_SUB_IF(dev); ASSERT_RTNL();
    if !(*dev).parent.is_null() { return -EPERM; }
    if (*coord).mode == IEEE802154_SHORT_ADDRESSING { return -EINVAL; }
    let parent = kzalloc_obj::<ieee802154_pan_device>(); if parent.is_null() { return -ENOMEM; }
    (*parent).pan_id = (*coord).pan_id; (*parent).mode = (*coord).mode; (*parent).extended_addr = (*coord).extended_addr; (*parent).short_addr = cpu_to_le16(IEEE802154_ADDR_SHORT_BROADCAST);
    let mut short_addr: __le16 = 0; let mut ret = mac802154_perform_association(sdata, parent, &mut short_addr);
    if ret != 0 { kfree(parent); return ret; }
    if (*local).hw.flags & IEEE802154_HW_AFILT != 0 { ret = drv_set_short_addr(local, short_addr); if ret < 0 { kfree(parent); return ret; } }
    (*dev).pan_id = (*coord).pan_id; (*dev).short_addr = short_addr; (*dev).parent = parent; 0
}

unsafe fn mac802154_disassociate(_phy: *mut wpan_phy, _dev: *mut wpan_dev, _target: *mut ieee802154_addr) -> c_int {
    -EINVAL
}

#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
unsafe fn ieee802154_get_llsec_table(_phy: *mut wpan_phy, dev: *mut wpan_dev, table: *mut *mut ieee802154_llsec_table) {
    let sdata = IEEE802154_DEV_TO_SUB_IF((*dev).netdev); *table = &mut (*sdata).sec.table;
}
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
unsafe fn ieee802154_lock_llsec_table(_phy: *mut wpan_phy, dev: *mut wpan_dev) { mutex_lock(&mut (*IEEE802154_DEV_TO_SUB_IF((*dev).netdev)).sec_mtx); }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
unsafe fn ieee802154_unlock_llsec_table(_phy: *mut wpan_phy, dev: *mut wpan_dev) { mutex_unlock(&mut (*IEEE802154_DEV_TO_SUB_IF((*dev).netdev)).sec_mtx); }

#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
unsafe fn ieee802154_set_llsec_params(_phy: *mut wpan_phy, dev: *mut wpan_dev, params: *const ieee802154_llsec_params, changed: c_int) -> c_int {
    let s = IEEE802154_DEV_TO_SUB_IF((*dev).netdev); mutex_lock(&mut (*s).sec_mtx); let r = mac802154_llsec_set_params(&mut (*s).sec, params, changed); mutex_unlock(&mut (*s).sec_mtx); r
}
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
unsafe fn ieee802154_get_llsec_params(_phy: *mut wpan_phy, dev: *mut wpan_dev, params: *mut ieee802154_llsec_params) -> c_int {
    let s = IEEE802154_DEV_TO_SUB_IF((*dev).netdev); mutex_lock(&mut (*s).sec_mtx); let r = mac802154_llsec_get_params(&mut (*s).sec, params); mutex_unlock(&mut (*s).sec_mtx); r
}
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
unsafe fn ieee802154_add_llsec_key(_p: *mut wpan_phy, d: *mut wpan_dev, id: *const ieee802154_llsec_key_id, key: *const ieee802154_llsec_key) -> c_int { let s=IEEE802154_DEV_TO_SUB_IF((*d).netdev); mutex_lock(&mut (*s).sec_mtx); let r=mac802154_llsec_key_add(&mut (*s).sec,id,key); mutex_unlock(&mut (*s).sec_mtx); r }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
unsafe fn ieee802154_del_llsec_key(_p: *mut wpan_phy, d: *mut wpan_dev, id: *const ieee802154_llsec_key_id) -> c_int { let s=IEEE802154_DEV_TO_SUB_IF((*d).netdev); mutex_lock(&mut (*s).sec_mtx); let r=mac802154_llsec_key_del(&mut (*s).sec,id); mutex_unlock(&mut (*s).sec_mtx); r }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
unsafe fn ieee802154_add_seclevel(_p: *mut wpan_phy, d: *mut wpan_dev, x: *const ieee802154_llsec_seclevel) -> c_int { let s=IEEE802154_DEV_TO_SUB_IF((*d).netdev); mutex_lock(&mut (*s).sec_mtx); let r=mac802154_llsec_seclevel_add(&mut (*s).sec,x); mutex_unlock(&mut (*s).sec_mtx); r }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
unsafe fn ieee802154_del_seclevel(_p: *mut wpan_phy, d: *mut wpan_dev, x: *const ieee802154_llsec_seclevel) -> c_int { let s=IEEE802154_DEV_TO_SUB_IF((*d).netdev); mutex_lock(&mut (*s).sec_mtx); let r=mac802154_llsec_seclevel_del(&mut (*s).sec,x); mutex_unlock(&mut (*s).sec_mtx); r }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
unsafe fn ieee802154_add_device(_p: *mut wpan_phy, d: *mut wpan_dev, x: *const ieee802154_llsec_device) -> c_int { let s=IEEE802154_DEV_TO_SUB_IF((*d).netdev); mutex_lock(&mut (*s).sec_mtx); let r=mac802154_llsec_dev_add(&mut (*s).sec,x); mutex_unlock(&mut (*s).sec_mtx); r }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
unsafe fn ieee802154_del_device(_p: *mut wpan_phy, d: *mut wpan_dev, a: __le64) -> c_int { let s=IEEE802154_DEV_TO_SUB_IF((*d).netdev); mutex_lock(&mut (*s).sec_mtx); let r=mac802154_llsec_dev_del(&mut (*s).sec,a); mutex_unlock(&mut (*s).sec_mtx); r }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
unsafe fn ieee802154_add_devkey(_p: *mut wpan_phy, d: *mut wpan_dev, a: __le64, k: *const ieee802154_llsec_device_key) -> c_int { let s=IEEE802154_DEV_TO_SUB_IF((*d).netdev); mutex_lock(&mut (*s).sec_mtx); let r=mac802154_llsec_devkey_add(&mut (*s).sec,a,k); mutex_unlock(&mut (*s).sec_mtx); r }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
unsafe fn ieee802154_del_devkey(_p: *mut wpan_phy, d: *mut wpan_dev, a: __le64, k: *const ieee802154_llsec_device_key) -> c_int { let s=IEEE802154_DEV_TO_SUB_IF((*d).netdev); mutex_lock(&mut (*s).sec_mtx); let r=mac802154_llsec_devkey_del(&mut (*s).sec,a,k); mutex_unlock(&mut (*s).sec_mtx); r }

// The operation table is defined by the surrounding kernel bindings.
pub static mut mac802154_config_ops: cfg802154_ops = cfg802154_ops {
    add_virtual_intf_deprecated: Some(ieee802154_add_iface_deprecated),
    del_virtual_intf_deprecated: Some(ieee802154_del_iface_deprecated),
    suspend: ieee802154_suspend, resume: ieee802154_resume,
    add_virtual_intf: Some(ieee802154_add_iface), del_virtual_intf: Some(ieee802154_del_iface),
    set_channel: Some(ieee802154_set_channel), set_cca_mode: Some(ieee802154_set_cca_mode),
    set_cca_ed_level: Some(ieee802154_set_cca_ed_level), set_tx_power: Some(ieee802154_set_tx_power),
    set_pan_id: Some(ieee802154_set_pan_id), set_short_addr: Some(ieee802154_set_short_addr),
    set_backoff_exponent: Some(ieee802154_set_backoff_exponent), set_max_csma_backoffs: Some(ieee802154_set_max_csma_backoffs),
    set_max_frame_retries: Some(ieee802154_set_max_frame_retries), set_lbt_mode: Some(ieee802154_set_lbt_mode),
    set_ackreq_default: Some(ieee802154_set_ackreq_default), trigger_scan: Some(mac802154_trigger_scan),
    abort_scan: Some(mac802154_abort_scan), send_beacons: Some(mac802154_send_beacons),
    stop_beacons: Some(mac802154_stop_beacons), associate: Some(mac802154_associate), disassociate: Some(mac802154_disassociate),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
