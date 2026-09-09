/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ipv4 in net namespaces
 */

// Dependencies supplied by the surrounding kernel translation.

pub struct ctl_table_header;
pub struct ipv4_devconf;
pub struct fib_rules_ops;
pub struct hlist_head;
pub struct fib_table;
pub struct sock;
pub struct inet_hashinfo;
pub struct tcp_fastopen_context;
pub struct udp_table;
pub struct ip_ra_chain;
pub struct fib_notifier_ops;
pub struct mr_table;
pub struct tcp_congestion_ops;
pub struct inet_peer_base;
pub struct fqdir;
pub struct mutex;
pub struct list_head;
pub struct delayed_work;

#[repr(C)]
pub struct local_ports {
    pub range: u32, // high << 16 | low
    pub warned: bool,
}

#[repr(C)]
pub struct ping_group_range {
    pub lock: seqlock_t,
    pub range: [kgid_t; 2],
}

#[repr(C)]
pub struct inet_timewait_death_row {
    pub tw_refcount: refcount_t,
    // Padding to avoid false sharing, tw_refcount can be often written
    pub hashinfo: *mut inet_hashinfo,
    pub sysctl_max_tw_buckets: i32,
}

#[cfg(feature = "CONFIG_IP_ROUTE_MULTIPATH")]
#[repr(C)]
pub struct sysctl_fib_multipath_hash_seed {
    pub user_seed: u32,
    pub mp_seed: u32,
}

#[repr(C)]
pub struct udp_tunnel_gro {
    pub sk: *mut sock,
    pub list: hlist_head,
}

#[repr(C)]
pub struct netns_ipv4 {
    // Cacheline organization is documented in
    // Documentation/networking/net_cachelines/netns_ipv4_sysctl.rst.
    // Please update the document when adding new fields.

    // TX readonly hotpath cache lines
    pub sysctl_tcp_early_retrans: u8,
    pub sysctl_tcp_tso_win_divisor: u8,
    pub sysctl_tcp_tso_rtt_log: u8,
    pub sysctl_tcp_autocorking: u8,
    pub sysctl_tcp_min_snd_mss: i32,
    pub sysctl_tcp_notsent_lowat: u32,
    pub sysctl_tcp_limit_output_bytes: i32,
    pub sysctl_tcp_min_rtt_wlen: i32,
    pub sysctl_tcp_wmem: [i32; 3],
    pub sysctl_ip_fwd_use_pmtu: u8,

    // TXRX readonly hotpath cache lines
    pub sysctl_tcp_shrink_window: u8,

    // RX readonly hotpath cache line
    pub sysctl_tcp_moderate_rcvbuf: u8,
    pub sysctl_ip_early_demux: u8,
    pub sysctl_tcp_early_demux: u8,
    pub sysctl_tcp_l3mdev_accept: u8,
    // 3 bytes hole, try to pack
    pub sysctl_tcp_reordering: i32,
    pub sysctl_tcp_rmem: [i32; 3],
    pub sysctl_tcp_rcvbuf_low_rtt: i32,

    // ICMP rate limiter hot cache line.
    pub icmp_global_credit: atomic_t,
    pub icmp_global_stamp: u32,

    pub tcp_death_row: inet_timewait_death_row,
    pub udp_table: *mut udp_table,

    #[cfg(feature = "CONFIG_NET_UDP_TUNNEL")]
    pub udp_tunnel_gro: [udp_tunnel_gro; 2],

    #[cfg(feature = "CONFIG_SYSCTL")]
    pub forw_hdr: *mut ctl_table_header,
    #[cfg(feature = "CONFIG_SYSCTL")]
    pub frags_hdr: *mut ctl_table_header,
    #[cfg(feature = "CONFIG_SYSCTL")]
    pub ipv4_hdr: *mut ctl_table_header,
    #[cfg(feature = "CONFIG_SYSCTL")]
    pub route_hdr: *mut ctl_table_header,
    #[cfg(feature = "CONFIG_SYSCTL")]
    pub xfrm4_hdr: *mut ctl_table_header,
    pub devconf_all: *mut ipv4_devconf,
    pub devconf_dflt: *mut ipv4_devconf,
    pub ra_chain: *mut ip_ra_chain,
    pub ra_mutex: mutex,

