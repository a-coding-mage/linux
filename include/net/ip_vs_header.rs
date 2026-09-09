/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of net/ip_vs.h. Included kernel types are external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type __u8 = u8; pub type __u16 = u16; pub type __u32 = u32; pub type __u64 = u64;
pub type __s8 = i8; pub type __s16 = i16; pub type __s32 = i32; pub type __be16 = u16; pub type __be32 = u32;
pub type u16_ = u16; pub type u32_ = u32; pub type u64_ = u64; pub type __wsum = u32;

/* External kernel declarations supplied by included headers. */
#[repr(C)] pub struct net { pub ipvs: *mut netns_ipvs }
#[repr(C)] pub struct netns_ipvs;
#[repr(C)] pub struct sk_buff { pub dev: *mut c_void }
#[repr(C)] pub struct ipv6hdr { pub saddr: in6_addr, pub daddr: in6_addr }
#[repr(C)] pub struct iphdr { pub ihl: u8, pub protocol: u8, pub saddr: u32, pub daddr: u32 }
#[repr(C)] pub struct in6_addr { pub s6_addr: [u8; 16] }
#[repr(C)] pub union nf_inet_addr { pub ip: u32, pub in6: in6_addr, pub all: [u32; 4] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct hlist_node { pub next: *mut hlist_node, pub pprev: *mut *mut hlist_node }
#[repr(C)] pub struct hlist_head { pub first: *mut hlist_node }
#[repr(C)] pub struct hlist_bl_node { pub next: *mut hlist_bl_node, pub pprev: *mut *mut hlist_bl_node }
#[repr(C)] pub struct hlist_bl_head { pub first: *mut hlist_bl_node }
#[repr(C)] pub struct rcu_head { pub next: *mut rcu_head, pub func: *mut c_void }
#[repr(C)] pub struct spinlock_t { _priv: [u8; 0] }
#[repr(C)] pub struct rw_semaphore { _priv: [u8; 0] }
#[repr(C)] pub struct mutex { _priv: [u8; 0] }
#[repr(C)] pub struct atomic_t { pub counter: c_int }
#[repr(C)] pub struct refcount_t { pub refs: atomic_t }
#[repr(C)] pub struct timer_list { _priv: [u8; 0] }
#[repr(C)] pub struct delayed_work { _priv: [u8; 0] }
#[repr(C)] pub struct work_struct { _priv: [u8; 0] }
#[repr(C)] pub struct module; #[repr(C)] pub struct task_struct; #[repr(C)] pub struct dst_entry;
#[repr(C)] pub struct ctl_table; #[repr(C)] pub struct ctl_table_header; #[repr(C)] pub struct cpumask;
#[repr(C)] pub struct seqcount_t { _priv: [u8; 0] }
#[repr(C)] pub struct siphash_key_t { pub key: [u64; 2] }
#[repr(C)] pub struct u64_stats_t { pub v: u64 }
#[repr(C)] pub struct u64_stats_sync { _priv: [u8; 0] }
#[repr(C)] pub struct tcp_states_t;
pub const AF_INET: c_int = 2; pub const AF_INET6: c_int = 10;
pub const IP_VS_HDR_INVERSE: c_int = 1; pub const IP_VS_HDR_ICMP: c_int = 2;
pub const IP_VS_DEST_F_OVERLOAD: u32 = 0x0002; pub const IP_VS_DEST_CF_AVAILABLE: u32 = 0x0001;
pub const IP_VS_CONN_TAB_MIN_BITS: c_int = 8; pub const IP_VS_CONN_TAB_MAX_BITS: c_int = 27;
pub const IP_VS_CONN_MAX: c_int = 1 << 30; pub const IP_VS_SVC_TAB_MIN_BITS: c_int = 4; pub const IP_VS_SVC_TAB_MAX_BITS: c_int = 20;
pub const IP_VS_AF_INET: c_int = 0; pub const IP_VS_AF_INET6: c_int = 1; pub const IP_VS_AF_MAX: c_int = 2;
pub const IPVS_EST_NICE: c_int = 0; pub const IPVS_EST_NTICKS: usize = 50; pub const IPVS_EST_LOAD_DIVISOR: u32 = 8;
pub const IPVS_EST_CPU_KTHREADS: u32 = 4; pub const IP_VS_RHT_TABLE_ID_MASK: u32 = 1 << 31;
pub const IP_VS_APP_TYPE_FTP: c_int = 1; pub const IP_VS_APP_MAX_PORTS: c_int = 8;
pub const DEFAULT_SYNC_THRESHOLD: c_int = 3; pub const DEFAULT_SYNC_PERIOD: c_int = 50; pub const DEFAULT_SYNC_VER: c_int = 1;
pub const DEFAULT_SLOPPY_TCP: c_int = 0; pub const DEFAULT_SLOPPY_SCTP: c_int = 0; pub const DEFAULT_SYNC_RETRIES: c_int = 0;
pub const IPVS_SYNC_WAKEUP_RATE: u32 = 8; pub const IPVS_SYNC_QLEN_MAX: u32 = 32; pub const IPVS_SYNC_PORTS_MAX: u32 = 64;

