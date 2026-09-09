/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2007-2012 Siemens AG
 *
 * Written by:
 * Pavel Smolenskiy <pavel.smolenskiy@gmail.com>
 * Maxim Gorbachyov <maxim.gorbachev@siemens.com>
 * Dmitry Eremin-Solenikov <dbaryshkov@gmail.com>
 * Alexander Smirnov <alex.bluesman.smirnov@gmail.com>
 */

// C dependencies: linux/interrupt.h, linux/mutex.h, linux/hrtimer.h,
// net/cfg802154.h, net/mac802154.h, net/nl802154.h,
// net/ieee802154_netdev.h, and llsec.h.

#[repr(u32)]
pub enum ieee802154_ongoing {
    IEEE802154_IS_SCANNING = 1 << 0,
    IEEE802154_IS_BEACONING = 1 << 1,
    IEEE802154_IS_ASSOCIATING = 1 << 2,
}

#[repr(C)]
pub struct ieee802154_local {
    pub hw: ieee802154_hw,
    pub ops: *const ieee802154_ops,
    pub addr_filt: ieee802154_hw_addr_filt,
    pub phy: *mut wpan_phy,
    pub open_count: i32,
    pub interfaces: list_head,
    pub iflist_mtx: mutex,
    pub workqueue: *mut workqueue_struct,
    pub mac_wq: *mut workqueue_struct,
    pub ifs_timer: hrtimer,
    pub scan_page: u8,
    pub scan_channel: u8,
    pub scan_beacon_req: ieee802154_beacon_req_frame,
    pub scan_req: *mut cfg802154_scan_request,
    pub scan_work: delayed_work,
    pub beacon_interval: c_uint,
    pub beacon: ieee802154_beacon_frame,
    pub beacon_req: *mut cfg802154_beacon_request,
    pub beacon_work: delayed_work,
    pub rx_beacon_list: list_head,
    pub rx_beacon_work: work_struct,
    pub rx_mac_cmd_list: list_head,
    pub rx_mac_cmd_work: work_struct,
    pub assoc_dev: *mut ieee802154_pan_device,
    pub assoc_done: completion,
    pub assoc_addr: __le16,
    pub assoc_status: u8,
    pub assoc_work: work_struct,
    pub started: bool,
    pub suspended: bool,
    pub ongoing: c_ulong,
    pub tasklet: tasklet_struct,
    pub skb_queue: sk_buff_head,
    pub tx_skb: *mut sk_buff,
    pub sync_tx_work: work_struct,
    pub tx_result: i32,
}

pub const IEEE802154_RX_MSG: i32 = 1;

#[repr(u32)]
pub enum ieee802154_sdata_state_bits { SDATA_STATE_RUNNING = 0 }

#[repr(C)]
pub struct ieee802154_sub_if_data {
    pub list: list_head,
    pub wpan_dev: wpan_dev,
    pub local: *mut ieee802154_local,
    pub dev: *mut net_device,
    pub iface_default_filtering: ieee802154_filtering_level,
    pub required_filtering: ieee802154_filtering_level,
    pub state: c_ulong,
    pub name: [c_char; IFNAMSIZ],
    pub sec_mtx: mutex,
    pub sec: mac802154_llsec,
}

extern "C" {
    pub static mac802154_wpan_phy_privid: *const c_void;
}

// The following container_of calls require the kernel's offset-aware macro.
pub unsafe fn hw_to_local(hw: *mut ieee802154_hw) -> *mut ieee802154_local { container_of_hw(hw) }
pub unsafe fn IEEE802154_DEV_TO_SUB_IF(dev: *const net_device) -> *mut ieee802154_sub_if_data { netdev_priv(dev) }
pub unsafe fn IEEE802154_WPAN_DEV_TO_SUB_IF(wpan_dev: *mut wpan_dev) -> *mut ieee802154_sub_if_data { container_of_wpan(wpan_dev) }

pub unsafe fn ieee802154_sdata_running(sdata: *mut ieee802154_sub_if_data) -> bool {
    test_bit(SDATA_STATE_RUNNING as usize, &(*sdata).state)
}

pub unsafe fn ieee802154_get_mac_cmd(skb: *mut sk_buff, mac_cmd: *mut u8) -> i32 {
    let mut mac_pl = ieee802154_mac_cmd_pl { cmd_id: 0 };
    if mac_cb(skb).type_ != IEEE802154_FC_TYPE_MAC_CMD { return -EINVAL; }
    let ret = ieee802154_mac_cmd_pl_pull(skb, &mut mac_pl);
    if ret != 0 { return ret; }
    *mac_cmd = mac_pl.cmd_id;
    0
}

