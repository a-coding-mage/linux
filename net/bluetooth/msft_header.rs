// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 Google Corporation
 */

pub const MSFT_FEATURE_MASK_BREDR_RSSI_MONITOR: u64 = 1u64 << 0;
pub const MSFT_FEATURE_MASK_LE_CONN_RSSI_MONITOR: u64 = 1u64 << 1;
pub const MSFT_FEATURE_MASK_LE_ADV_RSSI_MONITOR: u64 = 1u64 << 2;
pub const MSFT_FEATURE_MASK_LE_ADV_MONITOR: u64 = 1u64 << 3;
pub const MSFT_FEATURE_MASK_CURVE_VALIDITY: u64 = 1u64 << 4;
pub const MSFT_FEATURE_MASK_CONCURRENT_ADV_MONITOR: u64 = 1u64 << 5;

// Conditional compilation intent preserved from IS_ENABLED(CONFIG_BT_MSFTEXT).
#[cfg(CONFIG_BT_MSFTEXT)]
extern "C" {
    pub fn msft_monitor_supported(hdev: *mut hci_dev) -> bool;
    pub fn msft_register(hdev: *mut hci_dev);
    pub fn msft_release(hdev: *mut hci_dev);
    pub fn msft_do_open(hdev: *mut hci_dev);
    pub fn msft_do_close(hdev: *mut hci_dev);
    pub fn msft_vendor_evt(hdev: *mut hci_dev, data: *mut core::ffi::c_void, skb: *mut sk_buff);
    pub fn msft_get_features(hdev: *mut hci_dev) -> u64;
    pub fn msft_add_monitor_pattern(hdev: *mut hci_dev, monitor: *mut adv_monitor) -> i32;
    pub fn msft_remove_monitor(hdev: *mut hci_dev, monitor: *mut adv_monitor) -> i32;
    pub fn msft_req_add_set_filter_enable(req: *mut hci_request, enable: bool);
    pub fn msft_set_filter_enable(hdev: *mut hci_dev, enable: bool) -> i32;
    pub fn msft_suspend_sync(hdev: *mut hci_dev) -> i32;
    pub fn msft_resume_sync(hdev: *mut hci_dev) -> i32;
    pub fn msft_curve_validity(hdev: *mut hci_dev) -> bool;
}

#[cfg(not(CONFIG_BT_MSFTEXT))]
pub unsafe fn msft_monitor_supported(_hdev: *mut hci_dev) -> bool { false }

#[cfg(not(CONFIG_BT_MSFTEXT))]
pub unsafe fn msft_register(_hdev: *mut hci_dev) {}
#[cfg(not(CONFIG_BT_MSFTEXT))]
pub unsafe fn msft_release(_hdev: *mut hci_dev) {}
#[cfg(not(CONFIG_BT_MSFTEXT))]
pub unsafe fn msft_do_open(_hdev: *mut hci_dev) {}
#[cfg(not(CONFIG_BT_MSFTEXT))]
pub unsafe fn msft_do_close(_hdev: *mut hci_dev) {}
#[cfg(not(CONFIG_BT_MSFTEXT))]
pub unsafe fn msft_vendor_evt(
    _hdev: *mut hci_dev,
    _data: *mut core::ffi::c_void,
    _skb: *mut sk_buff,
) {}
#[cfg(not(CONFIG_BT_MSFTEXT))]
pub unsafe fn msft_get_features(_hdev: *mut hci_dev) -> u64 { 0 }
#[cfg(not(CONFIG_BT_MSFTEXT))]
pub unsafe fn msft_add_monitor_pattern(
    _hdev: *mut hci_dev,
    _monitor: *mut adv_monitor,
) -> i32 { -EOPNOTSUPP }
#[cfg(not(CONFIG_BT_MSFTEXT))]
pub unsafe fn msft_remove_monitor(
    _hdev: *mut hci_dev,
    _monitor: *mut adv_monitor,
) -> i32 { -EOPNOTSUPP }
#[cfg(not(CONFIG_BT_MSFTEXT))]
pub unsafe fn msft_req_add_set_filter_enable(_req: *mut hci_request, _enable: bool) {}
#[cfg(not(CONFIG_BT_MSFTEXT))]
pub unsafe fn msft_set_filter_enable(_hdev: *mut hci_dev, _enable: bool) -> i32 {
    -EOPNOTSUPP
}
#[cfg(not(CONFIG_BT_MSFTEXT))]
pub unsafe fn msft_suspend_sync(_hdev: *mut hci_dev) -> i32 { -EOPNOTSUPP }
#[cfg(not(CONFIG_BT_MSFTEXT))]
pub unsafe fn msft_resume_sync(_hdev: *mut hci_dev) -> i32 { -EOPNOTSUPP }
#[cfg(not(CONFIG_BT_MSFTEXT))]
pub unsafe fn msft_curve_validity(_hdev: *mut hci_dev) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
