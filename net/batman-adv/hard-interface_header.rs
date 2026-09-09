/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Marek Lindner, Simon Wunderlich
 */

// Translated from hard-interface.h. Definitions supplied by included headers
// remain external dependencies.

#[repr(C)]
pub enum batadv_hard_if_state {
    BATADV_IF_TO_BE_REMOVED,
    BATADV_IF_INACTIVE,
    BATADV_IF_ACTIVE,
    BATADV_IF_TO_BE_ACTIVATED,
}

#[repr(C)]
pub enum batadv_hard_if_bcast {
    BATADV_HARDIF_BCAST_OK = 0,
    BATADV_HARDIF_BCAST_NORECIPIENT,
    BATADV_HARDIF_BCAST_DUPFWD,
    BATADV_HARDIF_BCAST_DUPORIG,
}

pub type u8 = ::core::primitive::u8;
pub type u32 = ::core::primitive::u32;

// Opaque types and constants are provided by the corresponding translated headers.
pub struct notifier_block;
pub struct net_device;
pub struct kref;
pub struct batadv_hard_iface;
pub struct batadv_priv;

extern "C" {
    pub static mut batadv_hard_if_notifier: notifier_block;

    pub fn __batadv_get_real_netdev(net_device: *mut net_device) -> *mut net_device;
    pub fn batadv_get_real_netdev(net_device: *mut net_device) -> *mut net_device;
    pub fn batadv_netdev_get_wifi_flags(net_dev: *mut net_device) -> u32;
    pub fn batadv_hardif_get_wifi_flags(hard_iface: *mut batadv_hard_iface) -> u32;
    pub fn batadv_is_wifi_hardif(hard_iface: *mut batadv_hard_iface) -> bool;
    pub fn batadv_hardif_get_by_netdev(net_dev: *mut net_device) -> *mut batadv_hard_iface;
    pub fn batadv_hardif_enable_interface(
        net_dev: *mut net_device,
        mesh_iface: *mut net_device,
    ) -> ::core::ffi::c_int;
    pub fn batadv_hardif_disable_interface(hard_iface: *mut batadv_hard_iface);
    pub fn batadv_hardif_min_mtu(mesh_iface: *mut net_device) -> ::core::ffi::c_int;
    pub fn batadv_update_min_mtu(mesh_iface: *mut net_device);
    pub fn batadv_hardif_release(ref_: *mut kref);
    pub fn batadv_hardif_no_broadcast(
        if_outgoing: *mut batadv_hard_iface,
        orig_addr: *mut u8,
        orig_neigh: *mut u8,
    ) -> ::core::ffi::c_int;
    pub fn batadv_wifi_net_devices_init() -> ::core::ffi::c_int;
    pub fn batadv_wifi_net_devices_deinit();
}

/// Decrement the hard interface refcounter and possibly release it.
#[inline]
pub unsafe fn batadv_hardif_put(hard_iface: *mut batadv_hard_iface) {
    if hard_iface.is_null() {
        return;
    }

    // Equivalent to kref_put(&hard_iface->refcount, batadv_hardif_release).
    kref_put(
        &mut (*hard_iface).refcount,
        Some(batadv_hardif_release),
    );
}

/// Get a reference to the selected primary interface.
#[inline]
pub unsafe fn batadv_primary_if_get_selected(
    bat_priv: *mut batadv_priv,
) -> *mut batadv_hard_iface {
    let mut hard_iface: *mut batadv_hard_iface;

    rcu_read_lock();
    hard_iface = rcu_dereference((*bat_priv).primary_if);
    if hard_iface.is_null() {
        rcu_read_unlock();
        return hard_iface;
    }

    if !kref_get_unless_zero(&mut (*hard_iface).refcount) {
        hard_iface = ::core::ptr::null_mut();
    }

    rcu_read_unlock();
    hard_iface
}

/// Check if the given hardif is a cfg80211 wifi interface.
#[inline]
pub fn batadv_is_cfg80211(wifi_flags: u32) -> bool {
    let mut allowed_flags: u32 = 0;
    allowed_flags |= BATADV_HARDIF_WIFI_CFG80211_DIRECT;
    allowed_flags |= BATADV_HARDIF_WIFI_CFG80211_INDIRECT;
    (wifi_flags & allowed_flags) != 0
}

/// Check if flags belong to a wifi interface.
#[inline]
pub fn batadv_is_wifi(wifi_flags: u32) -> bool {
    wifi_flags != 0
}

extern "C" {
    fn kref_put(ref_: *mut kref, release: Option<unsafe extern "C" fn(*mut kref)>);
    fn kref_get_unless_zero(ref_: *mut kref) -> bool;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn rcu_dereference<T>(ptr: *mut T) -> *mut batadv_hard_iface;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
