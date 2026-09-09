/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translation of net/ip.h. C includes and external dependencies are supplied elsewhere. */

pub const IPV4_MAX_PMTU: u32 = 65535;
pub const IPV4_MIN_MTU: i32 = 68;

extern "C" {
    pub static mut sysctl_fib_sync_mem: u32;
    pub static mut sysctl_fib_sync_mem_min: u32;
    pub static mut sysctl_fib_sync_mem_max: u32;
}

#[repr(C)] pub struct sock;
#[repr(C)] pub struct sk_buff;
#[repr(C)] pub struct ip_options;
#[repr(C)] pub struct ip_options_rcu;
#[repr(C)] pub struct sockcm_cookie;
#[repr(C)] pub struct inet_sock;
#[repr(C)] pub struct net;
#[repr(C)] pub struct net_device;
#[repr(C)] pub struct packet_type;
#[repr(C)] pub struct rtable;
#[repr(C)] pub struct sockaddr;
#[repr(C)] pub struct sockaddr_unsized;
#[repr(C)] pub struct msghdr;
#[repr(C)] pub struct iphdr;
#[repr(C)] pub struct flowi;
#[repr(C)] pub struct flowi4;
#[repr(C)] pub struct sk_buff_head;
#[repr(C)] pub struct inet_cork;
#[repr(C)] pub struct kvec;
#[repr(C)] pub struct rcu_head;
#[repr(C)] pub struct dst_entry;
#[repr(C)] pub struct dst_metrics;
#[repr(C)] pub struct nlattr;
#[repr(C)] pub struct netlink_ext_ack;
#[repr(C)] pub struct flow_keys;
#[repr(C)] pub struct ipv6_pinfo;

pub type __be16 = u16; pub type __be32 = u32; pub type __sum16 = u16;
pub type __wsum = u32; pub type __u8 = u8; pub type __u16 = u16; pub type __s16 = i16;
pub type u8_t = u8; pub type u16_t = u16; pub type u32_t = u32; pub type u64_t = u64;
pub type kuid_t = u32; pub type sockptr_t = *mut core::ffi::c_void; pub type dscp_t = u8;

#[repr(C)]
pub struct inet_skb_parm { pub iif: i32, pub opt: ip_options, pub flags: u16, pub frag_max_size: u16 }
pub const IPSKB_FORWARDED: u16 = 1 << 0;
pub const IPSKB_XFRM_TUNNEL_SIZE: u16 = 1 << 1;
pub const IPSKB_XFRM_TRANSFORMED: u16 = 1 << 2;
pub const IPSKB_FRAG_COMPLETE: u16 = 1 << 3;
pub const IPSKB_REROUTED: u16 = 1 << 4;
pub const IPSKB_DOREDIRECT: u16 = 1 << 5;
pub const IPSKB_FRAG_PMTU: u16 = 1 << 6;
pub const IPSKB_L3SLAVE: u16 = 1 << 7;
pub const IPSKB_NOPOLICY: u16 = 1 << 8;
pub const IPSKB_MULTIPATH: u16 = 1 << 9;
pub const IPSKB_MCROUTE: u16 = 1 << 10;

#[inline] pub unsafe fn ipv4_l3mdev_skb(flags: u16) -> bool { flags & IPSKB_L3SLAVE != 0 }
#[inline] pub unsafe fn ip_hdrlen(skb: *const sk_buff) -> u32 { (ip_hdr(skb).as_ref().unwrap().ihl as u32) * 4 }

#[repr(C)] pub struct ipcm_cookie { pub sockc: sockcm_cookie, pub addr: __be32, pub oif: i32, pub opt: *mut ip_options_rcu, pub protocol: u8, pub ttl: u8, pub tos: i16, pub gso_size: u16 }
#[inline] pub unsafe fn ipcm_init(ipcm: *mut ipcm_cookie) { core::ptr::write_bytes(ipcm, 0, 1); (*ipcm).tos = -1; }

#[repr(C)] pub union ip_ra_chain_destructor { pub destructor: unsafe extern "C" fn(*mut sock), pub saved_sk: *mut sock }
#[repr(C)] pub struct ip_ra_chain { pub next: *mut ip_ra_chain, pub sk: *mut sock, pub u: ip_ra_chain_destructor, pub rcu: rcu_head }

pub const IP_CE: u16 = 0x8000; pub const IP_DF: u16 = 0x4000; pub const IP_MF: u16 = 0x2000; pub const IP_OFFSET: u16 = 0x1fff;
pub const IP_REPLY_ARG_NOSRCCHECK: i32 = 1;