#[repr(C)] pub struct ip_vs_iphdr { pub hdr_flags: c_int, pub off: __u32, pub len: __u32, pub fragoffs: __u16, pub protocol: __s16, pub flags: __s32, pub saddr: nf_inet_addr, pub daddr: nf_inet_addr }
#[repr(C)] pub struct ip_vs_seq { pub init_seq: __u32, pub delta: __u32, pub previous_delta: __u32 }
#[repr(C)] pub struct ip_vs_counters { pub conns: u64_stats_t, pub inpkts: u64_stats_t, pub outpkts: u64_stats_t, pub inbytes: u64_stats_t, pub outbytes: u64_stats_t }
#[repr(C)] pub struct ip_vs_cpu_stats { pub cnt: ip_vs_counters, pub syncp: u64_stats_sync }
#[repr(C)] pub struct ip_vs_kstats { pub conns:u64, pub inpkts:u64, pub outpkts:u64, pub inbytes:u64, pub outbytes:u64, pub cps:u64, pub inpps:u64, pub outpps:u64, pub inbps:u64, pub outbps:u64 }
#[repr(C)] pub struct ip_vs_estimator { pub list: hlist_node, pub last_inbytes:u64, pub last_outbytes:u64, pub last_conns:u64, pub last_inpkts:u64, pub last_outpkts:u64, pub cps:u64, pub inpps:u64, pub outpps:u64, pub inbps:u64, pub outbps:u64, pub ktid:i32, pub ktrow:i8, pub ktcid:i8 }
#[repr(C)] pub struct ip_vs_stats { pub kstats:ip_vs_kstats, pub est:ip_vs_estimator, pub cpustats:*mut ip_vs_cpu_stats, pub lock:spinlock_t, pub kstats0:ip_vs_kstats }
#[repr(C)] pub struct ip_vs_stats_rcu { pub s:ip_vs_stats, pub rcu_head:rcu_head }
#[repr(C)] pub struct ip_vs_rht { pub buckets:*mut hlist_bl_head, pub new_tbl:*mut ip_vs_rht, pub seqc:*mut seqcount_t, pub lock:*mut ip_vs_aligned_lock, pub mask:c_int, pub size:c_int, pub seqc_mask:c_int, pub lock_mask:c_int, pub table_id:u32, pub u_thresh:c_int, pub l_thresh:c_int, pub lfactor:c_int, pub bits:c_int, pub hash_key:siphash_key_t, pub rcu_head:rcu_head }
#[repr(C)] pub struct ip_vs_aligned_lock { pub l: spinlock_t }
#[repr(C)] pub struct ip_vs_conn_param { pub ipvs:*mut netns_ipvs, pub caddr:*const nf_inet_addr, pub vaddr:*const nf_inet_addr, pub cport:__be16, pub vport:__be16, pub protocol:__u16, pub af:u16, pub pe:*const ip_vs_pe, pub pe_data:*mut c_char, pub pe_data_len:__u8 }
#[repr(C)] pub struct ip_vs_dest_dst { pub dst_cache:*mut dst_entry, pub dst_cookie:u32, pub dst_saddr:nf_inet_addr, pub rcu_head:rcu_head }
#[repr(C)] pub struct ipvs_sync_daemon_cfg { pub mcast_group:nf_inet_addr, pub syncid:c_int, pub sync_maxlen:u16, pub mcast_port:u16, pub mcast_af:u8, pub mcast_ttl:u8, pub mcast_ifn:[c_char; 32] }

