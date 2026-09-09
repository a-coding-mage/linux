/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2008, 2009 open80211s Ltd.
 * Copyright (C) 2023-2024 Intel Corporation
 * Authors:    Luis Carlos Cobo <luisca@cozybit.com>
 *             Javier Cardona <javier@cozybit.com>
 */

/* Dependencies supplied by the surrounding kernel translation. */

/* Data structures */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mesh_path_flags {
    MESH_PATH_ACTIVE = 1 << 0,
    MESH_PATH_RESOLVING = 1 << 1,
    MESH_PATH_SN_VALID = 1 << 2,
    MESH_PATH_FIXED = 1 << 3,
    MESH_PATH_RESOLVED = 1 << 4,
    MESH_PATH_REQ_QUEUED = 1 << 5,
    MESH_PATH_DELETED = 1 << 6,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mesh_deferred_task_flags {
    MESH_WORK_HOUSEKEEPING,
    MESH_WORK_ROOT,
    MESH_WORK_DRIFT_ADJUST,
    MESH_WORK_MBSS_CHANGED,
}

#[repr(C)]
pub struct mesh_path {
    pub dst: [u8; ETH_ALEN],
    pub mpp: [u8; ETH_ALEN],
    pub rhash: rhash_head,
    pub walk_list: hlist_node,
    pub gate_list: hlist_node,
    pub sdata: *mut ieee80211_sub_if_data,
    pub next_hop: *mut sta_info,
    pub timer: timer_list,
    pub frame_queue: sk_buff_head,
    pub rcu: rcu_head,
    pub sn: u32,
    pub metric: u32,
    pub hop_count: u8,
    pub exp_time: c_ulong,
    pub discovery_timeout: u32,
    pub discovery_retries: u8,
    pub flags: mesh_path_flags,
    pub state_lock: spinlock_t,
    pub rann_snd_addr: [u8; ETH_ALEN],
    pub rann_metric: u32,
    pub last_preq_to_root: c_ulong,
    pub fast_tx_check: c_ulong,
    pub is_root: bool,
    pub is_gate: bool,
    pub path_change_count: u32,
}

pub const MESH_FAST_TX_CACHE_MAX_SIZE: u32 = 512;
pub const MESH_FAST_TX_CACHE_THRESHOLD_SIZE: u32 = 384;
pub const MESH_FAST_TX_CACHE_TIMEOUT: u32 = 8000;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ieee80211_mesh_fast_tx_type {
    MESH_FAST_TX_TYPE_LOCAL,
    MESH_FAST_TX_TYPE_PROXIED,
    MESH_FAST_TX_TYPE_FORWARDED,
    NUM_MESH_FAST_TX_TYPE,
}

#[repr(C)]
pub struct ieee80211_mesh_fast_tx_key {
    pub addr: [u8; ETH_ALEN],
    pub r#type: u16,
}

#[repr(C)]
pub struct ieee80211_mesh_fast_tx {
    pub rhash: rhash_head,
    pub key: ieee80211_mesh_fast_tx_key,
    pub fast_tx: ieee80211_fast_tx,
    pub hdr: [u8; core::mem::size_of::<ieee80211s_hdr>() + core::mem::size_of::<rfc1042_header>()],
    pub hdrlen: u16,
    pub mpath: *mut mesh_path,
    pub mppath: *mut mesh_path,
    pub walk_list: hlist_node,
    pub timestamp: c_ulong,
}

pub const RMC_BUCKETS: usize = 256;
pub const RMC_QUEUE_MAX_LEN: u32 = 4;
pub const RMC_TIMEOUT: u32 = 3 * HZ;

#[repr(C)]
pub struct rmc_entry {
    pub list: hlist_node,
    pub exp_time: c_ulong,
    pub seqnum: u32,
    pub sa: [u8; ETH_ALEN],
}

#[repr(C)]
pub struct mesh_rmc {
    pub bucket: [hlist_head; RMC_BUCKETS],
    pub idx_mask: u32,
}

pub const IEEE80211_MESH_HOUSEKEEPING_INTERVAL: u32 = 60 * HZ;
pub const MESH_PATH_EXPIRE: u32 = 600 * HZ;
pub const MESH_MAX_PLINKS: u32 = 256;
pub const MESH_MAX_MPATHS: u32 = 1024;
pub const MESH_FRAME_QUEUE_LEN: u32 = 10;

extern "C" {
    pub fn ieee80211_fill_mesh_addresses(hdr: *mut ieee80211_hdr, fc: *mut __le16, da: *const u8, sa: *const u8) -> c_int;
    pub fn ieee80211_new_mesh_header(sdata: *mut ieee80211_sub_if_data, meshhdr: *mut ieee80211s_hdr, addr4or5: *const c_char, addr6: *const c_char) -> c_uint;
    pub fn mesh_rmc_check(sdata: *mut ieee80211_sub_if_data, addr: *const u8, mesh_hdr: *mut ieee80211s_hdr) -> c_int;
    pub fn mesh_matches_local(sdata: *mut ieee80211_sub_if_data, ie: *mut ieee802_11_elems) -> bool;
    pub fn mesh_add_meshconf_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn mesh_add_meshid_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn mesh_add_rsn_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn mesh_add_vendor_ies(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn mesh_add_ht_cap_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn mesh_add_ht_oper_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn mesh_add_vht_cap_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn mesh_add_vht_oper_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn mesh_add_he_cap_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff, ie_len: u8) -> c_int;
    pub fn mesh_add_he_oper_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn mesh_add_he_6ghz_cap_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn mesh_add_eht_cap_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff, ie_len: u8) -> c_int;
    pub fn mesh_add_eht_oper_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn mesh_rmc_free(sdata: *mut ieee80211_sub_if_data);
    pub fn mesh_rmc_init(sdata: *mut ieee80211_sub_if_data) -> c_int;
    pub fn ieee80211s_init();
    pub fn ieee80211s_update_metric(local: *mut ieee80211_local, sta: *mut sta_info, st: *mut ieee80211_tx_status);
    pub fn ieee80211_mesh_init_sdata(sdata: *mut ieee80211_sub_if_data);
    pub fn ieee80211_mesh_teardown_sdata(sdata: *mut ieee80211_sub_if_data);
    pub fn ieee80211_start_mesh(sdata: *mut ieee80211_sub_if_data) -> c_int;
    pub fn ieee80211_stop_mesh(sdata: *mut ieee80211_sub_if_data);
    pub fn ieee80211_mesh_root_setup(ifmsh: *mut ieee80211_if_mesh);
    pub fn ieee80211_mesh_sync_ops_get(method: u8) -> *const ieee80211_mesh_sync_ops;
    pub fn ieee80211_mbss_info_change_notify(sdata: *mut ieee80211_sub_if_data, changed: u64);
    pub fn ieee80211_mps_local_status_update(sdata: *mut ieee80211_sub_if_data) -> u64;
    pub fn ieee80211_mps_set_sta_local_pm(sta: *mut sta_info, pm: nl80211_mesh_power_mode) -> u64;
    pub fn ieee80211_mps_set_frame_flags(sdata: *mut ieee80211_sub_if_data, sta: *mut sta_info, hdr: *mut ieee80211_hdr);
    pub fn ieee80211_mps_sta_status_update(sta: *mut sta_info);
    pub fn ieee80211_mps_rx_h_sta_process(sta: *mut sta_info, hdr: *mut ieee80211_hdr);
    pub fn ieee80211_mpsp_trigger_process(qc: *mut u8, sta: *mut sta_info, tx: bool, acked: bool);
    pub fn ieee80211_mps_frame_release(sta: *mut sta_info, elems: *mut ieee802_11_elems);
    pub fn mesh_nexthop_lookup(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn mesh_nexthop_resolve(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn mesh_path_start_discovery(sdata: *mut ieee80211_sub_if_data);
    pub fn mesh_path_lookup(sdata: *mut ieee80211_sub_if_data, dst: *const u8) -> *mut mesh_path;
    pub fn mpp_path_lookup(sdata: *mut ieee80211_sub_if_data, dst: *const u8) -> *mut mesh_path;
    pub fn mpp_path_add(sdata: *mut ieee80211_sub_if_data, dst: *const u8, mpp: *const u8) -> c_int;
    pub fn mesh_path_lookup_by_idx(sdata: *mut ieee80211_sub_if_data, idx: c_int) -> *mut mesh_path;
    pub fn mpp_path_lookup_by_idx(sdata: *mut ieee80211_sub_if_data, idx: c_int) -> *mut mesh_path;
    pub fn mesh_path_fix_nexthop(mpath: *mut mesh_path, next_hop: *mut sta_info);
    pub fn mesh_path_expire(sdata: *mut ieee80211_sub_if_data);
    pub fn mesh_rx_path_sel_frame(sdata: *mut ieee80211_sub_if_data, mgmt: *mut ieee80211_mgmt, len: usize);
    pub fn mesh_path_add(sdata: *mut ieee80211_sub_if_data, dst: *const u8) -> *mut mesh_path;
    pub fn mesh_path_add_gate(mpath: *mut mesh_path) -> c_int;
    pub fn mesh_path_send_to_gates(mpath: *mut mesh_path) -> c_int;
    pub fn mesh_gate_num(sdata: *mut ieee80211_sub_if_data) -> c_int;
    pub fn airtime_link_metric_get(local: *mut ieee80211_local, sta: *mut sta_info) -> u32;
    pub fn mesh_neighbour_update(sdata: *mut ieee80211_sub_if_data, hw_addr: *mut u8, ie: *mut ieee802_11_elems, rx_status: *mut ieee80211_rx_status);
    pub fn mesh_peer_accepts_plinks(ie: *mut ieee802_11_elems) -> bool;
    pub fn mesh_accept_plinks_update(sdata: *mut ieee80211_sub_if_data) -> u64;
    pub fn mesh_plink_timer(t: *mut timer_list);
    pub fn mesh_plink_broken(sta: *mut sta_info);
    pub fn mesh_plink_deactivate(sta: *mut sta_info) -> u64;
    pub fn mesh_plink_open(sta: *mut sta_info) -> u64;
    pub fn mesh_plink_block(sta: *mut sta_info) -> u64;
    pub fn mesh_rx_plink_frame(sdata: *mut ieee80211_sub_if_data, mgmt: *mut ieee80211_mgmt, len: usize, rx_status: *mut ieee80211_rx_status);
    pub fn mesh_sta_cleanup(sta: *mut sta_info);
    pub fn mesh_path_error_tx(sdata: *mut ieee80211_sub_if_data, ttl: u8, target: *const u8, target_sn: u32, target_rcode: u16, ra: *const u8) -> c_int;
    pub fn mesh_path_assign_nexthop(mpath: *mut mesh_path, sta: *mut sta_info);
    pub fn mesh_path_flush_pending(mpath: *mut mesh_path);
    pub fn mesh_path_tx_pending(mpath: *mut mesh_path);
    pub fn mesh_pathtbl_init(sdata: *mut ieee80211_sub_if_data);
    pub fn mesh_pathtbl_unregister(sdata: *mut ieee80211_sub_if_data);
    pub fn mesh_path_del(sdata: *mut ieee80211_sub_if_data, addr: *const u8) -> c_int;
    pub fn mesh_path_timer(t: *mut timer_list);
    pub fn mesh_path_flush_by_nexthop(sta: *mut sta_info);
    pub fn mesh_path_discard_frame(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff);
    pub fn mesh_path_tx_root_frame(sdata: *mut ieee80211_sub_if_data);
    pub fn mesh_action_is_path_sel(mgmt: *mut ieee80211_mgmt) -> bool;
    pub fn mesh_fast_tx_get(sdata: *mut ieee80211_sub_if_data, key: *mut ieee80211_mesh_fast_tx_key) -> *mut ieee80211_mesh_fast_tx;
    pub fn ieee80211_mesh_xmit_fast(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff, ctrl_flags: u32) -> bool;
    pub fn mesh_fast_tx_cache(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff, mpath: *mut mesh_path);
    pub fn mesh_fast_tx_gc(sdata: *mut ieee80211_sub_if_data);
    pub fn mesh_fast_tx_flush_addr(sdata: *mut ieee80211_sub_if_data, addr: *const u8);
    pub fn mesh_fast_tx_flush_mpath(mpath: *mut mesh_path);
    pub fn mesh_fast_tx_flush_sta(sdata: *mut ieee80211_sub_if_data, sta: *mut sta_info);
    pub fn mesh_path_refresh(sdata: *mut ieee80211_sub_if_data, mpath: *mut mesh_path, addr: *const u8);
}

#[cfg(feature = "CONFIG_MAC80211_MESH")]
pub unsafe fn mesh_plink_inc_estab_count(sdata: *mut ieee80211_sub_if_data) -> u64 {
    atomic_inc(&mut (*sdata).u.mesh.estab_plinks);
    mesh_accept_plinks_update(sdata) | BSS_CHANGED_BEACON
}

#[cfg(feature = "CONFIG_MAC80211_MESH")]
pub unsafe fn mesh_plink_dec_estab_count(sdata: *mut ieee80211_sub_if_data) -> u64 {
    atomic_dec(&mut (*sdata).u.mesh.estab_plinks);
    mesh_accept_plinks_update(sdata) | BSS_CHANGED_BEACON
}

#[cfg(feature = "CONFIG_MAC80211_MESH")]
pub unsafe fn mesh_plink_free_count(sdata: *mut ieee80211_sub_if_data) -> c_int {
    (*sdata).u.mesh.mshcfg.dot11MeshMaxPeerLinks as c_int - atomic_read(&(*sdata).u.mesh.estab_plinks)
}

#[cfg(feature = "CONFIG_MAC80211_MESH")]
pub unsafe fn mesh_plink_availables(sdata: *mut ieee80211_sub_if_data) -> bool {
    core::cmp::min(mesh_plink_free_count(sdata) as c_long, MESH_MAX_PLINKS as c_long - (*sdata).local.as_ref().unwrap().num_sta as c_long) > 0
}

#[cfg(feature = "CONFIG_MAC80211_MESH")]
pub unsafe fn mesh_path_activate(mpath: *mut mesh_path) {
    (*mpath).flags = ((*mpath).flags as u32 | MESH_PATH_ACTIVE as u32 | MESH_PATH_RESOLVED as u32) as mesh_path_flags;
}

#[cfg(feature = "CONFIG_MAC80211_MESH")]
pub unsafe fn mesh_path_sel_is_hwmp(sdata: *mut ieee80211_sub_if_data) -> bool {
    (*sdata).u.mesh.mesh_pp_id == IEEE80211_PATH_PROTOCOL_HWMP
}

#[cfg(feature = "CONFIG_MAC80211_MESH")]
extern "C" {
    pub fn mesh_path_flush_by_iface(sdata: *mut ieee80211_sub_if_data);
    pub fn mesh_sync_adjust_tsf(sdata: *mut ieee80211_sub_if_data);
    pub fn ieee80211s_stop();
}

#[cfg(not(feature = "CONFIG_MAC80211_MESH"))]
pub unsafe fn mesh_path_sel_is_hwmp(_sdata: *mut ieee80211_sub_if_data) -> bool { false }

#[cfg(not(feature = "CONFIG_MAC80211_MESH"))]
pub unsafe fn mesh_path_flush_by_iface(_sdata: *mut ieee80211_sub_if_data) {}

#[cfg(not(feature = "CONFIG_MAC80211_MESH"))]
pub unsafe fn ieee80211s_stop() {}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
