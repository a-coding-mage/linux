#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Translated from net/flow_offload.h. Kernel includes and symbols are supplied externally.

pub type u8 = core::primitive::u8;
pub type u16 = core::primitive::u16;
pub type u32 = core::primitive::u32;
pub type u64 = core::primitive::u64;
pub type s32 = core::primitive::i32;
pub type __be16 = u16;
pub type gfp_t = usize;
pub type c_int = i32;

pub enum flow_dissector {}
pub enum flow_dissector_key_meta {}
pub enum flow_dissector_key_basic {}
pub enum flow_dissector_key_control {}
pub enum flow_dissector_key_eth_addrs {}
pub enum flow_dissector_key_vlan {}
pub enum flow_dissector_key_arp {}
pub enum flow_dissector_key_ipv4_addrs {}
pub enum flow_dissector_key_ipv6_addrs {}
pub enum flow_dissector_key_ip {}
pub enum flow_dissector_key_ports {}
pub enum flow_dissector_key_ports_range {}
pub enum flow_dissector_key_icmp {}
pub enum flow_dissector_key_tcp {}
pub enum flow_dissector_key_ipsec {}
pub enum flow_dissector_key_mpls {}
pub enum flow_dissector_key_keyid {}
pub enum flow_dissector_key_enc_opts {}
pub enum flow_dissector_key_ct {}
pub enum flow_dissector_key_pppoe {}
pub enum flow_dissector_key_l2tpv3 {}
pub enum net_device {}
pub enum ip_tunnel_info {}
pub enum psample_group {}
pub enum nf_flowtable {}
pub enum action_gate_entry {}
pub enum netlink_ext_ack {}
pub enum net {}
pub enum Qdisc {}
pub enum tc_setup_type {}
pub enum enum_flow_dissector_key_id {}
pub enum list_head {}

