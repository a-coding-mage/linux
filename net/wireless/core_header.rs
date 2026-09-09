/* SPDX-License-Identifier: GPL-2.0 */
/* Wireless configuration interface internals. */
/* C headers and build-time configuration are supplied by other translation units. */

pub const WIPHY_IDX_INVALID: i32 = -1;

#[repr(C)]
pub struct cfg80211_scan_request_int { pub info: cfg80211_scan_info, pub notified: bool, pub req: cfg80211_scan_request }

#[repr(C)]
pub struct cfg80211_registered_device {
    pub ops: *const cfg80211_ops, pub list: list_head, pub rfkill_ops: rfkill_ops,
    pub rfkill_block: work_struct, pub country_ie_alpha2: [c_char; 2],
    pub requested_regd: *const ieee80211_regdomain, pub env: environment_cap,
    pub wiphy_idx: c_int, pub devlist_generation: c_int, pub wdev_id: c_int,
    pub opencount: c_int, pub dev_wait: wait_queue_head_t,
    pub beacon_registrations: list_head, pub beacon_registrations_lock: spinlock_t,
    pub num_running_ifaces: c_int, pub num_running_monitor_ifaces: c_int,
    pub cookie_counter: u64, pub bss_lock: spinlock_t, pub bss_list: list_head,
    pub bss_tree: rb_root, pub bss_generation: u32, pub bss_entries: u32,
    pub scan_req: *mut cfg80211_scan_request_int, pub int_scan_req: *mut cfg80211_scan_request_int,
    pub scan_msg: *mut sk_buff, pub sched_scan_req_list: list_head, pub suspend_at: time64_t,
    pub scan_done_wk: wiphy_work, pub cur_cmd_info: *mut genl_info,
    pub conn_work: work_struct, pub event_work: work_struct, pub dfs_update_channels_wk: delayed_work,
    pub background_radar_wdev: *mut wireless_dev, pub background_radar_chandef: cfg80211_chan_def,
    pub background_cac_done_wk: delayed_work, pub background_cac_abort_wk: work_struct,
    pub crit_proto_nlportid: u32, pub coalesce: *mut cfg80211_coalesce, pub destroy_work: work_struct,
    pub sched_scan_stop_wk: wiphy_work, pub sched_scan_res_wk: work_struct,
    pub radar_chandef: cfg80211_chan_def, pub propagate_radar_detect_wk: work_struct,
    pub cac_done_chandef: cfg80211_chan_def, pub propagate_cac_done_wk: work_struct,
    pub mgmt_registrations_update_wk: work_struct, pub mgmt_registrations_lock: spinlock_t,
    pub wiphy_work: work_struct, pub wiphy_work_list: list_head, pub wiphy_work_lock: spinlock_t,
    pub suspended: bool, pub wiphy: wiphy,
}

pub unsafe fn wiphy_to_rdev(wiphy: *mut wiphy) -> *mut cfg80211_registered_device {
    BUG_ON(wiphy.is_null()); container_of!(wiphy, cfg80211_registered_device, wiphy)
}

pub unsafe fn cfg80211_rdev_free_wowlan(rdev: *mut cfg80211_registered_device) {
    /* CONFIG_PM-controlled body; dependent kernel allocation and socket APIs are external. */
    #[cfg(feature = "CONFIG_PM")]
    { if (*rdev).wiphy.wowlan_config.is_null() { return; } /* translated cleanup is supplied externally */ }
}

pub unsafe fn cfg80211_assign_cookie(rdev: *mut cfg80211_registered_device) -> u64 {
    (*rdev).cookie_counter = (*rdev).cookie_counter.wrapping_add(1);
    let mut r = (*rdev).cookie_counter;
    if WARN_ON(r == 0) { (*rdev).cookie_counter = (*rdev).cookie_counter.wrapping_add(1); r = (*rdev).cookie_counter; }
    r
}

extern "C" { pub static mut cfg80211_wq: *mut workqueue_struct; pub static mut cfg80211_rdev_list: list_head; pub static mut cfg80211_rdev_list_generation: c_int; }
pub fn for_each_rdev_check_rtnl() -> c_int { ASSERT_RTNL(); 0 }
/* C macro for_each_rdev(rdev): if for_each_rdev_check_rtnl() {} else list_for_each_entry(...). */

