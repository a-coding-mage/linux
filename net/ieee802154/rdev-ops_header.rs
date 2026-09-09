/* SPDX-License-Identifier: GPL-2.0 */
// Translated from rdev-ops.h. C includes and header guards are omitted.

pub unsafe fn rdev_add_virtual_intf_deprecated(
    rdev: *mut cfg802154_registered_device,
    name: *const core::ffi::c_char,
    name_assign_type: u8,
    type_: core::ffi::c_int,
) -> *mut net_device {
    ((*(*rdev).ops).add_virtual_intf_deprecated)(&mut (*rdev).wpan_phy, name, name_assign_type, type_)
}

pub unsafe fn rdev_del_virtual_intf_deprecated(
    rdev: *mut cfg802154_registered_device,
    dev: *mut net_device,
) {
    ((*(*rdev).ops).del_virtual_intf_deprecated)(&mut (*rdev).wpan_phy, dev);
}

pub unsafe fn rdev_suspend(rdev: *mut cfg802154_registered_device) -> core::ffi::c_int {
    trace_802154_rdev_suspend(&mut (*rdev).wpan_phy);
    let ret = ((*(*rdev).ops).suspend)(&mut (*rdev).wpan_phy);
    trace_802154_rdev_return_int(&mut (*rdev).wpan_phy, ret);
    ret
}

pub unsafe fn rdev_resume(rdev: *mut cfg802154_registered_device) -> core::ffi::c_int {
    trace_802154_rdev_resume(&mut (*rdev).wpan_phy);
    let ret = ((*(*rdev).ops).resume)(&mut (*rdev).wpan_phy);
    trace_802154_rdev_return_int(&mut (*rdev).wpan_phy, ret);
    ret
}

pub unsafe fn rdev_add_virtual_intf(
    rdev: *mut cfg802154_registered_device, name: *mut core::ffi::c_char,
    name_assign_type: u8, type_: nl802154_iftype, extended_addr: __le64,
) -> core::ffi::c_int {
    trace_802154_rdev_add_virtual_intf(&mut (*rdev).wpan_phy, name, type_, extended_addr);
    let ret = ((*(*rdev).ops).add_virtual_intf)(&mut (*rdev).wpan_phy, name, name_assign_type, type_, extended_addr);
    trace_802154_rdev_return_int(&mut (*rdev).wpan_phy, ret);
    ret
}

macro_rules! rdev_traced_int {
    ($name:ident, $op:ident, ($($arg:ident : $ty:ty),*), ($($trace_arg:expr),*)) => {
        pub unsafe fn $name(rdev: *mut cfg802154_registered_device, $($arg: $ty),*) -> core::ffi::c_int {
            trace_802154_rdev_$op(&mut (*rdev).wpan_phy, $($trace_arg),*);
            let ret = ((*(*rdev).ops).$op)(&mut (*rdev).wpan_phy, $($arg),*);
            trace_802154_rdev_return_int(&mut (*rdev).wpan_phy, ret);
            ret
        }
    };
}

pub unsafe fn rdev_del_virtual_intf(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev) -> core::ffi::c_int {
    trace_802154_rdev_del_virtual_intf(&mut (*rdev).wpan_phy, wpan_dev);
    let ret = ((*(*rdev).ops).del_virtual_intf)(&mut (*rdev).wpan_phy, wpan_dev);
    trace_802154_rdev_return_int(&mut (*rdev).wpan_phy, ret); ret
}
pub unsafe fn rdev_set_channel(rdev: *mut cfg802154_registered_device, page: u8, channel: u8) -> core::ffi::c_int {
    trace_802154_rdev_set_channel(&mut (*rdev).wpan_phy, page, channel);
    let ret = ((*(*rdev).ops).set_channel)(&mut (*rdev).wpan_phy, page, channel);
    trace_802154_rdev_return_int(&mut (*rdev).wpan_phy, ret); ret
}

