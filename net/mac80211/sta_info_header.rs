/* SPDX-License-Identifier: GPL-2.0-only */
/* Direct Rust translation of sta_info.h. C headers and external kernel types
 * are intentionally left as external dependencies. */

#[repr(C)]
pub enum ieee80211_sta_info_flags {
    WLAN_STA_AUTH, WLAN_STA_ASSOC, WLAN_STA_PS_STA, WLAN_STA_AUTHORIZED,
    WLAN_STA_SHORT_PREAMBLE, WLAN_STA_WDS, WLAN_STA_CLEAR_PS_FILT, WLAN_STA_MFP,
    WLAN_STA_BLOCK_BA, WLAN_STA_PS_DRIVER, WLAN_STA_PSPOLL, WLAN_STA_TDLS_PEER,
    WLAN_STA_TDLS_PEER_AUTH, WLAN_STA_TDLS_INITIATOR, WLAN_STA_TDLS_CHAN_SWITCH,
    WLAN_STA_TDLS_OFF_CHANNEL, WLAN_STA_TDLS_WIDER_BW, WLAN_STA_UAPSD, WLAN_STA_SP,
    WLAN_STA_4ADDR_EVENT, WLAN_STA_INSERTED, WLAN_STA_RATE_CONTROL,
    WLAN_STA_TOFFSET_KNOWN, WLAN_STA_MPSP_OWNER, WLAN_STA_MPSP_RECIPIENT,
    WLAN_STA_PS_DELIVER, WLAN_STA_USES_ENCRYPTION, WLAN_STA_DECAP_OFFLOAD,
    NUM_WLAN_STA_FLAGS,
}

pub const ADDBA_RESP_INTERVAL: u32 = HZ;
pub const HT_AGG_MAX_RETRIES: u32 = 15;
pub const HT_AGG_BURST_RETRIES: u32 = 3;
pub const HT_AGG_RETRIES_PERIOD: u32 = 15 * HZ;
pub const HT_AGG_STATE_DRV_READY: u32 = 0;
pub const HT_AGG_STATE_RESPONSE_RECEIVED: u32 = 1;
pub const HT_AGG_STATE_OPERATIONAL: u32 = 2;
pub const HT_AGG_STATE_STOPPING: u32 = 3;
pub const HT_AGG_STATE_WANT_START: u32 = 4;
pub const HT_AGG_STATE_WANT_STOP: u32 = 5;
pub const HT_AGG_STATE_START_CB: u32 = 6;
pub const HT_AGG_STATE_STOP_CB: u32 = 7;
pub const HT_AGG_STATE_SENT_ADDBA: u32 = 8;
pub const AIRTIME_USE_TX: u32 = BIT(0);
pub const AIRTIME_USE_RX: u32 = BIT(1);

#[repr(C)] pub enum ieee80211_agg_stop_reason { AGG_STOP_DECLINED, AGG_STOP_LOCAL_REQUEST, AGG_STOP_PEER_REQUEST, AGG_STOP_DESTROY_STA }

#[repr(C)] pub struct airtime_info {
    pub rx_airtime: u64, pub tx_airtime: u64, pub last_active: c_ulong,
    pub deficit: i32, pub aql_tx_pending: atomic_t, pub aql_limit_low: u32,
    pub aql_limit_high: u32,
}

extern "C" {
    pub fn ieee80211_sta_update_pending_airtime(local: *mut ieee80211_local, sta: *mut sta_info, ac: u8, tx_airtime: u16, tx_completed: bool, mcast: bool);
}

#[repr(C)] pub struct tid_ampdu_tx {
    pub rcu_head: rcu_head, pub session_timer: timer_list, pub addba_resp_timer: timer_list,
    pub pending: sk_buff_head, pub sta: *mut sta_info, pub state: c_ulong, pub last_tx: c_ulong,
    pub timeout: u16, pub dialog_token: u8, pub stop_initiator: u8, pub tx_stop: bool,
    pub buf_size: u16, pub ssn: u16, pub failed_bar_ssn: u16, pub bar_pending: bool,
    pub amsdu: bool, pub ndp: bool, pub tid: u8,
}

