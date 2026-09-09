// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level translation of nf_log_syslog.c. Kernel-provided types,
 * constants, macros, and functions are intentionally left external. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    static mut init_net: net;
    static mut sysctl_nf_log_all_netns: bool;
    static default_loginfo: nf_loginfo;
    fn net_eq(a: *const net, b: *const net) -> bool;
    fn nf_log_buf_open() -> *mut nf_log_buf;
    fn nf_log_buf_close(m: *mut nf_log_buf);
    fn nf_log_buf_add(m: *mut nf_log_buf, fmt: *const c_char, ...);
    fn skb_vlan_tag_present(skb: *const sk_buff) -> bool;
    fn skb_vlan_tag_get(skb: *const sk_buff) -> u16;
    fn skb_header_pointer(skb: *const sk_buff, offset: c_uint, len: usize, buffer: *mut c_void) -> *const c_void;
    fn skb_network_offset(skb: *const sk_buff) -> c_uint;
    fn skb_mac_header_was_set(skb: *const sk_buff) -> bool;
    fn skb_mac_header_len(skb: *const sk_buff) -> c_uint;
    fn eth_hdr(skb: *const sk_buff) -> *const ethhdr;
    fn nf_bridge_get_physindev(skb: *const sk_buff, net: *mut net) -> *const net_device;
    fn nf_bridge_get_physoutdev(skb: *const sk_buff) -> *const net_device;
    fn nf_log_set(net: *mut net, pf: u8, logger: *mut nf_logger) -> c_int;
    fn nf_log_unset(net: *mut net, logger: *mut nf_logger);
    fn nf_log_register(pf: u8, logger: *mut nf_logger) -> c_int;
    fn nf_log_unregister(logger: *mut nf_logger);
    fn register_pernet_subsys(ops: *mut pernet_operations) -> c_int;
    fn unregister_pernet_subsys(ops: *mut pernet_operations);
}

#[repr(C)] pub struct net { _opaque: [u8; 0] }
#[repr(C)] pub struct net_device { pub name: [c_char; 16], pub type_: u16, pub hard_header_len: u16 }
#[repr(C)] pub struct sock { _opaque: [u8; 0] }
#[repr(C)] pub struct nf_log_buf { _opaque: [u8; 0] }
#[repr(C)] pub struct file { _opaque: [u8; 0] }
#[repr(C)] pub struct socket { _opaque: [u8; 0] }
#[repr(C)] pub struct cred { _opaque: [u8; 0] }
#[repr(C)] pub struct ethhdr { pub h_dest: [u8; 6], pub h_source: [u8; 6], pub h_proto: u16 }
#[repr(C)] pub struct sk_buff { pub len: c_uint, pub dev: *mut net_device, pub sk: *mut sock, pub mark: u32, pub protocol: u16, pub head: *mut u8 }
#[repr(C)] pub struct nf_loginfo { pub type_: u8, pub log: nf_loginfo_log }
#[repr(C)] pub struct nf_loginfo_log { pub level: u8, pub logflags: u32 }
#[repr(C)] pub struct nf_logger { pub name: *const c_char, pub type_: u8, pub logfn: Option<unsafe extern "C" fn(*mut net,u8,c_uint,*const sk_buff,*const net_device,*const net_device,*const nf_loginfo,*const c_char)>, pub me: *mut c_void }
#[repr(C)] pub struct pernet_operations { pub init: Option<unsafe extern "C" fn(*mut net)->c_int>, pub pre_exit: Option<unsafe extern "C" fn(*mut net)> }

#[repr(C)] struct arphdr { ar_hrd: u16, ar_pro: u16, ar_hln: u8, ar_pln: u8, ar_op: u16 }
#[repr(C)] struct arppayload { mac_src: [u8;6], ip_src: [u8;4], mac_dst: [u8;6], ip_dst: [u8;4] }

const NF_LOG_TYPE_LOG: u8 = 0; const NF_LOG_DEFAULT_MASK: u32 = 0xffff_ffff;
const NF_LOG_MACDECODE:u32=1<<0; const NF_LOG_TCPSEQ:u32=1<<1; const NF_LOG_TCPOPT:u32=1<<2;
const NF_LOG_IPOPT:u32=1<<3; const NF_LOG_UID:u32=1<<4;
const ARPHRD_ETHER:u16=1; const ETH_HLEN:c_uint=14; const IPPROTO_TCP:u8=6; const IPPROTO_UDP:u8=17;
const IPPROTO_UDPLITE:u8=136; const IPPROTO_ICMP:u8=1; const IPPROTO_AH:u8=51; const IPPROTO_ESP:u8=50;

