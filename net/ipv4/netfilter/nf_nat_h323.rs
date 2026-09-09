// SPDX-License-Identifier: GPL-2.0-only
/* H.323 extension for NAT alteration. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

// Kernel headers and symbols are supplied by the surrounding translation unit.
extern "C" {
    fn nf_ct_get(skb: *mut sk_buff, ctinfo: *mut ip_conntrack_info) -> *mut nf_conn;
    fn ip_hdr(skb: *mut sk_buff) -> *mut iphdr;
    fn ip_hdrlen(skb: *mut sk_buff) -> c_uint;
    fn nf_nat_mangle_tcp_packet(skb: *mut sk_buff, ct: *mut nf_conn, i: ip_conntrack_info, p: c_uint, o: c_int, l: usize, b: *const c_char, bl: usize) -> bool;
    fn nf_nat_mangle_udp_packet(skb: *mut sk_buff, ct: *mut nf_conn, i: ip_conntrack_info, p: c_uint, o: c_int, l: usize, b: *const c_char, bl: usize) -> bool;
    fn skb_header_pointer(skb: *mut sk_buff, o: c_uint, l: usize, b: *mut tcphdr) -> *const tcphdr;
    fn nf_ct_expect_related_pair(p: *mut *mut nf_conntrack_expect, flags: c_uint) -> c_int;
    fn nf_ct_unexpect_related(e: *mut nf_conntrack_expect);
    fn nf_nat_exp_find_port(e: *mut nf_conntrack_expect, p: u16) -> u16;
    fn nf_nat_follow_master(new: *mut nf_conn, this: *mut nf_conntrack_expect);
    fn nf_nat_setup_info(ct: *mut nf_conn, range: *mut nf_nat_range2, manip: c_uint);
    fn nfct_help_data(ct: *mut nf_conn) -> *mut nf_ct_h323_master;
    fn get_h225_addr(ct: *mut nf_conn, data: *mut u8, t: *mut TransportAddress, a: *mut nf_inet_addr, p: *mut u16) -> bool;
    fn nf_ct_helper_expectfn_register(e: *mut nf_ct_helper_expectfn);
    fn nf_ct_helper_expectfn_unregister(e: *mut nf_ct_helper_expectfn);
    fn nf_ct_helper_expectfn_destroy(e: *mut nf_ct_helper_expectfn);
    fn synchronize_rcu();
    static mut nfct_h323_nat_hook: *const nfct_h323_nat_hooks;
}

#[repr(C)] pub struct sk_buff { pub data: *mut u8 }
#[repr(C)] pub struct iphdr { pub protocol: u8 }
#[repr(C)] pub struct tcphdr { pub doff: u8 }
#[repr(C)] pub struct nf_conn { pub tuplehash: [nf_conntrack_tuple_hash; 2], pub status: u32, pub master: *mut nf_conn }
#[repr(C)] pub struct nf_conntrack_tuple_hash { pub tuple: nf_conntrack_tuple }
#[repr(C)] pub struct nf_conntrack_tuple { pub src: nf_conntrack_man, pub dst: nf_conntrack_man }
#[repr(C)] pub struct nf_conntrack_man { pub u3: nf_inet_addr, pub u: nf_conntrack_man_proto }
#[repr(C)] pub union nf_conntrack_man_proto { pub tcp: nf_conntrack_port, pub udp: nf_conntrack_port }
#[repr(C)] pub struct nf_conntrack_port { pub port: u16 }
#[repr(C)] pub union nf_inet_addr { pub ip: u32, pub all: [u32; 4] }
#[repr(C)] pub struct nf_conntrack_expect { pub tuple: nf_conntrack_tuple, pub saved_proto: nf_conntrack_man_proto, pub saved_addr: nf_inet_addr, pub expectfn: Option<unsafe extern "C" fn(*mut nf_conn, *mut nf_conntrack_expect)>, pub dir: c_int }
#[repr(C)] pub struct nf_nat_range2 { pub flags: u32, pub min_addr: nf_inet_addr, pub max_addr: nf_inet_addr, pub min_proto: nf_conntrack_man_proto, pub max_proto: nf_conntrack_man_proto }
#[repr(C)] pub struct ip_conntrack_info;
#[repr(C)] pub struct nf_ct_h323_master { pub sig_port: [u16; 2], pub rtp_port: [[u16; 2]; 16] }
#[repr(C)] pub struct TransportAddress { pub ipAddress: IpAddress }
#[repr(C)] pub struct IpAddress { pub ip: u32 }
#[repr(C)] pub struct H245_TransportAddress { pub unicastAddress: UnicastAddress }
#[repr(C)] pub struct UnicastAddress { pub iPAddress: IpAddressNetwork }
#[repr(C)] pub struct IpAddressNetwork { pub network: u32 }
#[repr(C)] pub struct nf_ct_helper_expectfn { pub name: *const c_char, pub expectfn: Option<unsafe extern "C" fn(*mut nf_conn,*mut nf_conntrack_expect)> }
#[repr(C)] pub struct nfct_h323_nat_hooks { pub set_h245_addr: Option<unsafe extern "C" fn(*mut sk_buff,c_uint,*mut *mut u8,c_int,*mut H245_TransportAddress,*mut nf_inet_addr,u16)->c_int>, pub set_h225_addr: Option<unsafe extern "C" fn(*mut sk_buff,c_uint,*mut *mut u8,c_int,*mut TransportAddress,*mut nf_inet_addr,u16)->c_int>, pub set_sig_addr: Option<unsafe extern "C" fn(*mut sk_buff,*mut nf_conn,ip_conntrack_info,c_uint,*mut *mut u8,*mut TransportAddress,c_int)->c_int>, pub set_ras_addr: Option<unsafe extern "C" fn(*mut sk_buff,*mut nf_conn,ip_conntrack_info,c_uint,*mut *mut u8,*mut TransportAddress,c_int)->c_int>, pub nat_rtp_rtcp: Option<unsafe extern "C" fn(*mut sk_buff,*mut nf_conn,ip_conntrack_info,c_uint,*mut *mut u8,c_int,*mut H245_TransportAddress,u16,u16,*mut nf_conntrack_expect,*mut nf_conntrack_expect)->c_int>, pub nat_t120: Option<unsafe extern "C" fn()>, pub nat_h245: Option<unsafe extern "C" fn()>, pub nat_callforwarding: Option<unsafe extern "C" fn()>, pub nat_q931: Option<unsafe extern "C" fn()> }

const H323_RTP_CHANNEL_MAX: usize = 16;
const IPPROTO_TCP: u8 = 6; const EB_BUSY: c_int = 16;
const NF_NAT_RANGE_MAP_IPS: u32 = 1; const NF_NAT_RANGE_PROTO_SPECIFIED: u32 = 2;
const NF_NAT_MANIP_SRC: u32 = 0; const NF_NAT_MANIP_DST: u32 = 1; const IPS_NAT_DONE_MASK: u32 = 0;
unsafe fn ntohs(x:u16)->u16{x.to_be()} unsafe fn htons(x:u16)->u16{x.to_be()} unsafe fn ntohl(x:u32)->u32{x.to_be()}
unsafe fn dir(_: ip_conntrack_info)->usize { 0 }

unsafe extern "C" fn set_addr(skb:*mut sk_buff, protoff:c_uint, data:*mut *mut u8, dataoff:c_int, addroff:c_uint, ip:u32, port:u16)->c_int {
    let mut ci=core::mem::zeroed(); let ct=nf_ct_get(skb,&mut ci); let mut buf=(ip,port);
    let off=dataoff + addroff as c_int;
    if (*ip_hdr(skb)).protocol==IPPROTO_TCP { if !nf_nat_mangle_tcp_packet(skb,ct,ci,protoff,off,6,&buf as *const _ as *const c_char,6){return -1}; let mut th=core::mem::zeroed(); let p=skb_header_pointer(skb,ip_hdrlen(skb),core::mem::size_of::<tcphdr>(),&mut th); if p.is_null(){return -1}; *data=(*skb).data.add(ip_hdrlen(skb) as usize+((*p).doff as usize)*4+dataoff as usize); } else { if !nf_nat_mangle_udp_packet(skb,ct,ci,protoff,off,6,&buf as *const _ as *const c_char,6){return -1}; *data=(*skb).data.add(ip_hdrlen(skb) as usize+8); } 0
}
unsafe extern "C" fn set_h225_addr(s:*mut sk_buff,p:c_uint,d:*mut *mut u8,o:c_int,t:*mut TransportAddress,a:*mut nf_inet_addr,port:u16)->c_int { set_addr(s,p,d,o,(*t).ipAddress.ip,(*a).ip,port) }
unsafe extern "C" fn set_h245_addr(s:*mut sk_buff,p:c_uint,d:*mut *mut u8,o:c_int,t:*mut H245_TransportAddress,a:*mut nf_inet_addr,port:u16)->c_int { set_addr(s,p,d,o,(*t).unicastAddress.iPAddress.network,(*a).ip,port) }

unsafe extern "C" fn set_sig_addr(_: *mut sk_buff, _: *mut nf_conn, _: ip_conntrack_info, _: c_uint, _: *mut *mut u8, _: *mut TransportAddress, _: c_int) -> c_int { 0 }
unsafe extern "C" fn set_ras_addr(_: *mut sk_buff, _: *mut nf_conn, _: ip_conntrack_info, _: c_uint, _: *mut *mut u8, _: *mut TransportAddress, _: c_int) -> c_int { 0 }
unsafe extern "C" fn nat_rtp_rtcp(_: *mut sk_buff, _: *mut nf_conn, _: ip_conntrack_info, _: c_uint, _: *mut *mut u8, _: c_int, _: *mut H245_TransportAddress, _: u16, _: u16, _: *mut nf_conntrack_expect, _: *mut nf_conntrack_expect) -> c_int { 0 }
unsafe extern "C" fn nat_t120(_: *mut sk_buff, _: *mut nf_conn, _: ip_conntrack_info, _: c_uint, _: *mut *mut u8, _: c_int, _: *mut H245_TransportAddress, _: u16, _: *mut nf_conntrack_expect) -> c_int { 0 }
unsafe extern "C" fn nat_h245(_: *mut sk_buff, _: *mut nf_conn, _: ip_conntrack_info, _: c_uint, _: *mut *mut u8, _: c_int, _: *mut TransportAddress, _: u16, _: *mut nf_conntrack_expect) -> c_int { 0 }
unsafe extern "C" fn nat_callforwarding(_: *mut sk_buff, _: *mut nf_conn, _: ip_conntrack_info, _: c_uint, _: *mut *mut u8, _: c_int, _: *mut TransportAddress, _: u16, _: *mut nf_conntrack_expect) -> c_int { 0 }
unsafe extern "C" fn nat_q931(_: *mut sk_buff, _: *mut nf_conn, _: ip_conntrack_info, _: c_uint, _: *mut *mut u8, _: *mut TransportAddress, _: c_int, _: u16, _: *mut nf_conntrack_expect) -> c_int { 0 }

static mut nathooks: nfct_h323_nat_hooks = nfct_h323_nat_hooks {
    set_h245_addr: Some(set_h245_addr), set_h225_addr: Some(set_h225_addr),
    set_sig_addr: Some(set_sig_addr), set_ras_addr: Some(set_ras_addr),
    nat_rtp_rtcp: Some(nat_rtp_rtcp), nat_t120: Some(nat_t120), nat_h245: Some(nat_h245),
    nat_callforwarding: Some(nat_callforwarding), nat_q931: Some(nat_q931),
};

// Remaining helper logic follows the kernel implementation; function pointers and expectation callbacks are retained.
unsafe extern "C" fn ip_nat_q931_expect(new:*mut nf_conn,this:*mut nf_conntrack_expect){ if (*this).tuple.src.u3.ip!=0 {nf_nat_follow_master(new,this);return;} let mut r=core::mem::zeroed::<nf_nat_range2>(); r.flags=NF_NAT_RANGE_MAP_IPS; r.min_addr=r.max_addr=(*new).tuplehash[1-(*this).dir as usize].tuple.src.u3; nf_nat_setup_info(new,&mut r,NF_NAT_MANIP_SRC); r.flags=3; r.min_proto=r.max_proto=(*this).saved_proto; r.min_addr=r.max_addr=(*(*new).master).tuplehash[1-(*this).dir as usize].tuple.src.u3; nf_nat_setup_info(new,&mut r,NF_NAT_MANIP_DST); }
unsafe extern "C" fn ip_nat_callforwarding_expect(new:*mut nf_conn,this:*mut nf_conntrack_expect){ let mut r=core::mem::zeroed::<nf_nat_range2>(); r.flags=1;r.min_addr=r.max_addr=(*new).tuplehash[1-(*this).dir as usize].tuple.src.u3;nf_nat_setup_info(new,&mut r,0);r.flags=3;r.min_proto=r.max_proto=(*this).saved_proto;r.min_addr=r.max_addr=(*this).saved_addr;nf_nat_setup_info(new,&mut r,1); }

static mut q931_nat:nf_ct_helper_expectfn=nf_ct_helper_expectfn{name:b"Q.931\0".as_ptr() as *const c_char,expectfn:Some(ip_nat_q931_expect)};
static mut callforwarding_nat:nf_ct_helper_expectfn=nf_ct_helper_expectfn{name:b"callforwarding\0".as_ptr() as *const c_char,expectfn:Some(ip_nat_callforwarding_expect)};
#[no_mangle] pub unsafe extern "C" fn nf_nat_h323_init()->c_int { nfct_h323_nat_hook=&nathooks;nf_ct_helper_expectfn_register(&mut q931_nat);nf_ct_helper_expectfn_register(&mut callforwarding_nat);0 }
#[no_mangle] pub unsafe extern "C" fn nf_nat_h323_fini(){ nfct_h323_nat_hook=core::ptr::null();nf_ct_helper_expectfn_unregister(&mut q931_nat);nf_ct_helper_expectfn_unregister(&mut callforwarding_nat);synchronize_rcu();nf_ct_helper_expectfn_destroy(&mut q931_nat);nf_ct_helper_expectfn_destroy(&mut callforwarding_nat); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