#[repr(C)] pub struct tid_ampdu_rx {
    pub rcu_head: rcu_head, pub reorder_lock: spinlock_t, pub reorder_buf_filtered: u64,
    pub sta: *mut sta_info, pub session_timer: timer_list, pub reorder_timer: timer_list,
    pub last_rx: c_ulong, pub head_seq_num: u16, pub stored_mpdu_num: u16, pub ssn: u16,
    pub buf_size: u16, pub timeout: u16, pub tid: u8, pub auto_seq: u8,
    pub removed: u8, pub started: u8,
    /* Flexible array member: reorder buffer entries follow this structure. */
    pub reorder: [tid_ampdu_reorder; 0],
}
#[repr(C)] pub struct tid_ampdu_reorder { pub buf: sk_buff_head, pub time: c_ulong }

#[repr(C)] pub struct sta_ampdu_mlme {
    pub tid_rx: [*mut tid_ampdu_rx; IEEE80211_NUM_TIDS], pub tid_rx_token: [u8; IEEE80211_NUM_TIDS],
    pub tid_rx_timer_expired: [c_ulong; BITS_TO_LONGS(IEEE80211_NUM_TIDS)],
    pub tid_rx_stop_requested: [c_ulong; BITS_TO_LONGS(IEEE80211_NUM_TIDS)],
    pub tid_rx_manage_offl: [c_ulong; BITS_TO_LONGS(2 * IEEE80211_NUM_TIDS)],
    pub agg_session_valid: [c_ulong; BITS_TO_LONGS(IEEE80211_NUM_TIDS)],
    pub unexpected_agg: [c_ulong; BITS_TO_LONGS(IEEE80211_NUM_TIDS)],
    pub work: wiphy_work, pub tid_tx: [*mut tid_ampdu_tx; IEEE80211_NUM_TIDS],
    pub tid_start_tx: [*mut tid_ampdu_tx; IEEE80211_NUM_TIDS],
    pub last_addba_req_time: [c_ulong; IEEE80211_NUM_TIDS],
    pub addba_req_num: [u8; IEEE80211_NUM_TIDS], pub dialog_token_allocator: u8,
}

pub const IEEE80211_TID_UNRESERVED: u8 = 0xff;
pub const IEEE80211_FAST_XMIT_MAX_IV: usize = 18;
#[repr(C)] pub struct ieee80211_fast_tx { pub key: *mut ieee80211_key, pub hdr_len: u8, pub sa_offs: u8, pub da_offs: u8, pub pn_offs: u8, pub band: u8, pub hdr: [u8; 30 + 2 + IEEE80211_FAST_XMIT_MAX_IV + core::mem::size_of::<rfc1042_header>()], pub rcu_head: rcu_head }
#[repr(C)] pub struct ieee80211_fast_rx { pub dev: *mut net_device, pub vif_type: nl80211_iftype, pub vif_addr: [u8; ETH_ALEN], pub rfc1042_hdr: [u8; 6], pub control_port_protocol: be16, pub expected_ds_bits: le16, pub icv_len: u8, pub key: u8, pub internal_forward: u8, pub uses_rss: u8, pub da_offs: u8, pub sa_offs: u8, pub rcu_head: rcu_head }

#[repr(C)] pub struct ieee80211_sta_rx_stats { pub packets: c_ulong, pub last_rx: c_ulong, pub num_duplicates: c_ulong, pub fragments: c_ulong, pub dropped: c_ulong, pub last_signal: i32, pub chains: u8, pub chain_signal_last: [i8; IEEE80211_MAX_CHAINS], pub last_rate: u32, pub syncp: u64_stats_sync, pub bytes: u64_stats_t, pub msdu: [u64_stats_t; IEEE80211_NUM_TIDS + 1] }
pub const IEEE80211_FRAGMENT_MAX: usize = 4;
#[repr(C)] pub struct ieee80211_fragment_entry { pub skb_list: sk_buff_head, pub first_frag_time: c_ulong, pub seq: u16, pub extra_len: u16, pub last_frag: u16, pub rx_queue: u8, pub check_sequential_pn: u8, pub is_protected: u8, pub last_pn: [u8; 6], pub key_color: c_uint }
#[repr(C)] pub struct ieee80211_fragment_cache { pub entries: [ieee80211_fragment_entry; IEEE80211_FRAGMENT_MAX], pub next: c_uint }