    #[cfg(feature = "CONFIG_IP_MULTIPLE_TABLES")]
    pub rules_ops: *mut fib_rules_ops,
    #[cfg(feature = "CONFIG_IP_MULTIPLE_TABLES")]
    pub fib_main: *mut fib_table,
    #[cfg(feature = "CONFIG_IP_MULTIPLE_TABLES")]
    pub fib_default: *mut fib_table,
    #[cfg(feature = "CONFIG_IP_MULTIPLE_TABLES")]
    pub fib_table_hash_lock: spinlock_t,
    #[cfg(feature = "CONFIG_IP_MULTIPLE_TABLES")]
    pub fib_rules_require_fldissect: u32,
    #[cfg(feature = "CONFIG_IP_MULTIPLE_TABLES")]
    pub fib_has_custom_rules: bool,
    pub fib_has_custom_local_routes: bool,
    pub fib_offload_disabled: bool,
    #[cfg(feature = "CONFIG_IP_ROUTE_CLASSID")]
    pub fib_num_tclassid_users: atomic_t,
    pub fib_table_hash: *mut hlist_head,
    pub fibnl: *mut sock,
    pub fib_info_hash: *mut hlist_head,
    pub fib_info_hash_bits: u32,
    pub fib_info_cnt: u32,
    pub mc_autojoin_sk: *mut sock,
    pub peers: *mut inet_peer_base,
    pub fqdir: *mut fqdir,