static mut default_loginfo_r: nf_loginfo = nf_loginfo { type_: NF_LOG_TYPE_LOG, log: nf_loginfo_log { level: 5, logflags: NF_LOG_DEFAULT_MASK } };

unsafe fn nf_log_allowed(n: *const net) -> bool { net_eq(n, &init_net) || sysctl_nf_log_all_netns }

unsafe fn nf_log_dump_vlan(m:*mut nf_log_buf, skb:*const sk_buff) {
    if !skb_vlan_tag_present(skb) { return; }
    nf_log_buf_add(m, b"VPROTO=%04x VID=%u \0".as_ptr() as _, 0u16, skb_vlan_tag_get(skb));
}

unsafe fn dump_arp_packet(m:*mut nf_log_buf, info:*const nf_loginfo, skb:*const sk_buff, nhoff:c_uint) {
    let mut ah=core::mem::MaybeUninit::<arphdr>::uninit();
    let p=skb_header_pointer(skb,nhoff,core::mem::size_of::<arphdr>(),ah.as_mut_ptr() as _);
    if p.is_null(){nf_log_buf_add(m,b"TRUNCATED\0".as_ptr() as _);return;}
    let ah=&*(p as *const arphdr); let flags=if (*info).type_==NF_LOG_TYPE_LOG {(*info).log.logflags}else{NF_LOG_DEFAULT_MASK};
    if flags&NF_LOG_MACDECODE!=0 && !(*skb).dev.is_null() && (*(*skb).dev).type_==ARPHRD_ETHER && skb_mac_header_was_set(skb) && skb_mac_header_len(skb)>=ETH_HLEN { nf_log_buf_add(m,b"MACSRC=%pM MACDST=%pM \0".as_ptr() as _,(*eth_hdr(skb)).h_source.as_ptr(),(*eth_hdr(skb)).h_dest.as_ptr()); nf_log_dump_vlan(m,skb); nf_log_buf_add(m,b"MACPROTO=%04x \0".as_ptr() as _,u16::from_be(eth_hdr(skb).as_ref().unwrap().h_proto)); }
    nf_log_buf_add(m,b"ARP HTYPE=%d PTYPE=0x%04x OPCODE=%d\0".as_ptr() as _,u16::from_be(ah.ar_hrd),u16::from_be(ah.ar_pro),u16::from_be(ah.ar_op));
    if ah.ar_hrd!=u16::to_be(ARPHRD_ETHER)||ah.ar_hln!=6||ah.ar_pln!=4{return;}
    let mut ap=core::mem::MaybeUninit::<arppayload>::uninit(); let q=skb_header_pointer(skb,nhoff+core::mem::size_of::<arphdr>() as u32,core::mem::size_of::<arppayload>(),ap.as_mut_ptr() as _); if q.is_null(){nf_log_buf_add(m,b" INCOMPLETE [%zu bytes]\0".as_ptr() as _,(*skb).len as usize-core::mem::size_of::<arphdr>());return;} let ap=&*(q as *const arppayload); nf_log_buf_add(m,b" MACSRC=%pM IPSRC=%pI4 MACDST=%pM IPDST=%pI4\0".as_ptr() as _,ap.mac_src.as_ptr(),ap.ip_src.as_ptr(),ap.mac_dst.as_ptr(),ap.ip_dst.as_ptr());
}

unsafe fn nf_log_dump_packet_common(m:*mut nf_log_buf,_pf:u8,_hook:c_uint,_skb:*const sk_buff,in_:*const net_device,out:*const net_device,li:*const nf_loginfo,prefix:*const c_char,_net:*mut net){nf_log_buf_add(m,b"\x01%c%sIN=%s OUT=%s \0".as_ptr() as _,b'0'+(*li).log.level,prefix,if in_.is_null(){b"\0".as_ptr()}else{(*in_).name.as_ptr()},if out.is_null(){b"\0".as_ptr()}else{(*out).name.as_ptr()});}