#[repr(C)] pub struct ieee80211_sta_removed_link_stats { pub rx_packets: u32, pub tx_packets: u32, pub rx_bytes: u64, pub tx_bytes: u64, pub tx_retries: u32, pub tx_failed: u32, pub rx_dropped_misc: u32, pub beacon_loss_count: u32, pub expected_throughput: u32, pub pertid_stats: ieee80211_pertid_stats }
#[repr(C)] pub struct ieee80211_pertid_stats { pub rx_msdu: u64, pub tx_msdu: u64, pub tx_msdu_retries: u64, pub tx_msdu_failed: u64 }

/* Link-specific station information. */
#[repr(C)] pub struct link_sta_info {
    pub addr: [u8; ETH_ALEN], pub link_id: u8, pub op_mode_nss: u8, pub capa_nss: u8,
    pub link_hash_node: rhlist_head, pub sta: *mut sta_info,
    pub gtk: [*mut ieee80211_key; NUM_DEFAULT_KEYS + NUM_DEFAULT_MGMT_KEYS + NUM_DEFAULT_BEACON_KEYS],
    pub pcpu_rx_stats: *mut ieee80211_sta_rx_stats, pub rx_stats: ieee80211_sta_rx_stats,
    pub rx_stats_avg: rx_stats_avg, pub status_stats: status_stats, pub tx_stats: tx_stats,
    pub op_mode_bw: ieee80211_sta_rx_bandwidth, pub rx_omi_bw_rx: ieee80211_sta_rx_bandwidth,
    pub rx_omi_bw_tx: ieee80211_sta_rx_bandwidth, pub rx_omi_bw_staging: ieee80211_sta_rx_bandwidth,
    pub uhr_usable_tx_width: ieee80211_sta_rx_bandwidth, pub uhr_dbe_enabled: bool,
    pub debugfs_dir: *mut dentry, pub pub_: *mut ieee80211_link_sta,
}
#[repr(C)] pub struct rx_stats_avg { pub signal: ewma_signal, pub chain_signal: [ewma_signal; IEEE80211_MAX_CHAINS] }
#[repr(C)] pub struct status_stats { pub filtered: c_ulong, pub retry_failed: c_ulong, pub retry_count: c_ulong, pub lost_packets: c_uint, pub last_pkt_time: c_ulong, pub msdu_retries: [u64; IEEE80211_NUM_TIDS+1], pub msdu_failed: [u64; IEEE80211_NUM_TIDS+1], pub last_ack: c_ulong, pub last_ack_signal: i8, pub ack_signal_filled: bool, pub avg_ack_signal: ewma_avg_signal }
#[repr(C)] pub struct tx_stats { pub packets: [u64; IEEE80211_NUM_ACS], pub bytes: [u64; IEEE80211_NUM_ACS], pub last_rate: ieee80211_tx_rate, pub last_rate_info: rate_info, pub msdu: [u64; IEEE80211_NUM_TIDS+1] }

