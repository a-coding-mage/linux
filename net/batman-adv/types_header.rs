/* SPDX-License-Identifier: GPL-2.0 */
/* Faithful Rust translation of types.h; external kernel/project types are dependencies. */

#[cfg(feature = "CONFIG_BATMAN_ADV_DAT")]
pub type batadv_dat_addr_t = u16;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum batadv_dhcp_recipient { BATADV_DHCP_NO = 0, BATADV_DHCP_TO_SERVER, BATADV_DHCP_TO_CLIENT }

pub const BATADV_TT_REMOTE_MASK: u16 = 0x00FF;
pub const BATADV_TT_SYNC_MASK: u16 = 0x00F0;

#[repr(C)] pub struct batadv_ogm_buf { pub buf: *mut core::ffi::c_void, pub len: usize, pub capacity: usize, pub header_length: usize }
#[repr(C)] pub struct batadv_hard_iface_bat_iv { pub ogm_buff: batadv_ogm_buf, pub ogm_seqno: atomic_t, pub reschedule_work: delayed_work, pub ogm_buff_mutex: mutex }
#[repr(C)] pub struct batadv_hard_iface_bat_v { pub elp_interval: u32, pub elp_seqno: atomic_t, pub elp_skb: *mut sk_buff, pub elp_wq: delayed_work, pub aggr_wq: delayed_work, pub aggr_list: sk_buff_head, pub aggr_list_enabled: u8, pub aggr_len: core::ffi::c_uint, pub throughput_override: u32, pub flags: u8 }
#[repr(C)] pub enum batadv_v_hard_iface_flags { BATADV_FULL_DUPLEX = 1 << 0, BATADV_WARNING_DEFAULT = 1 << 1 }
#[repr(C)] pub enum batadv_hard_iface_wifi_flags { BATADV_HARDIF_WIFI_WEXT_DIRECT = 1 << 0, BATADV_HARDIF_WIFI_CFG80211_DIRECT = 1 << 1, BATADV_HARDIF_WIFI_WEXT_INDIRECT = 1 << 2, BATADV_HARDIF_WIFI_CFG80211_INDIRECT = 1 << 3 }

#[repr(C)] pub struct batadv_wifi_net_device_state { pub l: rhash_head, pub netdev: *mut net_device, pub dev_tracker: netdevice_tracker, pub wifi_flags: u32, pub rcu: rcu_head }
#[repr(C)] pub struct batadv_hard_iface {
    pub if_status: i8, pub num_bcasts: u8, pub net_dev: *mut net_device, pub dev_tracker: netdevice_tracker, pub refcount: kref,
    pub batman_adv_ptype: packet_type, pub mesh_iface: *mut net_device, pub meshif_dev_tracker: netdevice_tracker, pub rcu: rcu_head,
    pub hop_penalty: u8, pub bat_iv: batadv_hard_iface_bat_iv,
    #[cfg(feature = "CONFIG_BATMAN_ADV_BATMAN_V")] pub bat_v: batadv_hard_iface_bat_v,
    pub neigh_list: hlist_head, pub neigh_list_lock: spinlock_t,
}
#[repr(C)] pub struct batadv_orig_ifinfo_bat_iv { pub bcast_own: [ulong; BATADV_TQ_LOCAL_WINDOW_SIZE as usize], pub bcast_own_sum: u8 }
#[repr(C)] pub struct batadv_orig_ifinfo { pub list: hlist_node, pub if_outgoing: *mut batadv_hard_iface, pub router: *mut batadv_neigh_node, pub last_real_seqno: u32, pub last_ttl: u8, pub last_seqno_forwarded: u32, pub batman_seqno_reset: ulong, pub bat_iv: batadv_orig_ifinfo_bat_iv, pub refcount: kref, pub rcu: rcu_head }
#[repr(C)] pub struct batadv_frag_table_entry { pub fragment_list: hlist_head, pub lock: spinlock_t, pub timestamp: ulong, pub seqno: u16, pub size: usize, pub total_size: u16 }
#[repr(C)] pub struct batadv_frag_list_entry { pub list: hlist_node, pub skb: *mut sk_buff, pub no: u8 }
#[repr(C)] pub struct batadv_vlan_tt { pub crc: u32, pub num_entries: atomic_t }
#[repr(C)] pub struct batadv_orig_node_vlan { pub vid: u16, pub tt: batadv_vlan_tt, pub list: hlist_node, pub refcount: kref, pub rcu: rcu_head }
#[repr(C)] pub struct batadv_orig_bat_iv { pub ogm_cnt_lock: spinlock_t }