#[repr(C)] pub struct ip_fraglist_iter { pub frag: *mut sk_buff, pub iph: *mut iphdr, pub offset: i32, pub hlen: u32 }
#[repr(C)] pub struct ip_frag_state { pub DF: bool, pub hlen: u32, pub ll_rs: u32, pub mtu: u32, pub left: u32, pub offset: i32, pub ptr: i32, pub not_last_frag: __be16 }
#[repr(C)] pub struct ip_reply_arg { pub iov: [kvec; 1], pub flags: i32, pub csum: __wsum, pub csumoffset: i32, pub bound_dev_if: i32, pub tos: u8, pub uid: kuid_t }

#[inline] pub unsafe fn ip_fraglist_next(iter: *mut ip_fraglist_iter) -> *mut sk_buff { let skb = (*iter).frag; (*iter).frag = (*skb).next; skb_mark_not_on_list(skb); skb }
#[inline] pub unsafe fn ip_defrag_user_in_between(user: u32, lower: u32, upper: u32) -> bool { user >= lower && user <= upper }
#[repr(C)] pub enum ip_defrag_users { IP_DEFRAG_LOCAL_DELIVER, IP_DEFRAG_CALL_RA_CHAIN, IP_DEFRAG_CONNTRACK_IN, __IP_DEFRAG_CONNTRACK_IN_END = 2 + 65535, IP_DEFRAG_CONNTRACK_OUT, __IP_DEFRAG_CONNTRACK_OUT_END = 4 + 65535, IP_DEFRAG_CONNTRACK_BRIDGE_IN, __IP_DEFRAG_CONNTRACK_BRIDGE_IN = 6 + 65535, IP_DEFRAG_VS_IN, IP_DEFRAG_VS_OUT, IP_DEFRAG_VS_FWD, IP_DEFRAG_AF_PACKET, IP_DEFRAG_MACVLAN }