#[repr(C)] pub struct sta_info {
    pub list: list_head, pub free_list: list_head, pub rcu_head: rcu_head, pub hash_node: rhlist_head,
    pub addr: [u8; ETH_ALEN], pub local: *mut ieee80211_local, pub sdata: *mut ieee80211_sub_if_data,
    pub ptk: [*mut ieee80211_key; NUM_DEFAULT_KEYS], pub ptk_idx: u8, pub rate_ctrl: *mut rate_control_ref,
    pub rate_ctrl_priv: *mut c_void, pub rate_ctrl_lock: spinlock_t, pub lock: spinlock_t,
    pub fast_tx: *mut ieee80211_fast_tx, pub fast_rx: *mut ieee80211_fast_rx, pub mesh: *mut mesh_sta,
    pub drv_deliver_wk: work_struct, pub listen_interval: u16, pub dead: bool, pub removed: bool, pub uploaded: bool,
    pub sta_state: ieee80211_sta_state, pub _flags: c_ulong, pub ps_lock: spinlock_t,
    pub ps_tx_buf: [sk_buff_head; IEEE80211_NUM_ACS], pub tx_filtered: [sk_buff_head; IEEE80211_NUM_ACS],
    pub driver_buffered_tids: c_ulong, pub txq_buffered_tids: c_ulong, pub assoc_at: u64, pub last_connected: c_long,
    pub last_seq_ctrl: [le16; IEEE80211_NUM_TIDS+1], pub tid_seq: [u16; IEEE80211_QOS_CTL_TID_MASK as usize + 1],
    pub airtime: [airtime_info; IEEE80211_NUM_ACS], pub airtime_weight: u16, pub ampdu_mlme: sta_ampdu_mlme,
    pub debugfs_dir: *mut dentry, pub reserved_tid: u8, pub amsdu_mesh_control: i8, pub tdls_chandef: cfg80211_chan_def,
    pub frags: ieee80211_fragment_cache, pub cur: ieee80211_sta_aggregates, pub deflink: link_sta_info,
    pub link: [*mut link_sta_info; IEEE80211_MLD_MAX_NUM_LINKS], pub rem_link_stats: ieee80211_sta_removed_link_stats,
    pub sta: ieee80211_sta,
}

pub const STA_MAX_TX_BUFFER: u32 = 64;
pub const STA_TX_BUFFER_EXPIRE: u32 = 10 * HZ;
pub const STA_INFO_CLEANUP_INTERVAL: u32 = 10 * HZ;