extern "C" {
    pub static mut mac802154_mlme_wpan: ieee802154_mlme_ops;
    pub fn ieee802154_rx(local: *mut ieee802154_local, skb: *mut sk_buff);
    pub fn ieee802154_xmit_sync_worker(work: *mut work_struct);
    pub fn ieee802154_sync_and_hold_queue(local: *mut ieee802154_local) -> i32;
    pub fn ieee802154_mlme_op_pre(local: *mut ieee802154_local) -> i32;
    pub fn ieee802154_mlme_tx(local: *mut ieee802154_local, sdata: *mut ieee802154_sub_if_data, skb: *mut sk_buff) -> i32;
    pub fn ieee802154_mlme_tx_locked(local: *mut ieee802154_local, sdata: *mut ieee802154_sub_if_data, skb: *mut sk_buff) -> i32;
    pub fn ieee802154_mlme_op_post(local: *mut ieee802154_local);
    pub fn ieee802154_mlme_tx_one_locked(local: *mut ieee802154_local, sdata: *mut ieee802154_sub_if_data, skb: *mut sk_buff) -> i32;
    pub fn ieee802154_monitor_start_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
    pub fn ieee802154_subif_start_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
    pub fn ieee802154_xmit_ifs_timer(timer: *mut hrtimer) -> hrtimer_restart;
    pub fn ieee802154_hold_queue(local: *mut ieee802154_local);
    pub fn ieee802154_release_queue(local: *mut ieee802154_local);
    pub fn ieee802154_disable_queue(local: *mut ieee802154_local);
    pub fn mac802154_dev_set_page_channel(dev: *mut net_device, page: u8, chan: u8);
    pub fn mac802154_get_params(dev: *mut net_device, params: *mut ieee802154_llsec_params) -> i32;
    pub fn mac802154_set_params(dev: *mut net_device, params: *const ieee802154_llsec_params, changed: i32) -> i32;
    pub fn mac802154_add_key(dev: *mut net_device, id: *const ieee802154_llsec_key_id, key: *const ieee802154_llsec_key) -> i32;
    pub fn mac802154_del_key(dev: *mut net_device, id: *const ieee802154_llsec_key_id) -> i32;
    pub fn mac802154_add_dev(dev: *mut net_device, llsec_dev: *const ieee802154_llsec_device) -> i32;
    pub fn mac802154_del_dev(dev: *mut net_device, dev_addr: __le64) -> i32;
    pub fn mac802154_add_devkey(dev: *mut net_device, device_addr: __le64, key: *const ieee802154_llsec_device_key) -> i32;
    pub fn mac802154_del_devkey(dev: *mut net_device, device_addr: __le64, key: *const ieee802154_llsec_device_key) -> i32;
    pub fn mac802154_add_seclevel(dev: *mut net_device, sl: *const ieee802154_llsec_seclevel) -> i32;
    pub fn mac802154_del_seclevel(dev: *mut net_device, sl: *const ieee802154_llsec_seclevel) -> i32;
    pub fn mac802154_lock_table(dev: *mut net_device);
    pub fn mac802154_get_table(dev: *mut net_device, t: *mut *mut ieee802154_llsec_table);
    pub fn mac802154_unlock_table(dev: *mut net_device);
    pub fn mac802154_wpan_update_llsec(dev: *mut net_device) -> i32;
    pub fn mac802154_scan_worker(work: *mut work_struct);
    pub fn mac802154_trigger_scan_locked(sdata: *mut ieee802154_sub_if_data, request: *mut cfg802154_scan_request) -> i32;
    pub fn mac802154_abort_scan_locked(local: *mut ieee802154_local, sdata: *mut ieee802154_sub_if_data) -> i32;
    pub fn mac802154_process_beacon(local: *mut ieee802154_local, skb: *mut sk_buff, page: u8, channel: u8) -> i32;
    pub fn mac802154_rx_beacon_worker(work: *mut work_struct);
    pub fn mac802154_beacon_worker(work: *mut work_struct);
    pub fn mac802154_send_beacons_locked(sdata: *mut ieee802154_sub_if_data, request: *mut cfg802154_beacon_request) -> i32;
    pub fn mac802154_stop_beacons_locked(local: *mut ieee802154_local, sdata: *mut ieee802154_sub_if_data) -> i32;
    pub fn mac802154_rx_mac_cmd_worker(work: *mut work_struct);
    pub fn mac802154_perform_association(sdata: *mut ieee802154_sub_if_data, coord: *mut ieee802154_pan_device, short_addr: *mut __le16) -> i32;
    pub fn mac802154_process_association_resp(sdata: *mut ieee802154_sub_if_data, skb: *mut sk_buff) -> i32;
    pub fn mac802154_send_disassociation_notif(sdata: *mut ieee802154_sub_if_data, target: *mut ieee802154_pan_device, reason: u8) -> i32;
    pub fn mac802154_process_disassociation_notif(sdata: *mut ieee802154_sub_if_data, skb: *mut sk_buff) -> i32;
    pub fn mac802154_process_association_req(sdata: *mut ieee802154_sub_if_data, skb: *mut sk_buff) -> i32;
    pub fn ieee802154_iface_init() -> i32;
    pub fn ieee802154_iface_exit();
    pub fn ieee802154_if_remove(sdata: *mut ieee802154_sub_if_data);
    pub fn ieee802154_if_add(local: *mut ieee802154_local, name: *const c_char, name_assign_type: c_uchar, type_: nl802154_iftype, extended_addr: __le64) -> *mut net_device;
    pub fn ieee802154_remove_interfaces(local: *mut ieee802154_local);
    pub fn ieee802154_stop_device(local: *mut ieee802154_local);
}

pub unsafe fn mac802154_is_scanning(local: *mut ieee802154_local) -> bool {
    test_bit(IEEE802154_IS_SCANNING as usize, &(*local).ongoing)
}
pub unsafe fn mac802154_is_beaconing(local: *mut ieee802154_local) -> bool {
    test_bit(IEEE802154_IS_BEACONING as usize, &(*local).ongoing)
}
pub unsafe fn mac802154_is_associating(local: *mut ieee802154_local) -> bool {
    test_bit(IEEE802154_IS_ASSOCIATING as usize, &(*local).ongoing)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