extern "C" {
    pub fn ip_hdr(skb: *const sk_buff) -> *mut iphdr;
    pub fn skb_mark_not_on_list(skb: *mut sk_buff);
    pub fn ip_defrag(net: *mut net, skb: *mut sk_buff, user: u32) -> i32;
    pub fn ip_forward(skb: *mut sk_buff) -> i32;
    pub fn ip_send_check(ip: *mut iphdr);
    pub fn ip_init();
    pub fn ip_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn ip_local_out(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn ip_options_build(skb: *mut sk_buff, opt: *mut ip_options, daddr: __be32, rt: *mut rtable);
    pub fn ip_options_fragment(skb: *mut sk_buff);
    pub fn ip_options_undo(opt: *mut ip_options);
    pub fn ip_options_rcv_srr(skb: *mut sk_buff, dev: *mut net_device) -> i32;
    pub fn ip_cmsg_recv_offset(msg: *mut msghdr, sk: *mut sock, skb: *mut sk_buff, tlen: i32, offset: i32);
    pub fn ip_sock_set_freebind(sk: *mut sock);
    pub fn ip_sock_set_mtu_discover(sk: *mut sock, val: i32) -> i32;
    pub fn ip_sock_set_pktinfo(sk: *mut sock);
    pub fn ip_sock_set_recverr(sk: *mut sock);
    pub fn ip_sock_set_tos(sk: *mut sock, val: i32);
    pub fn ip_call_ra_chain(skb: *mut sk_buff) -> bool;
}

#[inline] pub unsafe fn ip_is_fragment(iph: *const iphdr) -> bool { ((*iph).frag_off & (IP_MF | IP_OFFSET)).to_be() != 0 }
#[inline] pub unsafe fn inetdev_valid_mtu(mtu: u32) -> bool { mtu >= IPV4_MIN_MTU as u32 }
#[inline] pub unsafe fn ip_eth_mc_map(naddr: __be32, buf: *mut i8) { let mut addr = naddr.to_be(); *buf.add(0)=1; *buf.add(1)=0; *buf.add(2)=0x5e; *buf.add(5)=(addr&0xff) as i8; addr >>= 8; *buf.add(4)=(addr&0xff) as i8; addr >>= 8; *buf.add(3)=(addr&0x7f) as i8; }
#[inline] pub unsafe fn ip_ipgre_mc_map(naddr: __be32, broadcast: *const u8, buf: *mut i8) { if (*broadcast | *broadcast.add(1) | *broadcast.add(2) | *broadcast.add(3)) != 0 { core::ptr::copy_nonoverlapping(broadcast, buf as *mut u8, 4) } else { core::ptr::copy_nonoverlapping(&naddr as *const _, buf as *mut __be32, 1) } }

extern "C" {
    pub fn ip_frag_init(skb: *mut sk_buff, hlen: u32, ll_rs: u32, mtu: u32, df: bool, state: *mut ip_frag_state);
    pub fn ip_frag_next(skb: *mut sk_buff, state: *mut ip_frag_state) -> *mut sk_buff;
    pub fn ip_queue_xmit(sk: *mut sock, skb: *mut sk_buff, fl: *mut flowi) -> i32;
    pub fn ip_make_skb(sk: *mut sock, fl4: *mut flowi4, getfrag: Option<unsafe extern "C" fn(*mut core::ffi::c_void,*mut i8,i32,i32,i32,*mut sk_buff)->i32>, from: *mut core::ffi::c_void, length: i32, transhdrlen: i32, ipc: *mut ipcm_cookie, rtp: *mut *mut rtable, cork: *mut inet_cork, flags: u32) -> *mut sk_buff;
    pub fn ip_mtu_locked(dst: *const dst_entry) -> bool;
    pub fn ip_static_sysctl_init();
    pub fn igmp_mc_init() -> i32;
    pub fn ip_build_and_send_pkt(skb:*mut sk_buff, sk:*const sock, saddr:__be32, daddr:__be32, opt:*mut ip_options_rcu, tos:u8)->i32;
    pub fn ip_rcv(skb:*mut sk_buff, dev:*mut net_device, pt:*mut packet_type, orig_dev:*mut net_device)->i32;
    pub fn ip_local_deliver(skb:*mut sk_buff)->i32;
    pub fn ip_mr_input(skb:*mut sk_buff)->i32;
    pub fn ip_mr_output(net:*mut net, sk:*mut sock, skb:*mut sk_buff)->i32;
    pub fn ip_do_fragment(net:*mut net, sk:*mut sock, skb:*mut sk_buff, output:Option<unsafe extern "C" fn(*mut net,*mut sock,*mut sk_buff)->i32>)->i32;
    pub fn ip_list_rcv(head:*mut core::ffi::c_void, pt:*mut packet_type, dev:*mut net_device);
    pub fn ip_protocol_deliver_rcu(net:*mut net, skb:*mut sk_buff, proto:i32);
    pub fn ip_send_skb(net:*mut net, skb:*mut sk_buff)->i32;
    pub fn ip_push_pending_frames(sk:*mut sock, fl4:*mut flowi4)->i32;
    pub fn ip_flush_pending_frames(sk:*mut sock);
    pub fn ip_append_data(sk:*mut sock, fl4:*mut flowi4, getfrag:Option<unsafe extern "C" fn(*mut core::ffi::c_void,*mut i8,i32,i32,i32,*mut sk_buff)->i32>, from:*mut core::ffi::c_void, len:i32, protolen:i32, ipc:*mut ipcm_cookie, rt:*mut *mut rtable, flags:u32)->i32;
    pub fn ip_generic_getfrag(from:*mut core::ffi::c_void,to:*mut i8,offset:i32,len:i32,odd:i32,skb:*mut sk_buff)->i32;
    pub fn __ip_make_skb(sk:*mut sock,fl4:*mut flowi4,queue:*mut sk_buff_head,cork:*mut inet_cork)->*mut sk_buff;
    pub fn __ip_select_ident(net:*mut net, iph:*mut iphdr, segs:i32);
    pub fn inet_current_timestamp()->__be32;
    pub fn ip4_datagram_connect(sk:*mut sock,uaddr:*mut sockaddr_unsized,addr_len:i32)->i32;
    pub fn __ip4_datagram_connect(sk:*mut sock,uaddr:*mut sockaddr_unsized,addr_len:i32)->i32;
    pub fn ip4_datagram_release_cb(sk:*mut sock);
    pub fn ip_options_compile(net:*mut net,opt:*mut ip_options,skb:*mut sk_buff)->i32;
    pub fn ip_options_get(net:*mut net,optp:*mut *mut ip_options_rcu,data:sockptr_t,optlen:i32)->i32;
    pub fn ip_options_echo(net:*mut net,dopt:*mut ip_options,skb:*mut sk_buff)->i32;
    pub fn ip_recv_error(sk:*mut sock,msg:*mut msghdr,len:i32)->i32;
    pub fn ip_icmp_error(sk:*mut sock,skb:*mut sk_buff,err:i32,port:__be16,info:u32,payload:*mut u8);
    pub fn ip_local_error(sk:*mut sock,err:i32,daddr:__be32,dport:__be16,info:u32);
    pub fn icmp_global_allow(net:*mut net)->bool;
    pub fn icmp_global_consume(net:*mut net);
    pub fn ip_ra_control(sk:*mut sock,on:u8,destructor:Option<unsafe extern "C" fn(*mut sock)>)->i32;
    pub fn do_ip_setsockopt(sk:*mut sock,level:i32,optname:i32,optval:sockptr_t,optlen:u32)->i32;
    pub fn ip_setsockopt(sk:*mut sock,level:i32,optname:i32,optval:sockptr_t,optlen:u32)->i32;
    pub fn ip_getsockopt(sk:*mut sock,level:i32,optname:i32,optval:*mut i8,optlen:*mut i32)->i32;
    pub fn ip_misc_proc_init()->i32;
    pub fn ip_check_defrag(net:*mut net,skb:*mut sk_buff,user:u32)->*mut sk_buff;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