extern "C" {
    pub fn sta_info_move_state(sta: *mut sta_info, new_state: ieee80211_sta_state) -> c_int;
    pub fn ieee80211_assign_tid_tx(sta: *mut sta_info, tid: c_int, tid_tx: *mut tid_ampdu_tx);
    pub fn sta_info_hash_lookup(local: *mut ieee80211_local, addr: *const u8) -> *mut rhlist_head;
    pub fn sta_info_get(sdata: *mut ieee80211_sub_if_data, addr: *const u8) -> *mut sta_info;
    pub fn sta_info_get_bss(sdata: *mut ieee80211_sub_if_data, addr: *const u8) -> *mut sta_info;
    pub fn sta_info_get_by_addrs(local: *mut ieee80211_local, sta_addr: *const u8, vif_addr: *const u8) -> *mut sta_info;
    pub fn link_sta_info_hash_lookup(local: *mut ieee80211_local, addr: *const u8) -> *mut rhlist_head;
    pub fn link_sta_info_get_bss(sdata: *mut ieee80211_sub_if_data, addr: *const u8) -> *mut link_sta_info;
    pub fn sta_info_get_by_idx(sdata: *mut ieee80211_sub_if_data, idx: c_int) -> *mut sta_info;
    pub fn sta_info_alloc(sdata: *mut ieee80211_sub_if_data, addr: *const u8, gfp: gfp_t) -> *mut sta_info;
    pub fn sta_info_alloc_with_link(sdata: *mut ieee80211_sub_if_data, mld_addr: *const u8, link_id: c_uint, link_addr: *const u8, gfp: gfp_t) -> *mut sta_info;
    pub fn sta_info_free(local: *mut ieee80211_local, sta: *mut sta_info);
    pub fn sta_info_insert(sta: *mut sta_info) -> c_int;
    pub fn sta_info_insert_rcu(sta: *mut sta_info) -> c_int;
    pub fn __sta_info_destroy(sta: *mut sta_info) -> c_int;
    pub fn sta_info_destroy_addr(sdata: *mut ieee80211_sub_if_data, addr: *const u8) -> c_int;
    pub fn sta_info_destroy_addr_bss(sdata: *mut ieee80211_sub_if_data, addr: *const u8) -> c_int;
    pub fn sta_info_recalc_tim(sta: *mut sta_info);
    pub fn sta_info_init(local: *mut ieee80211_local) -> c_int;
    pub fn sta_info_stop(local: *mut ieee80211_local);
    pub fn __sta_info_flush(sdata: *mut ieee80211_sub_if_data, vlans: bool, link_id: c_int, do_not_flush_sta: *mut sta_info) -> c_int;
    pub fn sta_set_rate_info_tx(sta: *mut sta_info, rate: *const ieee80211_tx_rate, rinfo: *mut rate_info);
    pub fn sta_set_sinfo(sta: *mut sta_info, sinfo: *mut station_info, tidstats: bool);
    pub fn sta_set_accumulated_removed_links_sinfo(sta: *mut sta_info, sinfo: *mut station_info);
    pub fn sta_get_expected_throughput(sta: *mut sta_info) -> u32;
    pub fn ieee80211_sta_expire(sdata: *mut ieee80211_sub_if_data, exp_time: c_ulong);
    pub fn ieee80211_sta_allocate_link(sta: *mut sta_info, link_id: c_uint) -> c_int;
    pub fn ieee80211_sta_free_link(sta: *mut sta_info, link_id: c_uint);
    pub fn ieee80211_sta_activate_link(sta: *mut sta_info, link_id: c_uint) -> c_int;
    pub fn ieee80211_sta_remove_link(sta: *mut sta_info, link_id: c_uint);
    pub fn ieee80211_sta_ps_deliver_wakeup(sta: *mut sta_info);
    pub fn ieee80211_sta_ps_deliver_poll_response(sta: *mut sta_info);
    pub fn ieee80211_sta_ps_deliver_uapsd(sta: *mut sta_info);
    pub fn ieee80211_sta_last_active(sta: *mut sta_info, link_id: c_int) -> c_ulong;
    pub fn ieee80211_sta_init_nss_bw_capa(link_sta: *mut link_sta_info, chandef: *mut cfg80211_chan_def);
    pub fn ieee80211_sta_set_max_amsdu_subframes(sta: *mut sta_info, ext_capab: *const u8, ext_capab_len: c_uint);
    pub fn __ieee80211_sta_recalc_aggregates(sta: *mut sta_info, active_links: u16);
    pub fn ieee80211_sta_current_bw(link_sta: *mut link_sta_info, chandef: *mut cfg80211_chan_def, direction: ieee80211_sta_bw_direction) -> ieee80211_sta_rx_bandwidth;
    pub fn ieee80211_link_sta_update_rc_bw(link: *mut ieee80211_link_data, link_sta: *mut link_sta_info) -> bool;
}

#[inline] pub unsafe fn ieee80211_tdls_sta_link_id(sta: *mut sta_info) -> c_int { if (*sta).sta.valid_links != 0 { __ffs((*sta).sta.valid_links) } else { 0 } }
#[inline] pub unsafe fn sta_plink_state(sta: *mut sta_info) -> nl80211_plink_state { (*sta).mesh.as_ref().map_or(NL80211_PLINK_LISTEN, |m| m.plink_state) }
#[inline] pub unsafe fn set_sta_flag(sta: *mut sta_info, flag: ieee80211_sta_info_flags) { WARN_ON(flag as c_int == WLAN_STA_AUTH as c_int || flag as c_int == WLAN_STA_ASSOC as c_int || flag as c_int == WLAN_STA_AUTHORIZED as c_int); set_bit(flag as c_ulong, &mut (*sta)._flags); }
#[inline] pub unsafe fn clear_sta_flag(sta: *mut sta_info, flag: ieee80211_sta_info_flags) { clear_bit(flag as c_ulong, &mut (*sta)._flags); }
#[inline] pub unsafe fn test_sta_flag(sta: *mut sta_info, flag: ieee80211_sta_info_flags) -> c_int { test_bit(flag as c_ulong, &(*sta)._flags) }
#[inline] pub unsafe fn test_and_clear_sta_flag(sta: *mut sta_info, flag: ieee80211_sta_info_flags) -> c_int { test_and_clear_bit(flag as c_ulong, &mut (*sta)._flags) }
#[inline] pub unsafe fn test_and_set_sta_flag(sta: *mut sta_info, flag: ieee80211_sta_info_flags) -> c_int { test_and_set_bit(flag as c_ulong, &mut (*sta)._flags) }