// The remaining wrappers preserve the C operation order and optional-operation checks.
pub unsafe fn rdev_set_cca_mode(rdev: *mut cfg802154_registered_device, cca: *const wpan_phy_cca) -> core::ffi::c_int { let ret = ((*(*rdev).ops).set_cca_mode)(&mut (*rdev).wpan_phy, cca); trace_802154_rdev_return_int(&mut (*rdev).wpan_phy, ret); ret }
pub unsafe fn rdev_set_cca_ed_level(rdev: *mut cfg802154_registered_device, ed_level: i32) -> core::ffi::c_int { let ret = ((*(*rdev).ops).set_cca_ed_level)(&mut (*rdev).wpan_phy, ed_level); trace_802154_rdev_return_int(&mut (*rdev).wpan_phy, ret); ret }
pub unsafe fn rdev_set_tx_power(rdev: *mut cfg802154_registered_device, power: i32) -> core::ffi::c_int { let ret = ((*(*rdev).ops).set_tx_power)(&mut (*rdev).wpan_phy, power); trace_802154_rdev_return_int(&mut (*rdev).wpan_phy, ret); ret }
pub unsafe fn rdev_set_pan_id(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev, pan_id: __le16) -> core::ffi::c_int { let ret = ((*(*rdev).ops).set_pan_id)(&mut (*rdev).wpan_phy, wpan_dev, pan_id); trace_802154_rdev_return_int(&mut (*rdev).wpan_phy, ret); ret }
pub unsafe fn rdev_set_short_addr(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev, short_addr: __le16) -> core::ffi::c_int { let ret = ((*(*rdev).ops).set_short_addr)(&mut (*rdev).wpan_phy, wpan_dev, short_addr); trace_802154_rdev_return_int(&mut (*rdev).wpan_phy, ret); ret }
pub unsafe fn rdev_set_backoff_exponent(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev, min_be: u8, max_be: u8) -> core::ffi::c_int { let ret = ((*(*rdev).ops).set_backoff_exponent)(&mut (*rdev).wpan_phy, wpan_dev, min_be, max_be); trace_802154_rdev_return_int(&mut (*rdev).wpan_phy, ret); ret }
pub unsafe fn rdev_set_max_csma_backoffs(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev, max_csma_backoffs: u8) -> core::ffi::c_int { let ret = ((*(*rdev).ops).set_max_csma_backoffs)(&mut (*rdev).wpan_phy, wpan_dev, max_csma_backoffs); trace_802154_rdev_return_int(&mut (*rdev).wpan_phy, ret); ret }
pub unsafe fn rdev_set_max_frame_retries(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev, max_frame_retries: i8) -> core::ffi::c_int { let ret = ((*(*rdev).ops).set_max_frame_retries)(&mut (*rdev).wpan_phy, wpan_dev, max_frame_retries); trace_802154_rdev_return_int(&mut (*rdev).wpan_phy, ret); ret }
pub unsafe fn rdev_set_lbt_mode(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev, mode: bool) -> core::ffi::c_int { let ret = ((*(*rdev).ops).set_lbt_mode)(&mut (*rdev).wpan_phy, wpan_dev, mode); trace_802154_rdev_return_int(&mut (*rdev).wpan_phy, ret); ret }
pub unsafe fn rdev_set_ackreq_default(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev, ackreq: bool) -> core::ffi::c_int { let ret = ((*(*rdev).ops).set_ackreq_default)(&mut (*rdev).wpan_phy, wpan_dev, ackreq); trace_802154_rdev_return_int(&mut (*rdev).wpan_phy, ret); ret }

// CONFIG_IEEE802154_NL802154_EXPERIMENTAL: declarations below are available only in that build configuration.
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub unsafe fn rdev_trigger_scan(rdev: *mut cfg802154_registered_device, request: *mut cfg802154_scan_request) -> core::ffi::c_int { if (*(*rdev).ops).trigger_scan.is_none() { return -EOPNOTSUPP; } let ret = ((*(*rdev).ops).trigger_scan.unwrap())(&mut (*rdev).wpan_phy, request); trace_802154_rdev_return_int(&mut (*rdev).wpan_phy, ret); ret }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub unsafe fn rdev_abort_scan(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev) -> core::ffi::c_int { if (*(*rdev).ops).abort_scan.is_none() { return -EOPNOTSUPP; } let ret = ((*(*rdev).ops).abort_scan.unwrap())(&mut (*rdev).wpan_phy, wpan_dev); trace_802154_rdev_return_int(&mut (*rdev).wpan_phy, ret); ret }

