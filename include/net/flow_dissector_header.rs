/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from net/flow_dissector.h. External kernel types and constants
 * supplied by other headers are intentionally referenced, not implemented. */

#[repr(C)]
pub struct flow_dissector_key_control { pub thoff: u16, pub addr_type: u16, pub flags: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_basic { pub n_proto: __be16, pub ip_proto: u8, pub padding: u8 }
#[repr(C)] pub struct flow_dissector_key_tags { pub flow_label: u32 }

#[repr(C)] pub struct flow_dissector_key_vlan_bits { pub vlan_id: u16, pub vlan_dei: u16, pub vlan_priority: u16 }
#[repr(C)] pub union flow_dissector_key_vlan_union { pub bits: flow_dissector_key_vlan_bits, pub vlan_tci: __be16 }
#[repr(C)] pub struct flow_dissector_key_vlan { pub value: flow_dissector_key_vlan_union, pub vlan_tpid: __be16, pub vlan_eth_type: __be16, pub padding: u16 }

#[repr(C)] pub struct flow_dissector_mpls_lse { pub mpls_ttl: u32, pub mpls_bos: u32, pub mpls_tc: u32, pub mpls_label: u32 }
pub const FLOW_DIS_MPLS_MAX: usize = 7;
#[repr(C)] pub struct flow_dissector_key_mpls { pub ls: [flow_dissector_mpls_lse; FLOW_DIS_MPLS_MAX], pub used_lses: u8 }
pub unsafe fn dissector_set_mpls_lse(mpls: *mut flow_dissector_key_mpls, lse_index: i32) { (*mpls).used_lses |= (1i32.wrapping_shl(lse_index as u32)) as u8; }

pub const FLOW_DIS_TUN_OPTS_MAX: usize = 255;
#[repr(C)] pub struct flow_dissector_key_enc_opts { pub data: [u8; FLOW_DIS_TUN_OPTS_MAX], pub len: u8, pub dst_opt_type: u32 }
#[repr(C)] pub struct flow_dissector_key_keyid { pub keyid: __be32 }
#[repr(C)] pub struct flow_dissector_key_ipv4_addrs { pub src: __be32, pub dst: __be32 }
#[repr(C)] pub struct flow_dissector_key_ipv6_addrs { pub src: in6_addr, pub dst: in6_addr }
#[repr(C)] pub struct flow_dissector_key_tipc { pub key: __be32 }
#[repr(C)] pub union flow_dissector_key_addrs_union { pub v4addrs: flow_dissector_key_ipv4_addrs, pub v6addrs: flow_dissector_key_ipv6_addrs, pub tipckey: flow_dissector_key_tipc }
#[repr(C)] pub struct flow_dissector_key_addrs { pub value: flow_dissector_key_addrs_union }
pub const ETH_ALEN_LOCAL: usize = ETH_ALEN;
#[repr(C)] pub struct flow_dissector_key_arp { pub sip: u32, pub tip: u32, pub op: u8, pub sha: [u8; ETH_ALEN_LOCAL], pub tha: [u8; ETH_ALEN_LOCAL] }
#[repr(C)] pub struct flow_dissector_key_ports_pair { pub src: __be16, pub dst: __be16 }
#[repr(C)] pub union flow_dissector_key_ports_union { pub ports: __be32, pub pair: flow_dissector_key_ports_pair }
#[repr(C)] pub struct flow_dissector_key_ports { pub value: flow_dissector_key_ports_union }
#[repr(C)] pub union flow_dissector_key_ports_range_union { pub tp: flow_dissector_key_ports, pub range: flow_dissector_key_ports_range_pair }
#[repr(C)] pub struct flow_dissector_key_ports_range_pair { pub tp_min: flow_dissector_key_ports, pub tp_max: flow_dissector_key_ports }
#[repr(C)] pub struct flow_dissector_key_ports_range { pub value: flow_dissector_key_ports_range_union }
#[repr(C)] pub struct flow_dissector_key_icmp_pair { pub type_: u8, pub code: u8 }
#[repr(C)] pub struct flow_dissector_key_icmp { pub value: flow_dissector_key_icmp_pair, pub id: u16 }
#[repr(C)] pub struct flow_dissector_key_eth_addrs { pub dst: [u8; ETH_ALEN_LOCAL], pub src: [u8; ETH_ALEN_LOCAL] }
#[repr(C)] pub struct flow_dissector_key_tcp { pub flags: __be16 }
#[repr(C)] pub struct flow_dissector_key_ip { pub tos: u8, pub ttl: u8 }
#[repr(C)] pub struct flow_dissector_key_meta { pub ingress_ifindex: i32, pub ingress_iftype: u16, pub l2_miss: u8 }
#[repr(C)] pub struct flow_dissector_key_ct { pub ct_state: u16, pub ct_zone: u16, pub ct_mark: u32, pub ct_labels: [u32; 4] }
#[repr(C)] pub struct flow_dissector_key_hash { pub hash: u32 }
#[repr(C)] pub struct flow_dissector_key_num_of_vlans { pub num_of_vlans: u8 }
#[repr(C)] pub struct flow_dissector_key_pppoe { pub session_id: __be16, pub ppp_proto: __be16, pub type_: __be16 }
#[repr(C)] pub struct flow_dissector_key_l2tpv3 { pub session_id: __be32 }
#[repr(C)] pub struct flow_dissector_key_ipsec { pub spi: __be32 }
#[repr(C)] pub struct flow_dissector_key_cfm { pub mdl_ver: u8, pub opcode: u8 }
pub const FLOW_DIS_CFM_MDL_MASK: u32 = GENMASK(7, 5);
pub const FLOW_DIS_CFM_MDL_MAX: u32 = 7;

#[repr(C)] #[derive(Copy, Clone)] pub enum flow_dissector_ctrl_flags { FLOW_DIS_IS_FRAGMENT = TCA_FLOWER_KEY_FLAGS_IS_FRAGMENT, FLOW_DIS_FIRST_FRAG = TCA_FLOWER_KEY_FLAGS_FRAG_IS_FIRST, FLOW_DIS_F_TUNNEL_CSUM = TCA_FLOWER_KEY_FLAGS_TUNNEL_CSUM, FLOW_DIS_F_TUNNEL_DONT_FRAGMENT = TCA_FLOWER_KEY_FLAGS_TUNNEL_DONT_FRAGMENT, FLOW_DIS_F_TUNNEL_OAM = TCA_FLOWER_KEY_FLAGS_TUNNEL_OAM, FLOW_DIS_F_TUNNEL_CRIT_OPT = TCA_FLOWER_KEY_FLAGS_TUNNEL_CRIT_OPT, FLOW_DIS_ENCAPSULATION = TCA_FLOWER_KEY_FLAGS_MAX << 1 }
#[repr(C)] pub enum flow_dissect_ret { FLOW_DISSECT_RET_OUT_GOOD, FLOW_DISSECT_RET_OUT_BAD, FLOW_DISSECT_RET_PROTO_AGAIN, FLOW_DISSECT_RET_IPPROTO_AGAIN, FLOW_DISSECT_RET_CONTINUE }
#[repr(C)] pub enum flow_dissector_key_id { FLOW_DISSECTOR_KEY_CONTROL, FLOW_DISSECTOR_KEY_BASIC, FLOW_DISSECTOR_KEY_IPV4_ADDRS, FLOW_DISSECTOR_KEY_IPV6_ADDRS, FLOW_DISSECTOR_KEY_PORTS, FLOW_DISSECTOR_KEY_PORTS_RANGE, FLOW_DISSECTOR_KEY_ICMP, FLOW_DISSECTOR_KEY_ETH_ADDRS, FLOW_DISSECTOR_KEY_TIPC, FLOW_DISSECTOR_KEY_ARP, FLOW_DISSECTOR_KEY_VLAN, FLOW_DISSECTOR_KEY_FLOW_LABEL, FLOW_DISSECTOR_KEY_GRE_KEYID, FLOW_DISSECTOR_KEY_MPLS_ENTROPY, FLOW_DISSECTOR_KEY_ENC_KEYID, FLOW_DISSECTOR_KEY_ENC_IPV4_ADDRS, FLOW_DISSECTOR_KEY_ENC_IPV6_ADDRS, FLOW_DISSECTOR_KEY_ENC_CONTROL, FLOW_DISSECTOR_KEY_ENC_PORTS, FLOW_DISSECTOR_KEY_MPLS, FLOW_DISSECTOR_KEY_TCP, FLOW_DISSECTOR_KEY_IP, FLOW_DISSECTOR_KEY_CVLAN, FLOW_DISSECTOR_KEY_ENC_IP, FLOW_DISSECTOR_KEY_ENC_OPTS, FLOW_DISSECTOR_KEY_META, FLOW_DISSECTOR_KEY_CT, FLOW_DISSECTOR_KEY_HASH, FLOW_DISSECTOR_KEY_NUM_OF_VLANS, FLOW_DISSECTOR_KEY_PPPOE, FLOW_DISSECTOR_KEY_L2TPV3, FLOW_DISSECTOR_KEY_CFM, FLOW_DISSECTOR_KEY_IPSEC, FLOW_DISSECTOR_KEY_MAX }
pub const FLOW_DISSECTOR_F_PARSE_1ST_FRAG: u32 = BIT(0); pub const FLOW_DISSECTOR_F_STOP_AT_FLOW_LABEL: u32 = BIT(1); pub const FLOW_DISSECTOR_F_STOP_AT_ENCAP: u32 = BIT(2); pub const FLOW_DISSECTOR_F_STOP_BEFORE_ENCAP: u32 = BIT(3);
#[repr(C)] pub struct flow_dissector_key { pub key_id: flow_dissector_key_id, pub offset: usize }
#[repr(C)] pub struct flow_dissector { pub used_keys: u64, pub offset: [u16; FLOW_DISSECTOR_KEY_MAX as usize] }
#[repr(C)] pub struct flow_keys_basic { pub control: flow_dissector_key_control, pub basic: flow_dissector_key_basic }
#[repr(C)] pub struct flow_keys { pub control: flow_dissector_key_control, pub basic: flow_dissector_key_basic, pub tags: flow_dissector_key_tags, pub vlan: flow_dissector_key_vlan, pub cvlan: flow_dissector_key_vlan, pub keyid: flow_dissector_key_keyid, pub ports: flow_dissector_key_ports, pub icmp: flow_dissector_key_icmp, pub addrs: flow_dissector_key_addrs }
pub const FLOW_KEYS_DIGEST_LEN: usize = 16;
#[repr(C)] pub struct flow_keys_digest { pub data: [u8; FLOW_KEYS_DIGEST_LEN] }
#[repr(C)] pub struct bpf_flow_dissector { pub flow_keys: *mut bpf_flow_keys, pub skb: *const sk_buff, pub data: *const core::ffi::c_void, pub data_end: *const core::ffi::c_void }

pub const FLOW_DISSECTOR_KEY_MAX: usize = 33;
extern "C" { pub fn flow_get_u32_src(flow: *const flow_keys) -> __be32; pub fn flow_get_u32_dst(flow: *const flow_keys) -> __be32; pub static mut flow_keys_dissector: flow_dissector; pub static mut flow_keys_basic_dissector: flow_dissector; pub fn make_flow_keys_digest(digest: *mut flow_keys_digest, flow: *const flow_keys); }
pub unsafe fn flow_keys_have_l4(keys: *const flow_keys) -> bool { (*keys).ports.ports != 0 || (*keys).tags.flow_label != 0 }
pub unsafe fn dissector_uses_key(d: *const flow_dissector, key_id: flow_dissector_key_id) -> bool { (*d).used_keys & (1u64 << key_id as u32) != 0 }
pub unsafe fn skb_flow_dissector_target(d: *mut flow_dissector, key_id: flow_dissector_key_id, target: *mut core::ffi::c_void) -> *mut core::ffi::c_void { (target as *mut u8).add((*d).offset[key_id as usize] as usize) as *mut core::ffi::c_void }
extern "C" { pub fn flow_hash_from_keys(keys: *mut flow_keys) -> u32; pub fn flow_hash_from_keys_seed(keys: *mut flow_keys, keyval: *const siphash_key_t) -> u32; pub fn skb_flow_get_icmp_tci(skb: *const sk_buff, key_icmp: *mut flow_dissector_key_icmp, data: *const core::ffi::c_void, thoff: i32, hlen: i32); }
pub unsafe fn flow_dissector_init_keys(c: *mut flow_dissector_key_control, b: *mut flow_dissector_key_basic) { core::ptr::write_bytes(c, 0, 1); core::ptr::write_bytes(b, 0, 1); }
/* CONFIG_BPF_SYSCALL: extern declaration is conditional in the original header. */
#[cfg(feature = "CONFIG_BPF_SYSCALL")] extern "C" { pub fn flow_dissector_bpf_prog_attach_check(net: *mut net, prog: *mut bpf_prog) -> i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