#[repr(C)] pub struct batadv_orig_node {
    pub orig: [u8; ETH_ALEN], pub ifinfo_list: hlist_head, pub last_bonding_candidate: *mut batadv_orig_ifinfo,
    #[cfg(feature = "CONFIG_BATMAN_ADV_DAT")] pub dat_addr: batadv_dat_addr_t,
    pub last_seen: ulong, pub bcast_seqno_reset: ulong,
    #[cfg(feature = "CONFIG_BATMAN_ADV_MCAST")] pub mcast_handler_lock: spinlock_t,
    #[cfg(feature = "CONFIG_BATMAN_ADV_MCAST")] pub mcast_flags: u8,
    #[cfg(feature = "CONFIG_BATMAN_ADV_MCAST")] pub mcast_want_all_unsnoopables_node: hlist_node,
    #[cfg(feature = "CONFIG_BATMAN_ADV_MCAST")] pub mcast_want_all_ipv4_node: hlist_node,
    #[cfg(feature = "CONFIG_BATMAN_ADV_MCAST")] pub mcast_want_all_ipv6_node: hlist_node,
    #[cfg(feature = "CONFIG_BATMAN_ADV_MCAST")] pub mcast_want_all_rtr4_node: hlist_node,
    #[cfg(feature = "CONFIG_BATMAN_ADV_MCAST")] pub mcast_want_all_rtr6_node: hlist_node,
    pub capabilities: ulong, pub capa_initialized: ulong, pub last_ttvn: u8, pub tt_buff: *mut u8, pub tt_buff_len: u16,
    pub tt_buff_lock: spinlock_t, pub tt_lock: spinlock_t, pub bcast_bits: [ulong; BATADV_TQ_LOCAL_WINDOW_SIZE as usize], pub last_bcast_seqno: u32,
    pub neigh_list: hlist_head, pub neigh_list_lock: spinlock_t, pub hash_entry: hlist_node, pub bat_priv: *mut batadv_priv,
    pub bcast_seqno_lock: spinlock_t, pub refcount: kref, pub rcu: rcu_head, pub fragments: [batadv_frag_table_entry; BATADV_FRAG_BUFFER_COUNT as usize],
    pub vlan_list: hlist_head, pub vlan_list_lock: spinlock_t, pub bat_iv: batadv_orig_bat_iv,
}
#[repr(C)] pub enum batadv_orig_capabilities { BATADV_ORIG_CAPA_HAS_DAT, BATADV_ORIG_CAPA_HAS_TT, BATADV_ORIG_CAPA_HAS_MCAST }
#[repr(C)] pub struct batadv_gw_node { pub list: hlist_node, pub orig_node: *mut batadv_orig_node, pub bandwidth_down: u32, pub bandwidth_up: u32, pub refcount: kref, pub rcu: rcu_head }
#[repr(C)] pub struct batadv_hardif_neigh_node_bat_v { pub throughput: ewma_throughput, pub elp_interval: u32, pub elp_latest_seqno: u32, pub last_unicast_tx: ulong }
#[repr(C)] pub struct batadv_hardif_neigh_node { pub list: hlist_node, pub addr: [u8; ETH_ALEN], pub orig: [u8; ETH_ALEN], pub if_incoming: *mut batadv_hard_iface, pub last_seen: ulong, #[cfg(feature = "CONFIG_BATMAN_ADV_BATMAN_V")] pub bat_v: batadv_hardif_neigh_node_bat_v, pub refcount: kref, pub rcu: rcu_head }
#[repr(C)] pub struct batadv_neigh_node { pub list: hlist_node, #[cfg(feature = "CONFIG_BATMAN_ADV_BATMAN_V")] pub orig_node_id: *mut batadv_orig_node, pub addr: [u8; ETH_ALEN], pub ifinfo_list: hlist_head, pub ifinfo_lock: spinlock_t, pub if_incoming: *mut batadv_hard_iface, pub last_seen: ulong, pub hardif_neigh: *mut batadv_hardif_neigh_node, pub refcount: kref, pub rcu: rcu_head }
#[repr(C)] pub struct batadv_neigh_ifinfo_bat_iv { pub tq_recv: [u8; BATADV_TQ_GLOBAL_WINDOW_SIZE as usize], pub tq_index: u8, pub tq_avg: u8, pub real_bits: [ulong; BATADV_TQ_LOCAL_WINDOW_SIZE as usize], pub real_packet_count: u8 }
#[repr(C)] pub struct batadv_neigh_ifinfo_bat_v { pub throughput: u32, pub last_seqno: u32 }
#[repr(C)] pub struct batadv_neigh_ifinfo { pub list: hlist_node, pub if_outgoing: *mut batadv_hard_iface, pub bat_iv: batadv_neigh_ifinfo_bat_iv, #[cfg(feature = "CONFIG_BATMAN_ADV_BATMAN_V")] pub bat_v: batadv_neigh_ifinfo_bat_v, pub last_ttl: u8, pub refcount: kref, pub rcu: rcu_head }