#[repr(C)] pub struct flow_match { pub dissector: *mut flow_dissector, pub mask: *mut core::ffi::c_void, pub key: *mut core::ffi::c_void }
macro_rules! match_type { ($n:ident, $k:ty) => { #[repr(C)] pub struct $n { pub key: *mut $k, pub mask: *mut $k } }; }
match_type!(flow_match_meta, flow_dissector_key_meta);
match_type!(flow_match_basic, flow_dissector_key_basic);
match_type!(flow_match_control, flow_dissector_key_control);
match_type!(flow_match_eth_addrs, flow_dissector_key_eth_addrs);
match_type!(flow_match_vlan, flow_dissector_key_vlan);
match_type!(flow_match_arp, flow_dissector_key_arp);
match_type!(flow_match_ipv4_addrs, flow_dissector_key_ipv4_addrs);
match_type!(flow_match_ipv6_addrs, flow_dissector_key_ipv6_addrs);
match_type!(flow_match_ip, flow_dissector_key_ip);
match_type!(flow_match_ports, flow_dissector_key_ports);
match_type!(flow_match_ports_range, flow_dissector_key_ports_range);
match_type!(flow_match_icmp, flow_dissector_key_icmp);
match_type!(flow_match_tcp, flow_dissector_key_tcp);
match_type!(flow_match_ipsec, flow_dissector_key_ipsec);
match_type!(flow_match_mpls, flow_dissector_key_mpls);
match_type!(flow_match_enc_keyid, flow_dissector_key_keyid);
match_type!(flow_match_enc_opts, flow_dissector_key_enc_opts);
match_type!(flow_match_ct, flow_dissector_key_ct);
match_type!(flow_match_pppoe, flow_dissector_key_pppoe);
match_type!(flow_match_l2tpv3, flow_dissector_key_l2tpv3);

pub enum flow_rule {}

extern "C" {
    pub fn flow_rule_match_meta(rule: *const flow_rule, out: *mut flow_match_meta);
    pub fn flow_rule_match_basic(rule: *const flow_rule, out: *mut flow_match_basic);
    pub fn flow_rule_match_control(rule: *const flow_rule, out: *mut flow_match_control);
    pub fn flow_rule_match_eth_addrs(rule: *const flow_rule, out: *mut flow_match_eth_addrs);
    pub fn flow_rule_match_vlan(rule: *const flow_rule, out: *mut flow_match_vlan);
    pub fn flow_rule_match_cvlan(rule: *const flow_rule, out: *mut flow_match_vlan);
    pub fn flow_rule_match_arp(rule: *const flow_rule, out: *mut flow_match_arp);
    pub fn flow_rule_match_ipv4_addrs(rule: *const flow_rule, out: *mut flow_match_ipv4_addrs);
    pub fn flow_rule_match_ipv6_addrs(rule: *const flow_rule, out: *mut flow_match_ipv6_addrs);
    pub fn flow_rule_match_ip(rule: *const flow_rule, out: *mut flow_match_ip);
    pub fn flow_rule_match_ports(rule: *const flow_rule, out: *mut flow_match_ports);
    pub fn flow_rule_match_ports_range(rule: *const flow_rule, out: *mut flow_match_ports_range);
    pub fn flow_rule_match_tcp(rule: *const flow_rule, out: *mut flow_match_tcp);
    pub fn flow_rule_match_ipsec(rule: *const flow_rule, out: *mut flow_match_ipsec);
    pub fn flow_rule_match_icmp(rule: *const flow_rule, out: *mut flow_match_icmp);
    pub fn flow_rule_match_mpls(rule: *const flow_rule, out: *mut flow_match_mpls);
    pub fn flow_rule_match_enc_control(rule: *const flow_rule, out: *mut flow_match_control);
    pub fn flow_rule_match_enc_ipv4_addrs(rule: *const flow_rule, out: *mut flow_match_ipv4_addrs);
    pub fn flow_rule_match_enc_ipv6_addrs(rule: *const flow_rule, out: *mut flow_match_ipv6_addrs);
    pub fn flow_rule_match_enc_ip(rule: *const flow_rule, out: *mut flow_match_ip);
    pub fn flow_rule_match_enc_ports(rule: *const flow_rule, out: *mut flow_match_ports);
    pub fn flow_rule_match_enc_keyid(rule: *const flow_rule, out: *mut flow_match_enc_keyid);
    pub fn flow_rule_match_enc_opts(rule: *const flow_rule, out: *mut flow_match_enc_opts);
    pub fn flow_rule_match_ct(rule: *const flow_rule, out: *mut flow_match_ct);
    pub fn flow_rule_match_pppoe(rule: *const flow_rule, out: *mut flow_match_pppoe);
    pub fn flow_rule_match_l2tpv3(rule: *const flow_rule, out: *mut flow_match_l2tpv3);
}

#[repr(C)] #[derive(Copy, Clone)] pub enum flow_action_id { FLOW_ACTION_ACCEPT=0, FLOW_ACTION_DROP, FLOW_ACTION_TRAP, FLOW_ACTION_GOTO, FLOW_ACTION_REDIRECT, FLOW_ACTION_MIRRED, FLOW_ACTION_REDIRECT_INGRESS, FLOW_ACTION_MIRRED_INGRESS, FLOW_ACTION_VLAN_PUSH, FLOW_ACTION_VLAN_POP, FLOW_ACTION_VLAN_MANGLE, FLOW_ACTION_TUNNEL_ENCAP, FLOW_ACTION_TUNNEL_DECAP, FLOW_ACTION_MANGLE, FLOW_ACTION_ADD, FLOW_ACTION_CSUM, FLOW_ACTION_MARK, FLOW_ACTION_PTYPE, FLOW_ACTION_PRIORITY, FLOW_ACTION_RX_QUEUE_MAPPING, FLOW_ACTION_WAKE, FLOW_ACTION_QUEUE, FLOW_ACTION_SAMPLE, FLOW_ACTION_POLICE, FLOW_ACTION_CT, FLOW_ACTION_CT_METADATA, FLOW_ACTION_MPLS_PUSH, FLOW_ACTION_MPLS_POP, FLOW_ACTION_MPLS_MANGLE, FLOW_ACTION_GATE, FLOW_ACTION_PPPOE_PUSH, FLOW_ACTION_JUMP, FLOW_ACTION_PIPE, FLOW_ACTION_VLAN_PUSH_ETH, FLOW_ACTION_VLAN_POP_ETH, FLOW_ACTION_CONTINUE, NUM_FLOW_ACTIONS }
#[repr(C)] #[derive(Copy, Clone)] pub enum flow_action_mangle_base { FLOW_ACT_MANGLE_UNSPEC=0, FLOW_ACT_MANGLE_HDR_TYPE_ETH, FLOW_ACT_MANGLE_HDR_TYPE_IP4, FLOW_ACT_MANGLE_HDR_TYPE_IP6, FLOW_ACT_MANGLE_HDR_TYPE_TCP, FLOW_ACT_MANGLE_HDR_TYPE_UDP }
#[repr(C)] #[derive(Copy, Clone)] pub enum flow_action_hw_stats_bit { FLOW_ACTION_HW_STATS_IMMEDIATE_BIT, FLOW_ACTION_HW_STATS_DELAYED_BIT, FLOW_ACTION_HW_STATS_DISABLED_BIT, FLOW_ACTION_HW_STATS_NUM_BITS }
pub type flow_action_hw_stats = u32;
pub const FLOW_ACTION_HW_STATS_IMMEDIATE: flow_action_hw_stats = 1 << 0;
pub const FLOW_ACTION_HW_STATS_DELAYED: flow_action_hw_stats = 1 << 1;
pub const FLOW_ACTION_HW_STATS_ANY: flow_action_hw_stats = FLOW_ACTION_HW_STATS_IMMEDIATE | FLOW_ACTION_HW_STATS_DELAYED;
pub const FLOW_ACTION_HW_STATS_DISABLED: flow_action_hw_stats = 1 << 2;
pub const FLOW_ACTION_HW_STATS_DONT_CARE: flow_action_hw_stats = (1 << 3) - 1;

pub type action_destr = unsafe extern "C" fn(*mut core::ffi::c_void);
#[repr(C)] pub struct flow_action_cookie { pub cookie_len: u32, pub cookie: [u8; 0] }
#[repr(C)] pub struct flow_action_police { pub burst:u32, pub rate_bytes_ps:u64, pub peakrate_bytes_ps:u64, pub avrate:u32, pub overhead:u16, pub burst_pkt:u64, pub rate_pkt_ps:u64, pub mtu:u32, pub exceed: flow_action_police_action, pub notexceed: flow_action_police_action }
#[repr(C)] pub struct flow_action_police_action { pub act_id: flow_action_id, pub extval:u32 }

#[repr(C)] pub union flow_action_entry_data {
    pub chain_index:u32, pub dev:*mut net_device, pub vlan: flow_action_vlan, pub vlan_push_eth: flow_action_vlan_push_eth,
    pub mangle: flow_action_mangle, pub tunnel:*mut ip_tunnel_info, pub csum_flags:u32, pub mark:u32, pub ptype:u16, pub rx_queue:u16,
    pub priority:u32, pub queue:flow_action_queue, pub sample:flow_action_sample, pub police:flow_action_police, pub ct:flow_action_ct,
    pub ct_metadata:flow_action_ct_metadata, pub mpls_push:flow_action_mpls_push, pub mpls_pop:flow_action_mpls_pop,
    pub mpls_mangle:flow_action_mpls_mangle, pub gate:flow_action_gate, pub pppoe:flow_action_pppoe,
}
#[repr(C)] pub struct flow_action_vlan { pub vid:u16, pub proto:__be16, pub prio:u8 }
#[repr(C)] pub struct flow_action_vlan_push_eth { pub dst:[u8;6], pub src:[u8;6] }
#[repr(C)] pub struct flow_action_mangle { pub htype:flow_action_mangle_base, pub offset:u32, pub mask:u32, pub val:u32 }
#[repr(C)] pub struct flow_action_queue { pub ctx:u32, pub index:u32, pub vf:u8 }
#[repr(C)] pub struct flow_action_sample { pub psample_group:*mut psample_group, pub rate:u32, pub trunc_size:u32, pub truncate:bool }
#[repr(C)] pub struct flow_action_ct { pub action:c_int, pub zone:u16, pub flow_table:*mut nf_flowtable }
#[repr(C)] pub struct flow_action_ct_metadata { pub cookie:usize, pub mark:u32, pub labels:[u32;4], pub orig_dir:bool }
#[repr(C)] pub struct flow_action_mpls_push { pub label:u32, pub proto:__be16, pub tc:u8, pub bos:u8, pub ttl:u8 }
#[repr(C)] pub struct flow_action_mpls_pop { pub proto:__be16 }
#[repr(C)] pub struct flow_action_mpls_mangle { pub label:u32, pub tc:u8, pub bos:u8, pub ttl:u8 }
#[repr(C)] pub struct flow_action_gate { pub prio:s32, pub basetime:u64, pub cycletime:u64, pub cycletimeext:u64, pub num_entries:u32, pub entries:*mut action_gate_entry }
#[repr(C)] pub struct flow_action_pppoe { pub sid:u16 }
#[repr(C)] pub struct flow_action_entry { pub id:flow_action_id, pub hw_index:u32, pub cookie:usize, pub miss_cookie:u64, pub hw_stats:flow_action_hw_stats, pub destructor:Option<action_destr>, pub destructor_priv:*mut core::ffi::c_void, pub data:flow_action_entry_data, pub user_cookie:*mut flow_action_cookie }
#[repr(C)] pub struct flow_action { pub num_entries:u32, pub entries:[flow_action_entry;0] }

#[inline] pub unsafe fn flow_action_has_entries(a:*const flow_action)->bool { (*a).num_entries != 0 }
#[inline] pub unsafe fn flow_offload_has_one_action(a:*const flow_action)->bool { (*a).num_entries == 1 }
#[inline] pub unsafe fn flow_action_is_last_entry(a:*const flow_action, e:*const flow_action_entry)->bool { e == (*a).entries.as_ptr().add((*a).num_entries as usize - 1) }

#[repr(C)] pub struct flow_rule { pub match_:flow_match, pub action:flow_action }
#[repr(C)] pub struct flow_stats { pub pkts:u64, pub bytes:u64, pub drops:u64, pub lastused:u64, pub used_hw_stats:flow_action_hw_stats, pub used_hw_stats_valid:bool }
#[repr(C)] pub enum flow_block_command { FLOW_BLOCK_BIND, FLOW_BLOCK_UNBIND }
#[repr(C)] pub enum flow_block_binder_type { FLOW_BLOCK_BINDER_TYPE_UNSPEC, FLOW_BLOCK_BINDER_TYPE_CLSACT_INGRESS, FLOW_BLOCK_BINDER_TYPE_CLSACT_EGRESS, FLOW_BLOCK_BINDER_TYPE_RED_EARLY_DROP, FLOW_BLOCK_BINDER_TYPE_RED_MARK }
#[repr(C)] pub struct flow_block { pub cb_list:list_head }
#[repr(C)] pub struct flow_block_offload { pub command:flow_block_command, pub binder_type:flow_block_binder_type, pub block_shared:bool, pub unlocked_driver_cb:bool, pub net:*mut net, pub block:*mut flow_block, pub cb_list:list_head, pub driver_block_list:*mut list_head, pub extack:*mut netlink_ext_ack, pub sch:*mut Qdisc, pub cb_list_head:*mut list_head }
pub type flow_setup_cb_t = unsafe extern "C" fn(*mut tc_setup_type,*mut core::ffi::c_void,*mut core::ffi::c_void)->c_int;
#[repr(C)] pub struct flow_block_indr { pub list:list_head, pub dev:*mut net_device, pub sch:*mut Qdisc, pub binder_type:flow_block_binder_type, pub data:*mut core::ffi::c_void, pub cb_priv:*mut core::ffi::c_void, pub cleanup:Option<unsafe extern "C" fn(*mut flow_block_cb)> }
#[repr(C)] pub struct flow_block_cb { pub driver_list:list_head, pub list:list_head, pub cb:Option<flow_setup_cb_t>, pub cb_ident:*mut core::ffi::c_void, pub cb_priv:*mut core::ffi::c_void, pub release:Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, pub indr:flow_block_indr, pub refcnt:u32 }
#[repr(C)] pub enum flow_cls_command { FLOW_CLS_REPLACE, FLOW_CLS_DESTROY, FLOW_CLS_STATS, FLOW_CLS_TMPLT_CREATE, FLOW_CLS_TMPLT_DESTROY }
#[repr(C)] pub struct flow_cls_common_offload { pub chain_index:u32, pub protocol:__be16, pub prio:u32, pub skip_sw:bool, pub extack:*mut netlink_ext_ack }
#[repr(C)] pub struct flow_cls_offload { pub common:flow_cls_common_offload, pub command:flow_cls_command, pub use_act_stats:bool, pub cookie:usize, pub rule:*mut flow_rule, pub stats:flow_stats, pub classid:u32 }
#[repr(C)] pub enum offload_act_command { FLOW_ACT_REPLACE, FLOW_ACT_DESTROY, FLOW_ACT_STATS }
#[repr(C)] pub struct flow_offload_action { pub extack:*mut netlink_ext_ack, pub command:offload_act_command, pub id:flow_action_id, pub index:u32, pub cookie:usize, pub stats:flow_stats, pub action:flow_action }
#[inline] pub unsafe fn flow_stats_update(s:*mut flow_stats, bytes:u64, pkts:u64, drops:u64, lastused:u64, used:flow_action_hw_stats) { (*s).pkts=(*s).pkts.wrapping_add(pkts); (*s).bytes=(*s).bytes.wrapping_add(bytes); (*s).drops=(*s).drops.wrapping_add(drops); if lastused>(*s).lastused { (*s).lastused=lastused; } (*s).used_hw_stats|=used; (*s).used_hw_stats_valid=true; }

extern "C" {
    pub fn flow_action_cookie_create(data:*mut core::ffi::c_void, len:u32, gfp:gfp_t)->*mut flow_action_cookie;
    pub fn flow_action_cookie_destroy(cookie:*mut flow_action_cookie);
    pub fn flow_rule_alloc(num_actions:u32)->*mut flow_rule;
    pub fn offload_action_alloc(num_actions:u32)->*mut flow_offload_action;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
