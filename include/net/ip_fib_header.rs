/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Direct Rust translation of net/ip_fib.h. C includes and build configuration
 * symbols are supplied by the surrounding translation. */

#[repr(C)]
pub struct fib_config {
    pub fc_dst_len: u8,
    pub fc_dscp: dscp_t,
    pub fc_protocol: u8,
    pub fc_scope: u8,
    pub fc_type: u8,
    pub fc_gw_family: u8,
    pub fc_table: u32,
    pub fc_dst: __be32,
    pub fc_gw: fib_config__bindgen_ty_1,
    pub fc_oif: c_int,
    pub fc_flags: u32,
    pub fc_priority: u32,
    pub fc_prefsrc: __be32,
    pub fc_nh_id: u32,
    pub fc_mx: *mut nlattr,
    pub fc_mp: *mut rtnexthop,
    pub fc_mx_len: c_int,
    pub fc_mp_len: c_int,
    pub fc_flow: u32,
    pub fc_nlflags: u32,
    pub fc_nlinfo: nl_info,
    pub fc_encap: *mut nlattr,
    pub fc_encap_type: u16,
}

#[repr(C)]
pub union fib_config__bindgen_ty_1 {
    pub fc_gw4: __be32,
    pub fc_gw6: in6_addr,
}

pub struct fib_info;
pub struct rtable;