#[inline] pub unsafe fn net_ipvs(net: *mut net) -> *mut netns_ipvs { (*net).ipvs }
#[inline] pub unsafe fn ip_vs_af_index(af:c_int)->c_int { if af==AF_INET6 { IP_VS_AF_INET6 } else { IP_VS_AF_INET } }
#[inline] pub unsafe fn ip_vs_rht_same_table(t:*mut ip_vs_rht, hash_key:u32)->bool { ((*t).table_id ^ hash_key) & IP_VS_RHT_TABLE_ID_MASK == 0 }
#[inline] pub unsafe fn ip_vs_rht_build_hash_key(t:*mut ip_vs_rht, hash:u32)->u32 { (*t).table_id | (hash & !IP_VS_RHT_TABLE_ID_MASK) }

extern "C" {
    pub fn skb_header_pointer(skb:*const sk_buff, offset:c_int, len:c_int, buffer:*mut c_void)->*mut c_void;
    pub fn skb_network_offset(skb:*const sk_buff)->c_int; pub fn ipv6_find_hdr(skb:*const sk_buff, offset:*mut u32, target:c_int, fragoffs:*mut u16, flags:*mut i32)->c_int;
    pub fn ipv6_addr_equal(a:*const in6_addr,b:*const in6_addr)->bool; pub fn csum_partial(buf:*const c_void,len:usize,sum:__wsum)->__wsum;
    pub fn ip_vs_conn_fill_param(ipvs:*mut netns_ipvs,af:c_int,protocol:c_int,caddr:*const nf_inet_addr,cport:__be16,vaddr:*const nf_inet_addr,vport:__be16,p:*mut ip_vs_conn_param);
    pub fn ip_vs_stats_init_alloc(s:*mut ip_vs_stats)->c_int; pub fn ip_vs_stats_alloc()->*mut ip_vs_stats; pub fn ip_vs_stats_release(s:*mut ip_vs_stats); pub fn ip_vs_stats_free(s:*mut ip_vs_stats);
    pub fn ip_vs_rht_free(t:*mut ip_vs_rht); pub fn ip_vs_rht_rcu_free(head:*mut rcu_head); pub fn ip_vs_rht_alloc(buckets:c_int,scounts:c_int,locks:c_int)->*mut ip_vs_rht;
    pub fn ip_vs_proto_get(proto:u16)->*mut ip_vs_protocol; pub fn ip_vs_protocol_init()->c_int; pub fn ip_vs_protocol_cleanup();
    pub fn ip_vs_conn_init()->c_int; pub fn ip_vs_conn_cleanup(); pub fn ip_vs_conn_put(cp:*mut ip_vs_conn); pub fn ip_vs_dest_update_overload(dest:*mut ip_vs_dest,mode:c_int);
    pub fn ip_vs_start_estimator(ipvs:*mut netns_ipvs,stats:*mut ip_vs_stats)->c_int; pub fn ip_vs_stop_estimator(ipvs:*mut netns_ipvs,stats:*mut ip_vs_stats);
}

