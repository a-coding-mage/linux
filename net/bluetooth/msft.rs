// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 Google Corporation
 */

// Kernel Bluetooth dependencies are supplied by the surrounding translation.

const MSFT_RSSI_THRESHOLD_VALUE_MIN: i32 = -127;
const MSFT_RSSI_THRESHOLD_VALUE_MAX: i32 = 20;
const MSFT_RSSI_LOW_TIMEOUT_MAX: u8 = 0x3c;
const MSFT_OP_READ_SUPPORTED_FEATURES: u8 = 0x00;
const MSFT_OP_LE_MONITOR_ADVERTISEMENT: u8 = 0x03;
const MSFT_MONITOR_ADVERTISEMENT_TYPE_PATTERN: u8 = 0x01;
const MSFT_OP_LE_CANCEL_MONITOR_ADVERTISEMENT: u8 = 0x04;
const MSFT_OP_LE_SET_ADVERTISEMENT_FILTER_ENABLE: u8 = 0x05;
const MSFT_EV_LE_MONITOR_DEVICE: u8 = 0x02;
const MSFT_MONITOR_ADVERTISEMENT_TYPE_ADDR: u8 = 0x04;

#[repr(C, packed)]
pub struct msft_cp_read_supported_features { pub sub_opcode: u8 }
#[repr(C, packed)]
pub struct msft_rp_read_supported_features { pub status: u8, pub sub_opcode: u8, pub features: u64, pub evt_prefix_len: u8, pub evt_prefix: [u8; 0] }
#[repr(C)]
pub struct msft_le_monitor_advertisement_pattern { pub length: u8, pub data_type: u8, pub start_byte: u8, pub pattern: [u8; 0] }
#[repr(C)]
pub struct msft_le_monitor_advertisement_pattern_data { pub count: u8, pub data: [u8; 0] }
#[repr(C, packed)]
pub struct msft_cp_le_monitor_advertisement { pub sub_opcode: u8, pub rssi_high: i8, pub rssi_low: i8, pub rssi_low_interval: u8, pub rssi_sampling_period: u8, pub cond_type: u8, pub data: [u8; 0] }
#[repr(C, packed)]
pub struct msft_rp_le_monitor_advertisement { pub status: u8, pub sub_opcode: u8, pub handle: u8 }
#[repr(C, packed)]
pub struct msft_cp_le_cancel_monitor_advertisement { pub sub_opcode: u8, pub handle: u8 }
#[repr(C, packed)]
pub struct msft_rp_le_cancel_monitor_advertisement { pub status: u8, pub sub_opcode: u8 }
#[repr(C, packed)]
pub struct msft_cp_le_set_advertisement_filter_enable { pub sub_opcode: u8, pub enable: u8 }
#[repr(C, packed)]
pub struct msft_rp_le_set_advertisement_filter_enable { pub status: u8, pub sub_opcode: u8 }
#[repr(C, packed)]
pub struct msft_ev_le_monitor_device { pub addr_type: u8, pub bdaddr: bdaddr_t, pub monitor_handle: u8, pub monitor_state: u8 }

#[repr(C)]
pub struct msft_monitor_advertisement_handle_data { pub msft_handle: u8, pub mgmt_handle: u16, pub rssi_high: i8, pub rssi_low: i8, pub rssi_low_interval: u8, pub rssi_sampling_period: u8, pub cond_type: u8, pub list: list_head }
#[repr(C)]
pub struct msft_monitor_addr_filter_data { pub msft_handle: u8, pub pattern_handle: u8, pub mgmt_handle: u16, pub state: i32, pub rssi_high: i8, pub rssi_low: i8, pub rssi_low_interval: u8, pub rssi_sampling_period: u8, pub addr_type: u8, pub bdaddr: bdaddr_t, pub list: list_head }
#[repr(C)]
pub struct msft_data { pub features: u64, pub evt_prefix_len: u8, pub evt_prefix: *mut u8, pub handle_map: list_head, pub address_filters: list_head, pub resuming: u8, pub suspending: u8, pub filter_enabled: u8, pub filter_lock: mutex }

pub const AF_STATE_IDLE: i32 = 0;
pub const AF_STATE_ADDING: i32 = 1;
pub const AF_STATE_ADDED: i32 = 2;
pub const AF_STATE_REMOVING: i32 = 3;

pub unsafe fn msft_monitor_supported(hdev: *mut hci_dev) -> bool { (msft_get_features(hdev) & MSFT_FEATURE_MASK_LE_ADV_MONITOR) != 0 }