#[repr(C)] pub enum bss_source_type { BSS_SOURCE_DIRECT = 0, BSS_SOURCE_MBSSID, BSS_SOURCE_STA_PROFILE }
#[repr(C)] pub struct cfg80211_internal_bss {
    pub list: list_head, pub hidden_list: list_head, pub rbn: rb_node, pub ts: c_ulong,
    pub refcount: c_ulong, pub hold: atomic_t, pub parent_tsf: u64, pub parent_bssid: [u8; ETH_ALEN],
    pub bss_source: bss_source_type, pub pub_: cfg80211_bss,
}
pub unsafe fn bss_from_pub(pub_: *mut cfg80211_bss) -> *mut cfg80211_internal_bss { container_of!(pub_, cfg80211_internal_bss, pub_) }
pub unsafe fn cfg80211_hold_bss(mut bss: *mut cfg80211_internal_bss) { atomic_inc(&mut (*bss).hold); if !(*bss).pub_.transmitted_bss.is_null() { bss = bss_from_pub((*bss).pub_.transmitted_bss); atomic_inc(&mut (*bss).hold); } }
pub unsafe fn cfg80211_unhold_bss(mut bss: *mut cfg80211_internal_bss) { let mut r = atomic_dec_return(&mut (*bss).hold); WARN_ON(r < 0); if !(*bss).pub_.transmitted_bss.is_null() { bss = bss_from_pub((*bss).pub_.transmitted_bss); r = atomic_dec_return(&mut (*bss).hold); WARN_ON(r < 0); } }

#[repr(C)] pub struct cfg80211_beacon_registration { pub list: list_head, pub nlportid: u32 }
#[repr(C)] pub struct cfg80211_cqm_config { pub rcu_head: rcu_head, pub rssi_hyst: u32, pub last_rssi_event_value: i32, pub last_rssi_event_type: nl80211_cqm_rssi_threshold_event, pub use_range_api: bool, pub n_rssi_thresholds: c_int, pub rssi_thresholds: [i32; 0] }

#[repr(C)] pub enum cfg80211_event_type { EVENT_CONNECT_RESULT, EVENT_ROAMED, EVENT_DISCONNECTED, EVENT_IBSS_JOINED, EVENT_STOPPED, EVENT_PORT_AUTHORIZED }
#[repr(C)] pub struct cfg80211_event { pub list: list_head, pub type_: cfg80211_event_type, pub data: cfg80211_event_data, pub link_id: c_int }
#[repr(C)] pub union cfg80211_event_data { pub cr: cfg80211_connect_resp_params, pub rm: cfg80211_roam_info, pub dc: cfg80211_event_dc, pub ij: cfg80211_event_ij, pub pa: cfg80211_event_pa }
#[repr(C)] pub struct cfg80211_event_dc { pub ie: *const u8, pub ie_len: usize, pub reason: u16, pub locally_generated: bool }
#[repr(C)] pub struct cfg80211_event_ij { pub bssid: [u8; ETH_ALEN], pub channel: *mut ieee80211_channel }
#[repr(C)] pub struct cfg80211_event_pa { pub peer_addr: [u8; ETH_ALEN], pub td_bitmap: *const u8, pub td_bitmap_len: u8 }
#[repr(C)] pub struct cfg80211_cached_keys { pub params: [key_params; 4], pub data: [[u8; WLAN_KEY_LEN_WEP104]; 4], pub def: c_int }

extern "C" {
    pub fn cfg80211_rdev_by_wiphy_idx(wiphy_idx: c_int) -> *mut cfg80211_registered_device;
    pub fn get_wiphy_idx(wiphy: *mut wiphy) -> c_int; pub fn wiphy_idx_to_wiphy(wiphy_idx: c_int) -> *mut wiphy;
    pub fn cfg80211_switch_netns(rdev: *mut cfg80211_registered_device, net: *mut net) -> c_int;
    pub fn cfg80211_init_wdev(wdev: *mut wireless_dev); pub fn cfg80211_register_wdev(rdev: *mut cfg80211_registered_device, wdev: *mut wireless_dev);
    pub fn cfg80211_cqm_rssi_notify_work(wiphy: *mut wiphy, work: *mut wiphy_work);
    pub fn cfg80211_destroy_ifaces(rdev: *mut cfg80211_registered_device); pub fn cfg80211_close_dependents(rdev: *mut cfg80211_registered_device, wdev: *mut wireless_dev);
    pub fn cfg80211_dev_free(rdev: *mut cfg80211_registered_device); pub fn cfg80211_dev_rename(rdev: *mut cfg80211_registered_device, newname: *mut c_char) -> c_int;
    pub fn ieee80211_set_bitrate_flags(wiphy: *mut wiphy); pub fn cfg80211_bss_expire(rdev: *mut cfg80211_registered_device); pub fn cfg80211_bss_age(rdev: *mut cfg80211_registered_device, age_secs: c_ulong);
    pub fn cfg80211_update_assoc_bss_entry(wdev: *mut wireless_dev, link: c_uint, channel: *mut ieee80211_channel);
    pub fn cfg80211_scan(rdev: *mut cfg80211_registered_device) -> c_int;
    pub static mut cfg80211_disconnect_work: work_struct;
}