#[cfg(feature = "CONFIG_BATMAN_ADV_BLA")]
#[repr(C)] pub struct batadv_bcast_duplist_entry { pub orig: [u8; ETH_ALEN], pub crc: u32, pub entrytime: ulong }
#[repr(C)] pub enum batadv_counters { BATADV_CNT_TX, BATADV_CNT_TX_BYTES, BATADV_CNT_TX_DROPPED, BATADV_CNT_RX, BATADV_CNT_RX_BYTES, BATADV_CNT_FORWARD, BATADV_CNT_FORWARD_BYTES, BATADV_CNT_MGMT_TX, BATADV_CNT_MGMT_TX_BYTES, BATADV_CNT_MGMT_RX, BATADV_CNT_MGMT_RX_BYTES, BATADV_CNT_FRAG_TX, BATADV_CNT_FRAG_TX_BYTES, BATADV_CNT_FRAG_RX, BATADV_CNT_FRAG_RX_BYTES, BATADV_CNT_FRAG_FWD, BATADV_CNT_FRAG_FWD_BYTES, BATADV_CNT_TT_REQUEST_TX, BATADV_CNT_TT_REQUEST_RX, BATADV_CNT_TT_RESPONSE_TX, BATADV_CNT_TT_RESPONSE_RX, BATADV_CNT_TT_ROAM_ADV_TX, BATADV_CNT_TT_ROAM_ADV_RX, #[cfg(feature = "CONFIG_BATMAN_ADV_MCAST")] BATADV_CNT_MCAST_TX, #[cfg(feature = "CONFIG_BATMAN_ADV_MCAST")] BATADV_CNT_MCAST_TX_BYTES, #[cfg(feature = "CONFIG_BATMAN_ADV_MCAST")] BATADV_CNT_MCAST_TX_LOCAL, #[cfg(feature = "CONFIG_BATMAN_ADV_MCAST")] BATADV_CNT_MCAST_TX_LOCAL_BYTES, #[cfg(feature = "CONFIG_BATMAN_ADV_MCAST")] BATADV_CNT_MCAST_RX, #[cfg(feature = "CONFIG_BATMAN_ADV_MCAST")] BATADV_CNT_MCAST_RX_BYTES, #[cfg(feature = "CONFIG_BATMAN_ADV_MCAST")] BATADV_CNT_MCAST_RX_LOCAL, #[cfg(feature = "CONFIG_BATMAN_ADV_MCAST")] BATADV_CNT_MCAST_RX_LOCAL_BYTES, #[cfg(feature = "CONFIG_BATMAN_ADV_MCAST")] BATADV_CNT_MCAST_FWD, #[cfg(feature = "CONFIG_BATMAN_ADV_MCAST")] BATADV_CNT_MCAST_FWD_BYTES, #[cfg(feature = "CONFIG_BATMAN_ADV_DAT")] BATADV_CNT_DAT_GET_TX, #[cfg(feature = "CONFIG_BATMAN_ADV_DAT")] BATADV_CNT_DAT_GET_RX, #[cfg(feature = "CONFIG_BATMAN_ADV_DAT")] BATADV_CNT_DAT_PUT_TX, #[cfg(feature = "CONFIG_BATMAN_ADV_DAT")] BATADV_CNT_DAT_PUT_RX, #[cfg(feature = "CONFIG_BATMAN_ADV_DAT")] BATADV_CNT_DAT_CACHED_REPLY_TX, BATADV_CNT_NUM }