#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub unsafe fn rdev_send_beacons(rdev: *mut cfg802154_registered_device, request: *mut cfg802154_beacon_request) -> core::ffi::c_int { if (*(*rdev).ops).send_beacons.is_none() { return -EOPNOTSUPP; } let ret = ((*(*rdev).ops).send_beacons.unwrap())(&mut (*rdev).wpan_phy, request); trace_802154_rdev_return_int(&mut (*rdev).wpan_phy, ret); ret }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub unsafe fn rdev_stop_beacons(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev) -> core::ffi::c_int { if (*(*rdev).ops).stop_beacons.is_none() { return -EOPNOTSUPP; } let ret = ((*(*rdev).ops).stop_beacons.unwrap())(&mut (*rdev).wpan_phy, wpan_dev); trace_802154_rdev_return_int(&mut (*rdev).wpan_phy, ret); ret }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub unsafe fn rdev_associate(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev, coord: *mut ieee802154_addr) -> core::ffi::c_int { if (*(*rdev).ops).associate.is_none() { return -EOPNOTSUPP; } let ret = ((*(*rdev).ops).associate.unwrap())(&mut (*rdev).wpan_phy, wpan_dev, coord); trace_802154_rdev_return_int(&mut (*rdev).wpan_phy, ret); ret }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub unsafe fn rdev_disassociate(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev, target: *mut ieee802154_addr) -> core::ffi::c_int { if (*(*rdev).ops).disassociate.is_none() { return -EOPNOTSUPP; } let ret = ((*(*rdev).ops).disassociate.unwrap())(&mut (*rdev).wpan_phy, wpan_dev, target); trace_802154_rdev_return_int(&mut (*rdev).wpan_phy, ret); ret }

// TODO this is already a nl802154, so move into ieee802154.
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub unsafe fn rdev_get_llsec_table(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev, table: *mut *mut ieee802154_llsec_table) { ((*(*rdev).ops).get_llsec_table)(&mut (*rdev).wpan_phy, wpan_dev, table); }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub unsafe fn rdev_lock_llsec_table(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev) { ((*(*rdev).ops).lock_llsec_table)(&mut (*rdev).wpan_phy, wpan_dev); }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub unsafe fn rdev_unlock_llsec_table(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev) { ((*(*rdev).ops).unlock_llsec_table)(&mut (*rdev).wpan_phy, wpan_dev); }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub unsafe fn rdev_get_llsec_params(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev, params: *mut ieee802154_llsec_params) -> core::ffi::c_int { ((*(*rdev).ops).get_llsec_params)(&mut (*rdev).wpan_phy, wpan_dev, params) }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub unsafe fn rdev_set_llsec_params(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev, params: *const ieee802154_llsec_params, changed: u32) -> core::ffi::c_int { ((*(*rdev).ops).set_llsec_params)(&mut (*rdev).wpan_phy, wpan_dev, params, changed) }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub unsafe fn rdev_add_llsec_key(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev, id: *const ieee802154_llsec_key_id, key: *const ieee802154_llsec_key) -> core::ffi::c_int { ((*(*rdev).ops).add_llsec_key)(&mut (*rdev).wpan_phy, wpan_dev, id, key) }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub unsafe fn rdev_del_llsec_key(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev, id: *const ieee802154_llsec_key_id) -> core::ffi::c_int { ((*(*rdev).ops).del_llsec_key)(&mut (*rdev).wpan_phy, wpan_dev, id) }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub unsafe fn rdev_add_seclevel(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev, sl: *const ieee802154_llsec_seclevel) -> core::ffi::c_int { ((*(*rdev).ops).add_seclevel)(&mut (*rdev).wpan_phy, wpan_dev, sl) }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub unsafe fn rdev_del_seclevel(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev, sl: *const ieee802154_llsec_seclevel) -> core::ffi::c_int { ((*(*rdev).ops).del_seclevel)(&mut (*rdev).wpan_phy, wpan_dev, sl) }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub unsafe fn rdev_add_device(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev, dev_desc: *const ieee802154_llsec_device) -> core::ffi::c_int { ((*(*rdev).ops).add_device)(&mut (*rdev).wpan_phy, wpan_dev, dev_desc) }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub unsafe fn rdev_del_device(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev, extended_addr: __le64) -> core::ffi::c_int { ((*(*rdev).ops).del_device)(&mut (*rdev).wpan_phy, wpan_dev, extended_addr) }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub unsafe fn rdev_add_devkey(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev, extended_addr: __le64, devkey: *const ieee802154_llsec_device_key) -> core::ffi::c_int { ((*(*rdev).ops).add_devkey)(&mut (*rdev).wpan_phy, wpan_dev, extended_addr, devkey) }
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub unsafe fn rdev_del_devkey(rdev: *mut cfg802154_registered_device, wpan_dev: *mut wpan_dev, extended_addr: __le64, devkey: *const ieee802154_llsec_device_key) -> core::ffi::c_int { ((*(*rdev).ops).del_devkey)(&mut (*rdev).wpan_phy, wpan_dev, extended_addr, devkey) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
