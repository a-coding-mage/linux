/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2011 Instituto Nokia de Tecnologia
 *
 * Authors:
 *    Lauro Ramos Venancio <lauro.venancio@openbossa.org>
 *    Aloisio Almeida Jr <aloisio.almeida@openbossa.org>
 */

// Translated from the Linux NFC header. Types and symbols supplied by the
// included kernel headers are intentionally left as external dependencies.

pub const NFC_TARGET_MODE_IDLE: i32 = 0;
pub const NFC_TARGET_MODE_SLEEP: i32 = 1;

#[repr(C)]
pub struct nfc_protocol {
    pub id: i32,
    pub proto: *mut proto,
    pub owner: *mut module,
    pub create: Option<unsafe extern "C" fn(*mut net, *mut socket, *const nfc_protocol, i32) -> i32>,
}

#[repr(C)]
pub struct nfc_rawsock {
    pub sk: sock,
    pub dev: *mut nfc_dev,
    pub target_idx: u32,
    pub tx_work: work_struct,
    pub tx_work_scheduled: bool,
}

#[repr(C)]
pub struct nfc_sock_list {
    pub head: hlist_head,
    pub lock: rwlock_t,
}

#[inline]
pub unsafe fn nfc_rawsock_from_sk(sk: *mut sock) -> *mut nfc_rawsock {
    sk as *mut nfc_rawsock
}

#[inline]
pub unsafe fn to_rawsock_sk(tx_work: *mut work_struct) -> *mut sock {
    (tx_work as *mut u8).sub(std::mem::offset_of!(nfc_rawsock, tx_work)) as *mut sock
}

pub enum nfc_llcp_sdp_tlv {}

extern "C" {
    pub fn nfc_llcp_mac_is_down(dev: *mut nfc_dev);
    pub fn nfc_llcp_mac_is_up(dev: *mut nfc_dev, target_idx: u32, comm_mode: u8, rf_mode: u8);
    pub fn nfc_llcp_register_device(dev: *mut nfc_dev) -> i32;
    pub fn nfc_llcp_unregister_device(dev: *mut nfc_dev);
    pub fn nfc_llcp_set_remote_gb(dev: *mut nfc_dev, gb: *const u8, gb_len: u8) -> i32;
    pub fn nfc_llcp_general_bytes(dev: *mut nfc_dev, general_bytes_len: *mut usize) -> *mut u8;
    pub fn nfc_llcp_data_received(dev: *mut nfc_dev, skb: *mut sk_buff) -> i32;
    pub fn nfc_llcp_find_local(dev: *mut nfc_dev) -> *mut nfc_llcp_local;
    pub fn nfc_llcp_local_put(local: *mut nfc_llcp_local) -> i32;
    pub fn nfc_llcp_init() -> i32;
    pub fn nfc_llcp_exit();
    pub fn nfc_llcp_free_sdp_tlv(sdp: *mut nfc_llcp_sdp_tlv);
    pub fn nfc_llcp_free_sdp_tlv_list(head: *mut hlist_head);

    pub fn rawsock_init() -> i32;
    pub fn rawsock_exit();
    pub fn af_nfc_init() -> i32;
    pub fn af_nfc_exit();
    pub fn nfc_proto_register(nfc_proto: *const nfc_protocol) -> i32;
    pub fn nfc_proto_unregister(nfc_proto: *const nfc_protocol);

    pub static mut nfc_devlist_generation: i32;
    pub static mut nfc_devlist_mutex: mutex;

    pub fn nfc_genl_init() -> i32;
    pub fn nfc_genl_exit();
    pub fn nfc_genl_data_init(genl_data: *mut nfc_genl_data);
    pub fn nfc_genl_data_exit(genl_data: *mut nfc_genl_data);
    pub fn nfc_genl_targets_found(dev: *mut nfc_dev) -> i32;
    pub fn nfc_genl_target_lost(dev: *mut nfc_dev, target_idx: u32) -> i32;
    pub fn nfc_genl_device_added(dev: *mut nfc_dev) -> i32;
    pub fn nfc_genl_device_removed(dev: *mut nfc_dev) -> i32;
    pub fn nfc_genl_dep_link_up_event(dev: *mut nfc_dev, target_idx: u32, comm_mode: u8, rf_mode: u8) -> i32;
    pub fn nfc_genl_dep_link_down_event(dev: *mut nfc_dev) -> i32;
    pub fn nfc_genl_tm_activated(dev: *mut nfc_dev, protocol: u32) -> i32;
    pub fn nfc_genl_tm_deactivated(dev: *mut nfc_dev) -> i32;
    pub fn nfc_genl_llc_send_sdres(dev: *mut nfc_dev, sdres_list: *mut hlist_head) -> i32;
    pub fn nfc_genl_se_added(dev: *mut nfc_dev, se_idx: u32, type_: u16) -> i32;
    pub fn nfc_genl_se_removed(dev: *mut nfc_dev, se_idx: u32) -> i32;
    pub fn nfc_genl_se_transaction(dev: *mut nfc_dev, se_idx: u8, evt_transaction: *mut nfc_evt_transaction) -> i32;
    pub fn nfc_genl_se_connectivity(dev: *mut nfc_dev, se_idx: u8) -> i32;
    pub fn nfc_get_device(idx: u32) -> *mut nfc_dev;
}

pub unsafe fn nfc_put_device(dev: *mut nfc_dev) { put_device(&mut (*dev).dev); }
pub unsafe fn nfc_device_iter_init(iter: *mut class_dev_iter) { class_dev_iter_init(iter, &mut nfc_class, std::ptr::null_mut(), std::ptr::null_mut()); }
pub unsafe fn nfc_device_iter_next(iter: *mut class_dev_iter) -> *mut nfc_dev {
    let d = class_dev_iter_next(iter);
    if d.is_null() { return std::ptr::null_mut(); }
    to_nfc_dev(d)
}
pub unsafe fn nfc_device_iter_exit(iter: *mut class_dev_iter) { class_dev_iter_exit(iter); }

extern "C" {
    pub fn nfc_fw_download(dev: *mut nfc_dev, firmware_name: *const i8) -> i32;
    pub fn nfc_genl_fw_download_done(dev: *mut nfc_dev, firmware_name: *const i8, result: u32) -> i32;
    pub fn nfc_dev_up(dev: *mut nfc_dev) -> i32;
    pub fn nfc_dev_down(dev: *mut nfc_dev) -> i32;
    pub fn nfc_start_poll(dev: *mut nfc_dev, im_protocols: u32, tm_protocols: u32) -> i32;
    pub fn nfc_stop_poll(dev: *mut nfc_dev) -> i32;
    pub fn nfc_dep_link_up(dev: *mut nfc_dev, target_idx: i32, comm_mode: u8) -> i32;
    pub fn nfc_dep_link_down(dev: *mut nfc_dev) -> i32;
    pub fn nfc_activate_target(dev: *mut nfc_dev, target_idx: u32, protocol: u32) -> i32;
    pub fn nfc_deactivate_target(dev: *mut nfc_dev, target_idx: u32, mode: u8) -> i32;
    pub fn nfc_data_exchange(dev: *mut nfc_dev, target_idx: u32, skb: *mut sk_buff, cb: data_exchange_cb_t, cb_context: *mut core::ffi::c_void) -> i32;
    pub fn nfc_enable_se(dev: *mut nfc_dev, se_idx: u32) -> i32;
    pub fn nfc_disable_se(dev: *mut nfc_dev, se_idx: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
