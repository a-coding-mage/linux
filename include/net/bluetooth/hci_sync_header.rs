/* SPDX-License-Identifier: GPL-2.0 */
/*
 * BlueZ - Bluetooth protocol stack for Linux
 *
 * Copyright (C) 2021 Intel Corporation
 */

pub const HCI_REQ_DONE: i32 = 0;
pub const HCI_REQ_PEND: i32 = 1;
pub const HCI_REQ_CANCELED: i32 = 2;

// C macros:
// #define UINT_PTR(_handle) ((void *)((uintptr_t)_handle))
// #define PTR_UINT(_ptr) ((uintptr_t)((void *)_ptr))
#[inline]
pub unsafe fn uint_ptr<T>(handle: T) -> *mut core::ffi::c_void
where
    T: Copy,
{
    handle as usize as *mut core::ffi::c_void
}

#[inline]
pub unsafe fn ptr_uint<T>(ptr: *const T) -> usize {
    ptr as *const core::ffi::c_void as usize
}

// hci_req_sync_lock(hdev) and hci_req_sync_unlock(hdev) expand to mutex_lock
// and mutex_unlock on hdev->req_lock; the mutex and hci_dev definitions are
// supplied by the surrounding kernel bindings.

#[repr(C)]
pub struct hci_request {
    pub hdev: *mut hci_dev,
    pub cmd_q: sk_buff_head,
    /* If something goes wrong when building the HCI request, the error
     * value is stored in this field.
     */
    pub err: i32,
}

pub type hci_cmd_sync_work_func_t = unsafe extern "C" fn(
    hdev: *mut hci_dev,
    data: *mut core::ffi::c_void,
) -> i32;
pub type hci_cmd_sync_work_destroy_t = unsafe extern "C" fn(
    hdev: *mut hci_dev,
    data: *mut core::ffi::c_void,
    err: i32,
);

#[repr(C)]
pub struct hci_cmd_sync_work_entry {
    pub list: list_head,
    pub func: hci_cmd_sync_work_func_t,
    pub data: *mut core::ffi::c_void,
    pub destroy: hci_cmd_sync_work_destroy_t,
}

pub enum adv_info {}

/* Function with sync suffix shall not be called with hdev->lock held as they
 * wait the command to complete and in the meantime an event could be received
 * which could attempt to acquire hdev->lock causing a deadlock.
 */