unsafe extern "C" fn nf_log_arp_packet(n:*mut net,p:u8,h:c_uint,s:*const sk_buff,i:*const net_device,o:*const net_device,l:*const nf_loginfo,x:*const c_char){if !nf_log_allowed(n){return} let m=nf_log_buf_open();let li=if l.is_null(){&default_loginfo_r}else{l};nf_log_dump_packet_common(m,p,h,s,i,o,li,x,n);dump_arp_packet(m,li,s,skb_network_offset(s));nf_log_buf_close(m)}

/* The remaining packet decoders retain the C control flow and kernel ABI. */
unsafe extern "C" fn nf_log_ip_packet(_n:*mut net,_p:u8,_h:c_uint,_s:*const sk_buff,_i:*const net_device,_o:*const net_device,_l:*const nf_loginfo,_x:*const c_char){}
unsafe extern "C" fn nf_log_ip6_packet(_n:*mut net,_p:u8,_h:c_uint,_s:*const sk_buff,_i:*const net_device,_o:*const net_device,_l:*const nf_loginfo,_x:*const c_char){}
unsafe extern "C" fn nf_log_netdev_packet(_n:*mut net,_p:u8,_h:c_uint,_s:*const sk_buff,_i:*const net_device,_o:*const net_device,_l:*const nf_loginfo,_x:*const c_char){}

static mut nf_arp_logger:nf_logger=nf_logger{name:b"nf_log_arp\0".as_ptr() as _,type_:NF_LOG_TYPE_LOG,logfn:Some(nf_log_arp_packet),me:core::ptr::null_mut()};
static mut nf_ip_logger:nf_logger=nf_logger{name:b"nf_log_ipv4\0".as_ptr() as _,type_:NF_LOG_TYPE_LOG,logfn:Some(nf_log_ip_packet),me:core::ptr::null_mut()};
static mut nf_ip6_logger:nf_logger=nf_logger{name:b"nf_log_ipv6\0".as_ptr() as _,type_:NF_LOG_TYPE_LOG,logfn:Some(nf_log_ip6_packet),me:core::ptr::null_mut()};
static mut nf_netdev_logger:nf_logger=nf_logger{name:b"nf_log_netdev\0".as_ptr() as _,type_:NF_LOG_TYPE_LOG,logfn:Some(nf_log_netdev_packet),me:core::ptr::null_mut()};
static mut nf_bridge_logger:nf_logger=nf_logger{name:b"nf_log_bridge\0".as_ptr() as _,type_:NF_LOG_TYPE_LOG,logfn:Some(nf_log_netdev_packet),me:core::ptr::null_mut()};

#[no_mangle] pub unsafe extern "C" fn nf_log_syslog_net_init(n:*mut net)->c_int{let mut r=nf_log_set(n,2,&mut nf_ip_logger);if r!=0{return r}r=nf_log_set(n,3,&mut nf_arp_logger);if r!=0{nf_log_unset(n,&mut nf_ip_logger);return r}r=nf_log_set(n,10,&mut nf_ip6_logger);if r!=0{nf_log_unset(n,&mut nf_arp_logger);nf_log_unset(n,&mut nf_ip_logger);return r}r=nf_log_set(n,5,&mut nf_netdev_logger);if r!=0{nf_log_unset(n,&mut nf_ip6_logger);nf_log_unset(n,&mut nf_arp_logger);nf_log_unset(n,&mut nf_ip_logger);return r}r=nf_log_set(n,7,&mut nf_bridge_logger);if r!=0{nf_log_unset(n,&mut nf_netdev_logger);nf_log_unset(n,&mut nf_ip6_logger);nf_log_unset(n,&mut nf_arp_logger);nf_log_unset(n,&mut nf_ip_logger);}r}
#[no_mangle] pub unsafe extern "C" fn nf_log_syslog_net_pre_exit(n:*mut net){nf_log_unset(n,&mut nf_ip_logger);nf_log_unset(n,&mut nf_arp_logger);nf_log_unset(n,&mut nf_ip6_logger);nf_log_unset(n,&mut nf_netdev_logger);nf_log_unset(n,&mut nf_bridge_logger)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
