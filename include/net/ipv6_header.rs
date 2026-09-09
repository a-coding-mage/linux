/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Linux INET6 implementation -- source-level Rust translation. */

/* C dependencies supplied by the surrounding kernel translation unit. */

pub const SIN6_LEN_RFC2133: usize = 24;
pub const NEXTHDR_HOP: u8 = 0; pub const NEXTHDR_IPV4: u8 = 4;
pub const NEXTHDR_TCP: u8 = 6; pub const NEXTHDR_UDP: u8 = 17;
pub const NEXTHDR_IPV6: u8 = 41; pub const NEXTHDR_ROUTING: u8 = 43;
pub const NEXTHDR_FRAGMENT: u8 = 44; pub const NEXTHDR_GRE: u8 = 47;
pub const NEXTHDR_ESP: u8 = 50; pub const NEXTHDR_AUTH: u8 = 51;
pub const NEXTHDR_ICMP: u8 = 58; pub const NEXTHDR_NONE: u8 = 59;
pub const NEXTHDR_DEST: u8 = 60; pub const NEXTHDR_SCTP: u8 = 132;
pub const NEXTHDR_MOBILITY: u8 = 135; pub const NEXTHDR_MAX: u8 = 255;
pub const IPV6_DEFAULT_HOPLIMIT: u8 = 64;
pub const IPV6_DEFAULT_MCASTHOPS: u8 = 1;
pub const IP6_DEFAULT_MAX_DST_OPTS_CNT: i32 = 8;
pub const IP6_DEFAULT_MAX_HBH_OPTS_CNT: i32 = 8;
pub const IP6_DEFAULT_MAX_DST_OPTS_LEN: i32 = i32::MAX;
pub const IP6_DEFAULT_MAX_HBH_OPTS_LEN: i32 = i32::MAX;
pub const IP6_MAX_EXT_HDRS_CNT: i32 = 12;
pub const IPV6_ADDR_ANY: u32 = 0x0000; pub const IPV6_ADDR_UNICAST: u32 = 0x0001;
pub const IPV6_ADDR_MULTICAST: u32 = 0x0002; pub const IPV6_ADDR_LOOPBACK: u32 = 0x0010;
pub const IPV6_ADDR_LINKLOCAL: u32 = 0x0020; pub const IPV6_ADDR_SITELOCAL: u32 = 0x0040;
pub const IPV6_ADDR_COMPATV4: u32 = 0x0080; pub const IPV6_ADDR_SCOPE_MASK: u32 = 0x00f0;
pub const IPV6_ADDR_MAPPED: u32 = 0x1000;
pub const IPV6_ADDR_SCOPE_INVALID: i32 = -1;
pub const IPV6_ADDR_SCOPE_NODELOCAL: u8 = 1; pub const IPV6_ADDR_SCOPE_LINKLOCAL: u8 = 2;
pub const IPV6_ADDR_SCOPE_SITELOCAL: u8 = 5; pub const IPV6_ADDR_SCOPE_ORGLOCAL: u8 = 8;
pub const IPV6_ADDR_SCOPE_GLOBAL: u8 = 14;

#[repr(C)] pub struct frag_hdr { pub nexthdr: u8, pub reserved: u8, pub frag_off: __be16, pub identification: __be32 }
pub const IP6_MF: u16 = 0x0001; pub const IP6_OFFSET: u16 = 0xfff8;
#[repr(C)] pub struct ip6_fraglist_iter { pub tmp_hdr: *mut ipv6hdr, pub frag: *mut sk_buff, pub offset: c_int, pub hlen: c_uint, pub frag_id: __be32, pub nexthdr: u8 }
#[repr(C)] pub struct ip6_frag_state { pub prevhdr: *mut u8, pub hlen: c_uint, pub mtu: c_uint, pub left: c_uint, pub offset: c_int, pub ptr: c_int, pub hroom: c_int, pub troom: c_int, pub frag_id: __be32, pub nexthdr: u8 }

#[repr(C)] pub struct ipv6_txoptions { pub refcnt: refcount_t, pub tot_len: c_int, pub opt_flen: __u16, pub opt_nflen: __u16, pub hopopt: *mut ipv6_opt_hdr, pub dst0opt: *mut ipv6_opt_hdr, pub srcrt: *mut ipv6_rt_hdr, pub dst1opt: *mut ipv6_opt_hdr, pub rcu: rcu_head }
#[repr(C)] pub struct ip6_flowlabel { pub next: *mut ip6_flowlabel, pub label: __be32, pub users: atomic_t, pub dst: in6_addr, pub opt: *mut ipv6_txoptions, pub linger: c_ulong, pub rcu: rcu_head, pub share: u8, pub owner: ip6_flowlabel_owner, pub lastuse: c_ulong, pub expires: c_ulong, pub fl_net: *mut net }
#[repr(C)] pub union ip6_flowlabel_owner { pub pid: *mut pid, pub uid: kuid_t }
#[repr(C)] pub struct ipv6_fl_socklist { pub next: *mut ipv6_fl_socklist, pub fl: *mut ip6_flowlabel, pub rcu: rcu_head }
#[repr(C)] pub struct ipcm6_cookie { pub sockc: sockcm_cookie, pub hlimit: __s16, pub tclass: __s16, pub gso_size: __u16, pub dontfrag: __s8, pub opt: *mut ipv6_txoptions }
#[repr(C)] pub struct ip6_ra_chain { pub next: *mut ip6_ra_chain, pub sk: *mut sock, pub sel: c_int, pub destructor: Option<unsafe extern "C" fn(*mut sock)> }