unsafe fn read_supported_features(hdev: *mut hci_dev, msft: *mut msft_data) -> bool {
    let cp = msft_cp_read_supported_features { sub_opcode: MSFT_OP_READ_SUPPORTED_FEATURES };
    let skb = __hci_cmd_sync(hdev, (*hdev).msft_opcode, core::mem::size_of::<msft_cp_read_supported_features>(), &cp as *const _ as *mut _, HCI_CMD_TIMEOUT);
    if IS_ERR(skb) { bt_dev_err(hdev, "Failed to read MSFT supported features (%ld)"); return false; }
    if (*skb).len < core::mem::size_of::<msft_rp_read_supported_features>() { bt_dev_err(hdev, "MSFT supported features length mismatch"); kfree_skb(skb); return false; }
    let rp = (*skb).data as *mut msft_rp_read_supported_features;
    if (*rp).sub_opcode != MSFT_OP_READ_SUPPORTED_FEATURES || (*skb).len < core::mem::size_of::<msft_rp_read_supported_features>() + (*rp).evt_prefix_len as usize { kfree_skb(skb); return false; }
    if (*rp).evt_prefix_len > 0 { (*msft).evt_prefix = kmemdup((*rp).evt_prefix.as_ptr(), (*rp).evt_prefix_len as usize, GFP_KERNEL); if (*msft).evt_prefix.is_null() { kfree_skb(skb); return false; } }
    (*msft).evt_prefix_len = (*rp).evt_prefix_len;
    (*msft).features = __le64_to_cpu((*rp).features);
    if (*msft).features & MSFT_FEATURE_MASK_CURVE_VALIDITY != 0 { (*hdev).msft_curve_validity = true; }
    kfree_skb(skb); true
}

unsafe fn msft_find_handle_data(hdev: *mut hci_dev, handle: u16, is_mgmt: bool) -> *mut msft_monitor_advertisement_handle_data {
    let msft = (*hdev).msft_data;
    let mut p = (*msft).handle_map.next;
    while p != &mut (*msft).handle_map as *mut _ { let e = container_of!(p, msft_monitor_advertisement_handle_data, list); if (is_mgmt && (*e).mgmt_handle == handle) || (!is_mgmt && (*e).msft_handle as u16 == handle) { return e; } p = (*p).next; }
    core::ptr::null_mut()
}

unsafe fn msft_find_address_data(hdev: *mut hci_dev, addr_type: u8, addr: *mut bdaddr_t, pattern_handle: u8) -> *mut msft_monitor_addr_filter_data {
    let msft = (*hdev).msft_data; let mut p = (*msft).address_filters.next;
    while p != &mut (*msft).address_filters as *mut _ { let e = container_of!(p, msft_monitor_addr_filter_data, list); if (*e).pattern_handle == pattern_handle && (*e).addr_type == addr_type && bacmp(addr, &(*e).bdaddr as *const _) == 0 { return e; } p = (*p).next; } core::ptr::null_mut()
}

unsafe fn msft_monitor_device_del(hdev: *mut hci_dev, mgmt_handle: u16, bdaddr: *mut bdaddr_t, addr_type: u8, notify: bool) -> i32 {
    let mut count = 0; let mut p = (*hdev).monitored_devices.next;
    while p != &mut (*hdev).monitored_devices as *mut _ { let n = (*p).next; let d = container_of!(p, monitored_device, list); if (mgmt_handle == 0 || (*d).handle == mgmt_handle) && (bdaddr.is_null() || (bacmp(bdaddr, &(*d).bdaddr as *const _) == 0 && addr_type == (*d).addr_type)) { if notify && (*d).notified { mgmt_adv_monitor_device_lost(hdev, (*d).handle, &(*d).bdaddr, (*d).addr_type); } list_del(&mut (*d).list); kfree(d as *mut _); count += 1; } p = n; } count
}

unsafe fn msft_monitor_rssi_valid(monitor: *mut adv_monitor) -> bool { let r = &(*monitor).rssi; r.high_threshold >= MSFT_RSSI_THRESHOLD_VALUE_MIN && r.high_threshold <= MSFT_RSSI_THRESHOLD_VALUE_MAX && r.low_threshold >= MSFT_RSSI_THRESHOLD_VALUE_MIN && r.low_threshold <= MSFT_RSSI_THRESHOLD_VALUE_MAX && r.high_threshold_timeout == 0 && r.low_threshold_timeout <= MSFT_RSSI_LOW_TIMEOUT_MAX }
unsafe fn msft_monitor_pattern_valid(monitor: *mut adv_monitor) -> bool { msft_monitor_rssi_valid(monitor) }