#[repr(C)] pub enum ieee80211_sta_bw_direction { IEEE80211_STA_BW_RX_FROM_STA, IEEE80211_STA_BW_TX_TO_STA }
#[repr(C)] pub enum sta_stats_type { STA_STATS_RATE_TYPE_INVALID = 0, STA_STATS_RATE_TYPE_LEGACY, STA_STATS_RATE_TYPE_HT, STA_STATS_RATE_TYPE_VHT, STA_STATS_RATE_TYPE_HE, STA_STATS_RATE_TYPE_S1G, STA_STATS_RATE_TYPE_EHT, STA_STATS_RATE_TYPE_UHR }
pub const STA_STATS_RATE_INVALID: u32 = 0;
pub const STA_STATS_FIELD_TYPE: u32 = 0x0000000f; pub const STA_STATS_FIELD_BW: u32 = 0x000001f0; pub const STA_STATS_FIELD_RESERVED: u32 = 0x00000e00;
pub const STA_STATS_FIELD_LEGACY_IDX: u32 = 0x0000f000; pub const STA_STATS_FIELD_LEGACY_BAND: u32 = 0x000f0000; pub const STA_STATS_FIELD_HT_MCS: u32 = 0x000ff000; pub const STA_STATS_FIELD_VHT_MCS: u32 = 0x0000f000; pub const STA_STATS_FIELD_VHT_NSS: u32 = 0x000f0000; pub const STA_STATS_FIELD_SGI: u32 = 0x00100000;
pub const STA_STATS_FIELD_HE_MCS: u32 = 0x0000f000; pub const STA_STATS_FIELD_HE_NSS: u32 = 0x000f0000; pub const STA_STATS_FIELD_HE_RU: u32 = 0x00700000; pub const STA_STATS_FIELD_HE_GI: u32 = 0x01800000; pub const STA_STATS_FIELD_HE_DCM: u32 = 0x02000000;
pub const STA_STATS_FIELD_EHT_MCS: u32 = 0x0000f000; pub const STA_STATS_FIELD_EHT_NSS: u32 = 0x000f0000; pub const STA_STATS_FIELD_EHT_RU: u32 = 0x00f00000; pub const STA_STATS_FIELD_EHT_GI: u32 = 0x03000000;
pub const STA_STATS_FIELD_UHR_MCS: u32 = 0x0001f000; pub const STA_STATS_FIELD_UHR_NSS: u32 = 0x001e0000; pub const STA_STATS_FIELD_UHR_RU: u32 = 0x01e00000; pub const STA_STATS_FIELD_UHR_GI: u32 = 0x06000000; pub const STA_STATS_FIELD_UHR_ELR: u32 = 0x08000000; pub const STA_STATS_FIELD_UHR_IM: u32 = 0x10000000;
pub const STA_STATS_FIELD_S1G_MCS: u32 = 0x0000f000; pub const STA_STATS_FIELD_S1G_NSS: u32 = 0x000f0000;