#[repr(C)]
pub struct fib_nh_exception {
    pub fnhe_next: *mut fib_nh_exception,
    pub fnhe_genid: c_int,
    pub fnhe_daddr: __be32,
    pub fnhe_pmtu: u32,
    pub fnhe_mtu_locked: bool,
    pub fnhe_gw: __be32,
    pub fnhe_expires: c_ulong,
    pub fnhe_rth_input: *mut rtable,
    pub fnhe_rth_output: *mut rtable,
    pub fnhe_stamp: c_ulong,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct fnhe_hash_bucket { pub chain: *mut fib_nh_exception }

pub const FNHE_HASH_SHIFT: u32 = 11;
pub const FNHE_HASH_SIZE: u32 = 1 << FNHE_HASH_SHIFT;
pub const FNHE_RECLAIM_DEPTH: u32 = 5;

#[repr(C)]
pub struct fib_nh_common {
    pub nhc_dev: *mut net_device,
    pub nhc_dev_tracker: netdevice_tracker,
    pub nhc_oif: c_int,
    pub nhc_scope: c_uchar,
    pub nhc_family: u8,
    pub nhc_gw_family: u8,
    pub nhc_flags: c_uchar,
    pub nhc_lwtstate: *mut lwtunnel_state,
    pub nhc_gw: fib_nh_common__bindgen_ty_1,
    pub nhc_weight: c_int,
    pub nhc_upper_bound: atomic_t,
    pub nhc_pcpu_rth_output: *mut *mut rtable,
    pub nhc_rth_input: *mut rtable,
    pub nhc_exceptions: *mut fnhe_hash_bucket,
}

#[repr(C)]
pub union fib_nh_common__bindgen_ty_1 { pub ipv4: __be32, pub ipv6: in6_addr }

#[repr(C)]
pub struct fib_nh {
    pub nh_common: fib_nh_common,
    pub nh_hash: hlist_node,
    pub nh_parent: *mut fib_info,
    #[cfg(CONFIG_IP_ROUTE_CLASSID)]
    pub nh_tclassid: u32,
    pub nh_saddr: __be32,
    pub nh_saddr_genid: c_int,
}

pub struct nexthop;

#[repr(C)]
pub struct fib_info {
    pub fib_hash: hlist_node, pub fib_lhash: hlist_node, pub nh_list: list_head,
    pub fib_net: *mut net, pub fib_treeref: refcount_t, pub fib_clntref: refcount_t,
    pub fib_flags: c_uint, pub fib_dead: c_uchar, pub fib_protocol: c_uchar,
    pub fib_scope: c_uchar, pub fib_type: c_uchar, pub fib_prefsrc: __be32,
    pub fib_tb_id: u32, pub fib_priority: u32, pub fib_metrics: *mut dst_metrics,
    pub fib_nhs: c_int, pub fib_nh_is_v6: bool, pub nh_updated: bool,
    pub pfsrc_removed: bool, pub nh: *mut nexthop, pub rcu: rcu_head,
    pub fib_nh: [fib_nh; 0],
}

pub fn fib_info_update_nhc_saddr(net: *mut net, nhc: *mut fib_nh_common, scope: c_uchar) -> __be32;
pub fn fib_result_prefsrc(net: *mut net, res: *mut fib_result) -> __be32;

#[repr(C)]
pub struct fib_result {
    pub prefix: __be32, pub prefixlen: c_uchar, pub nh_sel: c_uchar,
    pub r#type: c_uchar, pub scope: c_uchar, pub tclassid: u32, pub dscp: dscp_t,
    pub nhc: *mut fib_nh_common, pub fi: *mut fib_info, pub table: *mut fib_table,
    pub fa_head: *mut hlist_head,
}

#[repr(C)]
pub struct fib_result_nl {
    pub fl_addr: __be32, pub fl_mark: u32, pub fl_tos: c_uchar, pub fl_scope: c_uchar,
    pub tb_id_in: c_uchar, pub tb_id: c_uchar, pub prefixlen: c_uchar,
    pub nh_sel: c_uchar, pub r#type: c_uchar, pub scope: c_uchar, pub err: c_int,
}

#[cfg(CONFIG_IP_MULTIPLE_TABLES)] pub const FIB_TABLE_HASHSZ: u32 = 256;
#[cfg(not(CONFIG_IP_MULTIPLE_TABLES))] pub const FIB_TABLE_HASHSZ: u32 = 2;

#[repr(C)]
pub struct fib_rt_info {
    pub fi: *mut fib_info, pub tb_id: u32, pub dst: __be32, pub dst_len: c_int,
    pub dscp: dscp_t, pub r#type: u8, pub offload: u8, pub trap: u8,
    pub offload_failed: u8, pub unused: u8,
}

#[repr(C)]
pub struct fib_entry_notifier_info { pub info: fib_notifier_info, pub dst: u32, pub dst_len: c_int, pub fi: *mut fib_info, pub dscp: dscp_t, pub r#type: u8, pub tb_id: u32 }
#[repr(C)]
pub struct fib_nh_notifier_info { pub info: fib_notifier_info, pub fib_nh: *mut fib_nh }

pub fn fib4_semantics_init(net: *mut net) -> c_int;
pub fn fib4_semantics_exit(net: *mut net);
pub fn call_fib4_notifier(nb: *mut notifier_block, event_type: fib_event_type, info: *mut fib_notifier_info) -> c_int;
pub fn call_fib4_notifiers(net: *mut net, event_type: fib_event_type, info: *mut fib_notifier_info) -> c_int;
pub fn fib4_notifier_init(net: *mut net) -> c_int;
pub fn fib4_notifier_exit(net: *mut net);
pub fn fib_info_notify_update(net: *mut net, info: *mut nl_info);
pub fn fib_notify(net: *mut net, nb: *mut notifier_block, extack: *mut netlink_ext_ack) -> c_int;

#[repr(C)]
pub struct fib_table {
    pub tb_hlist: hlist_node,
    pub tb_id: u32,
    pub tb_num_default: c_int,
    pub rcu: rcu_head,
    pub tb_data: *mut c_ulong,
    pub data: [c_ulong; 0],
}

pub fn fib_table_lookup(tb: *mut fib_table, flp: *const flowi4, res: *mut fib_result, fib_flags: c_int) -> c_int;
pub fn fib_table_insert(net: *mut net, tb: *mut fib_table, cfg: *mut fib_config, extack: *mut netlink_ext_ack) -> c_int;
pub fn fib_table_delete(net: *mut net, tb: *mut fib_table, cfg: *mut fib_config, extack: *mut netlink_ext_ack) -> c_int;
pub fn fib_table_flush(net: *mut net, tb: *mut fib_table, flush_all: bool) -> c_int;
pub fn fib_table_dump(table: *mut fib_table, skb: *mut sk_buff, cb: *mut netlink_callback, filter: *mut fib_dump_filter) -> c_int;
pub fn fib_table_flush_external(table: *mut fib_table);
pub fn fib_free_table(tb: *mut fib_table);

pub fn fib_new_table(net: *mut net, id: u32) -> *mut fib_table;
pub fn fib_get_table(net: *mut net, id: u32) -> *mut fib_table;
pub fn fib_lookup(net: *mut net, flp: *mut flowi4, res: *mut fib_result, flags: c_uint) -> c_int;
pub fn fib4_has_custom_rules(net: *const net) -> bool;
pub fn fib4_rule_default(rule: *const fib_rule) -> bool;
pub fn fib4_rules_dump(net: *mut net, nb: *mut notifier_block, extack: *mut netlink_ext_ack) -> c_int;
pub fn fib4_rules_seq_read(net: *const net) -> c_uint;
pub fn fib4_rules_early_flow_dissect(net: *mut net, skb: *mut sk_buff, fl4: *mut flowi4, flkeys: *mut flow_keys) -> bool;

pub fn fib_validate_source_reason(skb: *mut sk_buff, src: __be32, dst: __be32,
    dscp: dscp_t, oif: c_int, dev: *mut net_device, idev: *mut in_device,
    itag: *mut u32) -> skb_drop_reason;
pub fn fib_validate_source(skb: *mut sk_buff, src: __be32, dst: __be32,
    dscp: dscp_t, oif: c_int, dev: *mut net_device, idev: *mut in_device,
    itag: *mut u32) -> c_int;
pub fn fib_num_tclassid_users(net: *mut net) -> c_int;

pub static mut rtm_ipv4_policy: [nla_policy; 0];

#[repr(C)]
pub struct fib_dump_filter { pub table_id: u32, pub filter_set: bool, pub dump_routes: bool, pub dump_exceptions: bool, pub protocol: c_uchar, pub rt_type: c_uchar, pub flags: c_uint, pub dev: *mut net_device }

pub fn fib_dscp_masked_match(dscp: dscp_t, fl4: *const flowi4) -> bool;
pub fn ip_fib_init();
pub fn fib_gw_from_via(cfg: *mut fib_config, nla: *mut nlattr, extack: *mut netlink_ext_ack) -> c_int;
pub fn fib_compute_spec_dst(skb: *mut sk_buff) -> __be32;
pub fn fib_info_nh_uses_dev(fi: *mut fib_info, dev: *const net_device) -> bool;
pub fn fib_unmerge(net: *mut net) -> c_int;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