    pub sysctl_icmp_echo_ignore_all: u8,
    pub sysctl_icmp_echo_enable_probe: u8,
    pub sysctl_icmp_echo_ignore_broadcasts: u8,
    pub sysctl_icmp_ignore_bogus_error_responses: u8,
    pub sysctl_icmp_errors_use_inbound_ifaddr: u8,
    pub sysctl_icmp_errors_extension_mask: u8,
    pub sysctl_icmp_ratelimit: i32,
    pub sysctl_icmp_ratemask: i32,
    pub sysctl_icmp_msgs_per_sec: i32,
    pub sysctl_icmp_msgs_burst: i32,
    pub ip_rt_min_pmtu: u32,
    pub ip_rt_mtu_expires: i32,
    pub ip_rt_min_advmss: i32,
    pub ip_local_ports: local_ports,
    pub sysctl_tcp_ecn: u8,
    pub sysctl_tcp_ecn_option: u8,
    pub sysctl_tcp_ecn_option_beacon: u8,
    pub sysctl_tcp_ecn_fallback: u8,
    pub sysctl_ip_default_ttl: u8,
    pub sysctl_ip_no_pmtu_disc: u8,
    pub sysctl_ip_fwd_update_priority: u8,
    pub sysctl_ip_nonlocal_bind: u8,
    pub sysctl_ip_autobind_reuse: u8,
    // Shall we try to damage output packets if routing dev changes?
    pub sysctl_ip_dynaddr: u8,
    pub sysctl_ip_local_port_step_width: u32,
    #[cfg(feature = "CONFIG_NET_L3_MASTER_DEV")]
    pub sysctl_raw_l3mdev_accept: u8,
    pub sysctl_udp_early_demux: u8,
    pub sysctl_nexthop_compat_mode: u8,
    pub sysctl_fwmark_reflect: u8,
    pub sysctl_tcp_fwmark_accept: u8,
    pub sysctl_tcp_mtu_probing: u8,
    pub sysctl_tcp_mtu_probe_floor: i32,
    pub sysctl_tcp_base_mss: i32,
    pub sysctl_tcp_probe_threshold: i32,
    pub sysctl_tcp_probe_interval: u32,
    pub sysctl_tcp_keepalive_time: i32,
    pub sysctl_tcp_keepalive_intvl: i32,
    pub sysctl_tcp_keepalive_probes: u8,
    pub sysctl_tcp_syn_retries: u8,
    pub sysctl_tcp_synack_retries: u8,
    pub sysctl_tcp_syncookies: u8,
    pub sysctl_tcp_migrate_req: u8,
    pub sysctl_tcp_comp_sack_nr: u8,
    pub sysctl_tcp_backlog_ack_defer: u8,
    pub sysctl_tcp_pingpong_thresh: u8,
    pub sysctl_tcp_retries1: u8,
    pub sysctl_tcp_retries2: u8,
    pub sysctl_tcp_orphan_retries: u8,
    pub sysctl_tcp_tw_reuse: u8,
    pub sysctl_tcp_tw_reuse_delay: u32,
    pub sysctl_tcp_fin_timeout: i32,
    pub sysctl_tcp_sack: u8,
    pub sysctl_tcp_window_scaling: u8,
    pub sysctl_tcp_timestamps: u8,
    pub sysctl_tcp_rto_min_us: i32,
    pub sysctl_tcp_rto_max_ms: i32,
    pub sysctl_tcp_recovery: u8,
    pub sysctl_tcp_thin_linear_timeouts: u8,
    pub sysctl_tcp_slow_start_after_idle: u8,
    pub sysctl_tcp_retrans_collapse: u8,
    pub sysctl_tcp_stdurg: u8,
    pub sysctl_tcp_rfc1337: u8,
    pub sysctl_tcp_abort_on_overflow: u8,
    pub sysctl_tcp_fack: u8, // obsolete
    pub sysctl_tcp_max_reordering: i32,
    pub sysctl_tcp_adv_win_scale: i32, // obsolete
    pub sysctl_tcp_dsack: u8,
    pub sysctl_tcp_app_win: u8,
    pub sysctl_tcp_frto: u8,
    pub sysctl_tcp_nometrics_save: u8,
    pub sysctl_tcp_no_ssthresh_metrics_save: u8,
    pub sysctl_tcp_workaround_signed_windows: u8,
    pub sysctl_tcp_challenge_ack_limit: i32,
    pub sysctl_tcp_min_tso_segs: u8,
    pub sysctl_tcp_reflect_tos: u8,
    pub sysctl_tcp_invalid_ratelimit: i32,
    pub sysctl_tcp_pacing_ss_ratio: i32,
    pub sysctl_tcp_pacing_ca_ratio: i32,
    pub sysctl_tcp_child_ehash_entries: u32,
    pub sysctl_tcp_comp_sack_rtt_percent: i32,
    pub sysctl_tcp_comp_sack_delay_ns: usize,
    pub sysctl_tcp_comp_sack_slack_ns: usize,
    pub sysctl_max_syn_backlog: i32,
    pub sysctl_tcp_fastopen: i32,
    pub tcp_congestion_control: *const tcp_congestion_ops,
    pub tcp_fastopen_ctx: *mut tcp_fastopen_context,
    pub sysctl_tcp_fastopen_blackhole_timeout: u32,
    pub tfo_active_disable_times: atomic_t,
    pub tfo_active_disable_stamp: usize,
    pub tcp_challenge_timestamp: u32,
    pub tcp_challenge_count: u32,
    pub sysctl_tcp_plb_enabled: u8,
    pub sysctl_tcp_plb_idle_rehash_rounds: u8,
    pub sysctl_tcp_plb_rehash_rounds: u8,
    pub sysctl_tcp_plb_suspend_rto_sec: u8,
    pub sysctl_tcp_plb_cong_thresh: i32,
    pub sysctl_udp_wmem_min: i32,
    pub sysctl_udp_rmem_min: i32,
    pub sysctl_fib_notify_on_flag_change: u8,
    pub sysctl_tcp_syn_linear_timeouts: u8,
    #[cfg(feature = "CONFIG_NET_L3_MASTER_DEV")]
    pub sysctl_udp_l3mdev_accept: u8,
    pub sysctl_igmp_llm_reports: u8,
    pub sysctl_igmp_max_memberships: i32,
    pub sysctl_igmp_max_msf: i32,
    pub sysctl_igmp_qrv: i32,
    pub ping_group_range: ping_group_range,
    pub ping_port_rover: u16,
    pub dev_addr_genid: atomic_t,
    pub sysctl_udp_child_hash_entries: u32,
    #[cfg(feature = "CONFIG_SYSCTL")]
    pub sysctl_local_reserved_ports: *mut usize,
    #[cfg(feature = "CONFIG_SYSCTL")]
    pub sysctl_ip_prot_sock: i32,
    #[cfg(all(feature = "CONFIG_IP_MROUTE", not(feature = "CONFIG_IP_MROUTE_MULTIPLE_TABLES")))]
    pub mrt: *mut mr_table,
    #[cfg(all(feature = "CONFIG_IP_MROUTE", feature = "CONFIG_IP_MROUTE_MULTIPLE_TABLES"))]
    pub mr_tables: list_head,
    #[cfg(all(feature = "CONFIG_IP_MROUTE", feature = "CONFIG_IP_MROUTE_MULTIPLE_TABLES"))]
    pub mr_rules_ops: *mut fib_rules_ops,
    #[cfg(feature = "CONFIG_IP_MROUTE")]
    pub ipmr_notifier_ops: *mut fib_notifier_ops,
    #[cfg(feature = "CONFIG_IP_MROUTE")]
    pub ipmr_seq: atomic_t,
    #[cfg(feature = "CONFIG_IP_MROUTE")]
    pub mfc_mutex: mutex,
    #[cfg(feature = "CONFIG_IP_ROUTE_MULTIPATH")]
    pub sysctl_fib_multipath_hash_seed: sysctl_fib_multipath_hash_seed,
    #[cfg(feature = "CONFIG_IP_ROUTE_MULTIPATH")]
    pub sysctl_fib_multipath_hash_fields: u32,
    #[cfg(feature = "CONFIG_IP_ROUTE_MULTIPATH")]
    pub sysctl_fib_multipath_use_neigh: u8,
    #[cfg(feature = "CONFIG_IP_ROUTE_MULTIPATH")]
    pub sysctl_fib_multipath_hash_policy: u8,
    pub notifier_ops: *mut fib_notifier_ops,
    pub fib_seq: u32, // writes protected by rtnl_mutex
    pub rt_genid: atomic_t,
    pub ip_id_key: siphash_key_t,
    pub inet_addr_lst: *mut hlist_head,
    pub addr_chk_work: delayed_work,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