extern "C" {
    pub fn hci_cmd_sync_alloc(hdev: *mut hci_dev, opcode: u16, plen: u32, param: *const core::ffi::c_void, sk: *mut sock) -> *mut sk_buff;
    pub fn __hci_cmd_sync(hdev: *mut hci_dev, opcode: u16, plen: u32, param: *const core::ffi::c_void, timeout: u32) -> *mut sk_buff;
    pub fn hci_cmd_sync(hdev: *mut hci_dev, opcode: u16, plen: u32, param: *const core::ffi::c_void, timeout: u32) -> *mut sk_buff;
    pub fn __hci_cmd_sync_ev(hdev: *mut hci_dev, opcode: u16, plen: u32, param: *const core::ffi::c_void, event: u8, timeout: u32) -> *mut sk_buff;
    pub fn __hci_cmd_sync_sk(hdev: *mut hci_dev, opcode: u16, plen: u32, param: *const core::ffi::c_void, event: u8, timeout: u32, sk: *mut sock) -> *mut sk_buff;
    pub fn __hci_cmd_sync_status(hdev: *mut hci_dev, opcode: u16, plen: u32, param: *const core::ffi::c_void, timeout: u32) -> i32;
    pub fn __hci_cmd_sync_status_sk(hdev: *mut hci_dev, opcode: u16, plen: u32, param: *const core::ffi::c_void, event: u8, timeout: u32, sk: *mut sock) -> i32;
    pub fn __hci_reset_sync(hdev: *mut hci_dev) -> i32;
    pub fn hci_cmd_sync_status(hdev: *mut hci_dev, opcode: u16, plen: u32, param: *const core::ffi::c_void, timeout: u32) -> i32;

    pub fn hci_cmd_sync_init(hdev: *mut hci_dev);
    pub fn hci_cmd_sync_clear(hdev: *mut hci_dev);
    pub fn hci_cmd_sync_cancel(hdev: *mut hci_dev, err: i32);
    pub fn hci_cmd_sync_cancel_sync(hdev: *mut hci_dev, err: i32);
    pub fn hci_cmd_sync_submit(hdev: *mut hci_dev, func: hci_cmd_sync_work_func_t, data: *mut core::ffi::c_void, destroy: hci_cmd_sync_work_destroy_t) -> i32;
    pub fn hci_cmd_sync_queue(hdev: *mut hci_dev, func: hci_cmd_sync_work_func_t, data: *mut core::ffi::c_void, destroy: hci_cmd_sync_work_destroy_t) -> i32;
    pub fn hci_cmd_sync_queue_once(hdev: *mut hci_dev, func: hci_cmd_sync_work_func_t, data: *mut core::ffi::c_void, destroy: hci_cmd_sync_work_destroy_t) -> i32;
    pub fn hci_cmd_sync_run(hdev: *mut hci_dev, func: hci_cmd_sync_work_func_t, data: *mut core::ffi::c_void, destroy: hci_cmd_sync_work_destroy_t) -> i32;
    pub fn hci_cmd_sync_run_once(hdev: *mut hci_dev, func: hci_cmd_sync_work_func_t, data: *mut core::ffi::c_void, destroy: hci_cmd_sync_work_destroy_t) -> i32;
    pub fn hci_cmd_sync_lookup_entry(hdev: *mut hci_dev, func: hci_cmd_sync_work_func_t, data: *mut core::ffi::c_void, destroy: hci_cmd_sync_work_destroy_t) -> *mut hci_cmd_sync_work_entry;
    pub fn hci_cmd_sync_cancel_entry(hdev: *mut hci_dev, entry: *mut hci_cmd_sync_work_entry);
    pub fn hci_cmd_sync_dequeue(hdev: *mut hci_dev, func: hci_cmd_sync_work_func_t, data: *mut core::ffi::c_void, destroy: hci_cmd_sync_work_destroy_t) -> bool;

    pub fn hci_update_eir_sync(hdev: *mut hci_dev) -> i32;
    pub fn hci_update_class_sync(hdev: *mut hci_dev) -> i32;
    pub fn hci_update_eir_sync(hdev: *mut hci_dev) -> i32;
    pub fn hci_update_class_sync(hdev: *mut hci_dev) -> i32;
    pub fn hci_update_name_sync(hdev: *mut hci_dev, name: *const u8) -> i32;
    pub fn hci_write_ssp_mode_sync(hdev: *mut hci_dev, mode: u8) -> i32;
    pub fn hci_get_random_address(hdev: *mut hci_dev, require_privacy: bool, use_rpa: bool, adv_instance: *mut adv_info, own_addr_type: *mut u8, rand_addr: *mut bdaddr_t) -> i32;
    pub fn hci_update_random_address_sync(hdev: *mut hci_dev, require_privacy: bool, rpa: bool, own_addr_type: *mut u8) -> i32;
    pub fn hci_update_scan_rsp_data_sync(hdev: *mut hci_dev, instance: u8) -> i32;
    pub fn hci_update_adv_data_sync(hdev: *mut hci_dev, instance: u8) -> i32;
    pub fn hci_update_adv_data(hdev: *mut hci_dev, instance: u8) -> i32;
    pub fn hci_schedule_adv_instance_sync(hdev: *mut hci_dev, instance: u8, force: bool) -> i32;
    pub fn hci_setup_ext_adv_instance_sync(hdev: *mut hci_dev, instance: u8) -> i32;
    pub fn hci_start_ext_adv_sync(hdev: *mut hci_dev, instance: u8) -> i32;
    pub fn hci_enable_ext_advertising_sync(hdev: *mut hci_dev, instance: u8) -> i32;
    pub fn hci_enable_advertising_sync(hdev: *mut hci_dev) -> i32;
    pub fn hci_enable_advertising(hdev: *mut hci_dev) -> i32;
    pub fn hci_start_per_adv_sync(hdev: *mut hci_dev, instance: u8, sid: u8, data_len: u8, data: *mut u8, flags: u32, min_interval: u16, max_interval: u16, sync_interval: u16) -> i32;
    pub fn hci_disable_per_advertising_sync(hdev: *mut hci_dev, instance: u8) -> i32;
    pub fn hci_remove_advertising_sync(hdev: *mut hci_dev, sk: *mut sock, instance: u8, force: bool) -> i32;
    pub fn hci_disable_advertising_sync(hdev: *mut hci_dev) -> i32;
    pub fn hci_clear_adv_instance_sync(hdev: *mut hci_dev, sk: *mut sock, instance: u8, force: bool) -> i32;
    pub fn hci_update_passive_scan_sync(hdev: *mut hci_dev) -> i32;
    pub fn hci_update_passive_scan(hdev: *mut hci_dev) -> i32;
    pub fn hci_read_rssi_sync(hdev: *mut hci_dev, handle: __le16) -> i32;
    pub fn hci_read_tx_power_sync(hdev: *mut hci_dev, handle: __le16, type_: u8) -> i32;
    pub fn hci_write_sc_support_sync(hdev: *mut hci_dev, val: u8) -> i32;
    pub fn hci_read_clock_sync(hdev: *mut hci_dev, cp: *mut hci_cp_read_clock) -> i32;
    pub fn hci_write_fast_connectable_sync(hdev: *mut hci_dev, enable: bool) -> i32;
    pub fn hci_update_scan_sync(hdev: *mut hci_dev) -> i32;
    pub fn hci_update_scan(hdev: *mut hci_dev) -> i32;
    pub fn hci_write_le_host_supported_sync(hdev: *mut hci_dev, le: u8, simul: u8) -> i32;
    pub fn hci_remove_ext_adv_instance_sync(hdev: *mut hci_dev, instance: u8, sk: *mut sock) -> i32;
    pub fn hci_read_local_oob_data_sync(hdev: *mut hci_dev, ext: bool, sk: *mut sock) -> *mut sk_buff;
    pub fn hci_reset_sync(hdev: *mut hci_dev) -> i32;
    pub fn hci_dev_open_sync(hdev: *mut hci_dev) -> i32;
    pub fn hci_dev_close_sync(hdev: *mut hci_dev) -> i32;
    pub fn hci_powered_update_sync(hdev: *mut hci_dev) -> i32;
    pub fn hci_set_powered_sync(hdev: *mut hci_dev, val: u8) -> i32;
    pub fn hci_update_discoverable_sync(hdev: *mut hci_dev) -> i32;
    pub fn hci_update_discoverable(hdev: *mut hci_dev) -> i32;
    pub fn hci_update_connectable_sync(hdev: *mut hci_dev) -> i32;
    pub fn hci_inquiry_sync(hdev: *mut hci_dev, length: u8, num_rsp: u8) -> i32;
    pub fn hci_start_discovery_sync(hdev: *mut hci_dev) -> i32;
    pub fn hci_stop_discovery_sync(hdev: *mut hci_dev) -> i32;
    pub fn hci_suspend_sync(hdev: *mut hci_dev) -> i32;
    pub fn hci_resume_sync(hdev: *mut hci_dev) -> i32;
    pub fn hci_abort_conn_sync(hdev: *mut hci_dev, conn: *mut hci_conn, reason: u8) -> i32;
    pub fn hci_le_create_cis_sync(hdev: *mut hci_dev) -> i32;
    pub fn hci_le_remove_cig_sync(hdev: *mut hci_dev, handle: u8) -> i32;
    pub fn hci_le_terminate_big_sync(hdev: *mut hci_dev, handle: u8, reason: u8) -> i32;
    pub fn hci_le_big_terminate_sync(hdev: *mut hci_dev, handle: u8) -> i32;
    pub fn hci_le_pa_terminate_sync(hdev: *mut hci_dev, handle: u16) -> i32;
    pub fn hci_connect_acl_sync(hdev: *mut hci_dev, conn: *mut hci_conn) -> i32;
    pub fn hci_connect_le_sync(hdev: *mut hci_dev, conn: *mut hci_conn) -> i32;
    pub fn hci_cancel_connect_sync(hdev: *mut hci_dev, conn: *mut hci_conn) -> i32;
    pub fn hci_le_conn_update_sync(hdev: *mut hci_dev, conn: *mut hci_conn, params: *mut hci_conn_params) -> i32;
    pub fn hci_le_conn_rate_request(hdev: *mut hci_dev, conn: *mut hci_conn) -> i32;
    pub fn hci_connect_pa_sync(hdev: *mut hci_dev, conn: *mut hci_conn) -> i32;
    pub fn hci_connect_big_sync(hdev: *mut hci_dev, conn: *mut hci_conn) -> i32;
    pub fn hci_past_sync(conn: *mut hci_conn, le: *mut hci_conn) -> i32;
    pub fn hci_le_read_remote_features(conn: *mut hci_conn) -> i32;
    pub fn hci_acl_change_pkt_type(conn: *mut hci_conn, pkt_type: u16) -> i32;
    pub fn hci_le_set_phy(conn: *mut hci_conn, tx_phys: u8, rx_phys: u8) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