/* Remaining declarations retain C layout and external kernel types. */
#[repr(C)] pub struct batadv_priv_tt { pub vn: atomic_t, pub ogm_append_cnt: atomic_t, pub local_changes: usize, pub changes_list: list_head, pub local_hash: *mut batadv_hashtable, pub global_hash: *mut batadv_hashtable, pub req_list: hlist_head, pub roam_list: list_head, pub changes_list_lock: spinlock_t, pub req_list_lock: spinlock_t, pub roam_list_lock: spinlock_t, pub last_changeset: *mut u8, pub last_changeset_len: u16, pub last_changeset_lock: spinlock_t, pub commit_lock: spinlock_t, pub work: delayed_work }
#[repr(C)] pub struct batadv_tp_unacked { pub seqno: u32, pub len: u32, pub list: list_head }
#[repr(C)] pub struct batadv_skb_cb { pub num_bcasts: u8 }
#[repr(C)] pub struct batadv_tt_common_entry { pub addr: [u8; ETH_ALEN], pub vid: u16, pub hash_entry: hlist_node, pub flags: u16, pub added_at: ulong, pub refcount: kref, pub rcu: rcu_head }
#[repr(C)] pub struct batadv_tt_local_entry { pub common: batadv_tt_common_entry, pub last_seen: ulong, pub vlan: *mut batadv_meshif_vlan }
#[repr(C)] pub struct batadv_tt_global_entry { pub common: batadv_tt_common_entry, pub orig_list: hlist_head, pub orig_list_count: atomic_t, pub list_lock: spinlock_t, pub roam_at: ulong }
#[repr(C)] pub struct batadv_tt_orig_list_entry { pub orig_node: *mut batadv_orig_node, pub ttvn: u8, pub flags: u8, pub list: hlist_node, pub refcount: kref, pub rcu: rcu_head }
#[repr(C)] pub struct batadv_tt_change_node { pub list: list_head, pub change: batadv_tvlv_tt_change }
#[repr(C)] pub struct batadv_tt_req_node { pub addr: [u8; ETH_ALEN], pub issued_at: ulong, pub refcount: kref, pub list: hlist_node }
#[repr(C)] pub struct batadv_tt_roam_node { pub addr: [u8; ETH_ALEN], pub vid: u16, pub counter: atomic_t, pub first_time: ulong, pub list: list_head }