#[repr(C)] pub struct ip_vs_protocol { pub next:*mut ip_vs_protocol, pub name:*mut c_char, pub protocol:u16, pub num_states:u16, pub dont_defrag:c_int }
#[repr(C)] pub struct ip_vs_pe { pub n_list:list_head, pub name:*mut c_char, pub refcnt:atomic_t, pub module:*mut module }
#[repr(C)] pub struct ip_vs_service { pub s_list:hlist_bl_node, pub hash_key:u32, pub af:u16, pub protocol:u16, pub addr:nf_inet_addr, pub fwmark:u32, pub refcnt:atomic_t, pub port:__be16, pub flags:c_uint, pub timeout:c_uint, pub netmask:__be32, pub ipvs:*mut netns_ipvs, pub destinations:list_head, pub num_dests:u32, pub stats:ip_vs_stats, pub scheduler:*mut c_void, pub sched_lock:spinlock_t, pub sched_data:*mut c_void, pub pe:*mut ip_vs_pe, pub conntrack_afmask:c_int, pub rcu_head:rcu_head }
#[repr(C)] pub struct ip_vs_dest { pub n_list:list_head, pub d_list:hlist_node, pub af:u16, pub port:__be16, pub addr:nf_inet_addr, pub flags:c_uint, pub conn_flags:atomic_t, pub weight:atomic_t, pub cflags:usize, pub last_weight:atomic_t, pub tun_type:u16, pub tun_port:__be16, pub tun_flags:u16, pub refcnt:refcount_t, pub stats:ip_vs_stats, pub idle_start:usize, pub activeconns:atomic_t, pub totalconns:atomic_t, pub persistconns:atomic_t, pub u_threshold:u32, pub l_threshold:u32, pub l_threshold_val:u32, pub dst_lock:spinlock_t, pub dest_dst:*mut ip_vs_dest_dst, pub svc:*mut ip_vs_service, pub protocol:u16, pub vport:__be16, pub vaddr:nf_inet_addr, pub vfwmark:u32, pub rcu_head:rcu_head, pub t_list:list_head, pub in_rs_table:u32 }
#[repr(C)] pub struct ip_vs_conn { pub hn0:ip_vs_conn_hnode, pub af:u8, pub cport:__be16, pub hn1:ip_vs_conn_hnode, pub daf:u8, pub dport:__be16, pub dest:*mut ip_vs_dest, pub n_control:atomic_t, pub flags:__u32, pub control:*mut ip_vs_conn, pub pe:*const ip_vs_pe, pub pe_data:*mut c_char, pub pe_data_len:u8, pub state:u16, pub old_state:u16, pub caddr:nf_inet_addr, pub vaddr:nf_inet_addr, pub daddr:nf_inet_addr, pub fwmark:u32, pub vport:__be16, pub protocol:u16, pub app:*mut c_void, pub app_data:*mut c_void, pub in_seq:ip_vs_seq, pub out_seq:ip_vs_seq, pub timer:timer_list, pub timeout:usize, pub lock:spinlock_t, pub refcnt:refcount_t, pub in_pkts:atomic_t, pub sync_endtime:usize, pub ipvs:*mut netns_ipvs, pub rcu_head:rcu_head }
#[repr(C)] pub struct ip_vs_conn_hnode { pub node:hlist_bl_node, pub hash_key:u32, pub dir:u8 }

#[inline] pub unsafe fn ip_vs_dest_conn_overhead(d:*mut ip_vs_dest)->c_int { ((*d).activeconns.counter << 8)+(*d).totalconns.counter }
#[inline] pub unsafe fn ip_vs_dest_inactconns(d:*const ip_vs_dest)->c_int { ((*d).totalconns.counter-(*d).activeconns.counter).max(0) }
#[inline] pub unsafe fn ip_vs_check_diff4(old:__be32,new:__be32,sum:__wsum)->__wsum { let diff=[!old,new]; csum_partial(diff.as_ptr() as *const c_void,core::mem::size_of_val(&diff),sum) }
#[inline] pub unsafe fn ip_vs_check_diff2(old:__be16,new:__be16,sum:__wsum)->__wsum { let diff=[!old,new]; csum_partial(diff.as_ptr() as *const c_void,core::mem::size_of_val(&diff),sum) }

/* Remaining prototypes from the header remain external kernel interfaces. */
extern "C" {
    pub fn ip_vs_new_conn_out(svc:*mut ip_vs_service,dest:*mut ip_vs_dest,skb:*mut sk_buff,iph:*const ip_vs_iphdr,dport:__be16,cport:__be16)->*mut ip_vs_conn;
    pub fn ip_vs_conn_in_get(p:*const ip_vs_conn_param)->*mut ip_vs_conn; pub fn ip_vs_conn_out_get(p:*const ip_vs_conn_param)->*mut ip_vs_conn;
    pub fn ip_vs_conn_new(p:*const ip_vs_conn_param,dest_af:c_int,daddr:*const nf_inet_addr,dport:__be16,flags:c_uint,dest:*mut ip_vs_dest,fwmark:u32)->*mut ip_vs_conn;
    pub fn ip_vs_schedule(svc:*mut ip_vs_service,skb:*mut sk_buff,pd:*mut c_void,ignored:*mut c_int,iph:*mut ip_vs_iphdr)->*mut ip_vs_conn;
    pub fn ip_vs_null_xmit(skb:*mut sk_buff,cp:*mut ip_vs_conn,pp:*mut ip_vs_protocol,iph:*mut ip_vs_iphdr)->c_int;
    pub fn ip_vs_nat_icmp(skb:*mut sk_buff,pp:*mut ip_vs_protocol,cp:*mut ip_vs_conn,dir:c_int,toff:c_uint,has_ports:bool,ciph:*mut ip_vs_iphdr)->bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
