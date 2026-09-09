/* SPDX-License-Identifier: GPL-2.0 */
// Rust representation of cfg80211's trace header.  The Linux tracepoint
// declarations are kept as zero-sized declaration items; their payload and
// formatting are supplied by the tracepoint implementation used by callers.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

pub const MAXNAME: usize = 32;

#[repr(C)]
pub struct MacEntry { pub entry_mac: [u8; 6] }

#[inline]
pub unsafe fn mac_assign(dst: *mut u8, src: *const u8) {
    if !src.is_null() { core::ptr::copy_nonoverlapping(src, dst, 6); }
    else { core::ptr::write_bytes(dst, 0, 6); }
}

#[inline]
pub const fn bool_to_str(value: bool) -> &'static str { if value { "true" } else { "false" } }

// C tracepoint declaration forms (TRACE_EVENT, DECLARE_EVENT_CLASS and
// DEFINE_EVENT) become declaration-only Rust items.  All argument evaluation,
// field capture, ordering, and formatting remain external tracepoint behavior.
macro_rules! trace_event { ($name:ident) => { #[derive(Copy, Clone, Debug)] pub struct $name; }; }
macro_rules! declare_event_class { ($name:ident) => { #[derive(Copy, Clone, Debug)] pub struct $name; }; }
macro_rules! define_event { ($class:ident, $name:ident) => { #[derive(Copy, Clone, Debug)] pub struct $name; }; }

// Event declarations translated from trace.h.
trace_event!(wiphy_delayed_work_queue); trace_event!(wiphy_hrtimer_work_queue); trace_event!(wiphy_work_worker_start);
trace_event!(rdev_suspend); trace_event!(rdev_return_int); trace_event!(rdev_scan); trace_event!(rdev_get_antenna);
trace_event!(rdev_add_virtual_intf); trace_event!(rdev_change_virtual_intf); trace_event!(rdev_add_key);
trace_event!(rdev_set_default_key); trace_event!(rdev_set_default_mgmt_key); trace_event!(rdev_set_default_beacon_key);
trace_event!(rdev_start_ap); trace_event!(rdev_change_beacon); trace_event!(rdev_stop_ap); trace_event!(rdev_end_cac);
trace_event!(rdev_dump_station); trace_event!(rdev_return_int_station_info); trace_event!(rdev_dump_mpath);
trace_event!(rdev_get_mpp); trace_event!(rdev_dump_mpp); trace_event!(rdev_return_int_mpath_info);
trace_event!(rdev_return_int_mesh_config); trace_event!(rdev_update_mesh_config); trace_event!(rdev_join_mesh);
trace_event!(rdev_change_bss); trace_event!(rdev_inform_bss); trace_event!(rdev_set_txq_params);
trace_event!(rdev_libertas_set_mesh_channel); trace_event!(rdev_set_monitor_channel); trace_event!(rdev_auth);
trace_event!(rdev_assoc); trace_event!(rdev_deauth); trace_event!(rdev_disassoc); trace_event!(rdev_mgmt_tx_cancel_wait);
trace_event!(rdev_set_power_mgmt); trace_event!(rdev_connect); trace_event!(rdev_update_connect_params);
trace_event!(rdev_set_cqm_rssi_config); trace_event!(rdev_set_cqm_rssi_range_config); trace_event!(rdev_set_cqm_txe_config);
trace_event!(rdev_disconnect); trace_event!(rdev_join_ibss); trace_event!(rdev_join_ocb); trace_event!(rdev_set_wiphy_params);
trace_event!(rdev_get_tx_power); trace_event!(rdev_set_tx_power); trace_event!(rdev_return_int_int);
trace_event!(rdev_testmode_cmd); trace_event!(rdev_testmode_dump); trace_event!(rdev_set_bitrate_mask);
trace_event!(rdev_update_mgmt_frame_registrations); trace_event!(rdev_return_int_tx_rx); trace_event!(rdev_return_void_tx_rx);
trace_event!(rdev_set_antenna); trace_event!(rdev_tdls_mgmt); trace_event!(rdev_dump_survey);
trace_event!(rdev_return_int_survey_info); trace_event!(rdev_tdls_oper); trace_event!(rdev_probe_peer);
trace_event!(rdev_remain_on_channel); trace_event!(rdev_return_int_cookie); trace_event!(rdev_cancel_remain_on_channel);
trace_event!(rdev_mgmt_tx); trace_event!(rdev_tx_control_port); trace_event!(rdev_set_noack_map); trace_event!(rdev_return_chandef);
trace_event!(rdev_start_nan); trace_event!(rdev_nan_change_conf); trace_event!(rdev_add_nan_func); trace_event!(rdev_del_nan_func);
trace_event!(rdev_nan_set_local_sched); trace_event!(rdev_nan_set_peer_sched); trace_event!(rdev_set_mac_acl);
trace_event!(rdev_update_ft_ies); trace_event!(rdev_crit_proto_start); trace_event!(rdev_crit_proto_stop);
trace_event!(rdev_channel_switch); trace_event!(rdev_set_qos_map); trace_event!(rdev_set_ap_chanwidth);
trace_event!(rdev_add_tx_ts); trace_event!(rdev_del_tx_ts); trace_event!(rdev_tdls_channel_switch);
trace_event!(rdev_tdls_cancel_channel_switch); trace_event!(rdev_set_pmk); trace_event!(rdev_del_pmk);
trace_event!(rdev_external_auth); trace_event!(rdev_start_radar_detection); trace_event!(rdev_set_mcast_rate);
trace_event!(rdev_set_coalesce); trace_event!(rdev_set_multicast_to_unicast); trace_event!(rdev_get_ftm_responder_stats);
trace_event!(rdev_set_fils_aad); trace_event!(rdev_update_owe_info); trace_event!(rdev_probe_mesh_link);
trace_event!(rdev_set_tid_config); trace_event!(rdev_reset_tid_config); trace_event!(rdev_set_sar_specs);
trace_event!(rdev_color_change); trace_event!(rdev_set_radar_background); trace_event!(rdev_del_link_station);
trace_event!(rdev_set_hw_timestamp); trace_event!(rdev_set_ttlm); trace_event!(rdev_set_epcs);
trace_event!(cfg80211_return_bool); trace_event!(cfg80211_send_rx_assoc); trace_event!(cfg80211_tx_mlme_mgmt);
trace_event!(cfg80211_send_assoc_failure); trace_event!(cfg80211_michael_mic_failure); trace_event!(cfg80211_ready_on_channel);
trace_event!(cfg80211_ready_on_channel_expired); trace_event!(cfg80211_tx_mgmt_expired); trace_event!(cfg80211_new_sta);
trace_event!(cfg80211_rx_mgmt); trace_event!(cfg80211_mgmt_tx_status); trace_event!(cfg80211_control_port_tx_status);
trace_event!(cfg80211_rx_control_port); trace_event!(cfg80211_cqm_rssi_notify); trace_event!(cfg80211_reg_can_beacon);
trace_event!(cfg80211_ch_switch_notify); trace_event!(cfg80211_ch_switch_started_notify); trace_event!(cfg80211_radar_event);
trace_event!(cfg80211_cac_event); trace_event!(cfg80211_ibss_joined); trace_event!(cfg80211_probe_status);
trace_event!(cfg80211_cqm_pktloss_notify); trace_event!(cfg80211_pmksa_candidate_notify); trace_event!(cfg80211_report_obss_beacon);
trace_event!(cfg80211_tdls_oper_request); trace_event!(cfg80211_scan_done); trace_event!(cfg80211_get_bss);
trace_event!(cfg80211_inform_bss_frame); trace_event!(cfg80211_report_wowlan_wakeup); trace_event!(cfg80211_ft_event);
trace_event!(cfg80211_stop_link); trace_event!(cfg80211_pmsr_report); trace_event!(cfg80211_pmsr_complete);
trace_event!(cfg80211_update_owe_info_event); trace_event!(cfg80211_bss_color_notify); trace_event!(cfg80211_assoc_comeback);
trace_event!(cfg80211_links_removed); trace_event!(cfg80211_mlo_reconf_add_done); trace_event!(rdev_assoc_ml_reconf);
trace_event!(cfg80211_epcs_changed); trace_event!(cfg80211_next_nan_dw_notif); trace_event!(cfg80211_nan_cluster_joined);
trace_event!(cfg80211_incumbent_signal_notify); trace_event!(cfg80211_nan_sched_update_done);
trace_event!(cfg80211_nan_ulw_update); trace_event!(cfg80211_nan_channel_evac);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