/* Callback declarations and feature-specific private structs from the header. */
#[repr(C)] pub struct batadv_algo_iface_ops { pub activate: Option<unsafe extern "C" fn(*mut batadv_hard_iface)>, pub enable: Option<unsafe extern "C" fn(*mut batadv_hard_iface) -> i32>, pub enabled: Option<unsafe extern "C" fn(*mut batadv_hard_iface)>, pub disable: Option<unsafe extern "C" fn(*mut batadv_hard_iface)>, pub update_mac: Option<unsafe extern "C" fn(*mut batadv_hard_iface)>, pub primary_set: Option<unsafe extern "C" fn(*mut batadv_hard_iface)> }
#[repr(C)] pub struct batadv_algo_neigh_ops { pub hardif_init: Option<unsafe extern "C" fn(*mut batadv_hardif_neigh_node)>, pub cmp: Option<unsafe extern "C" fn(*mut batadv_neigh_node,*mut batadv_hard_iface,*mut batadv_neigh_node,*mut batadv_hard_iface)->i32>, pub is_similar_or_better: Option<unsafe extern "C" fn(*mut batadv_neigh_node,*mut batadv_hard_iface,*mut batadv_neigh_node,*mut batadv_hard_iface)->bool>, pub dump: Option<unsafe extern "C" fn(*mut sk_buff,*mut netlink_callback,*mut batadv_priv,*mut batadv_hard_iface)> }
#[repr(C)] pub struct batadv_algo_orig_ops { pub dump: Option<unsafe extern "C" fn(*mut sk_buff,*mut netlink_callback,*mut batadv_priv,*mut batadv_hard_iface)> }
#[repr(C)] pub struct batadv_algo_gw_ops { pub init_sel_class: Option<unsafe extern "C" fn(*mut batadv_priv)>, pub sel_class_max: u32, pub get_best_gw_node: Option<unsafe extern "C" fn(*mut batadv_priv)->*mut batadv_gw_node>, pub is_eligible: Option<unsafe extern "C" fn(*mut batadv_priv,*mut batadv_orig_node,*mut batadv_orig_node)->bool>, pub dump: Option<unsafe extern "C" fn(*mut sk_buff,*mut netlink_callback,*mut batadv_priv)> }
#[repr(C)] pub struct batadv_algo_ops { pub list: hlist_node, pub name: *mut i8, pub iface: batadv_algo_iface_ops, pub neigh: batadv_algo_neigh_ops, pub orig: batadv_algo_orig_ops, pub gw: batadv_algo_gw_ops }
#[repr(C)] pub struct batadv_dat_entry { pub ip: __be32, pub mac_addr: atomic64_t, pub vid: u16, pub last_update: ulong, pub hash_entry: hlist_node, pub refcount: kref, pub rcu: rcu_head }
#[repr(C)] pub struct batadv_hw_addr { pub list: hlist_node, pub addr: [u8; ETH_ALEN] }
#[repr(C)] pub struct batadv_dat_candidate { pub r#type: i32, pub orig_node: *mut batadv_orig_node }
#[repr(C)] pub struct batadv_tvlv_container { pub list: hlist_node, pub tvlv_hdr: batadv_tvlv_hdr, pub refcount: kref }
#[repr(C)] pub struct batadv_tvlv_handler { pub list: hlist_node, pub ogm_handler: Option<unsafe extern "C" fn(*mut batadv_priv,*mut batadv_orig_node,u8,*mut core::ffi::c_void,u16)>, pub unicast_handler: Option<unsafe extern "C" fn(*mut batadv_priv,*mut u8,*mut u8,*mut core::ffi::c_void,u16)->i32>, pub mcast_handler: Option<unsafe extern "C" fn(*mut batadv_priv,*mut sk_buff)->i32>, pub r#type: u8, pub version: u8, pub flags: u8, pub refcount: kref, pub rcu: rcu_head }
#[repr(C)] pub enum batadv_tvlv_handler_flags { BATADV_TVLV_HANDLER_OGM_CIFNOTFND = 1 << 1 }
#[repr(C)] pub struct batadv_tp_vars_common { pub list: hlist_node, pub timer: timer_list, pub bat_priv: *mut batadv_priv, pub other_end: [u8; ETH_ALEN], pub session: [u8; 2], pub refcount: kref, pub rcu: rcu_head }
#[repr(C)] pub struct batadv_tp_sender_cc { pub fast_recovery: u8, pub dup_acks: u8, pub dec_cwnd: u16, pub cwnd: u32, pub ss_threshold: u32, pub last_acked: u32, pub last_sent: u32, pub recover: u32, pub rto: u32, pub srtt: u32, pub rttvar: u32 }
#[repr(C)] pub struct batadv_tp_sender { pub common: batadv_tp_vars_common, pub start_time: ulong, pub send_result: atomic_t, pub finish_work: delayed_work, pub finished: completion, pub test_length: u32, pub icmp_uid: u8, pub cc: batadv_tp_sender_cc, pub cc_lock: spinlock_t, pub tot_sent: atomic64_t, pub more_bytes: wait_queue_head_t, pub prerandom_offset: u32, pub prerandom_lock: spinlock_t }
#[repr(C)] pub struct batadv_tp_receiver { pub common: batadv_tp_vars_common, pub receiving: atomic_t, pub last_recv: u32, pub last_recv_time: ulong, pub unacked_list: list_head, pub ack_seqno_lock: spinlock_t, pub unacked_count: usize }
#[repr(C)] pub struct batadv_meshif_vlan { pub bat_priv: *mut batadv_priv, pub vid: u16, pub ap_isolation: u8, pub tt: batadv_vlan_tt, pub list: hlist_node, pub refcount: kref, pub rcu: rcu_head }
#[repr(C)] pub struct batadv_priv_bat_v { pub ogm_buff: batadv_ogm_buf, pub ogm_seqno: atomic_t, pub ogm_buff_mutex: mutex, pub ogm_wq: delayed_work }
#[repr(C)] pub struct batadv_forw_packet { pub list: hlist_node, pub cleanup_list: hlist_node, pub send_time: ulong, pub own: u8, pub skb: *mut sk_buff, pub packet_len: u16, pub direct_link_flags: [ulong; BATADV_MAX_AGGREGATION_PACKETS as usize], pub num_packets: u8, pub delayed_work: delayed_work, pub if_incoming: *mut batadv_hard_iface, pub if_outgoing: *mut batadv_hard_iface, pub queue_left: *mut atomic_t }
#[repr(C)] pub struct batadv_priv_gw { pub gateway_list: hlist_head, pub list_lock: spinlock_t, pub curr_gw: *mut batadv_gw_node, pub generation: core::ffi::c_uint, pub mode: batadv_gw_modes, pub sel_class: u32, pub bandwidth_down: u32, pub bandwidth_up: u32, pub reselect: atomic_t }
#[repr(C)] pub struct batadv_priv_tvlv { pub container_list: hlist_head, pub handler_list: hlist_head, pub container_list_lock: spinlock_t, pub handler_list_lock: spinlock_t }
#[repr(C)] pub struct batadv_priv { pub mesh_state: batadv_mesh_state, pub mesh_iface: *mut net_device, pub mtu_set_by_user: i32, pub bat_counters: *mut u64, pub aggregated_ogms: u8, pub bonding: u8, pub fragmentation: u8, pub packet_size_max: i32, pub frag_seqno: atomic_t, pub orig_interval: u32, pub hop_penalty: u8, pub isolation_mark: u32, pub isolation_mark_mask: u32, pub bcast_seqno: atomic_t, pub bcast_queue_left: atomic_t, pub batman_queue_left: atomic_t, pub forw_bat_list: hlist_head, pub forw_bcast_list: hlist_head, pub tp_sender_list: hlist_head, pub tp_receiver_list: hlist_head, pub orig_hash: *mut batadv_hashtable, pub forw_bat_list_lock: spinlock_t, pub forw_bcast_list_lock: spinlock_t, pub tp_list_lock: spinlock_t, pub tp_num: atomic_t, pub hardif_generation: core::ffi::c_uint, pub orig_work: delayed_work, pub primary_if: *mut batadv_hard_iface, pub algo_ops: *mut batadv_algo_ops, pub meshif_vlan_list: hlist_head, pub meshif_vlan_list_lock: spinlock_t, pub gw: batadv_priv_gw, pub tt: batadv_priv_tt, pub tvlv: batadv_priv_tvlv }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