#[inline] pub const fn sta_stats_field(mask: u32, value: u32) -> u32 { (value << mask.trailing_zeros()) & mask }
#[inline] pub const fn sta_stats_get(mask: u32, value: u32) -> u32 { (value & mask) >> mask.trailing_zeros() }
#[inline] pub unsafe fn sta_stats_encode_rate(s: *const ieee80211_rx_status) -> u32 {
    let mut r = sta_stats_field(STA_STATS_FIELD_BW, (*s).bw as u32);
    match (*s).encoding { RX_ENC_HT | RX_ENC_VHT | RX_ENC_S1G => if (*s).enc_flags & RX_ENC_FLAG_SHORT_GI != 0 { r |= sta_stats_field(STA_STATS_FIELD_SGI, 1); }, _ => {} }
    match (*s).encoding {
        RX_ENC_VHT => { r |= sta_stats_field(STA_STATS_FIELD_TYPE, STA_STATS_RATE_TYPE_VHT as u32) | sta_stats_field(STA_STATS_FIELD_VHT_NSS, (*s).nss as u32) | sta_stats_field(STA_STATS_FIELD_VHT_MCS, (*s).rate_idx as u32); }
        RX_ENC_HT => { r |= sta_stats_field(STA_STATS_FIELD_TYPE, STA_STATS_RATE_TYPE_HT as u32) | sta_stats_field(STA_STATS_FIELD_HT_MCS, (*s).rate_idx as u32); }
        RX_ENC_LEGACY => { r |= sta_stats_field(STA_STATS_FIELD_TYPE, STA_STATS_RATE_TYPE_LEGACY as u32) | sta_stats_field(STA_STATS_FIELD_LEGACY_BAND, (*s).band as u32) | sta_stats_field(STA_STATS_FIELD_LEGACY_IDX, (*s).rate_idx as u32); }
        RX_ENC_HE => { r |= sta_stats_field(STA_STATS_FIELD_TYPE, STA_STATS_RATE_TYPE_HE as u32) | sta_stats_field(STA_STATS_FIELD_HE_NSS, (*s).nss as u32) | sta_stats_field(STA_STATS_FIELD_HE_MCS, (*s).rate_idx as u32) | sta_stats_field(STA_STATS_FIELD_HE_GI, (*s).he_gi as u32) | sta_stats_field(STA_STATS_FIELD_HE_RU, (*s).he_ru as u32) | sta_stats_field(STA_STATS_FIELD_HE_DCM, (*s).he_dcm as u32); }
        RX_ENC_EHT => { r |= sta_stats_field(STA_STATS_FIELD_TYPE, STA_STATS_RATE_TYPE_EHT as u32) | sta_stats_field(STA_STATS_FIELD_EHT_NSS, (*s).nss as u32) | sta_stats_field(STA_STATS_FIELD_EHT_MCS, (*s).rate_idx as u32) | sta_stats_field(STA_STATS_FIELD_EHT_GI, (*s).eht.gi as u32) | sta_stats_field(STA_STATS_FIELD_EHT_RU, (*s).eht.ru as u32); }
        RX_ENC_UHR => { r |= sta_stats_field(STA_STATS_FIELD_TYPE, STA_STATS_RATE_TYPE_UHR as u32) | sta_stats_field(STA_STATS_FIELD_UHR_NSS, (*s).nss as u32) | sta_stats_field(STA_STATS_FIELD_UHR_MCS, (*s).rate_idx as u32) | sta_stats_field(STA_STATS_FIELD_UHR_GI, (*s).uhr.gi as u32) | sta_stats_field(STA_STATS_FIELD_UHR_RU, (*s).uhr.ru as u32) | sta_stats_field(STA_STATS_FIELD_UHR_ELR, (*s).uhr.elr as u32) | sta_stats_field(STA_STATS_FIELD_UHR_IM, (*s).uhr.im as u32); }
        RX_ENC_S1G => { r |= sta_stats_field(STA_STATS_FIELD_TYPE, STA_STATS_RATE_TYPE_S1G as u32) | sta_stats_field(STA_STATS_FIELD_S1G_NSS, (*s).nss as u32) | sta_stats_field(STA_STATS_FIELD_S1G_MCS, (*s).rate_idx as u32); }
        _ => return STA_STATS_RATE_INVALID,
    }
    r
}

/* External kernel types, constants and helpers are supplied by dependent headers. */
pub type c_void = core::ffi::c_void; pub type c_int = i32; pub type c_uint = u32; pub type c_long = isize; pub type c_ulong = usize;
extern "C" { pub static HZ: u32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