/* Remaining declarations retain their C ABI and are provided by other translation units. */
extern "C" {
    pub static default_mesh_config: mesh_config; pub static default_mesh_setup: mesh_setup;
    pub fn __cfg80211_join_ibss(rdev: *mut cfg80211_registered_device, dev: *mut net_device, params: *mut cfg80211_ibss_params, connkeys: *mut cfg80211_cached_keys) -> c_int;
    pub fn cfg80211_clear_ibss(dev: *mut net_device, nowext: bool); pub fn cfg80211_leave_ibss(rdev: *mut cfg80211_registered_device, dev: *mut net_device, nowext: bool) -> c_int;
    pub fn __cfg80211_ibss_joined(dev: *mut net_device, bssid: *const u8, channel: *mut ieee80211_channel);
    pub fn cfg80211_ibss_wext_join(rdev: *mut cfg80211_registered_device, wdev: *mut wireless_dev) -> c_int;
    pub fn __cfg80211_join_mesh(rdev: *mut cfg80211_registered_device, dev: *mut net_device, setup: *mut mesh_setup, conf: *const mesh_config) -> c_int;
    pub fn cfg80211_leave_mesh(rdev: *mut cfg80211_registered_device, dev: *mut net_device) -> c_int;
    pub fn cfg80211_set_mesh_channel(rdev: *mut cfg80211_registered_device, wdev: *mut wireless_dev, chandef: *mut cfg80211_chan_def) -> c_int;
    pub fn cfg80211_join_ocb(rdev: *mut cfg80211_registered_device, dev: *mut net_device, setup: *mut ocb_setup) -> c_int; pub fn cfg80211_leave_ocb(rdev: *mut cfg80211_registered_device, dev: *mut net_device) -> c_int;
    pub fn cfg80211_stop_ap(rdev: *mut cfg80211_registered_device, dev: *mut net_device, link: c_int, notify: bool) -> c_int;
    pub fn cfg80211_mlme_auth(rdev: *mut cfg80211_registered_device, dev: *mut net_device, req: *mut cfg80211_auth_request) -> c_int;
    pub fn cfg80211_mlme_assoc(rdev: *mut cfg80211_registered_device, dev: *mut net_device, req: *mut cfg80211_assoc_request, extack: *mut netlink_ext_ack) -> c_int;
    pub fn cfg80211_mlme_deauth(rdev: *mut cfg80211_registered_device, dev: *mut net_device, bssid: *const u8, ie: *const u8, ie_len: c_int, reason: u16, local_state_change: bool) -> c_int;
    pub fn cfg80211_mlme_disassoc(rdev: *mut cfg80211_registered_device, dev: *mut net_device, ap_addr: *const u8, ie: *const u8, ie_len: c_int, reason: u16, local_state_change: bool) -> c_int;
    pub fn cfg80211_mlme_down(rdev: *mut cfg80211_registered_device, dev: *mut net_device);
    pub fn cfg80211_mlme_register_mgmt(wdev: *mut wireless_dev, snd_pid: u32, frame_type: u16, match_data: *const u8, match_len: c_int, multicast_rx: bool, extack: *mut netlink_ext_ack) -> c_int;
    pub fn cfg80211_mgmt_registrations_update_wk(wk: *mut work_struct); pub fn cfg80211_mlme_unregister_socket(wdev: *mut wireless_dev, nlpid: u32); pub fn cfg80211_mlme_purge_registrations(wdev: *mut wireless_dev);
    pub fn cfg80211_mlme_mgmt_tx(rdev: *mut cfg80211_registered_device, wdev: *mut wireless_dev, params: *mut cfg80211_mgmt_tx_params, cookie: u64) -> c_int;
    pub fn cfg80211_oper_and_ht_capa(ht_capa: *mut ieee80211_ht_cap, mask: *const ieee80211_ht_cap); pub fn cfg80211_oper_and_vht_capa(vht_capa: *mut ieee80211_vht_cap, mask: *const ieee80211_vht_cap);
    pub fn cfg80211_connect(rdev: *mut cfg80211_registered_device, dev: *mut net_device, connect: *mut cfg80211_connect_params, connkeys: *mut cfg80211_cached_keys, prev_bssid: *const u8) -> c_int;
    pub fn __cfg80211_connect_result(dev: *mut net_device, params: *mut cfg80211_connect_resp_params, wextev: bool); pub fn __cfg80211_disconnected(dev: *mut net_device, ie: *const u8, ie_len: usize, reason: u16, from_ap: bool);
    pub fn cfg80211_disconnect(rdev: *mut cfg80211_registered_device, dev: *mut net_device, reason: u16, wextev: bool) -> c_int; pub fn __cfg80211_roamed(wdev: *mut wireless_dev, info: *mut cfg80211_roam_info);
    pub fn __cfg80211_port_authorized(wdev: *mut wireless_dev, peer_addr: *const u8, td_bitmap: *const u8, td_bitmap_len: u8);
    pub fn cfg80211_mgd_wext_connect(rdev: *mut cfg80211_registered_device, wdev: *mut wireless_dev) -> c_int; pub fn cfg80211_autodisconnect_wk(wiphy: *mut wiphy, work: *mut wiphy_work);
    pub fn cfg80211_conn_work(work: *mut work_struct); pub fn cfg80211_sme_scan_done(dev: *mut net_device); pub fn cfg80211_sme_rx_assoc_resp(wdev: *mut wireless_dev, status: u16) -> bool;
    pub fn cfg80211_sme_rx_auth(wdev: *mut wireless_dev, buf: *const u8, len: usize); pub fn cfg80211_sme_disassoc(wdev: *mut wireless_dev); pub fn cfg80211_sme_deauth(wdev: *mut wireless_dev); pub fn cfg80211_sme_auth_timeout(wdev: *mut wireless_dev); pub fn cfg80211_sme_assoc_timeout(wdev: *mut wireless_dev); pub fn cfg80211_sme_abandon_assoc(wdev: *mut wireless_dev);
    pub fn cfg80211_supported_cipher_suite(wiphy: *mut wiphy, cipher: u32) -> bool; pub fn cfg80211_valid_key_idx(wdev: *mut wireless_dev, key_idx: c_int, pairwise: bool, mac_addr: *const u8) -> bool;
    pub fn cfg80211_validate_key_settings(rdev: *mut cfg80211_registered_device, wdev: *mut wireless_dev, params: *mut key_params, key_idx: c_int, pairwise: bool, mac_addr: *const u8) -> c_int;
    pub fn __cfg80211_scan_done(wiphy: *mut wiphy, wk: *mut wiphy_work); pub fn ___cfg80211_scan_done(rdev: *mut cfg80211_registered_device, send_message: bool);
    pub fn cfg80211_add_sched_scan_req(rdev: *mut cfg80211_registered_device, req: *mut cfg80211_sched_scan_request); pub fn cfg80211_sched_scan_req_possible(rdev: *mut cfg80211_registered_device, want_multi: bool) -> c_int;
    pub fn cfg80211_sched_scan_results_wk(work: *mut work_struct); pub fn cfg80211_stop_sched_scan_req(rdev: *mut cfg80211_registered_device, req: *mut cfg80211_sched_scan_request, driver_initiated: bool) -> c_int; pub fn __cfg80211_stop_sched_scan(rdev: *mut cfg80211_registered_device, reqid: u64, driver_initiated: bool) -> c_int;
    pub fn cfg80211_upload_connect_keys(wdev: *mut wireless_dev); pub fn cfg80211_change_iface(rdev: *mut cfg80211_registered_device, dev: *mut net_device, ntype: nl80211_iftype, params: *mut vif_params) -> c_int;
    pub fn cfg80211_process_rdev_events(rdev: *mut cfg80211_registered_device); pub fn cfg80211_process_wiphy_works(rdev: *mut cfg80211_registered_device, end: *mut wiphy_work); pub fn cfg80211_process_wdev_events(wdev: *mut wireless_dev);
    pub fn cfg80211_does_bw_fit_range(freq_range: *const ieee80211_freq_range, center_freq_khz: u32, bw_khz: u32) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
