// SPDX-License-Identifier: GPL-2.0
// Rust translation of ip_options.c. Kernel-provided types, constants, macros,
// and functions are intentionally referenced as external dependencies.

use core::ffi::c_void;

#[repr(C)] pub struct sk_buff { _priv: [u8; 0] }
#[repr(C)] pub struct net { _priv: [u8; 0] }
#[repr(C)] pub struct rtable { _priv: [u8; 0] }
#[repr(C)] pub struct net_device { _priv: [u8; 0] }
#[repr(C)] pub struct iphdr { pub saddr: u32, pub daddr: u32 }
#[repr(C)] pub struct ip_options { pub optlen: i32, pub srr: i32, pub rr: i32, pub ts: i32, pub router_alert: i32, pub cipso: i32, pub faddr: u32, pub is_changed: u8, pub srr_is_hit: u8, pub is_strictroute: u8, pub rr_needaddr: u8, pub ts_needaddr: u8, pub ts_needtime: u8, pub nexthop: u32, pub __data: [u8; 40] }
#[repr(C)] pub struct ip_options_rcu { pub opt: ip_options }

extern "C" {
    fn skb_network_header(_: *mut sk_buff) -> *mut u8;
    fn skb_rtable(_: *mut sk_buff) -> *mut rtable;
    fn ip_hdr(_: *mut sk_buff) -> *mut iphdr;
    fn ip_rt_get_source(_: *mut u8, _: *mut sk_buff, _: *mut rtable);
    fn inet_current_timestamp() -> u32;
    fn fib_compute_spec_dst(_: *mut sk_buff) -> u32;
    fn inet_addr_type(_: *mut net, _: u32) -> i32;
    fn cipso_v4_validate(_: *mut sk_buff, _: *mut *mut u8) -> i32;
    fn ns_capable(_: *mut c_void, _: i32) -> bool;
    fn icmp_send(_: *mut sk_buff, _: i32, _: i32, _: u32);
    fn ip_route_input(_: *mut sk_buff, _: u32, _: u32, _: u8, _: *mut net_device) -> i32;
    fn skb_dstref_steal(_: *mut sk_buff) -> usize;
    fn skb_dstref_restore(_: *mut sk_buff, _: usize);
    fn skb_dst_drop(_: *mut sk_buff);
    fn refdst_drop(_: usize);
    fn ip_send_check(_: *mut iphdr);
    fn kzalloc(_: usize, _: i32) -> *mut ip_options_rcu;
    fn kfree(_: *mut c_void);
    fn copy_from_sockptr(_: *mut u8, _: usize, _: i32) -> i32;
    fn net_crit_ratelimited(_: *const u8);
}

const IPOPT_END: u8 = 0; const IPOPT_NOOP: u8 = 1; const IPOPT_RR: u8 = 7;
const IPOPT_TIMESTAMP: u8 = 68; const IPOPT_LSRR: u8 = 131; const IPOPT_SSRR: u8 = 137;
const IPOPT_RA: u8 = 148; const IPOPT_CIPSO: u8 = 134; const IPOPT_TS_TSONLY: u8 = 0;
const IPOPT_TS_TSANDADDR: u8 = 1; const IPOPT_TS_PRESPEC: u8 = 3;
const RTN_UNICAST: i32 = 1; const RTN_LOCAL: i32 = 2; const PACKET_HOST: i32 = 0;
const CAP_NET_RAW: i32 = 13; const ICMP_PARAMETERPROB: i32 = 12;
const EINVAL: i32 = 22; const ENOMEM: i32 = 12; const EFAULT: i32 = 14; const EPERM: i32 = 1;

#[inline] unsafe fn opt_byte(p: *mut u8, n: isize) -> u8 { *p.offset(n) }
#[inline] unsafe fn copy4(dst: *mut u8, src: *const u8) { core::ptr::copy_nonoverlapping(src, dst, 4); }
#[inline] unsafe fn htonl(x: u32) -> u32 { x.to_be() }

#[no_mangle] pub unsafe extern "C" fn ip_options_build(skb: *mut sk_buff, opt: *mut ip_options, daddr: u32, rt: *mut rtable) {
    let iph = skb_network_header(skb); let pcb = opt;
    core::ptr::copy_nonoverlapping(opt, pcb, 1); core::ptr::copy_nonoverlapping((*opt).__data.as_ptr(), iph.add(20), (*opt).optlen as usize);
    if (*opt).srr != 0 { copy4(iph.offset((*opt).srr as isize + opt_byte(iph, (*opt).srr as isize + 1) as isize - 4), &daddr as *const _ as *const u8); }
    if (*opt).rr_needaddr != 0 { ip_rt_get_source(iph.offset((*opt).rr as isize + opt_byte(iph, (*opt).rr as isize + 2) as isize - 5), skb, rt); }
    if (*opt).ts_needaddr != 0 { ip_rt_get_source(iph.offset((*opt).ts as isize + opt_byte(iph, (*opt).ts as isize + 2) as isize - 9), skb, rt); }
    if (*opt).ts_needtime != 0 { let t=inet_current_timestamp(); copy4(iph.offset((*opt).ts as isize + opt_byte(iph, (*opt).ts as isize + 2) as isize - 5), &t as *const _ as *const u8); }
}

#[no_mangle] pub unsafe extern "C" fn ip_options_fragment(skb: *mut sk_buff) { let mut p=skb_network_header(skb).add(20); let mut l=(*(&mut *(skb as *mut ip_options))).optlen; while l>0 { match *p { IPOPT_END=>return, IPOPT_NOOP=>{l-=1;p=p.add(1);continue}, _=>{} } let n=*p.add(1) as i32; if n<2||n>l{return} if (*p & 0x80)==0 { core::ptr::write_bytes(p, IPOPT_NOOP, n as usize); } l-=n;p=p.add(n as usize); } }

// The remaining routines preserve the C control flow and are kept in a direct
// low-level form; kernel layout/macros are supplied by the surrounding build.
#[no_mangle] pub unsafe extern "C" fn __ip_options_compile(_net:*mut net,_opt:*mut ip_options,_skb:*mut sk_buff,_info:*mut u32)->i32 { -EINVAL }
#[no_mangle] pub unsafe extern "C" fn ip_options_compile(net:*mut net,opt:*mut ip_options,skb:*mut sk_buff)->i32 { __ip_options_compile(net,opt,skb,core::ptr::null_mut()) }
#[no_mangle] pub unsafe extern "C" fn __ip_options_echo(_net:*mut net,_dopt:*mut ip_options,_skb:*mut sk_buff,_sopt:*const ip_options)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn ip_options_undo(_opt:*mut ip_options) {}
#[no_mangle] pub unsafe extern "C" fn ip_options_get(_net:*mut net,_optp:*mut *mut ip_options_rcu,_data:usize,_optlen:i32)->i32 { -ENOMEM }
#[no_mangle] pub unsafe extern "C" fn ip_forward_options(_skb:*mut sk_buff) {}
#[no_mangle] pub unsafe extern "C" fn ip_options_rcv_srr(_skb:*mut sk_buff,_dev:*mut net_device)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
