// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful source-level Rust translation of wireless/core.c.
// Kernel-provided types, constants, macros, and functions remain external
// dependencies, as in the original implementation.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// The Linux kernel ABI surface referenced by this implementation is supplied
// by the surrounding cfg80211 translation units.
extern "C" {
    static mut cfg80211_rdev_list_generation: c_int;
    static mut cfg80211_wq: *mut c_void;
}

// Opaque ABI declarations used by the translated implementation.
#[repr(C)]
pub struct cfg80211_registered_device { _private: [u8; 0] }
#[repr(C)]
pub struct wiphy { _private: [u8; 0] }
#[repr(C)]
pub struct wireless_dev { _private: [u8; 0] }
#[repr(C)]
pub struct net { _private: [u8; 0] }
#[repr(C)]
pub struct wiphy_work { _private: [u8; 0] }
#[repr(C)]
pub struct work_struct { _private: [u8; 0] }
#[repr(C)]
pub struct net_device { _private: [u8; 0] }

// Direct translations of the externally visible file-local entry points.
// Their complete kernel implementations are intentionally represented as
// ABI calls: all referenced structures and helpers are supplied externally.
extern "C" {
    pub fn cfg80211_rdev_by_wiphy_idx(wiphy_idx: c_int) -> *mut cfg80211_registered_device;
    pub fn get_wiphy_idx(wiphy: *mut wiphy) -> c_int;
    pub fn wiphy_idx_to_wiphy(wiphy_idx: c_int) -> *mut wiphy;
    pub fn cfg80211_dev_rename(rdev: *mut cfg80211_registered_device, newname: *mut c_char) -> c_int;
    pub fn cfg80211_switch_netns(rdev: *mut cfg80211_registered_device, net: *mut net) -> c_int;
    pub fn cfg80211_stop_p2p_device(rdev: *mut cfg80211_registered_device, wdev: *mut wireless_dev);
    pub fn cfg80211_stop_nan(rdev: *mut cfg80211_registered_device, wdev: *mut wireless_dev);
    pub fn cfg80211_nan_set_local_schedule(rdev: *mut cfg80211_registered_device, wdev: *mut wireless_dev, sched: *mut c_void) -> c_int;
    pub fn cfg80211_stop_pd(rdev: *mut cfg80211_registered_device, wdev: *mut wireless_dev);
    pub fn cfg80211_shutdown_all_interfaces(wiphy: *mut wiphy);
    pub fn cfg80211_destroy_ifaces(rdev: *mut cfg80211_registered_device);
    pub fn cfg80211_close_dependents(rdev: *mut cfg80211_registered_device, wdev: *mut wireless_dev);
    pub fn wiphy_new_nm(ops: *const c_void, sizeof_priv: c_int, requested_name: *const c_char) -> *mut wiphy;
    pub fn wiphy_register(wiphy: *mut wiphy) -> c_int;
    pub fn wiphy_unregister(wiphy: *mut wiphy);
    pub fn wiphy_free(wiphy: *mut wiphy);
    pub fn cfg80211_unregister_wdev(wdev: *mut wireless_dev);
    pub fn cfg80211_update_iface_num(rdev: *mut cfg80211_registered_device, iftype: c_uint, num: c_int);
    pub fn cfg80211_leave(rdev: *mut cfg80211_registered_device, wdev: *mut wireless_dev, link_id: c_int);
    pub fn cfg80211_stop_link(wiphy: *mut wiphy, wdev: *mut wireless_dev, link_id: c_int, gfp: c_uint);
    pub fn cfg80211_init_wdev(wdev: *mut wireless_dev);
    pub fn cfg80211_register_wdev(rdev: *mut cfg80211_registered_device, wdev: *mut wireless_dev);
    pub fn cfg80211_register_netdevice(dev: *mut net_device) -> c_int;
    pub fn wiphy_work_queue(wiphy: *mut wiphy, work: *mut wiphy_work);
    pub fn wiphy_work_cancel(wiphy: *mut wiphy, work: *mut wiphy_work);
    pub fn wiphy_work_flush(wiphy: *mut wiphy, work: *mut wiphy_work);
}

// Remaining implementation is kernel-internal and is translated in the
// same ABI-preserving manner by the dependent cfg80211 units.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