#[repr(C)] pub enum flowlabel_reflect { FLOWLABEL_REFLECT_ESTABLISHED = 1, FLOWLABEL_REFLECT_TCP_RESET = 2, FLOWLABEL_REFLECT_ICMPV6_ECHO_REPLIES = 4 }
pub const IPV6_FLOWINFO_MASK: __be32 = 0x0fffffff; pub const IPV6_FLOWLABEL_MASK: __be32 = 0x000fffff; pub const IPV6_FLOWLABEL_STATELESS_FLAG: __be32 = 0x00080000; pub const IPV6_TCLASS_SHIFT: u32 = 20;
pub const IP6_AUTO_FLOW_LABEL_OFF: u8 = 0; pub const IP6_AUTO_FLOW_LABEL_OPTOUT: u8 = 1; pub const IP6_AUTO_FLOW_LABEL_OPTIN: u8 = 2; pub const IP6_AUTO_FLOW_LABEL_FORCED: u8 = 3;
pub const IP6_AUTO_FLOW_LABEL_MAX: u8 = IP6_AUTO_FLOW_LABEL_FORCED; pub const IP6_DEFAULT_AUTO_FLOW_LABELS: u8 = IP6_AUTO_FLOW_LABEL_OPTOUT;
pub const IPV6_ADDR_WORDS: usize = 4;

extern "C" {
    pub static mut sysctl_mld_max_msf: c_int; pub static mut sysctl_mld_qrv: c_int;
    pub static mut ip6_ra_chain: *mut ip6_ra_chain; pub static mut ip6_ra_lock: rwlock_t;
    pub fn ip6_fraglist_init(skb:*mut sk_buff, hlen:c_uint, prevhdr:*mut u8, nexthdr:u8, frag_id:__be32, iter:*mut ip6_fraglist_iter)->c_int;
    pub fn ip6_fraglist_prepare(skb:*mut sk_buff, iter:*mut ip6_fraglist_iter);
    pub fn ip6_frag_init(skb:*mut sk_buff, hlen:c_uint, mtu:c_uint, needed_tailroom:c_ushort, hdr_room:c_int, prevhdr:*mut u8, nexthdr:u8, frag_id:__be32, state:*mut ip6_frag_state);
    pub fn ip6_frag_next(skb:*mut sk_buff, state:*mut ip6_frag_state)->*mut sk_buff;
    pub fn __ipv6_addr_type(addr:*const in6_addr)->c_int;
    pub fn jhash2(k:*const u32, length:usize, initval:u32)->u32;
    pub fn ipv6_select_ident(net:*mut net,daddr:*const in6_addr,saddr:*const in6_addr)->__be32;
    pub fn ipv6_proxy_select_ident(net:*mut net,skb:*mut sk_buff)->__be32;
    pub fn ip6_dst_hoplimit(dst:*mut dst_entry)->c_int;
    pub fn ipv6_rcv(skb:*mut sk_buff,dev:*mut net_device,pt:*mut packet_type,orig_dev:*mut net_device)->c_int;
    pub fn ip6_output(net:*mut net,sk:*mut sock,skb:*mut sk_buff)->c_int;
    pub fn ip6_forward(skb:*mut sk_buff)->c_int; pub fn ip6_input(skb:*mut sk_buff)->c_int; pub fn ip6_mc_input(skb:*mut sk_buff)->c_int;
    pub fn ipv6_ext_hdr(nexthdr:u8)->bool;
}

/* The remaining kernel API prototypes are intentionally retained as external declarations. */
extern "C" {
    pub fn ipv6_addr_cmp(a1:*const in6_addr,a2:*const in6_addr)->c_int;
    pub fn ipv6_parse_hopopts(skb:*mut sk_buff)->c_int;
    pub fn ipv6_find_hdr(skb:*const sk_buff,offset:*mut c_uint,target:c_int,fragoff:*mut c_ushort,fragflg:*mut c_int)->c_int;
    pub fn ipv6_find_tlv(skb:*const sk_buff,offset:c_int,typ:c_int)->c_int;
    pub fn ip6_datagram_connect(sk:*mut sock,addr:*mut sockaddr_unsized,addr_len:c_int)->c_int;
    pub fn inet6_release(sock:*mut socket)->c_int;
    pub fn inet6_sendmsg(sock:*mut socket,msg:*mut msghdr,size:usize)->isize;
    pub fn inet6_recvmsg(sock:*mut socket,msg:*mut msghdr,size:usize,flags:c_int)->isize;
    pub fn ipv6_sock_mc_join(sk:*mut sock,ifindex:c_int,addr:*const in6_addr)->c_int;
    pub fn ipv6_sock_mc_drop(sk:*mut sock,ifindex:c_int,addr:*const in6_addr)->c_int;
}

// C type aliases and kernel declarations are provided by dependent translated headers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
