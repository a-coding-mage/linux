/* Translation of nf_flow_table.h. C includes and configuration-selected
 * dependencies are supplied by the surrounding kernel translation. */

use core::ffi::c_void;

#[repr(C)] pub struct nf_flow_key { pub meta: flow_dissector_key_meta, pub control: flow_dissector_key_control, pub enc_control: flow_dissector_key_control, pub basic: flow_dissector_key_basic, pub vlan: flow_dissector_key_vlan, pub cvlan: flow_dissector_key_vlan, pub addr: nf_flow_key_addr, pub enc_key_id: flow_dissector_key_keyid, pub enc_addr: nf_flow_key_enc_addr, pub tcp: flow_dissector_key_tcp, pub tp: flow_dissector_key_ports }
#[repr(C)] pub union nf_flow_key_addr { pub ipv4: flow_dissector_key_ipv4_addrs, pub ipv6: flow_dissector_key_ipv6_addrs }
#[repr(C)] pub union nf_flow_key_enc_addr { pub enc_ipv4: flow_dissector_key_ipv4_addrs, pub enc_ipv6: flow_dissector_key_ipv6_addrs }
#[repr(C)] pub struct nf_flow_match { pub dissector: flow_dissector, pub key: nf_flow_key, pub mask: nf_flow_key }
#[repr(C)] pub struct nf_flow_rule { pub r#match: nf_flow_match, pub rule: *mut flow_rule }
#[repr(C)] pub struct nf_flowtable_type { pub list: list_head, pub family: i32, pub init: Option<unsafe extern "C" fn(*mut nf_flowtable) -> i32>, pub gc: Option<unsafe extern "C" fn(*const flow_offload) -> bool>, pub setup: Option<unsafe extern "C" fn(*mut nf_flowtable,*mut net_device,flow_block_command)->i32>, pub action: Option<unsafe extern "C" fn(*mut net,*mut flow_offload,flow_offload_tuple_dir,*mut nf_flow_rule)->i32>, pub free: Option<unsafe extern "C" fn(*mut nf_flowtable)>, pub get: Option<unsafe extern "C" fn(*mut nf_flowtable)>, pub put: Option<unsafe extern "C" fn(*mut nf_flowtable)>, pub hook: *mut c_void, pub owner: *mut module }
#[repr(C)] pub struct nf_flowtable { pub flags: u32, pub priority: i32, pub rhashtable: rhashtable, pub list: list_head, pub r#type: *const nf_flowtable_type, pub gc_work: delayed_work, pub flow_block: flow_block, pub flow_block_lock: rw_semaphore, pub net: possible_net_t }
pub const NF_FLOWTABLE_HW_OFFLOAD: u32 = 0x1;
pub const NF_FLOWTABLE_COUNTER: u32 = 0x2;
#[inline] pub unsafe fn nf_flowtable_hw_offload(f: *mut nf_flowtable) -> bool { (*f).flags & NF_FLOWTABLE_HW_OFFLOAD != 0 }
#[repr(C)] #[derive(Copy,Clone)] pub enum flow_offload_tuple_dir { FLOW_OFFLOAD_DIR_ORIGINAL = IP_CT_DIR_ORIGINAL as isize, FLOW_OFFLOAD_DIR_REPLY = IP_CT_DIR_REPLY as isize }
pub const FLOW_OFFLOAD_DIR_MAX: usize = IP_CT_DIR_MAX as usize;
#[repr(C)] pub enum flow_offload_xmit_type { FLOW_OFFLOAD_XMIT_UNSPEC=0, FLOW_OFFLOAD_XMIT_NEIGH, FLOW_OFFLOAD_XMIT_XFRM, FLOW_OFFLOAD_XMIT_DIRECT, FLOW_OFFLOAD_XMIT_TC }
pub const NF_FLOW_TABLE_ENCAP_MAX: usize = 2;
#[repr(C)] pub union flow_offload_tunnel_addr { pub src_v4: in_addr, pub src_v6: in6_addr }
#[repr(C)] pub union flow_offload_tunnel_dst { pub dst_v4: in_addr, pub dst_v6: in6_addr }
#[repr(C)] pub struct flow_offload_tunnel { pub src: flow_offload_tunnel_addr, pub dst: flow_offload_tunnel_dst, pub inner_proto: u8 }
#[repr(C)] pub struct flow_offload_encap { pub id: u16, pub proto: __be16 }
#[repr(C)] pub union flow_offload_tuple_addr { pub src_v4: in_addr, pub src_v6: in6_addr }
#[repr(C)] pub union flow_offload_tuple_dst { pub dst_v4: in_addr, pub dst_v6: in6_addr }
#[repr(C)] pub union flow_offload_tuple_out { pub ifidx: u32, pub out: flow_offload_tuple_out_eth, pub tc: flow_offload_tuple_tc }
#[repr(C)] pub struct flow_offload_tuple_out_eth { pub ifidx: u32, pub h_source: [u8; ETH_ALEN], pub h_dest: [u8; ETH_ALEN] }
#[repr(C)] pub struct flow_offload_tuple_tc { pub iifidx: u32 }
#[repr(C)] pub struct flow_offload_tuple { pub src: flow_offload_tuple_addr, pub dst: flow_offload_tuple_dst, pub src_port: __be16, pub dst_port: __be16, pub iifidx: i32, pub l3proto: u8, pub l4proto: u8, pub encap: [flow_offload_encap; NF_FLOW_TABLE_ENCAP_MAX], pub tun: flow_offload_tunnel, pub __hash: [u8;0], pub bitfields: u16, pub mtu: u16, pub dst_cookie: u32, pub dst_cache: *mut dst_entry, pub output: flow_offload_tuple_out }
#[repr(C)] pub struct flow_offload_tuple_rhash { pub node: rhash_head, pub tuple: flow_offload_tuple }
#[repr(C)] pub enum nf_flow_flags { NF_FLOW_SNAT, NF_FLOW_DNAT, NF_FLOW_CLOSING, NF_FLOW_TEARDOWN, NF_FLOW_HW, NF_FLOW_HW_DYING, NF_FLOW_HW_DEAD, NF_FLOW_HW_PENDING, NF_FLOW_HW_BIDIRECTIONAL, NF_FLOW_HW_ESTABLISHED }
#[repr(C)] pub enum flow_offload_type { NF_FLOW_OFFLOAD_UNSPEC=0, NF_FLOW_OFFLOAD_ROUTE }
#[repr(C)] pub struct flow_offload { pub tuplehash: [flow_offload_tuple_rhash; FLOW_OFFLOAD_DIR_MAX], pub ct: *mut nf_conn, pub flags: c_ulong, pub r#type: u16, pub timeout: u32, pub rcu_head: rcu_head }
pub const NF_FLOW_TIMEOUT: u64 = 30 * HZ as u64;
#[inline] pub unsafe fn nf_flowtable_time_stamp() -> u32 { jiffies as u32 }
extern "C" { pub fn flow_offload_get_timeout(flow:*mut flow_offload)->c_ulong; pub fn flow_offload_alloc(ct:*mut nf_conn)->*mut flow_offload; pub fn flow_offload_free(flow:*mut flow_offload); pub fn flow_offload_route_init(flow:*mut flow_offload, route:*mut nf_flow_route); pub fn flow_offload_add(ft:*mut nf_flowtable, flow:*mut flow_offload)->i32; pub fn flow_offload_refresh(ft:*mut nf_flowtable, flow:*mut flow_offload, force:bool); pub fn flow_offload_lookup(ft:*mut nf_flowtable, tuple:*mut flow_offload_tuple)->*mut flow_offload_tuple_rhash; pub fn nf_flow_table_gc_run(ft:*mut nf_flowtable); pub fn nf_flow_table_gc_cleanup(ft:*mut nf_flowtable, dev:*mut net_device); pub fn nf_flow_table_cleanup(dev:*mut net_device); pub fn nf_flow_table_init(ft:*mut nf_flowtable)->i32; pub fn nf_flow_table_free(ft:*mut nf_flowtable); pub fn flow_offload_teardown(flow:*mut flow_offload); }
#[inline] pub unsafe fn nf_flow_timeout_delta(timeout:u32)->i32 { (timeout.wrapping_sub(nf_flowtable_time_stamp())) as i32 }
#[repr(C)] pub struct nf_flow_route { pub tuple: [nf_flow_route_tuple; FLOW_OFFLOAD_DIR_MAX] }
#[repr(C)] pub struct nf_flow_route_tuple { pub dst:*mut dst_entry, pub input:nf_flow_route_in, pub output:nf_flow_route_out, pub xmit_type:flow_offload_xmit_type }
#[repr(C)] pub struct nf_flow_route_in { pub ifindex:u32, pub encap:[flow_offload_encap;NF_FLOW_TABLE_ENCAP_MAX], pub tun:flow_offload_tunnel, pub bitfields:u8 }
#[repr(C)] pub struct nf_flow_route_out { pub ifindex:u32, pub hw_ifindex:u32, pub h_source:[u8;ETH_ALEN], pub h_dest:[u8;ETH_ALEN], pub needs_gso_segment:u8 }
#[repr(C)] pub struct flow_ports { pub source:__be16, pub dest:__be16 }
#[inline] pub unsafe fn nf_flow_dst_check(t:*mut flow_offload_tuple)->bool { (*t).dst_cache.is_null() || dst_check((*t).dst_cache,(*t).dst_cookie) }
extern "C" { pub fn nft_flow_route(pkt:*const nft_pktinfo,ct:*const nf_conn,route:*mut nf_flow_route,dir:ip_conntrack_dir,ft:*mut nft_flowtable)->i32; pub fn nf_flowtable_by_dev(dev:*const net_device)->*mut nf_flowtable; pub fn nf_flow_offload_xdp_setup(ft:*mut nf_flowtable,dev:*mut net_device,cmd:flow_block_command)->i32; pub fn nf_flow_offload_ip_hook(priv_:*mut c_void,skb:*mut sk_buff,state:*const nf_hook_state)->u32; pub fn nf_flow_offload_ipv6_hook(priv_:*mut c_void,skb:*mut sk_buff,state:*const nf_hook_state)->u32; pub fn nf_flow_table_offload_init()->i32; pub fn nf_flow_table_offload_exit(); }
extern "C" { pub fn nf_flow_snat_port(flow:*const flow_offload,skb:*mut sk_buff,thoff:u32,protocol:u8,dir:flow_offload_tuple_dir); pub fn nf_flow_dnat_port(flow:*const flow_offload,skb:*mut sk_buff,thoff:u32,protocol:u8,dir:flow_offload_tuple_dir); pub fn nf_flow_table_offload_flush(ft:*mut nf_flowtable); pub fn nf_flow_table_offload_flush_cleanup(ft:*mut nf_flowtable); pub fn nf_flow_table_offload_setup(ft:*mut nf_flowtable,dev:*mut net_device,cmd:flow_block_command)->i32; pub fn nf_flow_rule_route_ipv4(net:*mut net,flow:*mut flow_offload,dir:flow_offload_tuple_dir,rule:*mut nf_flow_rule)->i32; pub fn nf_flow_rule_route_ipv6(net:*mut net,flow:*mut flow_offload,dir:flow_offload_tuple_dir,rule:*mut nf_flow_rule)->i32; pub fn nf_flow_offload_add(ft:*mut nf_flowtable,flow:*mut flow_offload); pub fn nf_flow_offload_refresh(ft:*mut nf_flowtable,flow:*mut flow_offload); pub fn nf_flow_offload_del(ft:*mut nf_flowtable,flow:*mut flow_offload); pub fn nf_flow_offload_stats(ft:*mut nf_flowtable,flow:*mut flow_offload); }
extern "C" { pub fn down_write(s:*mut rw_semaphore); pub fn up_write(s:*mut rw_semaphore); pub fn flow_block_cb_lookup(b:*mut flow_block,cb:*mut c_void,p:*mut c_void)->*mut flow_block_cb; pub fn flow_block_cb_alloc(cb:*mut c_void,p:*mut c_void,ident:*mut c_void,data:*mut c_void)->*mut flow_block_cb; pub fn flow_block_cb_free(cb:*mut flow_block_cb); pub fn list_add_tail(n:*mut list_head,h:*mut list_head); pub fn list_del(n:*mut list_head); pub fn warn_on(c:bool)->bool; }
#[inline] pub unsafe fn nf_flow_table_offload_add_cb(ft:*mut nf_flowtable, cb:*mut c_void, priv_:*mut c_void)->i32 { let b=&mut (*ft).flow_block; down_write(&mut (*ft).flow_block_lock); let old=flow_block_cb_lookup(b,cb,priv_); if !old.is_null(){up_write(&mut (*ft).flow_block_lock);return -17;} let x=flow_block_cb_alloc(cb,priv_,priv_,core::ptr::null_mut()); if x.is_null(){up_write(&mut (*ft).flow_block_lock);return -12;} list_add_tail(&mut (*x).list,&mut (*b).cb_list); up_write(&mut (*ft).flow_block_lock); if let Some(get)=(*ft).r#type.as_ref().and_then(|t|t.get){get(ft)} 0 }
#[inline] pub unsafe fn nf_flow_table_offload_del_cb(ft:*mut nf_flowtable,cb:*mut c_void,priv_:*mut c_void){ let b=&mut (*ft).flow_block; down_write(&mut (*ft).flow_block_lock); let x=flow_block_cb_lookup(b,cb,priv_); if !x.is_null(){list_del(&mut (*x).list);flow_block_cb_free(x);}else{warn_on(true);}up_write(&mut (*ft).flow_block_lock);if let Some(put)=(*ft).r#type.as_ref().and_then(|t|t.put){put(ft)} }
#[cfg(any(feature="CONFIG_NF_FLOW_TABLE_PROCFS"))] extern "C" { pub fn nf_flow_table_init_proc(net:*mut net)->i32; pub fn nf_flow_table_fini_proc(net:*mut net); }
#[cfg(any(feature="CONFIG_DEBUG_INFO_BTF",feature="CONFIG_DEBUG_INFO_BTF_MODULES"))] extern "C" { pub fn nf_flow_register_bpf()->i32; }
#[cfg(not(any(feature="CONFIG_DEBUG_INFO_BTF",feature="CONFIG_DEBUG_INFO_BTF_MODULES")))] #[inline] pub unsafe fn nf_flow_register_bpf()->i32 { 0 }
#[inline] pub unsafe fn __nf_flow_pppoe_proto(skb:*const sk_buff)->__be16 { let p=*((skb_mac_header(skb).add(ETH_HLEN as usize + core::mem::size_of::<pppoe_hdr>())) as *const __be16); if p==htons(PPP_IP) { htons(ETH_P_IP) } else if p==htons(PPP_IPV6) { htons(ETH_P_IPV6) } else { 0 } }
#[inline] pub unsafe fn nf_flow_pppoe_proto(skb:*mut sk_buff, inner:*mut __be16)->bool { if !pskb_may_pull(skb, (ETH_HLEN+PPPOE_SES_HLEN) as u32) { return false; } *inner=__nf_flow_pppoe_proto(skb); true }
#[cfg(not(any(feature="CONFIG_NF_FLOW_TABLE_PROCFS")))] #[inline] pub unsafe fn nf_flow_table_init_proc(_: *mut net)->i32 { 0 }
#[cfg(not(any(feature="CONFIG_NF_FLOW_TABLE_PROCFS")))] #[inline] pub unsafe fn nf_flow_table_fini_proc(_: *mut net) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