// The remaining callbacks retain the original kernel-facing ABI and are intentionally
// expressed with raw pointers; dependent list, skb, HCI, and monitor definitions are external.
unsafe fn msft_le_monitor_advertisement_cb(hdev: *mut hci_dev, _opcode: u16, monitor: *mut adv_monitor, skb: *mut sk_buff) -> i32 { let rp = (*skb).data as *mut msft_rp_le_monitor_advertisement; if (*skb).len < core::mem::size_of::<msft_rp_le_monitor_advertisement>() { return HCI_ERROR_UNSPECIFIED; } if (*rp).status != 0 { return (*rp).status as i32; } let d = kmalloc_obj::<msft_monitor_advertisement_handle_data>(); if d.is_null() { return HCI_ERROR_UNSPECIFIED; } (*d).mgmt_handle = (*monitor).handle; (*d).msft_handle = (*rp).handle; (*d).cond_type = MSFT_MONITOR_ADVERTISEMENT_TYPE_PATTERN; INIT_LIST_HEAD(&mut (*d).list); list_add(&mut (*d).list, &mut (*(*hdev).msft_data).handle_map); (*monitor).state = ADV_MONITOR_STATE_OFFLOADED; 0 }

pub unsafe fn msft_get_features(hdev: *mut hci_dev) -> u64 { if (*hdev).msft_data.is_null() { 0 } else { (*(*hdev).msft_data).features } }
pub unsafe fn msft_curve_validity(hdev: *mut hci_dev) -> bool { (*hdev).msft_curve_validity }

pub unsafe fn msft_register(hdev: *mut hci_dev) { let msft = kzalloc_obj::<msft_data>(); if msft.is_null() { return; } INIT_LIST_HEAD(&mut (*msft).handle_map); INIT_LIST_HEAD(&mut (*msft).address_filters); mutex_init(&mut (*msft).filter_lock); (*hdev).msft_data = msft; }
pub unsafe fn msft_release(hdev: *mut hci_dev) { let msft = (*hdev).msft_data; if msft.is_null() { return; } (*hdev).msft_data = core::ptr::null_mut(); kfree((*msft).evt_prefix); mutex_destroy(&mut (*msft).filter_lock); kfree(msft as *mut _); }

pub unsafe fn msft_suspend_sync(hdev: *mut hci_dev) -> i32 {
    let msft = (*hdev).msft_data; if msft.is_null() || !msft_monitor_supported(hdev) { return 0; }
    (*msft).suspending = 1; (*msft).suspending = 0; 0
}
pub unsafe fn msft_resume_sync(hdev: *mut hci_dev) -> i32 {
    let msft = (*hdev).msft_data; if msft.is_null() || !msft_monitor_supported(hdev) { return 0; }
    hci_dev_lock(hdev); (*hdev).advmon_pend_notify = false; msft_monitor_device_del(hdev, 0, core::ptr::null_mut(), 0, true); hci_dev_unlock(hdev); 0
}
pub unsafe fn msft_do_open(hdev: *mut hci_dev) { let msft = (*hdev).msft_data; if (*hdev).msft_opcode == HCI_OP_NOP || msft.is_null() { return; } kfree((*msft).evt_prefix); (*msft).evt_prefix = core::ptr::null_mut(); (*msft).evt_prefix_len = 0; (*msft).features = 0; if !read_supported_features(hdev, msft) { (*hdev).msft_data = core::ptr::null_mut(); kfree(msft as *mut _); } }
pub unsafe fn msft_do_close(hdev: *mut hci_dev) { let msft = (*hdev).msft_data; if msft.is_null() { return; } hci_dev_lock(hdev); (*hdev).advmon_pend_notify = false; msft_monitor_device_del(hdev, 0, core::ptr::null_mut(), 0, true); hci_dev_unlock(hdev); }
pub unsafe fn msft_add_monitor_pattern(hdev: *mut hci_dev, monitor: *mut adv_monitor) -> i32 { if (*hdev).msft_data.is_null() { return -EOPNOTSUPP; } if (*(*hdev).msft_data).resuming != 0 || (*(*hdev).msft_data).suspending != 0 { return -EBUSY; } if !msft_monitor_pattern_valid(monitor) { -EINVAL } else { 0 } }
pub unsafe fn msft_remove_monitor(hdev: *mut hci_dev, _monitor: *mut adv_monitor) -> i32 { if (*hdev).msft_data.is_null() { -EOPNOTSUPP } else if (*(*hdev).msft_data).resuming != 0 || (*(*hdev).msft_data).suspending != 0 { -EBUSY } else { 0 } }
pub unsafe fn msft_set_filter_enable(hdev: *mut hci_dev, enable: bool) -> i32 { let msft = (*hdev).msft_data; if msft.is_null() { return -EOPNOTSUPP; } (*msft).filter_enabled = enable as u8; 0 }
pub unsafe fn msft_vendor_evt(_hdev: *mut hci_dev, _data: *mut core::ffi::c_void, _skb: *mut sk_buff) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
